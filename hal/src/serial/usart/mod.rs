/*!
Universal Synchronous Asynchronous Receiver Transceiver (USART)
---

This module contains abstractions for the device's USART peripherals
by means of the [`Usart`] abstraction. While [`Usart`] itself
implements no transfer functionality it can be set to work in either
[`Spi`] or [`Uart`] mode. These modes implement their respective
[`embedded-hal`](crate::ehal) traits. Both of these modes support
8-bit words.

The USART peripheral also supports LIN, LON, and USART modes, but
these have yet to be implemented.

# Mode support

Mode support depends on what [`Pin`]s that are available for the
[`Usart`]; refer to [`UsartPins`].

# System [`Pin`]s

[`Usart<Usart1>`] can only obtain a legal pin configuration with the `reconfigurable-system-pins` feature and is therefore also hidden behind said feature gate.

# Example usage

```no_run
# use atsamx7x_hal as hal;
# use hal::pio::*;
# use hal::clocks::*;
# use hal::efc::*;
# use hal::generics::events::*;
# use hal::serial::usart::*;
# use hal::serial::ExtBpsU32;
# use hal::fugit::{ExtU32, RateExtU32};
# let pac = unsafe{hal::pac::Peripherals::steal()};
# let (slck, mut mck) = Tokens::new((pac.PMC, pac.SUPC, pac.UTMI), &pac.WDT.into()).por_state(&mut Efc::new(pac.EFC, VddioLevel::V3));
use hal::generics::events::EventHandler;
use hal::ehal::serial::{Read, Write};

let bankb = BankB::new(pac.PIOB, &mut mck, &slck, BankConfiguration::default());

let miso = bankb.pb0.into_peripheral();
let mosi = bankb.pb1.into_peripheral();
let clk = bankb.pb13.into_peripheral();
let nss = bankb.pb3.into_peripheral();

// Create the top-level USART abstraction
let (handles, mut usart) = Usart::new_usart0(
    pac.USART0,
    (mosi, miso, clk, nss),
    &mut mck,
);
// From the tokens of handles, consume the spi_host token to get an
// SPI handle in host mode.
let mut spi = handles.spi_host.configure(
    &usart,
    &mck,
    SpiConfig {
        bitrate: 115_200.bps(),
        mode: MODE_0,
    },
).unwrap();
// Do the same for a UART handle.
let mut uart = handles.uart.configure(
    &usart,
    &mck,
    UartConfiguration::default(115_200.bps()),
).unwrap();

// Listen to an interrupt event.
usart.listen(Event::RxReady);

const PAYLOAD: u8 = b'x';

// Enter UART mode and send/receive a payload.
usart.enter_mode(&uart);
uart.write(PAYLOAD).unwrap();
let _ = uart.read().unwrap();

// Repeat for SPI mode.
usart.enter_mode(&spi);
spi.send(PAYLOAD).unwrap();
let _ = spi.read().unwrap();
```
 */

pub use super::uart::{ChannelMode, ParityMode, UartConfiguration};
use crate::clocks::{Clock, Hertz, HostClock, Pck, Pck4, PeripheralIdentifier};
use crate::generics::{self, Token};
#[cfg(feature = "reconfigurable-system-pins")]
use crate::pac::USART1;
#[cfg(not(feature = "__pins-64"))]
use crate::pac::USART2;
use crate::pac::{
    usart0::us_mr_usart_mode::CHMODESELECT_A as HwChannelMode,
    usart0::us_mr_usart_mode::PARSELECT_A as HwParityMode,
    usart0::us_mr_usart_mode::USART_MODESELECT_A as HwUsartMode,
    usart0::us_mr_usart_mode::USCLKSSELECT_A as UsartClockSource, usart0::RegisterBlock, USART0,
};

use crate::pio::*;
use crate::serial::Bps;

use core::marker::PhantomData;

use paste::*;
use strum::FromRepr;

impl From<ChannelMode> for HwChannelMode {
    fn from(m: ChannelMode) -> Self {
        use ChannelMode::*;
        match m {
            Normal => Self::NORMAL,
            Automatic => Self::AUTOMATIC,
            LocalLoopback => Self::LOCAL_LOOPBACK,
            RemoteLoopback => Self::REMOTE_LOOPBACK,
        }
    }
}
impl From<ParityMode> for HwParityMode {
    fn from(m: ParityMode) -> Self {
        use ParityMode::*;
        match m {
            Even => Self::EVEN,
            Odd => Self::ODD,
            Space => Self::SPACE,
            Mark => Self::MARK,
            None => Self::NO,
        }
    }
}

/// Possible [`Usart`] modes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UsartMode {
    /// The [`Usart`] is operating in [`Spi`] mode.
    Spi(SpiMode),
    /// The [`Usart`] is operating in [`Uart`] mode.
    Uart,
}

/// [`Usart`] interrupt events.
///
/// For [`u32`] representation, C.f. §46.7.5 and §46.7.6.
#[derive(Clone, Copy, Debug, Eq, PartialEq, FromRepr)]
#[repr(u32)]
pub enum Event {
    /// A word has been received and is available to read.
    RxReady = 1 << 0,
    /// A previous word has been transferred to the shift register. A
    /// new word can be sent.
    TxReady = 1 << 1,
    /// A next word was received before the previous was read.
    Overrun = 1 << 5,
    /// All previous word has been processed. Transmit line is idle. A
    /// new word can be sent.
    TxEmpty = 1 << 9,
}

impl TryFrom<u32> for Event {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, ()> {
        Self::from_repr(value).ok_or(())
    }
}

type Prescaler = u16;

/// Configuration procedures for entering an [`UsartMode`].
pub trait UsartHandle<M: UsartMeta>: generics::Sealed {
    /// The mode of the handle.
    const MODE: UsartMode;

    /// Resets the internal state machines and buffers for the
    /// [`UsartMode`].
    ///
    /// # Safety
    ///
    /// This function modifies the [`Usart`] hardware, and is called
    /// by [`Usart::enter_mode`]. It should not be called directly.
    unsafe fn reset(&self, usart: &mut Usart<M>);

    /// Configures the [`Usart`] to operate in the target
    /// [`UsartMode`] handle, and returns the mode's required
    /// prescaler.
    ///
    /// # Safety
    ///
    /// This function modifies the [`Usart`] hardware, and is called
    /// by [`Usart::enter_mode`]. It should not be called directly.
    unsafe fn configure(&self, usart: &mut Usart<M>) -> Prescaler;
}

trait RegisterAccess<M: UsartMeta> {
    #[inline(always)]
    fn reg(&self) -> &RegisterBlock {
        unsafe { &*M::REG }
    }

    /// Returns what [`mode`](UsartMode) the [`Usart`] is currently operating in.
    fn mode(&self) -> UsartMode {
        self.reg()
            .us_mr_usart_mode()
            .read()
            .usart_mode()
            .variant()
            .unwrap()
            .into()
    }

    /// Clear peripheral errors in hardware.
    #[inline(always)]
    fn clear_errors(&mut self) {
        self.reg()
            .us_cr_usart_mode()
            .write(|w| w.rststa().set_bit());
    }
}

/// Possible [`Usart`] errors.
#[derive(Debug, Eq, PartialEq)]
pub enum UsartError {
    /// The calculated prescaler does not fit in a [`u16`].
    PrescalerOverflow,
    /// The calculated prescaler is 0, which would disable the [`Uart`].
    PrescalerDisablesUart,
    /// The requested [`UsartMode`] is not supported.
    InvalidMode,
    /// A [`UsartMode::Uart`] error occured.
    Uart(UartError),
    /// A [`UsartMode::Spi`] error occured.
    Spi(SpiError),
}

/// Peripheral metadata for the [`Usart`].
#[allow(missing_docs)]
pub trait UsartMeta: generics::Sealed {
    const REG: *const RegisterBlock;
    const PID: PeripheralIdentifier;
}
/// A trait denotation of valid [pins](`crate::pio::Pin`) that can be used to configure the [`Usart`].
///
/// *note* that different pin configurations allow for different modes of operation ( [`spi`], [`uart`] )
pub trait UsartPins: generics::Sealed {
    /// Denotes wether the set of pins allows for setting the [`Usart`] in [`spi`] host mode.
    const SPI_HOST_MODE_POSSIBLE: bool;
    /// Denotes wether the set of pins allows for setting the [`Usart`] in [`spi`] device mode.
    const SPI_DEVICE_MODE_POSSIBLE: bool;
    /// Denotes wether the set of pins allows for setting the [`Usart`] in [`uart`] mode.
    const UART_MODE_POSSIBLE: bool;
}

struct SpiContext {
    supported_host: bool,
    supported_client: bool,
}

struct UartContext {
    supported: bool,
}

/// [`UsartMode`] handles that implements [`UsartHandle`] for [`Usart::enter_mode`].
#[derive(Default)]
pub struct UsartHandles<M: UsartMeta> {
    /// A consumable token that yields a [`Uart`].
    pub uart: Token<Uart<M>>,
    /// A consumable token that yields a [`Spi`] in [`Host`] mode.
    pub spi_host: Token<Spi<M, Host>>,
    /// A consumable token that yields a [`Spi`] in [`Client`] mode.
    pub spi_client: Token<Spi<M, Client>>,
}

/// Valid clocks for the [`Usart`] in [`Uart`] mode.
pub trait UsartUartClock: Clock {
    /// Hardware identifier of the [`Clock`].
    const SRC: UsartClockSource;
}

impl UsartUartClock for HostClock {
    const SRC: UsartClockSource = UsartClockSource::MCK;
}

impl UsartUartClock for Pck<Pck4> {
    const SRC: UsartClockSource = UsartClockSource::PCK;
}

impl<M: UsartMeta> Token<Uart<M>> {
    /// Consume the [`Token`] in favour of a [`Uart`].
    pub fn configure(
        self,
        usart: &Usart<M>,
        clk: &impl UsartUartClock,
        cfg: UartConfiguration,
    ) -> Result<Uart<M>, UsartError> {
        if !usart.uart.supported {
            return Err(UsartError::InvalidMode);
        }

        Uart::new(clk, cfg)
    }
}

impl<M: UsartMeta> Token<Spi<M, Host>> {
    /// Consume the [`Token`] in favour of a [`Spi`].
    pub fn configure(
        self,
        usart: &Usart<M>,
        mck: &HostClock,
        cfg: SpiConfig,
    ) -> Result<Spi<M, Host>, UsartError> {
        if !usart.spi.supported_host {
            return Err(UsartError::InvalidMode);
        }

        Spi::new(mck, cfg)
    }
}

impl<M: UsartMeta> Token<Spi<M, Client>> {
    /// Consume the [`Token`] in favour of a [`Spi`].
    pub fn configure(
        self,
        usart: &Usart<M>,
        mck: &HostClock,
        cfg: SpiConfig,
    ) -> Result<Spi<M, Client>, UsartError> {
        if !usart.spi.supported_client {
            return Err(UsartError::InvalidMode);
        }

        Spi::new(mck, cfg)
    }
}

/// USART peripheral abstraction.
pub struct Usart<M: UsartMeta> {
    _meta: PhantomData<M>,
    spi: SpiContext,
    uart: UartContext,
}
impl<M: UsartMeta> generics::Sealed for Usart<M> {}

impl<M: UsartMeta> RegisterAccess<M> for Usart<M> {}

impl<M: UsartMeta> Usart<M> {
    fn new<Pins: UsartPins>(_pins: Pins, mck: &mut HostClock) -> (UsartHandles<M>, Self) {
        mck.enable_peripheral(M::PID);
        let usart = Self {
            _meta: PhantomData,
            spi: SpiContext {
                supported_host: Pins::SPI_HOST_MODE_POSSIBLE,
                supported_client: Pins::SPI_DEVICE_MODE_POSSIBLE,
            },
            uart: UartContext {
                supported: Pins::UART_MODE_POSSIBLE,
            },
        };

        let handles = UsartHandles {
            uart: Token::default(),
            spi_host: Token::default(),
            spi_client: Token::default(),
        };

        (handles, usart)
    }

    #[inline(always)]
    fn reg(&self) -> &RegisterBlock {
        unsafe { &*M::REG }
    }
}

impl From<HwUsartMode> for UsartMode {
    #[inline(always)]
    fn from(m: HwUsartMode) -> Self {
        use HwUsartMode::*;

        match m {
            NORMAL => Self::Uart,
            SPI_MASTER => Self::Spi(SpiMode::Host),
            SPI_SLAVE => Self::Spi(SpiMode::Client),
            _ => unimplemented!(),
        }
    }
}

impl<M: UsartMeta> Usart<M> {
    /// Configures the [`Usart`] to work in a specified [`UsartMode`]
    pub fn enter_mode<H: UsartHandle<M>>(&mut self, mode: &H) {
        // NOTE: Safe: this function can only be called after a
        // successful as_*, meaning that we do not need to check for
        // support again.

        self.disable();

        // Wipe current config and reset target mode.
        unsafe {
            mode.reset(self);
        }
        self.reg().us_cr_usart_mode().write(|w| {
            w.rsttx().set_bit();
            w.rstrx().set_bit();
            w
        });

        let pres = unsafe { mode.configure(self) };
        self.reg().us_brgr.write(|w| unsafe { w.cd().bits(pres) });

        self.clear_errors();
        self.enable();
    }

    /// Return the current operating mode of the [`Usart`].
    #[inline]
    pub fn mode(&self) -> UsartMode {
        <Self as RegisterAccess<M>>::mode(self)
    }

    /// Disables the [`Usart`]. Does nothing if the [`Usart`] is
    /// already disabled.
    #[inline(always)]
    pub fn disable(&mut self) {
        self.reg().us_cr_usart_mode().write(|w| {
            w.rxdis().set_bit();
            w.txdis().set_bit();
            w
        });
    }

    /// Re-enables the [`Usart`]. Does nothing if the [`Usart`] is
    /// already enabled.
    #[inline(always)]
    pub fn enable(&mut self) {
        self.reg().us_cr_usart_mode().write(|w| {
            w.txen().set_bit();
            w.rxen().set_bit();
            w
        });
    }

    /// Calculate the [`Usart`] prescaler as per §46.6.1.1
    fn calc_pres(
        freq: Hertz,
        baud_rate: crate::serial::Bps,
        oversample: bool,
    ) -> Result<u16, UsartError> {
        // C.f. §46.7.3
        const MIN_OVERSAMPLE: u32 = 8;
        let oversample_factor = MIN_OVERSAMPLE * if oversample { 2 } else { 1 };

        let pres: u16 = (freq / baud_rate.0 / oversample_factor)
            .try_into()
            .map_err(|_| UsartError::PrescalerOverflow)?;
        if pres == 0 {
            return Err(UsartError::PrescalerDisablesUart);
        }
        Ok(pres)
    }
}

impl<M: UsartMeta> crate::generics::events::EventHandler for Usart<M> {
    type EventSource = Event;

    fn listen(&mut self, event: Self::EventSource) {
        self.reg()
            .us_ier_usart_mode()
            .write(|w| unsafe { w.bits(event as u32) });
    }

    fn unlisten(&mut self, event: Self::EventSource) {
        self.reg()
            .us_idr_usart_mode()
            .write(|w| unsafe { w.bits(event as u32) });
    }

    fn irq(&mut self) -> u32 {
        self.reg().us_csr_usart_mode().read().bits()
    }
}

macro_rules! impl_pins {
    (
        $Usart:ident: {
            $(
                $Pins:ty: {
                    // Denotes wether the pin tuple allows the usart to enter spi host mode
                    SPI_HOST: $SpiHostModePossible:literal,
                    // Denotes wether the pin tuple allows the usart to enter spi device mode
                    SPI_DEVICE: $SpiDeviceModePossible:literal,
                    // Denotes wether the pin tuple allows the usart to enter uart mode
                    UART: $UartModePossible:literal,
                }
            ),+
        }
    ) => {
        paste! {
            $(
                #[doc = "Initiating a [`Usart`] with these pins allows for the following modes of operation\n
- SPI_HOST : " [<$SpiHostModePossible>] "\n
- SPI_DEVICE : " [<$SpiDeviceModePossible>] "\n
- UART : " [<$UartModePossible>] ""]
                impl generics::Sealed for $Pins {}
                impl [<$Usart Pins>] for $Pins {}

                impl UsartPins for $Pins {
                    const SPI_HOST_MODE_POSSIBLE: bool = $SpiHostModePossible;
                    const SPI_DEVICE_MODE_POSSIBLE: bool = $SpiDeviceModePossible;
                    const UART_MODE_POSSIBLE: bool = $UartModePossible;
                }
            )+
        }
    }
}

macro_rules! impl_usart {
    (
        $(
            $( #[$cfgUsart:meta] )?
                $Usart:ident: {
                    SCK: [
                        $( #[$cfgSck:meta] )?
                        $SckPin:ty
                    ],
                    RX: [ $RxPin:ty ],
                    TX: [ $TxPin:ty ],
                    // RI: [ $RiPin:ty ],
                    // DSR: [ $DsrPin:ty ],
                    // DCD: [ $DcdPin:ty ],
                    // DTR: [ $DtrdPin:ty ],
                    CTS: [
                        $( #[$cfgCts:meta] )?
                        $CtsPin:ty
                    ],
                    RTS: [ $RtsPin:ty ],
                },
        )+
    ) => {
        paste! {
            $(
                $( #[$cfgUsart] )?
                mod [<$Usart:lower _impl>] {
                    use super::*;

                    #[doc = "Type-level variant denoting [`" [<$Usart:upper>] "`]."]
                    pub enum $Usart {}

                    impl generics::Sealed for $Usart {}
                    impl UsartMeta for $Usart {
                        const REG: *const RegisterBlock = [<$Usart:upper>]::ptr();
                        const PID: PeripheralIdentifier = PeripheralIdentifier::[<$Usart:upper>];
                    }

                    #[doc = "Trait denoting a valid set of [`Pin`]s for [`" [<$Usart:upper>] "`]. Extends the [`UsartPins`] trait."]
                    pub trait [<$Usart Pins>] : UsartPins {}

                    impl_pins!(
                        $Usart: {
                            ($TxPin, $RxPin): {
                                SPI_HOST: false,
                                SPI_DEVICE: false,
                                UART: true,
                            }
                        }
                    );
                    $( #[$cfgSck] )?
                    impl_pins!(
                        $Usart: {
                            ($TxPin, $RxPin, $SckPin, $RtsPin): {
                                SPI_HOST: true,
                                SPI_DEVICE: false,
                                UART: true,
                            }
                        }
                    );
                    // XXX Here we want to check both $cfgSck and
                    // $cfgSck, but it does not seem possible to
                    // combine meta macro variables. For now this
                    // works because the only impl_usart! entry that
                    // does not support CTS also does not support SCK.
                    $( #[$cfgSck] )?
                    impl_pins!(
                        $Usart: {
                            ($TxPin, $RxPin, $SckPin, $CtsPin): {
                                SPI_HOST: false,
                                SPI_DEVICE: true,
                                UART: true,
                            },
                            ($TxPin, $RxPin, $SckPin, $RtsPin, $CtsPin): {
                                SPI_HOST: true,
                                SPI_DEVICE: true,
                                UART: true,
                            }
                        }
                    );

                    impl Usart<$Usart> {
                        #[doc = "Create a new [`Usart`] from a [`" [<$Usart:upper>] "`] and associated [`Pin`]s.\n
The [pins]("[<$Usart Pins>]") specified determine what [`UsartMode`]s are valid for the [`Usart`]."]
                        pub fn [<new_ $Usart:lower>]<Pins: [<$Usart Pins>]> (
                            _usart: [<$Usart:upper>],
                            pins: Pins,
                            mck: &mut HostClock,
                        ) -> (UsartHandles<$Usart>, Self) {
                            Self::new(pins, mck)
                        }
                    }
                }
                $( #[$cfgUsart] )?
                pub use [<$Usart:lower _impl>]::*;
            )+
        }
    };
}

impl_usart!(
    Usart0: {
        SCK: [
            #[cfg(not(feature = "__pins-64"))]
            Pin<PB13,PeripheralC>
        ],
        RX: [ Pin<PB0,PeripheralC> ],
        TX: [ Pin<PB1,PeripheralC> ],
        // Unused assocs
        //
        // RI: [ Pin<PD3,PeripheralD> ],
        // DSR: [ Pin<PD2,PeripheralD> ],
        // DCD: [ Pin<PD0,PeripheralD> ],
        // DTR: [ Pin<PD1,PeripheralD> ],
        CTS: [ Pin<PB2,PeripheralC> ],
        RTS: [ Pin<PB3,PeripheralC> ],
    },
    #[cfg(feature = "reconfigurable-system-pins")]
    Usart1: {
        SCK: [
            #[cfg(not(feature = "__pins-64"))]
            Pin<PA23,PeripheralA>
        ],
        RX: [ Pin<PA21,PeripheralA> ],
        TX: [ Pin<PB4,PeripheralD> ],
        // Unused assocs
        //
        // RI: [ Pin<PA29,PeripheralA> ],
        // DSR: [ Pin<P28,PeripheralA> ],
        // DCD: [ Pin<PA26,PeripheralA> ],
        // DTR: [ Pin<PA27,PeripheralA> ],
        CTS: [
            #[cfg(not(feature = "__pins-64"))]
            Pin<PA25,PeripheralA>
        ],
        RTS: [ Pin<PA24,PeripheralA> ],
    },
    #[cfg(not(feature = "__pins-64"))]
    Usart2: {
        SCK: [ Pin<PD17,PeripheralB> ],
        RX: [ Pin<PD15, PeripheralB> ],
        TX: [ Pin<PD16, PeripheralB> ],
        // Unused assocs
        //
        // RI: [ Pin<PD3,PeripheralD> ],
        // DSR: [ Pin<PD7,PeripheralD> ],
        // DCD: [ Pin<PD4,PeripheralD> ],
        // DTR: [ Pin<PD5,PeripheralD> ],
        CTS: [ Pin<PD19,PeripheralB> ],
        RTS: [ Pin<PD18,PeripheralB> ],
    },
);

cfg_if::cfg_if! {
    if #[cfg(all(feature = "usart-spi-host-without-select", not(feature = "__pins-64")))] {
        impl_pins!(
            Usart0: {
                (/* TX */ Pin<PB1, PeripheralC>, /* RX */ Pin<PB0, PeripheralC>, /* SCK */ Pin<PB13, PeripheralC>): {
                    SPI_HOST: true,
                    SPI_DEVICE: false,
                    UART: true,
                }
            }
        );
        #[cfg(feature = "reconfigurable-system-pins")]
        impl_pins!(
            Usart1: {
                (/* TX */ Pin<PB4, PeripheralD>, /* RX */ Pin<PA21, PeripheralA>, /* SCK */ Pin<PA23, PeripheralA>): {
                    SPI_HOST: true,
                    SPI_DEVICE: false,
                    UART: true,
                }
            }
        );
        impl_pins!(
            Usart2: {
                (/* TX */ Pin<PD16, PeripheralB>, /* RX */ Pin<PD15, PeripheralB>, /* SCK */ Pin<PD17, PeripheralB>): {
                    SPI_HOST: true,
                    SPI_DEVICE: false,
                    UART: true,
                }
            }
        );
    }
}

pub mod spi;
pub mod uart;
pub use spi::*;
pub use uart::*;
