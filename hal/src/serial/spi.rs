/*!
Serial Communication (SPI)
---

This module contains the abstractions for the device's SPI
peripherals, by use of the [`Spi`] abstraction. While hardware
supports word sizes of 8 to 16 bits, this current implementation
supports 8-bit words.

When an [`Spi`] has been created, a bus client is selected via
[`Spi::select`], returning a [`Client`], upon which `embedded-hal`
trais are implemented.

# Example usage

```
let mut efc = {
    use hal::efc::{Efc, VddioLevel};
    Efc::new(ctx.device.EFC, VddioLevel::V3)
};
use hal::pmc::{
    HostClockConfig, MainCkSource, MckDivider, MckPrescaler, Megahertz,
};

let mut pmc = hal::pmc::Pmc::new(ctx.device.PMC, &ctx.device.WDT.into());
let mainck = pmc
    .get_mainck(MainCkSource::ExternalBypass(12.MHz()))
    .unwrap();
let (_, hclk) = pmc
    .get_hclk(
        HostClockConfig {
            pres: MckPrescaler::CLK_1,
            div: MckDivider::EQ_PCK,
        },
        &mainck,
        &mut efc,
    )
    .unwrap();

let bankd = BankD::new(ctx.device.PIOD, &mut pmc, BankConfiguration::default());
let bankb = BankB::new(ctx.device.PIOB, &mut pmc, BankConfiguration::default());

let miso = bankd.pd20.into_peripheral();
let pck = bankd.pd22.into_peripheral();
let mosi = bankd.pd21.into_peripheral();
let pcs0 = bankb.pb2.into_peripheral();

let mut spi = Spi::new_spi0(
    ctx.device.SPI0,
    (pck, mosi, miso),
    SpiConfiguration::default(),
    &mut pmc,
)
.unwrap();

spi.setup_client(
    &pcs0,
    ClientConfiguration::default(
        115_200.bps(),
        hal::ehal::spi::MODE_0,
    )
    .delay_before_clock_edge(20.nanos()),
    &hclk,
);

let mut client = spi.select(&pcs0).unwrap(); // borrows spi
client.write(b"Hello").unwrap();
```
*/

use super::Bps;
use crate::clocks::{Clock, HostClock, PeripheralIdentifier};
use crate::ehal::blocking;
use crate::fugit::{ExtU32, NanosDurationU32 as NanosDuration};
use crate::pio::*;
#[cfg(feature = "pins-144")]
use crate::target_device::SPI1;
use crate::target_device::{spi0::RegisterBlock, SPI0};
use crate::{ehal, nb};
use ehal::spi::Mode;

use core::marker::PhantomData;

use paste::*;

/// Possible [`Spi`] hardware events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Receive Data Register Full Interrupt
    Rdrf,
    /// SPI Transmit Data Register Empty Interrupt
    Tdre,
    /// Mode Fault Error Interrupt
    Modf,
    /// Overrun Error Interrupt
    Ovres,
    /// NSS Rising Interrupt
    Nssr,
    /// Transmission Registers Empty Interrupt
    Txempty,
    /// Underrun Error Interrupt
    Undes,
}

/// Metadata for a [`Spi`] peripheral.
#[allow(missing_docs)]
pub trait SpiMeta {
    const REG: *const RegisterBlock;
    const PID: PeripheralIdentifier;
}

/// Possible [`Spi`] and [`Client`] errors.
#[derive(Debug, Clone, Copy)]
pub enum SpiError {
    /// The next word was received by the peripheral before the
    /// previous one was read.
    Overrun,
    /// A [`Spi::select`]ed line has not been previously setup via
    /// [`Spi::setup_client`].
    UnconfiguredClient,
    /// The requested [`Spi`] bitrate could not be applied.
    ImpossibleFreq,
    /// A requested [`Spi`] delay could not be applied.
    ImpossibleDelay,
}

/// [`Client`] configuration.
#[derive(Clone, Copy)]
pub struct ClientConfiguration {
    /// Delay between consecutive word transfers
    pub delay_between_transfer: NanosDuration,
    /// Delay between chip selection assertion and the first clock
    /// edge.
    pub delay_before_clock_edge: NanosDuration,
    /// Communication bitrate to the [`Client`].
    pub bitrate: Bps,
    /// See documentation for [`Mode`].
    pub mode: Mode,
}

impl ClientConfiguration {
    /// Generates a default [`Client`] configuration: no delay before
    /// the clock first clock edge, and no delay between consecutive
    /// word transfers.
    pub fn default(bitrate: Bps, mode: Mode) -> Self {
        Self {
            mode,
            delay_between_transfer: 0.nanos(),
            delay_before_clock_edge: 0.nanos(),
            bitrate,
        }
    }

    /// [`Self::mode`] override.
    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    /// [`Self::delay_between_transfer`] override.
    pub fn delay_between_transfer(mut self, delay: NanosDuration) -> Self {
        self.delay_between_transfer = delay;
        self
    }

    /// [`Self::delay_before_clock_edge`] override.
    pub fn delay_before_clock_edge(mut self, delay: NanosDuration) -> Self {
        self.delay_before_clock_edge = delay;
        self
    }
}

/// [`Spi`] configuration.
#[derive(Clone, Copy)]
pub struct SpiConfiguration {
    /// Whether the local loopback test mode should be enabled. When
    /// enabled, MISO is internally connected to MOSI.
    pub test_mode: bool,
}

impl SpiConfiguration {
    /// Generates a default [`Spi`] configuration: test mode inactive.
    pub fn default() -> Self {
        SpiConfiguration { test_mode: false }
    }

    /// [`SpiConfiguration::test_mode`] override.
    pub fn test_mode(mut self, bit: bool) -> Self {
        self.test_mode = bit;
        self
    }
}

/// SPI peripheral abstraction.
pub struct Spi<M: SpiMeta> {
    meta: PhantomData<M>,
    cs_configured: [bool; 4],
}

impl<M: SpiMeta> Spi<M> {
    #[inline]
    fn reg(&self) -> &RegisterBlock {
        unsafe { &*M::REG }
    }

    fn new(mck: &mut HostClock, cfg: SpiConfiguration) -> Result<Self, SpiError> {
        mck.enable_peripheral(M::PID);

        let mut spi = Spi {
            meta: PhantomData,
            cs_configured: [false; 4],
        };
        spi.reconfigure(cfg)?;

        Ok(spi)
    }

    /// Reconfigure the [`Spi`] with a new [`SpiConfiguration`].
    pub fn reconfigure(&mut self, cfg: SpiConfiguration) -> Result<(), SpiError> {
        // Disable the spi and software reset it into a known state.
        self.reg().spi_cr.write(|w| {
            w.spien().clear_bit();
            w.spidis().set_bit()
        });
        self.reg().spi_cr.write(|w| w.swrst().set_bit());

        // configure SPI mode
        self.reg().spi_mr.modify(|_, w| {
            // enter master/host mode
            w.mstr().set_bit();

            // the chip select lines are directly connected to a
            // peripheral device; no 4-bit to 16-bit HW decode is
            // used.
            w.pcsdec().bit(false);

            // variable peripheral select
            w.ps().bit(true);

            w.llb().bit(cfg.test_mode);

            // disable modefault detection: a multi-host feature
            w.modfdis().bit(true);

            // C.f. §41.8.2
            const MINIMUM_TICKS_BETWEEN_CHIP_SELECTS: u8 = 6;
            unsafe { w.dlybcs().bits(MINIMUM_TICKS_BETWEEN_CHIP_SELECTS) };

            w
        });

        self.reg().spi_cr.write(|w| {
            w.spidis().clear_bit();
            w.spien().set_bit();
            w
        });

        Ok(())
    }

    fn calc_delay_ticks(
        clk_period: NanosDuration,
        min_delay: NanosDuration,
    ) -> Result<u8, SpiError> {
        (min_delay / clk_period)
            // ensure the number of ticks is at least min_delay long
            .checked_add(1)
            .and_then(|t| t.try_into().ok())
            .ok_or(SpiError::ImpossibleDelay)
    }

    /// Calculate the delay before falling edge of select pin.
    ///
    /// C.f. §41.8.9
    fn calc_dlybs(clk: &HostClock, min_delay: NanosDuration) -> Result<u8, SpiError> {
        let pck_period: NanosDuration = clk.freq().into_duration();
        if pck_period / 2 >= min_delay {
            return Ok(0);
        }
        Self::calc_delay_ticks(pck_period, min_delay)
    }

    /// Calculate the delay between consecutive transfers.
    ///
    /// C.f. §41.8.9
    fn calc_dlybct(clk: &HostClock, min_delay: NanosDuration) -> Result<u8, SpiError> {
        if min_delay.to_nanos() == 0 {
            return Ok(0);
        }
        const DLYBCT_PCK_DIV: u32 = 32;
        let div_period: NanosDuration = (clk.freq() / DLYBCT_PCK_DIV).into_duration();
        Self::calc_delay_ticks(div_period, min_delay)
    }

    fn setup_client_inner(
        &mut self,
        cs: u8,
        conf: ClientConfiguration,
        clk: &HostClock,
    ) -> Result<(), SpiError> {
        // Find the serial clock bit rate prescaler
        let scbr: u8 = (clk.freq() / conf.bitrate.0)
            .try_into()
            .map_err(|_| SpiError::ImpossibleFreq)?;
        if scbr == 0 {
            return Err(SpiError::ImpossibleFreq);
        }

        // Find delay prescalers
        let dlybct = Self::calc_dlybct(clk, conf.delay_between_transfer)?;
        let dlybs = Self::calc_dlybs(clk, conf.delay_before_clock_edge)?;

        self.reg().spi_csr[cs as usize].modify(|_, w| {
            unsafe {
                w.scbr().bits(scbr);
                w.dlybs().bits(dlybs);
                w.dlybct().bits(dlybct);
            }

            // use 8b words
            w.bits_()._8_bit();

            // C.f. Table 41-2, p. 933
            use ehal::spi::{Phase, Polarity};
            match (conf.mode.phase, conf.mode.polarity) {
                (Phase::CaptureOnFirstTransition, Polarity::IdleLow) => {
                    w.cpol().clear_bit();
                    w.ncpha().set_bit();
                }
                (Phase::CaptureOnSecondTransition, Polarity::IdleLow) => {
                    w.cpol().clear_bit();
                    w.ncpha().clear_bit();
                }
                (Phase::CaptureOnFirstTransition, Polarity::IdleHigh) => {
                    w.cpol().set_bit();
                    w.ncpha().set_bit();
                }
                (Phase::CaptureOnSecondTransition, Polarity::IdleHigh) => {
                    w.cpol().set_bit();
                    w.ncpha().clear_bit();
                }
            }

            // Keep client selected until lastxfer is called before
            // the last write.
            w.csaat().set_bit();

            w
        });

        self.cs_configured[cs as usize] = true;

        Ok(())
    }

    /// Listen for a [`Spi`] interrupt  [`Event`].
    pub fn listen(&mut self, event: Event) {
        self.reg().spi_ier.write(|w| match event {
            Event::Rdrf => w.rdrf().set_bit(),
            Event::Tdre => w.tdre().set_bit(),
            Event::Modf => w.modf().set_bit(),
            Event::Ovres => w.ovres().set_bit(),
            Event::Nssr => w.nssr().set_bit(),
            Event::Txempty => w.txempty().set_bit(),
            Event::Undes => w.undes().set_bit(),
        });
    }

    /// Stop listening for a [`Spi`] interrupt [`Event`].
    pub fn unlisten(&mut self, event: Event) {
        self.reg().spi_idr.write(|w| match event {
            Event::Rdrf => w.rdrf().set_bit(),
            Event::Tdre => w.tdre().set_bit(),
            Event::Modf => w.modf().set_bit(),
            Event::Ovres => w.ovres().set_bit(),
            Event::Nssr => w.nssr().set_bit(),
            Event::Txempty => w.txempty().set_bit(),
            Event::Undes => w.undes().set_bit(),
        });
    }
}

/// A selected [`Spi`] bus client.
pub struct Client<'spi, M: SpiMeta> {
    meta: PhantomData<&'spi M>,
    last_xfer: bool,
    cs: u8,
}

impl<'spi, M: SpiMeta> Client<'spi, M> {
    #[inline]
    fn reg(&self) -> &RegisterBlock {
        unsafe { &*M::REG }
    }

    /// Marks the next word as the last, deasserting the select line
    /// after the word transfer.
    pub fn lastxfer(&mut self) {
        self.last_xfer = true;
    }

    fn write_inner(&mut self, words: &[u8], use_lastxfer: bool) -> Result<(), SpiError> {
        use ehal::spi::FullDuplex;

        let len = words.len();
        for (i, word) in words.iter().enumerate() {
            if use_lastxfer && i == len - 1 {
                self.lastxfer();
            }
            nb::block!(self.send(*word))?;
            nb::block!(self.read())?;
        }

        Ok(())
    }

    fn transfer_inner<'w>(
        &mut self,
        words: &'w mut [u8],
        use_lastxfer: bool,
    ) -> Result<&'w [u8], SpiError> {
        use ehal::spi::FullDuplex;

        let len = words.len();
        for (i, word) in words.iter_mut().enumerate() {
            if use_lastxfer && i == len - 1 {
                self.lastxfer();
            }
            nb::block!(self.send(*word))?;
            *word = nb::block!(self.read())?;
        }

        Ok(words)
    }
}

impl<'spi, M: SpiMeta> Spi<M> {
    fn select_inner(&'spi mut self, cs: u8) -> Result<Client<'spi, M>, SpiError> {
        if self.cs_configured.get(cs as usize).is_none() {
            return Err(SpiError::UnconfiguredClient);
        }

        Ok(Client {
            meta: PhantomData,
            last_xfer: false,
            cs,
        })
    }
}

macro_rules! impl_spi {
    (
        $(
            $( #[$cfg:meta] )?
            $Spi:ident: {
                MISO: [ $MisoType:ty ],
                MOSI: [ $MosiType:ty ],
                SPCK: [ $SpckType:ty ],
                CS: [ $(($NpCsPin:ty, $Cs:literal)),+ ],
            },
        )+
    ) => {
        paste! {
            $(
                $( #[$cfg] )?
                mod [<$Spi:lower _impl>] {
                    use super::*;

                    #[doc = "Trait that identifies valid MOSI [`Pin`]s for [`" [<$Spi:upper>] "`]."]
                    pub trait [<$Spi MosiPin>] {}
                    #[doc = "Trait that identifies valid MISO [`Pin`]s for [`" [<$Spi:upper>] "`]."]
                    pub trait [<$Spi MisoPin>] {}
                    #[doc = "Trait that identifies valid SPCK [`Pin`]s for [`" [<$Spi:upper>] "`]."]
                    pub trait [<$Spi SpckPin>] {}
                    #[doc = "Trait that identifies valid NPCS [`Pin`]s for [`" [<$Spi:upper>] "`]."]
                    pub trait [<$Spi NpCsPin>] {
                        /// Numerical identifier for this NPCS (select) pin.
                        const CS: u8;
                    }

                    impl [<$Spi MosiPin>] for $MosiType {}
                    impl [<$Spi MisoPin>] for $MisoType {}
                    impl [<$Spi SpckPin>] for $SpckType {}
                    $(
                        impl [<$Spi NpCsPin>] for $NpCsPin {
                            const CS: u8 = $Cs;
                        }
                    )+

                    #[doc = "Type-level variant denoting [`" [<$Spi:upper>] "`]."]
                    pub enum $Spi {}


                    impl SpiMeta for $Spi {
                        const REG: *const RegisterBlock = [<$Spi:upper>]::ptr();
                        const PID: PeripheralIdentifier = PeripheralIdentifier::[<$Spi:upper>];
                    }

                    impl Spi<$Spi>  {
                        #[doc = "Create a new [`Spi`] from a [`" [<$Spi:upper>] "`], associated [`Pin`]s, and valid [`HostClock`]."]
                        pub fn [<new_ $Spi:lower>](
                            _spi: [<$Spi:upper>],
                            _pins: (impl [<$Spi SpckPin>], impl [<$Spi MosiPin>], impl [<$Spi MisoPin>]),
                            cfg: SpiConfiguration,
                            mck: &mut HostClock) -> Result<Self, SpiError> {
                            Self::new(mck, cfg)
                        }

                        /// Selects the client connected to the given
                        /// [`Pin`] and returns a [`Client`], prepared
                        /// for bus transmission and receive.
                        ///
                        /// # Errors
                        ///
                        /// Will yield a
                        /// [`SpiError::UnconfiguredClient`] unless
                        /// [`Spi::setup_client`] has been called with
                        /// the same `S`.
                        pub fn select<S: [<$Spi NpCsPin>]>(&mut self, _client: &S) -> Result<Client<$Spi>, SpiError> {
                            self.select_inner(S::CS)
                        }

                        /// Configures the given `S` for a future [`Spi::select`].
                        pub fn setup_client<S: [<$Spi NpCsPin>]>(&mut self, _client: &S, conf: ClientConfiguration, clk: &HostClock) -> Result<(), SpiError> {
                            self.setup_client_inner(S::CS, conf, clk)
                        }
                    }
                }
                $( #[$cfg] )?
                pub use [<$Spi:lower _impl>]::*;
            )+
        }
    };
}

impl_spi!(
    Spi0: {
        MISO: [ Pin<PD20, PeripheralB> ],
        MOSI: [ Pin<PD21, PeripheralB> ],
        SPCK: [ Pin<PD22, PeripheralB> ],
        CS: [
            (Pin<PA31, PeripheralA>, 1),
            (Pin<PB2, PeripheralD>, 0),
            (Pin<PD12, PeripheralC>, 2),
            (Pin<PD25, PeripheralB>, 1),
            (Pin<PD27, PeripheralB>, 3)
        ],
    },

    #[cfg(feature = "pins-144")]
    Spi1: {
        MISO: [ Pin<PC26, PeripheralC> ],
        MOSI: [ Pin<PC27, PeripheralC> ],
        SPCK: [ Pin<PC24, PeripheralC> ],
        CS: [
            (Pin<PC25, PeripheralC>, 0),
            (Pin<PC28, PeripheralC>, 1),
            (Pin<PC29, PeripheralC>, 2),
            (Pin<PC30, PeripheralC>, 3),
            (Pin<PD0, PeripheralC>, 1),
            (Pin<PD1, PeripheralC>, 2),
            (Pin<PD2, PeripheralC>, 3)
        ],
    },
);

impl<'spi, M: SpiMeta> ehal::spi::FullDuplex<u8> for Client<'spi, M> {
    type Error = SpiError;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let sr = self.reg().spi_sr.read();
        if sr.rdrf().bit_is_clear() {
            return Err(nb::Error::WouldBlock);
        } else if sr.ovres().bit_is_set() {
            return Err(nb::Error::Other(SpiError::Overrun));
        }

        Ok(self.reg().spi_rdr.read().rd().bits().try_into().unwrap())
    }

    /// Sends a word to the client.
    ///
    /// If [`Client::lastxfer`] was previously called, the select line
    /// is deasserted after this word has been transferred. Otherwise
    /// the select line remains asserted.
    fn send(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        let sr = self.reg().spi_sr.read();
        if sr.tdre().bit_is_clear() {
            return Err(nb::Error::WouldBlock);
        } else if sr.ovres().bit_is_set() {
            return Err(nb::Error::Other(SpiError::Overrun));
        }

        self.reg().spi_tdr.write(|w| {
            unsafe {
                w.td().bits(word as u16);
                w.pcs().bits(self.cs);
            }

            w.lastxfer().bit(self.last_xfer);
            w
        });
        self.last_xfer = false;
        Ok(())
    }
}

impl<'spi, M: SpiMeta> blocking::spi::Transactional<u8> for Client<'spi, M> {
    type Error = SpiError;

    /// Execute the provided transactions, deasserting the select line
    /// after the last transmitted word.
    fn exec<'a>(
        &mut self,
        operations: &mut [blocking::spi::Operation<'a, u8>],
    ) -> Result<(), Self::Error> {
        use blocking::spi::Operation;

        let len = operations.len();
        for (i, o) in operations.iter_mut().enumerate() {
            let last_op = i == len - 1;

            if let Some(e) = match o {
                Operation::Transfer(words) => self.transfer_inner(words, last_op).err(),
                Operation::Write(words) => self.write_inner(words, last_op).err(),
            } {
                return Err(e);
            }
        }

        Ok(())
    }
}

impl<'spi, M: SpiMeta> blocking::spi::Write<u8> for Client<'spi, M> {
    type Error = SpiError;

    /// Write `words` to the client, discarding all words received
    /// from the client, and deasserting the select line after the
    /// last transmitted word.
    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.write_inner(words, true)
    }
}

impl<'spi, M: SpiMeta> blocking::spi::Transfer<u8> for Client<'spi, M> {
    type Error = SpiError;

    /// Write `words` to the client, recording all words received from
    /// the client into the same `words`, and deasserting the select
    /// line after the last transmitted word.
    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        self.transfer_inner(words, true)
    }
}
