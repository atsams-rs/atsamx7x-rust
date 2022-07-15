/*!
Clock hierarchy configuration
---

This module exposes a safe and expressive API in order to configure
the device's clock hierarchy. It is based upon clock singletons
([`Token`]s), which are consumed during configuration of a certian
clock, ensuring that clocks cannot be modified when it is a dependency
for another clock or peripheral.

Names and dependencies for each clock mirrors that of Figure 31-1 in
§31.3. In this figure, all clocks except the USB FS Clock (USB_48M),
and GCLKx can be configured in the current API.

Out of reset, the [`SlowClock`] and [`MainClock`] are running in
`InternalRC` mode; the [`ProcessorClock`] and [`HostClock`] are fed by
the [`MainClock`], at 12 MHz. This reset state is not mirrored in the
API: when wrapped in a [`Token`], a clock is presumed to be disabled.

With the current API, each clock can only be configured once.

# Example: single clock configuration

For example, if we want to configure the [`MainClock`]:

```no_run
# use hal::clocks::{self,*};
use atsamx7x_hal as hal;
use hal::fugit::RateExtU32;

let pac = hal::target_device::Peripherals::take().unwrap();
let clocks = Tokens::new((pac.PMC, pac.SUPC, pac.UTMI), &pac.WDT.into());
let mainck = clocks
    .mainck
    .configure_external_normal(12.MHz())
    .unwrap();
```

The [`Token<MainClock>`] is consumed above. If we try to configure it
again, compilation will fail:

```compile_fail
# use hal::clocks::{self, *};
# use atsamx7x_hal as hal;
# use hal::fugit::RateExtU32;
#
# let pac = hal::target_device::Peripherals::take().unwrap();
# let clocks = Tokens::new((pac.PMC, pac.SUPC, pac.UTMI), &pac.WDT.into());
# let mainck = clocks
#     .mainck
#     .configure_external_normal(12.MHz())
#     .unwrap();
// Compilation failure: use of moved value clocks.mainck
let another_mainck = clocks
    .mainck
    .configure_internal(MainRcFreq::_12_MHZ)
    .unwrap();
```

Notice that the [`Watchdog`] is *enabled* out of reset, hence why
[`Tokens::new`] requires a reference to [`Watchdog<Disabled>`].

[`Watchdog`]: crate::watchdog::Watchdog
[`Watchdog::Disabled`]: crate::watchdog::Watchdog<crate::watchdog::Disabled>

# Example: full-hierarchy configuration

```no_run
# use hal::clocks::{self, *};
use atsamx7x_hal as hal;
use hal::efc::{Efc, VddioLevel};
use hal::fugit::RateExtU32;

// configure the clock hierarchy
let pac = hal::target_device::Peripherals::take().unwrap();
let clocks = Tokens::new((pac.PMC, pac.SUPC, pac.UTMI), &pac.WDT.into());
let slck = clocks.slck.configure_external_normal();
let mainck = clocks
    .mainck
    .configure_external_normal(12.MHz())
    .unwrap();
let pllack = clocks
    .pllack
    .configure(&mainck, PllaConfig { div: 1, mult: 12 })
    .unwrap();
let upllck = clocks.upllck.configure(&mainck).unwrap();
let upllckdiv = clocks.upllckdiv.configure(&upllck, UpllDivider::Div2);
let pck: Pck<Pck4> = clocks.pcks.pck4.configure(&mainck, 1);

// configure the processor and peripheral clocks
let mut efc = Efc::new(pac.EFC, VddioLevel::V3);
let (hclk, mck) = HostClockController::new(clocks.hclk, clocks.mck)
    .configure(
        &pllack,
        &mut efc,
        HostClockConfig {
            pres: MckPrescaler::CLK_1,
            div: MckDivider::EQ_PCK,
        },
    )
    .unwrap();
```
*/

use crate::target_device::{pmc, supc, utmi, PMC, SUPC, UTMI};
use crate::watchdog::{Disabled, Watchdog};

use core::marker::PhantomData;
pub use fugit::{HertzU32 as Hertz, MegahertzU32 as Megahertz};
use paste::paste;

/// Internal "Main RC" (for [`MainClock`]) or "Slow RC" (for
/// [`SlowClock`]) oscillator.
pub enum InternalRC {}
/// External crystal powered by the MCU that is connected to XIN and
/// XOUT (for [`MainClock`]) or XIN32 and XOUT32 (for [`SlowClock`]).
pub enum ExternalNormal {}
/// External clock signal connected to XIN, XOUT potentially
/// unconnected (for [`MainClock`]); or XIN32, XOUT32 potentially
/// unconnected (for [`SlowClock`]). Bypasses the oscillator otherwise
/// used when using [`ExternalNormal`].
pub enum ExternalBypass {}

/// Time to wait until an observed clock is stable, from the
/// perpective of SLCK (@ 32.768KHz).
///
/// Refer to §31.20.8, for example.
const COMMON_WAIT_UNTIL_STABLE_62_MILLISECS: u8 = u8::MAX;

/// A generic device clock.
pub trait Clock {
    /// Returns the (calculated) frequency of this clock.
    fn freq(&self) -> Hertz;
}

unsafe trait RegisterAccess {
    #[inline(always)]
    fn pmc(&self) -> &pmc::RegisterBlock {
        unsafe { &*PMC::ptr() }
    }

    #[inline(always)]
    fn supc(&self) -> &supc::RegisterBlock {
        unsafe { &*SUPC::ptr() }
    }

    #[inline(always)]
    fn utmi(&self) -> &utmi::RegisterBlock {
        unsafe { &*UTMI::ptr() }
    }
}

/// A [`Clock`] singleton that is consumed on configuration.
pub struct Token<C: Clock>(PhantomData<C>);
impl<C: Clock> Token<C> {
    pub(crate) fn new() -> Self {
        Self(PhantomData)
    }
}

// Safe: RegisterBlock owner is consumed during Tokens creation
unsafe impl<C: Clock> RegisterAccess for Token<C> {}

/// Set of [`Token`]s for all device [`Clock`]s.
#[allow(missing_docs)]
pub struct Tokens {
    pub slck: Token<SlowClock<InternalRC>>,
    pub mainck: Token<MainClock<InternalRC>>,
    pub pllack: Token<PllaClock>,
    pub upllck: Token<UpllClock>,
    pub upllckdiv: Token<UpllDivClock>,
    pub hclk: Token<ProcessorClock>,
    pub mck: Token<HostClock>,
    pub pcks: PckTokens,
}

impl Tokens {
    /// Create the set of all [`Clock`] [`Token`]s.
    pub fn new(_clocks: (PMC, SUPC, UTMI), _wd: &Watchdog<Disabled>) -> Self {
        Self {
            slck: Token::new(),
            mainck: Token::new(),
            pllack: Token::new(),
            upllck: Token::new(),
            upllckdiv: Token::new(),
            hclk: Token::new(),
            mck: Token::new(),
            pcks: PckTokens::new(),
        }
    }
}

/// Possible [`Clock`] configuration errors.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ClockError {
    /// [`MainClock`] frequency must be in the range of 3MHz to 20MHz
    /// (inclusive).
    InvalidMainCk,

    /// [`PllaClock`] multiplier and divider must be in the ranges of
    /// `2..=63` and `1..=127`, respectively.
    InvalidPllaCk,

    /// [`UpllClock`] input frequency must be 12 MHz or 16 MHz.
    InvalidUpllCk(Hertz),

    /// [`HostClockController`] input frequency is too high: a valid
    /// number of flash wait states could not be calculated.
    ///
    /// Lower the input frequency to 137 MHz or 150MHz (for
    /// [`VddioLevel::V1`] and [`VddioLevel::V3`], respectively).
    ///
    /// [`VddioLevel::V1`]: crate::efc::VddioLevel::V1
    /// [`VddioLevel::V3`]: crate::efc::VddioLevel::V3
    InvalidHccFreq(Megahertz),

    /// The V70/V71 must be driven with VDDIO = 3.3V, typical.
    #[cfg(any(feature = "v70", feature = "v71"))]
    InvalidVddioLevel,
}

#[doc(hidden)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u32)]
pub enum PeripheralIdentifier {
    /// 0   (NVIC + !PMC CC) Supply Controller
    SUPC = 0,
    /// 1   (NVIC + !PMC CC) Reset Controller
    RSTC = 1,
    /// 2   (NVIC + !PMC CC) Real Time Clock
    RTC = 2,
    /// 3   (NVIC + !PMC CC) Real Time Timer
    RTT = 3,
    /// 4   (NVIC + !PMC CC) Watchdog Timer
    WDT = 4,
    /// 5   (NVIC + !PMC CC) Power Management Controller
    PMC = 5,
    /// 6   (NVIC + !PMC CC) Enhanced Embedded Flash Controller
    EFC = 6,
    /// 7   (NVIC + PMC CC) Universal Asynchronous Receiver/Transmitter
    UART0 = 7,
    /// 8   (NVIC + PMC CC) Universal Asynchronous Receiver/Transmitter
    UART1 = 8,
    /// 9   (!NVIC + PMC CC) Static Memory Controller
    SMC = 9,
    /// 10  (NVIC + PMC CC) Parallel I/O Controller A
    PIOA = 10,
    /// 11  (NVIC + PMC CC) Parallel I/O Controller B
    PIOB = 11,
    /// 12  (NVIC + PMC CC) Parallel I/O Controller C
    PIOC = 12,
    /// 13  (NVIC + PMC CC) Universal Synchronous/Asynchronous Receiver/Transmitter
    USART0 = 13,
    /// 14  (NVIC + PMC CC) Universal Synchronous/Asynchronous Receiver/Transmitter
    USART1 = 14,
    /// 15  (NVIC + PMC CC) Universal Synchronous/Asynchronous Receiver/Transmitter
    USART2 = 15,
    /// 16  (NVIC + PMC CC) Parallel I/O Controller D
    PIOD = 16,
    /// 17  (NVIC + PMC CC) Parallel I/O Controller E
    PIOE = 17,
    /// 18  (NVIC + PMC CC) Multimedia Card Interface
    HSMCI = 18,
    /// 19  (NVIC + PMC CC) Two-wire Interface (I2C-compatible)
    TWIHS0 = 19,
    /// 20  (NVIC + PMC CC) Two-wire Interface (I2C-compatible)
    TWIHS1 = 20,
    /// 21  (NVIC + PMC CC) Serial Peripheral Interface
    SPI0 = 21,
    /// 22  (NVIC + PMC CC) Synchronous Serial Controller
    SSC = 22,
    /// 23  (NVIC + PMC CC) 16-bit Timer Counter 0, Channel 0
    TC0_CHANNEL0 = 23,
    /// 24  (NVIC + PMC CC) 16-bit Timer Counter 0, Channel 1
    TC0_CHANNEL1 = 24,
    /// 25  (NVIC + PMC CC) 16-bit Timer Counter 0, Channel 2
    TC0_CHANNEL2 = 25,
    /// 26  (NVIC + PMC CC) 16-bit Timer Counter 1, Channel 0
    TC1_CHANNEL0 = 26,
    /// 27  (NVIC + PMC CC) 16-bit Timer Counter 1, Channel 1
    TC1_CHANNEL1 = 27,
    /// 28  (NVIC + PMC CC) 16-bit Timer Counter 1, Channel 2
    TC1_CHANNEL2 = 28,
    /// 29  (NVIC + PMC CC) Analog Front-End Controller
    AFEC0 = 29,
    /// 30  (NVIC + PMC CC) Digital-to-Analog Converter
    DACC = 30,
    /// 31  (NVIC + PMC CC) Pulse Width Modulation Controller
    PWM0 = 31,
    /// 32  (NVIC + PMC CC) Integrity Check Monitor
    ICM = 32,
    /// 33  (NVIC + PMC CC) Analog Comparator Controller
    ACC = 33,
    /// 34  (NVIC + PMC CC) USB Host / Device Controller
    USBHS = 34,
    /// 35  (NVIC + PMC CC) CAN IRQ Line 0
    MCAN0 = 35,
    /// 36  (NVIC + !PMC CC) CAN IRQ Line 1
    MCAN0INT1 = 36,
    /// 37  (NVIC + PMC CC) CAN IRQ Line 0
    MCAN1 = 37,
    /// 38  (NVIC + !PMC CC) CAN IRQ Line 1
    MCAN1INT1 = 38,
    /// 39  (NVIC + PMC CC) Ethernet MAC
    GMAC = 39,
    /// 40  (NVIC + PMC CC) Analog Front End Controller
    AFEC1 = 40,
    /// 41  (NVIC + PMC CC) Two-wire Interface
    TWIHS2 = 41,
    /// 42  (NVIC + PMC CC) Serial Peripheral Interface
    SPI1 = 42,
    /// 43  (NVIC + PMC CC) Quad I/O Serial Peripheral Interface
    QSPI = 43,
    /// 44  (NVIC + PMC CC) Universal Asynchronous Receiver/ Transmitter
    UART2 = 44,
    /// 45  (NVIC + PMC CC) Universal Asynchronous Receiver/ Transmitter
    UART3 = 45,
    /// 46  (NVIC + PMC CC) Universal Asynchronous Receiver/ Transmitter
    UART4 = 46,
    /// 47  (NVIC + PMC CC) 16-bit Timer Counter 2, Channel 0
    TC2_CHANNEL0 = 47,
    /// 48  (NVIC + PMC CC) 16-bit Timer Counter 2, Channel 1
    TC2_CHANNEL1 = 48,
    /// 49  (NVIC + PMC CC) 16-bit Timer Counter 2, Channel 2
    TC2_CHANNEL2 = 49,
    /// 50  (NVIC + PMC CC) 16-bit Timer Counter 3, Channel 0
    TC3_CHANNEL0 = 50,
    /// 51  (NVIC + PMC CC) 16-bit Timer Counter 3, Channel 1
    TC3_CHANNEL1 = 51,
    /// 52  (NVIC + PMC CC) 16-bit Timer Counter 3, Channel 2
    TC3_CHANNEL2 = 52,
    /// 53  (NVIC + PMC CC) MediaLB IRQ 0
    MLB_IRQ0 = 53,
    /// 54  (NVIC + !PMC CC) MediaLB IRQ 1
    MLB_IRQ1 = 54,
    /// 55  (NVIC + !PMC CC) Reserved
    _RESERVED = 55,
    /// 56  (NVIC + PMC CC) Advanced Encryption Standard
    AES = 56,
    /// 57  (NVIC + PMC CC) True Random Number Generator
    TRNG = 57,
    /// 58  (NVIC + PMC CC) DMA Controller
    XDMAC = 58,
    /// 59  (NVIC + PMC CC) Image Sensor Interface
    ISI = 59,
    /// 60  (NVIC + PMC CC) Pulse Width Modulation Controller
    PWM1 = 60,
    /// 61 (NVIC:FPU + !PMC CC) ARM Floating Point Unit interrupt associated with OFC, UFC, IOC, DZC and IDC bits
    ARM = 61,
    /// 62  (NVIC + !PMC CC) SDRAM Controller
    SDRAMC = 62,
    /// 63  (NVIC + !PMC CC) Reinforced Safety Watchdog Timer
    RSWDT = 63,
    /// 64  (NVIC:CCW + !PMC CC) ARM Cache ECC Warning
    ARM_CACHE_ECC_WARNING = 64,
    /// 65  (NVIC:CCF + !PMC CC) ARM Cache ECC Fault
    ARM_CACHE_ECC_FAULT = 65,
    /// 66  (NVIC:Q1 + !PMC CC) GMAC Queue 1 Interrupt signal toggled on a DMA write to the first word of each DMA data buffer associated with queue 1
    GMAC_Q1 = 66,
    /// 67  (NVIC:Q2 + !PMC CC) GMAC Queue 2 Interrupt signal toggled on a DMA write to the first word of each DMA data buffer associated with queue 2
    GMAC_Q2 = 67,
    /// 68  (NVIC:IX + !PMC CC) –Floating Point Unit Interrupt IXC associated with FPU cumulative exception bit
    ARM_FPU_IXC_FPU = 68,
    /// 69  (NVIC + PMC CC) Inter-IC Sound Controller
    // TODO: How does this get enabled with PMC? The PMC bits only go up to 63...
    I2SC0 = 69,
    /// 70  (NVIC + PMC CC) Inter-IC Sound Controller
    // TODO: How does this get enabled with PMC? The PMC bits only go up to 63...
    I2SC1 = 70,
    /// 71  (NVIC:Q3 + !PMC CC) GMAC Queue 3 Interrupt signal toggled on a DMA write to the first word of each DMA data buffer associated with queue 3
    GMAC_Q3 = 71,
    /// 72  (NVIC:Q4 + !PMC CC) GMAC Queue 4 Interrupt signal toggled on a DMA write to the first word of each DMA data buffer associated with queue 4
    GMAC_Q4 = 72,
    /// 73  (NVIC:Q5 + !PMC CC) GMAC Queue 5 Interrupt signal toggled on a DMA write to the first word of each DMA data buffer associated with queue 5
    GMAC_Q5 = 73,
}

impl PeripheralIdentifier {
    pub(crate) const fn requires_enable(&self) -> bool {
        use PeripheralIdentifier::*;
        // These pids do not require PMC clocking
        !matches!(
            self,
            SUPC | RSTC
                | RTC
                | RTT
                | WDT
                | PMC
                | EFC
                | MCAN0INT1
                | MCAN1INT1
                | MLB_IRQ1
                | _RESERVED
                | ARM
                | SDRAMC
                | RSWDT
                | ARM_CACHE_ECC_WARNING
                | ARM_CACHE_ECC_FAULT
                | GMAC_Q1
                | GMAC_Q2
                | ARM_FPU_IXC_FPU
                | GMAC_Q3
                | GMAC_Q4
                | GMAC_Q5
        )
    }
}

mod hcc;
pub use hcc::*;

mod slck;
pub use slck::*;

mod mainck;
pub use mainck::*;

mod pllack;
pub use pllack::*;

mod upllck;
pub use upllck::*;

mod pck;
pub use pck::*;
