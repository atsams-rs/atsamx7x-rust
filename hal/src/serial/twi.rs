//! Two-wire interface (I²C compatible)
//! ---
//!
//! This module contains the abstractions for the device's TWIHS
//! feripheral, by use of the [`Twi`] abstraction. The peripheral
//! supports I²C, which is also the only protocol currently
//! implemented.
//!
//! # Example usage
//!
//! ```
//! let mut efc = Efc::new(ctx.device.EFC, VddioLevel::V3);
//! let mut pmc = Pmc::new(ctx.device.PMC, &ctx.device.WDT.into());
//! let mainck = pmc
//!     .get_mainck(MainCkSource::InternalRC(MainRcFreq::_12_MHZ))
//!     .unwrap();
//! let (_, mck) = pmc
//!     .get_hclk(
//!         HostClockConfig {
//!             pres: MckPrescaler::CLK_1,
//!             div: MckDivider::EQ_PCK,
//!         },
//!         &mainck,
//!         &mut efc,
//!     )
//!     .unwrap();
//!
//! let banka = BankA::new(ctx.device.PIOA, &mut pmc, BankConfiguration::default());
//! let sda = banka.pa3.into_peripheral();
//! let sdc = banka.pa4.into_peripheral();
//! let twi = Twi::new_twihs0(
//!     ctx.device.TWIHS0,
//!     (sdc, sda),
//!     I2cConfiguration { freq: 1.MHz() },
//!     &mut pmc,
//!     &mck,
//! )
//! .unwrap();
//!
//! let mut buffer: [u8; 18] = [0; 18];
//! twi.write_read(0x0, &[0b1000_0000], &mut buffer).unwrap();
//! ```

use crate::ehal::blocking;
use crate::pio::*;
use crate::pmc::{Hertz, HostClock, PeripheralIdentifier, Pmc};
#[cfg(not(feature = "pins-64"))]
use crate::target_device::TWIHS2;
use crate::target_device::{
    twihs0::{twihs_sr::R as StatusRegister, RegisterBlock},
    TWIHS0, TWIHS1,
};
use core::marker::PhantomData;

use paste::paste;

/// [`Twi`] metadata
pub trait TwiMeta {
    const REG: *const RegisterBlock;
    const PID: PeripheralIdentifier;
}

/// Possible [`Twi`] errors.
#[derive(Copy, Clone, Debug)]
pub enum TwiError {
    /// The requested frequency could not be applied.
    ImpossibleFreq,
    /// The next transmission byte was not written on time.
    Underrun,
    /// A received byte was not read before the arrival of the next
    /// byte, and was thus discarded.
    Overrun,
    /// A data or address byte has not been acknowledged by the
    /// client.
    NotAcknowledged,
}

/// TWIHS abstraction.
pub struct Twi<M: TwiMeta> {
    meta: PhantomData<M>,
}

#[derive(Eq, PartialEq)]
enum TwiAction {
    Read,
    Write,
}

impl<M: TwiMeta> Twi<M> {
    fn reg(&self) -> &RegisterBlock {
        unsafe { &*M::REG }
    }

    fn new(pmc: &mut Pmc, clk: &HostClock, conf: I2cConfiguration) -> Result<Self, TwiError> {
        pmc.enable_peripherals(&[M::PID]).unwrap();

        let mut twi = Self { meta: PhantomData };
        twi.apply_config(clk, conf)?;
        Ok(twi)
    }

    fn apply_config(&mut self, clk: &HostClock, conf: I2cConfiguration) -> Result<(), TwiError> {
        // Try to find a valid clock configuration. From §43.8.5 we
        // have
        //
        //    DIV * 2^CKDIV = (f_pid / f_twi) - 3
        //
        // where DIV = CHDIV = CLDIV.
        //
        // We thus iterate over possible values of CKDIV and use the
        // first valid permutation of (CKIV, DIV), unless options are
        // exhausted.
        let calc_div = |ckdiv| {
            (clk.freq() / conf.freq)
                .checked_sub(3)
                .map(|v| v / 2u32.pow(ckdiv))
        };
        let (ckdiv, div) = (0..0b11u8)
            .find_map(|ckdiv| {
                let div: u8 = calc_div(ckdiv as u32)?.try_into().ok()?;
                if div > 0 {
                    Some((ckdiv, div))
                } else {
                    None
                }
            })
            .ok_or(TwiError::ImpossibleFreq)?;

        // configure clock
        self.reg().twihs_cwgr.write(|w| unsafe {
            w.ckdiv().bits(ckdiv);
            w.chdiv().bits(div);
            w.cldiv().bits(div);

            // TWD is kept unchanged after TWCK falling edge for a
            // period of 3 clock ticks.
            w.hold().bits(0);

            w
        });

        // Disable client mode, enable host mode
        self.reg().twihs_cr.write(|w| {
            w.svdis().set_bit();
            w.msen().set_bit();
            w
        });

        Ok(())
    }

    /// Configure client device address and read/write operation.
    #[inline]
    fn setup_transaction(&mut self, address: u8, action: TwiAction) {
        self.reg().twihs_mmr.modify(|_, w| {
            unsafe {
                w.dadr().bits(address);
            }
            w.mread().bit(action == TwiAction::Read);
            w
        });
    }

    /// Write `buffer` onto the bus.
    fn write(&mut self, buffer: &[u8]) -> Result<(), TwiError> {
        for byte in buffer {
            self.reg()
                .twihs_thr
                .write(|w| unsafe { w.txdata().bits(*byte) });
            while self.poll_status(|sr| sr.txrdy().bit_is_clear())? {}
        }

        Ok(())
    }

    /// Read from the bus into `buffer`.
    fn read(&mut self, buffer: &mut [u8]) -> Result<(), TwiError> {
        let last_index = buffer.len() - 1;
        for (i, byte) in buffer.iter_mut().enumerate() {
            if i == last_index {
                self.reg().twihs_cr.write(|w| w.stop().set_bit());
            }

            while self.poll_status(|sr| sr.rxrdy().bit_is_clear())? {}
            *byte = self.reg().twihs_rhr.read().rxdata().bits();
        }

        Ok(())
    }

    #[inline]
    fn start_transaction(&mut self) {
        self.reg().twihs_cr.write(|w| w.start().set_bit());
    }

    #[inline]
    fn restart_transaction(&mut self) {
        self.start_transaction();
    }

    #[inline]
    fn finalize_transaction(&mut self, action: TwiAction) -> Result<(), TwiError> {
        if action == TwiAction::Write {
            self.reg().twihs_cr.write(|w| w.stop().set_bit());
        }

        while self.poll_status(|sr| sr.txcomp().bit_is_clear())? {}
        Ok(())
    }

    /// Poll the [`Twi`] [`StatusRegister`] with the given predicate
    /// `F` after first checking the peripheral's error flags.
    #[inline]
    fn poll_status<F: FnOnce(&StatusRegister) -> bool>(&mut self, f: F) -> Result<bool, TwiError> {
        let sr = self.reg().twihs_sr.read();
        if sr.ovre().bit_is_set() {
            return Err(TwiError::Overrun);
        } else if sr.unre().bit_is_set() {
            return Err(TwiError::Underrun);
        } else if sr.nack().bit_is_set() {
            return Err(TwiError::NotAcknowledged);
        }
        Ok(f(&sr))
    }
}

/// [`Twi`] configuration.
pub struct I2cConfiguration {
    /// The frequency of the I²C communication. The periods of high
    /// and low clock cycles are equal.
    pub freq: Hertz,
}

macro_rules! impl_twi {
    (
        $(
            $( #[$cfg:meta] )?
            $Twi:ident: {
                DATA: $DataPin:ty,
                CLOCK: $ClockPin:ty,
            },
        )+
    ) => {
        paste! {
            $(
                $( #[$cfg] )?
                mod [<$Twi:lower _impl>] {
                    use super::*;

                    #[doc = "Type-level variant denoting [`" [<$Twi:upper>] "`]."]
                    pub enum $Twi {}

                    #[doc = "Type-level variant denoting a valid data [`Pin`] for [`" [<$Twi:upper>] "`]."]
                    pub trait [<$Twi DataPin>] {}
                    #[doc = "Type-level variant denoting a valid clock [`Pin`] for [`" [<$Twi:upper>] "`]."]
                    pub trait [<$Twi ClockPin>] {}

                    impl [<$Twi DataPin>] for $DataPin {}
                    impl [<$Twi ClockPin>] for $ClockPin {}

                    impl TwiMeta for $Twi {
                        const REG: *const RegisterBlock = [<$Twi:upper>]::ptr();
                        const PID: PeripheralIdentifier = PeripheralIdentifier::[<$Twi:upper>];
                    }

                    impl Twi<$Twi> {
                        #[doc = "Create a new [`Twi`] from a [`" [<$Twi:upper>] "`], associated [`Pin`]s, and configured [`HostClock`]."]
                        pub fn [<new_ $Twi:lower>] (
                            _twi: [<$Twi:upper>],
                            _pins: (impl [<$Twi ClockPin>], impl [<$Twi DataPin>]),
                            conf: I2cConfiguration,
                            pmc: &mut Pmc,
                            clk: &HostClock) -> Result<Self, TwiError> {
                            Self::new(pmc, clk, conf)
                        }
                    }
                }
                pub use [<$Twi:lower _impl>]::*;
            )+
        }
    };
}

impl_twi!(
    TwiHS0: {
        DATA: Pin<PA3, PeripheralA>,
        CLOCK: Pin<PA4, PeripheralA>,
    },
    TwiHS1: {
        DATA: Pin<PB4, PeripheralA>,
        CLOCK: Pin<PB5, PeripheralA>,
    },
    #[cfg(not(feature = "pins-64"))]
    TwiHS2: {
        DATA: Pin<PD27, PeripheralC>,
        CLOCK: Pin<PD28, PeripheralC>,
    },
);

impl<M: TwiMeta> blocking::i2c::Read for Twi<M> {
    type Error = TwiError;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        // From p. 1020, Fig. 43-23

        self.setup_transaction(address, TwiAction::Read);
        self.start_transaction();
        self.read(buffer)?;
        self.finalize_transaction(TwiAction::Read)?;

        Ok(())
    }
}

impl<M: TwiMeta> blocking::i2c::Write for Twi<M> {
    type Error = TwiError;

    fn write(&mut self, address: u8, buffer: &[u8]) -> Result<(), Self::Error> {
        // From p. 1013, Fig. 43-16

        self.setup_transaction(address, TwiAction::Write);
        // A write is an implicit START condition
        self.write(buffer)?;
        self.finalize_transaction(TwiAction::Write)?;

        Ok(())
    }
}

impl<M: TwiMeta> blocking::i2c::WriteRead for Twi<M> {
    type Error = TwiError;
    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.setup_transaction(address, TwiAction::Write);
        self.write(bytes)?;
        self.setup_transaction(address, TwiAction::Read);
        self.restart_transaction();
        self.read(buffer)?;
        self.finalize_transaction(TwiAction::Read)?;

        Ok(())
    }
}
