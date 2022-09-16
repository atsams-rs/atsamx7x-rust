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

Interrupt event management is handled by the [`event system`](crate::generics::events).

# Example usage

```no_run
# use atsamx7x_hal as hal;
# use hal::pio::*;
# use hal::clocks::*;
# use hal::efc::*;
# use hal::serial::spi::*;
# use hal::serial::ExtBpsU32;
# use hal::fugit::ExtU32;
# let pac = hal::pac::Peripherals::take().unwrap();
# let (slck, mut mck) = Tokens::new((pac.PMC, pac.SUPC, pac.UTMI), &pac.WDT.into()).por_state(&mut Efc::new(pac.EFC, VddioLevel::V3));

let bankd = BankD::new(pac.PIOD, &mut mck, &slck, BankConfiguration::default());
let bankb = BankB::new(pac.PIOB, &mut mck, &slck, BankConfiguration::default());

let miso = bankd.pd20.into_peripheral();
let pck = bankd.pd22.into_peripheral();
let mosi = bankd.pd21.into_peripheral();
let pcs0 = bankb.pb2.into_peripheral();

let mut spi = Spi::new_spi0(
    pac.SPI0,
    (pck, mosi, miso),
    SpiConfiguration::default(),
    &mut mck,
)
.unwrap();

spi.setup_client(
    &pcs0,
    ClientConfiguration::default(
        115_200.bps(),
        hal::ehal::spi::MODE_0,
    )
    .delay_before_clock_edge(20.nanos()),
    &mck,
);

use hal::ehal::blocking::spi::Write;

let mut client = spi.select(&pcs0).unwrap(); // borrows spi
client.write(b"Hello").unwrap();
```
*/

use super::Bps;
use crate::clocks::{Clock, HostClock, PeripheralIdentifier};
use crate::ehal::blocking;
use crate::fugit::{ExtU32, NanosDurationU32 as NanosDuration};
#[cfg(feature = "pins-144")]
use crate::pac::SPI1;
use crate::pac::{spi0::tdr::PCSSELECT_AW as HwChipSelect, spi0::RegisterBlock, SPI0};
use crate::{ehal, nb};
use crate::{generics, pio::*};
use core::marker::PhantomData;
use ehal::spi::Mode;
use strum::FromRepr;

use paste::*;

trait ChipSelect {
    fn as_index(&self) -> usize;
    fn as_variant(&self) -> HwChipSelect;
}

impl ChipSelect for HwChipSelect {
    /// Returns the offset index in the memory layout that
    /// dereferences the configuration register of the given
    /// [`HwChipSelect`].
    #[inline(always)]
    fn as_index(&self) -> usize {
        // The #[repr(u8)] of cs are that of a zero moving left in a
        // ones(4) field, when an external decoder is not used:
        //
        //    NPCS0 = 1110
        //    NPCS1 = 1101
        //    NPCS2 = 1011
        //    NPCS3 = 0111
        //
        // C.f. §41.8.4
        (*self as u8).trailing_ones().try_into().unwrap()
    }

    /// Returns the hardware descriptor used to select a chip.
    #[inline(always)]
    fn as_variant(&self) -> Self {
        *self
    }
}

/// Possible [`Spi`] hardware events.
///
/// C.f. §41.8.5
#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(u32)]
pub enum Event {
    /// Receive Data Register Full Interrupt
    Rdrf = 1 << 0,
    /// SPI Transmit Data Register Empty Interrupt
    Tdre = 1 << 1,
    /// Mode Fault Error Interrupt
    Modf = 1 << 2,
    /// Overrun Error Interrupt
    Ovres = 1 << 3,
    /// NSS Rising Interrupt
    Nssr = 1 << 8,
    /// Transmission Registers Empty Interrupt
    Txempty = 1 << 9,
    /// Underrun Error Interrupt
    Undes = 1 << 10,
}

/// Metadata for a [`Spi`] peripheral.
#[allow(missing_docs)]
pub trait SpiMeta: generics::Sealed {
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
impl<M: SpiMeta> generics::Sealed for Spi<M> {}

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
        self.reg().cr.write(|w| {
            w.spien().clear_bit();
            w.spidis().set_bit()
        });
        self.reg().cr.write(|w| w.swrst().set_bit());

        // configure SPI mode
        self.reg().mr.modify(|_, w| {
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

        self.reg().cr.write(|w| {
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
        cs: impl ChipSelect,
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

        let cs = cs.as_index();

        self.reg().csr[cs].modify(|_, w| {
            unsafe {
                w.scbr().bits(scbr);
                w.dlybs().bits(dlybs);
                w.dlybct().bits(dlybct);
            }

            // use 8b words
            w.bits_()._8_bit();

            use ehal::spi::{Phase, Polarity};
            match conf.mode.polarity {
                Polarity::IdleLow => w.cpol().idle_low(),
                Polarity::IdleHigh => w.cpol().idle_high(),
            };
            match conf.mode.phase {
                Phase::CaptureOnFirstTransition => w.ncpha().valid_leading_edge(),
                Phase::CaptureOnSecondTransition => w.ncpha().valid_trailing_edge(),
            };

            // Keep client selected until lastxfer is called before
            // the last write.
            w.csaat().set_bit();

            w
        });

        self.cs_configured[cs] = true;

        Ok(())
    }
}

/// A selected [`Spi`] bus client.
pub struct Client<'spi, M: SpiMeta> {
    meta: PhantomData<&'spi M>,
    last_xfer: bool,
    cs: HwChipSelect,
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

impl TryFrom<u32> for Event {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, ()> {
        match Self::from_repr(value) {
            Some(val) => Ok(val),
            _ => Err(()),
        }
    }
}

impl<M: SpiMeta> crate::generics::events::EventHandler for Spi<M> {
    type EventSource = Event;
    fn listen(&mut self, event: Self::EventSource) {
        self.reg().ier.write(|w| match event {
            Event::Rdrf => w.rdrf().set_bit(),
            Event::Tdre => w.tdre().set_bit(),
            Event::Modf => w.modf().set_bit(),
            Event::Ovres => w.ovres().set_bit(),
            Event::Nssr => w.nssr().set_bit(),
            Event::Txempty => w.txempty().set_bit(),
            Event::Undes => w.undes().set_bit(),
        });
    }
    fn unlisten(&mut self, event: Self::EventSource) {
        self.reg().idr.write(|w| match event {
            Event::Rdrf => w.rdrf().set_bit(),
            Event::Tdre => w.tdre().set_bit(),
            Event::Modf => w.modf().set_bit(),
            Event::Ovres => w.ovres().set_bit(),
            Event::Nssr => w.nssr().set_bit(),
            Event::Txempty => w.txempty().set_bit(),
            Event::Undes => w.undes().set_bit(),
        });
    }
    fn irq(&mut self) -> u32 {
        self.reg().imr.read().bits() & self.reg().sr.read().bits()
    }
}

impl<'spi, M: SpiMeta> Spi<M> {
    fn select_inner<CS: ChipSelect>(&'spi mut self, cs: CS) -> Result<Client<'spi, M>, SpiError> {
        if self.cs_configured.get(cs.as_index()).is_none() {
            return Err(SpiError::UnconfiguredClient);
        }

        Ok(Client {
            meta: PhantomData,
            last_xfer: false,
            cs: cs.as_variant(),
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
                CS: [ $(($NpCsPin:ty, $Cs:expr)),+ ],
            },
        )+
    ) => {
        paste! {
            $(
                $( #[$cfg] )?
                mod [<$Spi:lower _impl>] {
                    use super::*;

                    #[doc = "Trait that identifies valid MOSI [`Pin`]s for [`" [<$Spi:upper>] "`]."]
                    pub trait [<$Spi MosiPin>]: generics::Sealed {}
                    #[doc = "Trait that identifies valid MISO [`Pin`]s for [`" [<$Spi:upper>] "`]."]
                    pub trait [<$Spi MisoPin>]: generics::Sealed {}
                    #[doc = "Trait that identifies valid SPCK [`Pin`]s for [`" [<$Spi:upper>] "`]."]
                    pub trait [<$Spi SpckPin>]: generics::Sealed {}
                    #[doc = "Trait that identifies valid NPCS [`Pin`]s for [`" [<$Spi:upper>] "`]."]
                    pub trait [<$Spi NpCsPin>]: generics::Sealed {
                        /// Hardware identifier for this NPCS (select) pin.
                        const CS: HwChipSelect;
                    }

                    impl [<$Spi MosiPin>] for $MosiType {}
                    impl [<$Spi MisoPin>] for $MisoType {}
                    impl [<$Spi SpckPin>] for $SpckType {}
                    $(
                        impl [<$Spi NpCsPin>] for $NpCsPin {
                            const CS: HwChipSelect = $Cs;
                        }
                    )+

                    #[doc = "Type-level variant denoting [`" [<$Spi:upper>] "`]."]
                    pub enum $Spi {}

                    impl generics::Sealed for $Spi {}
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
            (Pin<PA31, PeripheralA>, HwChipSelect::NPCS1),
            (Pin<PB2,  PeripheralD>, HwChipSelect::NPCS0),
            (Pin<PD12, PeripheralC>, HwChipSelect::NPCS2),
            (Pin<PD25, PeripheralB>, HwChipSelect::NPCS1),
            (Pin<PD27, PeripheralB>, HwChipSelect::NPCS3)
        ],
    },

    #[cfg(feature = "pins-144")]
    Spi1: {
        MISO: [ Pin<PC26, PeripheralC> ],
        MOSI: [ Pin<PC27, PeripheralC> ],
        SPCK: [ Pin<PC24, PeripheralC> ],
        CS: [
            (Pin<PC25, PeripheralC>, HwChipSelect::NPCS0),
            (Pin<PC28, PeripheralC>, HwChipSelect::NPCS1),
            (Pin<PC29, PeripheralC>, HwChipSelect::NPCS2),
            (Pin<PC30, PeripheralC>, HwChipSelect::NPCS3),
            (Pin<PD0,  PeripheralC>, HwChipSelect::NPCS1),
            (Pin<PD1,  PeripheralC>, HwChipSelect::NPCS2),
            (Pin<PD2,  PeripheralC>, HwChipSelect::NPCS3)
        ],
    },
);

impl<'spi, M: SpiMeta> ehal::spi::FullDuplex<u8> for Client<'spi, M> {
    type Error = SpiError;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let sr = self.reg().sr.read();
        if sr.rdrf().bit_is_clear() {
            return Err(nb::Error::WouldBlock);
        } else if sr.ovres().bit_is_set() {
            return Err(nb::Error::Other(SpiError::Overrun));
        }

        Ok(self.reg().rdr.read().rd().bits().try_into().unwrap())
    }

    /// Sends a word to the client.
    ///
    /// If [`Client::lastxfer`] was previously called, the select line
    /// is deasserted after this word has been transferred. Otherwise
    /// the select line remains asserted.
    fn send(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        let sr = self.reg().sr.read();
        if sr.tdre().bit_is_clear() {
            return Err(nb::Error::WouldBlock);
        } else if sr.ovres().bit_is_set() {
            return Err(nb::Error::Other(SpiError::Overrun));
        }

        self.reg().tdr.write(|w| {
            unsafe {
                w.td().bits(word as u16);
            }

            w.pcs().variant(self.cs);
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
