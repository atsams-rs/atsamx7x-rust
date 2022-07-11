//! Serial Communication (UART)
//! ---
//!
//! This module contains the abstractions for the device's UART
//! peripheral, by use of the [`Uart`] abstraction. The UART only
//! supports 8-bit words with a single stop bit, but supports five
//! parity options:
//!
//! - [`ParityMode::Even`],
//! - [`ParityMode::Odd`],
//! - [`ParityMode::Space`],
//! - [`ParityMode::Mark`], or
//! - [`ParityMode::None`],
//!
//! # Example usage
//!
//! ```no_run
//! use crate::{pio::*, pmc::*, serial::uart::*};
//!
//! let ctx = crate::target_device::Peripherals::take().unwrap();
//! let mut pmc = hal::pmc::Pmc::new(ctx.device.PMC, &ctx.device.WDT.into());
//! // Get main clock
//! let mainck = pmc
//!     .get_mainck(MainCkSource::ExternalBypass(Megahertz::from_raw(12)))
//!     .unwrap();
//! let upllck = pmc.get_upllck(&mainck, &mut ctx.device.UTMI).unwrap();
//! let upllckdiv = pmc.get_upllckdiv(&upllck, UpllDivider::Div2);
//! let pck: Pck<Pck4> = pmc.get_pck(&upllckdiv, 100 - 1);
//!
//! let banka = BankA::new(ctx.device.PIOA, &mut pmc, BankConfiguration::default());
//!
//! let tx = banka.pa10.into_peripheral::<A>();
//! let rx = banka.pa9.into_peripheral::<A>();
//! let mut uart = Uart::new_uart0(
//!     ctx.device.UART0,
//!     (tx, rx),
//!     UartConfiguration::params_8n1(115_200.bps()).mode(ChannelMode::LOCAL_LOOPBACK),
//!     &mut pmc,
//!     &pck,
//! )
//! .unwrap();
//! uart.listen(Event::RxReady);
//!
//! uart.bwrite(0xff).unwrap();
//! assert_eq!(uart.read().unwrap(), 0xff);
//! ```

use crate::ehal::{self, blocking};
use crate::pio::*;
use crate::pmc::{Hertz, HostClock, Pck, Pck4, PeripheralIdentifier, Pmc};
use crate::serial::Bps;
use crate::target_device::uart0::uart_mr::{
    CHMODE_A as ChannelModeInner, PAR_A as ParityModeInner,
};
use crate::target_device::{uart0::RegisterBlock, UART0, UART1, UART2};
#[cfg(all(
    any(feature = "e70", feature = "s70", feature = "v71"),
    any(feature = "pins-100", feature = "pins-144")
))]
use crate::target_device::{UART3, UART4};

use core::marker::PhantomData;

use nb;
use paste::paste;

/// Available parity modes
pub enum ParityMode {
    Even,
    Odd,
    Space,
    Mark,
    None,
}

impl From<ParityMode> for ParityModeInner {
    fn from(m: ParityMode) -> Self {
        use ParityMode::*;
        use ParityModeInner::*;
        match m {
            Even => EVEN,
            Odd => ODD,
            Space => SPACE,
            Mark => MARK,
            None => NO,
        }
    }
}

/// Available channel modes
pub enum ChannelMode {
    /// Normal operation.
    Normal,
    /// Bit-by-bit retransmission echo of any received words. Words
    /// written by software are dropped, but received words are
    /// available in hardware.
    Automatic,
    /// Transmit and receive line are internally connected. Words
    /// written by software are echoed back, but never leave the
    /// device.
    LocalLoopback,
    /// Same as [`ChannelMode::Normal`], but echoed words are not
    /// received by the hardware.
    RemoteLoopback,
}

impl From<ChannelMode> for ChannelModeInner {
    fn from(m: ChannelMode) -> Self {
        use ChannelMode::*;
        use ChannelModeInner::*;
        match m {
            Normal => NORMAL,
            Automatic => AUTOMATIC,
            LocalLoopback => LOCAL_LOOPBACK,
            RemoteLoopback => REMOTE_LOOPBACK,
        }
    }
}

/// Metadata for a UART peripheral
pub trait UartMeta {
    const REG: *const RegisterBlock;
    const PID: PeripheralIdentifier;
}

/// [`Uart`] configuration
pub struct UartConfiguration {
    /// Baud rate that the [`Uart`] communicates at.
    pub baud_rate: Bps,
    /// Parity mode of the [`Uart`].
    pub parity: ParityMode,
    /// Whether the [`Uart`] filters the receive line using a
    /// three-sample filter (2 over 3 majority).
    pub oversample: bool,
    /// The mode of the [`Uart`].
    pub mode: ChannelMode,
}

impl UartConfiguration {
    /// Return a [`ParityMode::None`], [`ChannelMode::Normal`]
    /// configuration using the given baud rate, with oversampling
    /// enabled.
    pub fn default(baud_rate: Bps) -> Self {
        Self {
            baud_rate,
            parity: ParityMode::None,
            oversample: true,
            mode: ChannelMode::Normal,
        }
    }

    /// [`UartConfiguration::oversample`] override.
    pub fn oversample(mut self, bit: bool) -> Self {
        self.oversample = bit;
        self
    }

    /// [`UartConfiguration::mode`] override.
    pub fn mode(mut self, mode: ChannelMode) -> Self {
        self.mode = mode;
        self
    }

    /// [`UartConfiguration::parity`] override.
    pub fn parity(mut self, mode: ParityMode) -> Self {
        self.parity = mode;
        self
    }
}

#[derive(Debug, PartialEq, Clone)]
/// Possible [`Uart`] errors
pub enum UartError {
    /// An impossible baud rate was requested.
    BaudRateNotInRange,
    /// The calculated prescaler does not fit in a [`u16`].
    PrescalerOverflow,
}

/// A valid input clock for the [`Uart`].
pub trait UartClock {
    /// C.f. §47.6.2
    const BRSRCCK: bool;

    fn freq(&self) -> Hertz;
}
impl UartClock for HostClock {
    const BRSRCCK: bool = false;

    fn freq(&self) -> Hertz {
        self.freq()
    }
}
impl UartClock for Pck<Pck4> {
    const BRSRCCK: bool = true;

    fn freq(&self) -> Hertz {
        self.freq()
    }
}

/// Transmit component of a [`Uart`]
pub struct Tx<M: UartMeta> {
    meta: PhantomData<M>,
}

impl<M: UartMeta> Tx<M> {
    fn reg(&self) -> &RegisterBlock {
        unsafe { &*M::REG }
    }

    /// Listen for [`Event::TxReady`] interrupt events.
    pub fn listen_ready(&mut self) {
        self.reg().uart_ier.write(|w| w.txrdy().set_bit());
    }

    /// Listen for [`Event::TxEmpty`] interrupt events.
    pub fn listen_idle(&mut self) {
        self.reg().uart_ier.write(|w| w.txempty().set_bit());
    }

    /// Do not listen for [`Event::TxReady`] interrupt events.
    pub fn unlisten_ready(&mut self) {
        self.reg().uart_idr.write(|w| w.txrdy().set_bit());
    }

    /// Do not listen for [`Event::TxEmpty`] interrupt events.
    pub fn unlisten_idle(&mut self) {
        self.reg().uart_idr.write(|w| w.txempty().set_bit());
    }
}

/// Receive component of a [`Uart`]
pub struct Rx<M: UartMeta> {
    meta: PhantomData<M>,
}

impl<M: UartMeta> Rx<M> {
    fn reg(&self) -> &RegisterBlock {
        unsafe { &*M::REG }
    }

    /// Listen for [`Event::RxReady`] interrupt events.
    pub fn listen(&mut self) {
        self.reg().uart_ier.write(|w| w.rxrdy().set_bit());
    }

    /// Do not listen for [`Event::RxReady`] interrupt events.
    pub fn unlisten(&mut self) {
        self.reg().uart_idr.write(|w| w.rxrdy().set_bit());
    }
}

/// [`Uart`] peripheral abstraction.
pub struct Uart<M: UartMeta> {
    meta: PhantomData<M>,
    tx: Tx<M>,
    rx: Rx<M>,
}

/// Interrupt events of the [`Uart`].
pub enum Event {
    /// The next word can be sent.
    TxReady,
    /// A new word has been received.
    RxReady,
    /// All previous words have been serialized.
    TxEmpty,
}

impl<M: UartMeta> Uart<M> {
    fn reg(&self) -> &RegisterBlock {
        unsafe { &*M::REG }
    }

    fn calc_prescaler(baud_rate: Bps, clk: &impl UartClock) -> Result<u16, UartError> {
        // ensure requested baud rate is valid: not zero nor below
        // allowed minimum nor above possible maximum; c.f. §47.5.1
        const STATIC_PRESCALER: u32 = 16;
        if baud_rate.0.to_Hz() == 0
            || clk.freq() / (STATIC_PRESCALER * u16::MAX as u32) > baud_rate.0
            || clk.freq() / STATIC_PRESCALER < baud_rate.0
        {
            return Err(UartError::BaudRateNotInRange);
        }

        (clk.freq() / (STATIC_PRESCALER * baud_rate.0))
            .try_into()
            .map_err(|_| UartError::PrescalerOverflow)
    }

    fn new<C: UartClock>(
        pmc: &mut Pmc,
        clk: &C,
        conf: UartConfiguration,
    ) -> Result<Self, UartError> {
        pmc.enable_peripherals(&[M::PID]).unwrap();

        let mut uart = Self {
            meta: PhantomData,
            tx: Tx { meta: PhantomData },
            rx: Rx { meta: PhantomData },
        };
        uart.apply_config(clk, conf)?;
        Ok(uart)
    }

    fn apply_config<C: UartClock>(
        &mut self,
        clk: &C,
        conf: UartConfiguration,
    ) -> Result<(), UartError> {
        // set clock source, parity, and recv filter
        self.reg().uart_mr.modify(|_, w| {
            w.brsrcck().bit(C::BRSRCCK);
            w.filter().bit(conf.oversample);
            w.par().variant(conf.parity.into());
            w.chmode().variant(conf.mode.into());
            w
        });

        // find and configure baud rate prescaler
        let pres = Self::calc_prescaler(conf.baud_rate, clk)?;
        self.reg().uart_brgr.write(|w| unsafe { w.cd().bits(pres) });

        // finalize: enable the uart
        self.reg().uart_cr.write(|w| {
            w.rxen().set_bit();
            w.txen().set_bit();
            w
        });

        Ok(())
    }

    /// Reconfigure the [`Uart`] with a new configuration.
    ///
    /// # Errors
    ///
    /// Return [`nb::Error::WouldBlock`] if the [`Uart`] is currently
    /// transmitting data. Returns an [`UartError`] if the requested
    /// baud rate cannot be applied.
    pub fn reconfigure<C: UartClock>(
        &mut self,
        clk: &C,
        conf: UartConfiguration,
    ) -> nb::Result<(), UartError> {
        if self.reg().uart_sr.read().txempty().bit_is_clear() {
            return Err(nb::Error::WouldBlock);
        }

        // disable the uart
        self.reg().uart_cr.write(|w| {
            w.rxdis().set_bit();
            w.txdis().set_bit();
            w
        });

        self.apply_config(clk, conf).map_err(nb::Error::Other)
    }

    /// Listen to a [`Uart`] interrupt event.
    pub fn listen(&mut self, event: Event) {
        self.reg().uart_ier.write(|w| match event {
            Event::TxReady => w.txrdy().set_bit(),
            Event::RxReady => w.rxrdy().set_bit(),
            Event::TxEmpty => w.txempty().set_bit(),
        });
    }

    /// Do not listen to a [`Uart`] interrupt event.
    pub fn unlisten(&mut self, event: Event) {
        self.reg().uart_idr.write(|w| match event {
            Event::TxReady => w.txrdy().set_bit(),
            Event::RxReady => w.rxrdy().set_bit(),
            Event::TxEmpty => w.txempty().set_bit(),
        });
    }

    /// Split the [`Uart`] into two separate [`Tx`] and [`Rx`]
    /// components.
    pub fn split(self) -> (Tx<M>, Rx<M>) {
        (self.tx, self.rx)
    }
}

macro_rules! impl_uart {
    (
        $(
            $( #[$cfg1:meta] )?
            $Uart:ident: {
                RX: [ $RxPin:ty ],
                TX: [$(
                    $( #[$cfg2:meta] )?
                    $TxPin:ty,
                )+],
            },
        )+
    ) => {
        paste! {
            $(
                $( #[$cfg1] )?
                mod [<$Uart:lower _impl>] {
                    use super::*;

                    #[doc = "Type-level variant denoting [`" [<$Uart:upper>] "`]."]
                    pub enum $Uart {}

                    #[doc = "Type-level variant denoting a valid trasmit [`Pin`] for [`" [<$Uart:upper>] "`]."]
                    pub trait [<$Uart TxPin>] {}
                    #[doc = "Type-level variant denoting a valid receive [`Pin`] for [`" [<$Uart:upper>] "`]."]
                    pub trait [<$Uart RxPin>] {}

                    $(
                        $( #[$cfg2] )?
                        impl [<$Uart TxPin>] for $TxPin {}
                    )+
                    impl [<$Uart RxPin>] for $RxPin {}

                    impl UartMeta for $Uart {
                        const REG: *const RegisterBlock = [<$Uart:upper>]::ptr();
                        const PID: PeripheralIdentifier = PeripheralIdentifier::[<$Uart:upper>];
                    }

                    impl Uart<$Uart> {
                        #[doc = "Create a new [`Uart`] from a [`" [<$Uart:upper>] "`], associated [`Pin`]s, and valid [`UartClock`]."]
                        pub fn [<new_ $Uart:lower>] (
                            _uart: [<$Uart:upper>],
                            _pins: (impl [<$Uart TxPin>], impl [<$Uart RxPin>]),
                            conf: UartConfiguration,
                            pmc: &mut Pmc,
                            clk: &impl UartClock) -> Result<Self, UartError> {
                            Self::new(pmc, clk, conf)
                        }
                    }
                }
                $( #[$cfg1] )?
                pub use [<$Uart:lower _impl>]::*;
            )+
        }
    };
}

impl_uart!(
    Uart0: {
        RX: [ Pin<PA9, PeripheralA> ],
        TX: [ Pin<PA10, PeripheralA>, ],
    },
    Uart1: {
        RX: [ Pin<PA5, PeripheralC> ],
        TX: [
            Pin<PA4, PeripheralC>,
            #[cfg(feature = "pins-144")]
            Pin<PA6, PeripheralC>,
            Pin<PD26, PeripheralD>,
        ],
    },
    Uart2: {
        RX: [ Pin<PD25, PeripheralC> ],
        TX: [ Pin<PD26, PeripheralC>, ],
    },
    #[cfg(all(
        any(feature = "e70", feature = "s70", feature = "v71"),
        any(feature = "pins-100", feature = "pins-144")
    ))]
    Uart3: {
        RX: [ Pin<PD28, PeripheralA> ],
        TX: [ Pin<PD30, PeripheralA>, Pin<PD31, PeripheralB>, ],
    },
    #[cfg(all(
        any(feature = "e70", feature = "s70", feature = "v71"),
        any(feature = "pins-100", feature = "pins-144")
    ))]
    Uart4: {
        RX: [ Pin<PD18, PeripheralC> ],
        TX: [ Pin<PD3, PeripheralC>, Pin<PD19, PeripheralC>, ],
    },
);

#[derive(Debug)]
/// Possible hardware errors when reading a word from the [`Uart`] peripheral.
pub enum UartReadError {
    /// The frame's stop bit was read as zero.
    Framing,
    /// Parity check of the received frame failed.
    Parity,
    /// A word was received before the previous word was read.
    Overrun,
}

macro_rules! impl_read {
    ($($T:ty,)+) => {
        $(
            impl<M: UartMeta> ehal::serial::Read<u8> for $T {
                type Error = UartReadError;

                /// Reads a single word from the [`Uart`] peripheral.
                ///
                /// # Errors
                ///
                /// The read will fail with [`nb::Error::WouldBlock`] if a word
                /// has yet to be received, and will report any read [`UartReadError`].
                ///
                /// Because it is not possible to acknowledge/clear a singlar
                /// [`UartReadError`] in hardware, and in order to keep a clean
                /// API, [`UartReadError`]s are reported in order of priority, as follows:
                ///
                /// 1. [`UartReadError::Framing`],
                /// 2. [`UartReadError::Parity`], and
                /// 3. [`UartReadError::Overrun`].
                ///
                /// Effectively, this means that you you will get a
                /// [`UartReadError::Framing`] until its cause is solved, and the
                /// error will then "downgrade" to a potential
                /// [`UartReadError::Parity`], until that too is resolved. Repeat
                /// for [`UartReadError::Overrun`]. If no errors are detected, the
                /// received word is returned.
                fn read(&mut self) -> nb::Result<u8, Self::Error> {
                    if self.reg().uart_sr.read().rxrdy().bit_is_clear() {
                        return Err(nb::Error::WouldBlock);
                    }

                    let sr = self.reg().uart_sr.read();
                    if sr.ovre().bit_is_set() {
                        return Err(nb::Error::Other(UartReadError::Overrun));
                    } else if sr.pare().bit_is_set() {
                        return Err(nb::Error::Other(UartReadError::Parity));
                    } else if sr.frame().bit_is_set() {
                        return Err(nb::Error::Other(UartReadError::Framing));
                    }

                    let read = self.reg().uart_rhr.read().rxchr().bits();
                    Ok(read)
                }
            }

            impl<M: UartMeta> $T {
                /// Clear the [`UartReadError`] flags in hardware.
                #[inline(always)]
                pub fn clear_errors(&mut self) {
                    self.reg().uart_cr.write(|w| w.rststa().set_bit());
                }
            }
        )+
    }
}

macro_rules! impl_write {
    ($($T:ty,)+) => {
        $(
            impl<M: UartMeta> ehal::serial::Write<u8> for $T {
                type Error = ();

                /// Writes a single word to the [`Uart`] peripheral.
                ///
                /// # Errors
                ///
                /// Returns [`nb::Error::WouldBlock`] if the last word has yet to
                /// be processed.
                fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
                    if self.reg().uart_sr.read().txrdy().bit_is_clear() {
                        return Err(nb::Error::WouldBlock);
                    }
                    self.reg()
                        .uart_thr
                        .write(|w| unsafe { w.txchr().bits(byte) });
                    Ok(())
                }

                fn flush(&mut self) -> nb::Result<(), Self::Error> {
                    if self.reg().uart_sr.read().txempty().bit_is_clear() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        Ok(())
                    }
                }
            }

            // Provide a blocking impl, derived from the non-blocking impl above.
            impl<M: UartMeta> blocking::serial::write::Default<u8> for $T {}
        )+
    }
}

impl_read!(Rx<M>, Uart<M>,);
impl_write!(Tx<M>, Uart<M>,);
