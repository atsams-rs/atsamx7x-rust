/*!
Serial Communication (UART)
---

This module contains the abstractions for the device's UART
peripheral, by use of the [`Uart`] abstraction. The UART only
supports 8-bit words with a single stop bit, but supports five
parity options:

- [`ParityMode::Even`],
- [`ParityMode::Odd`],
- [`ParityMode::Space`],
- [`ParityMode::Mark`], or
- [`ParityMode::None`],

Interrupt event management is handled by the [`event system`](crate::generics::events).

# Example usage

```no_run
# use atsamx7x_hal as hal;
# use hal::pio::*;
# use hal::clocks::*;
# use hal::efc::*;
# use hal::generics::events::*;
# use hal::serial::uart::*;
# use hal::serial::ExtU32 as _;
# use hal::fugit::{ExtU32, RateExtU32};
# let pac = hal::target_device::Peripherals::take().unwrap();
let clocks = Tokens::new((pac.PMC, pac.SUPC, pac.UTMI), &pac.WDT.into());
let slck = clocks.slck.configure_external_normal();
let mainck = clocks
    .mainck
    .configure_external_normal(12.MHz())
    .unwrap();
let mut efc = Efc::new(pac.EFC, VddioLevel::V3);
let (_hclk, mut mck) = HostClockController::new(clocks.hclk, clocks.mck)
    .configure(
        &mainck,
        &mut efc,
        HostClockConfig {
            pres: HccPrescaler::Div1,
            div: MckDivider::Div1,
        },
    )
    .unwrap();
let pck: Pck<Pck4> = clocks.pcks.pck4.configure(&mainck, 1);

let banka = BankA::new(pac.PIOA, &mut mck, &slck, BankConfiguration::default());
let tx = banka.pa10.into_peripheral::<A>();
let rx = banka.pa9.into_peripheral::<A>();

let mut uart = Uart::new_uart0(
    pac.UART0,
    (tx, rx),
    UartConfiguration::default(115_200.bps()),
    PeripheralClock::Other(&mut mck, &pck),
)
.unwrap();
uart.listen(Event::RxReady);

use hal::ehal::serial::{Read, Write};

uart.write(0xff).unwrap();
assert_eq!(uart.read().unwrap(), 0xff);
```
*/

use crate::clocks::{Clock, HostClock, Pck, Pck4, PeripheralClock, PeripheralIdentifier};
use crate::ehal::{self, blocking};
use crate::pio::*;
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
use strum::FromRepr;

/// Available parity modes
#[allow(missing_docs)]
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[allow(missing_docs)]
pub trait UartMeta {
    const REG: *const RegisterBlock;
    const PID: PeripheralIdentifier;
}

/// [`Uart`] configuration
#[derive(Clone, Copy)]
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

#[derive(Debug, Eq, PartialEq, Clone)]
/// Possible [`Uart`] errors
pub enum UartError {
    /// An impossible baud rate was requested.
    BaudRateNotInRange,
    /// The calculated prescaler does not fit in a [`u16`].
    PrescalerOverflow,
}

/// A valid input clock for the [`Uart`].
pub trait UartClock: Clock {
    /// C.f. §47.6.2
    const BRSRCCK: bool;
}

impl UartClock for HostClock {
    const BRSRCCK: bool = false;
}

impl UartClock for Pck<Pck4> {
    const BRSRCCK: bool = true;
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
///
/// C.f. §47.6.6
#[derive(Clone, Copy, FromRepr)]
#[repr(u32)]
pub enum Event {
    /// A new word has been received.
    RxReady = 1 << 0,
    /// The next word can be sent.
    TxReady = 1 << 1,
    /// All previous words have been serialized.
    TxEmpty = 1 << 9,
    /// Rx buffer overrun error.
    ///
    /// Cleared by calling [`clear_errors`](Uart::clear_errors)
    Ovre = 1 << 5,
    /// Framing error.
    ///
    /// Cleared by calling [`clear_errors`](Uart::clear_errors)
    Frame = 1 << 6,
    /// Parity error.
    ///
    /// Cleared by calling [`clear_errors`](Uart::clear_errors)
    Pare = 1 << 7,
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
        clk: PeripheralClock<C>,
        conf: UartConfiguration,
    ) -> Result<Self, UartError> {
        let mut uart = Self {
            meta: PhantomData,
            tx: Tx { meta: PhantomData },
            rx: Rx { meta: PhantomData },
        };

        match clk {
            PeripheralClock::Host(mck) => {
                mck.enable_peripheral(M::PID);
                uart.apply_config(mck, conf)?;
            }
            PeripheralClock::Other(mck, clk) => {
                mck.enable_peripheral(M::PID);
                uart.apply_config(clk, conf)?;
            }
        };

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
    /// Split the [`Uart`] into two separate [`Tx`] and [`Rx`]
    /// components.
    pub fn split(self) -> (Tx<M>, Rx<M>) {
        (self.tx, self.rx)
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

impl<M: UartMeta> crate::generics::events::EventHandler for Uart<M> {
    type EventSource = Event;

    fn listen(&mut self, event: Self::EventSource) {
        self.reg().uart_ier.write(|w| match event {
            Event::TxReady => w.txrdy().set_bit(),
            Event::RxReady => w.rxrdy().set_bit(),
            Event::TxEmpty => w.txempty().set_bit(),
            Event::Ovre => w.ovre().set_bit(),
            Event::Frame => w.frame().set_bit(),
            Event::Pare => w.pare().set_bit(),
        });
    }

    fn unlisten(&mut self, event: Self::EventSource) {
        self.reg().uart_idr.write(|w| match event {
            Event::TxReady => w.txrdy().set_bit(),
            Event::RxReady => w.rxrdy().set_bit(),
            Event::TxEmpty => w.txempty().set_bit(),
            Event::Ovre => w.ovre().set_bit(),
            Event::Frame => w.frame().set_bit(),
            Event::Pare => w.pare().set_bit(),
        });
    }

    fn irq(&mut self) -> u32 {
        self.reg().uart_imr.read().bits() & self.reg().uart_sr.read().bits()
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
                        pub fn [<new_ $Uart:lower>]<C: UartClock>(
                            _uart: [<$Uart:upper>],
                            _pins: (impl [<$Uart TxPin>], impl [<$Uart RxPin>]),
                            conf: UartConfiguration,
                            clk: PeripheralClock<C>,
                        ) -> Result<Self, UartError> {
                            Self::new(clk, conf)
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
