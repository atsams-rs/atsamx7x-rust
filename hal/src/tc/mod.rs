/*!
Timer Counter (TC)
---

The [`Tc`] is a peripheral that contain three [`Channel`]s. Each [`Channel`] can [`Generate`] or [`Capture`] a signal.
In [`Generate`], PWM signals can be emitted on connected [`Pin`]s, drive another [`Channel`] via [`Channel::chain`], and provide delay/scheduling primitives.
In [`Capture`], a [`Channel`] can measure frequencies on an [`ChannelInputPin<_, _, A>`].

Refer to §50 for a full description on the capabilities offered by a [`Tc`].

[`Pin`]: crate::pio::Pin
[`ehal::timer::Countdown`]: crate::ehal::timer::CountDown

# Example usage

### Sampling a [`ChannelInputPin<_, _, A>`] frequency

```no_run
# use atsamx7x_hal as hal;
# use hal::pio::*;
# use hal::clocks::*;
# use hal::efc::*;
# use hal::tc::*;
# use hal::fugit::ExtU32;
# let pac = hal::pac::Peripherals::take().unwrap();
# let (slck, mut mck) = Tokens::new((pac.PMC, pac.SUPC, pac.UTMI), &pac.WDT.into()).por_state(&mut Efc::new(pac.EFC, VddioLevel::V3));
let banka = hal::pio::BankA::new(
    pac.PIOA,
    &mut mck,
    &slck,
    BankConfiguration::default(),
);
let ioa = banka.pa0.into_peripheral();

let tc = Tc::new_tc0(pac.TC0, &mut mck);
let mut counter = tc
    .channel_0
    .capture::<{12_000_000 / 128}>(&mck, ioa)
    .unwrap()
    .into_freq_measure(CounterConfiguration {
        sampling: CaptureSamplingRatio::Eight,
        leading_criteria: CaptureMode::Rising,
        trailing_criteria: CaptureMode::Rising,
    });

counter.sample_freq(100.millis());
```

### Create a [`rtic_monotonic::Monotonic`]

```no_run
# use atsamx7x_hal as hal;
# use hal::pio::*;
# use hal::clocks::*;
# use hal::efc::*;
# use hal::tc::*;
# let pac = hal::pac::Peripherals::take().unwrap();
# let (slck, mut mck) = Tokens::new((pac.PMC, pac.SUPC, pac.UTMI), &pac.WDT.into()).por_state(&mut Efc::new(pac.EFC, VddioLevel::V3));
let tc = Tc::new_tc0(pac.TC0, &mut mck);
let driver = tc
    .channel_0
    .generate::<12_000_000>(&mck)
.unwrap();

// Create a Monotonic with a frequency of 1Hz.
let mono: Monotonic<Tc0, Ch1, Channel<Tc0, Ch0, Generate<HostClock, 12_000_000>>, 1> = tc
    .channel_1
    .generate::<12_000_000>(&mck)
    .unwrap()
    .chain(driver)
    .into_monotonic()
    .unwrap();
```

### Create a [`ehal::timer::CountDown`]

```no_run
# use atsamx7x_hal as hal;
# use hal::pio::*;
# use hal::clocks::*;
# use hal::efc::*;
# use hal::tc::*;
# let pac = hal::pac::Peripherals::take().unwrap();
# let (slck, mut mck) = Tokens::new((pac.PMC, pac.SUPC, pac.UTMI), &pac.WDT.into()).por_state(&mut Efc::new(pac.EFC, VddioLevel::V3));
let tc = Tc::new_tc0(pac.TC0, &mut mck);
let driver = tc
    .channel_0
    .generate::<12_000_000>(&mck)
.unwrap();

use hal::ehal::timer::{CountDown, Cancel};
use hal::ehal::blocking::delay::DelayMs;

// Create a Timer with a frequency of 100Hz.
let mut timer: Timer<Tc0, Ch1, Channel<Tc0, Ch0, Generate<HostClock, 12_000_000>>, 100> = tc
    .channel_1
    .generate::<12_000_000>(&mck)
    .unwrap()
    .chain(driver)
    .into_timer()
.unwrap();

timer.start(100.millis());
timer.cancel().unwrap();
timer.delay_ms(100);
```

# [`Monotonic`]/[`Timer`] life-times and limitations

The "life-time" of a [`Channel`] is here defined as the time until its internal 16-bit counter overflows, after which its dependant type may behave differently.

A [`Channel`] is driven by the [`HostClock`] or another [`Channel`]. The maximum life-time of the former (if [`HostClock`] is clocked as 12MHz) is
```
const STATIC_HOSTCLOCK_DIVIDER: u32 = 128;
assert_eq!(
    1 as f32
        / (12_000_000 as f32 / STATIC_HOSTCLOCK_DIVIDER as f32 / u16::MAX as f32),
    0.69904 /* seconds */
);
```
Logically, [`Monotonic`] is not implemented for such a [`Channel`]: a scheduler that is not strictly monotonically increasing for longer than 700ms is not very useful.

For the latter, which is [`Channel::chain`]ed, the driving [`Channel`]'s prescaler is used to generate a clock signal for the downstream [`Channel`].
The maximum life-time for this [`Channel`] is instead
```
const STATIC_HOSTCLOCK_DIVIDER: u32 = 128;
const FEEDBACK_PRESCALER: u32 = 2;
const MAXIMUM_PRESCALER: u32 = u16::MAX as u32;
assert_eq!(
    1 as f32
        / (12_000_000 as f32
            / STATIC_HOSTCLOCK_DIVIDER as f32
            / MAXIMUM_PRESCALER as f32
            / FEEDBACK_PRESCALER as f32
            / MAXIMUM_PRESCALER as f32),
    91623.17 /* seconds */
);
```
With a life-time of a bit over 25h, a [`Monotonic`] implementation is more logical.
To further remove life-time concerns, the [`Monotonic`] extends the underlying [`Channel`] to 32-bits, but see also the next section.

Of course, life-time is balanced with the accuracy (frequency) of the [`Monotonic`]/[`Timer`].
A higher frequency leads to a lower life-time, but a lower frequency may result in real-time deadlines being missed: if a frequency of 1Hz is used, and a task is scheduled for the 1.5s mark, it will not be executed until the 2s mark.

To calculate the life-time of a [`Monotonic`], use the following formula:
```ignore
LIFE_TIME = (1 / FREQ) * u32::MAX
```
For example, at 100Hz, the life-time is ~1.3 years.

Calculating the required frequency for a particular use-case is left as an exercise for the reader.

## [`Monotonic`] clock drift

In the present implementation of [`Monotonic`], clock drift is induced when the internal 16-bit counter overflows.

## Scheduling in RTIC with an overflowing [`Monotonic`]

In RTIC, when a [`rtic_monotonic::Monotonic`] implementation's hardware counter overflows, the scheduler transforms to a semi-monotonic scheduler.
While it will otherwise operate as per usual, the forward-schedulability is halved.
For example, with a ~70s life-time [`Channel`] it is possible to schedule events up to 70s into the future from system start.
When the scheduler eventually overflows, it will only be possible to schedule events up to 35s into the future.

# Interrupt bindings

Each of the 12 [`Channel`]s has its own interrupt vector. However, in
[`crate::pac::Interrupt`], these interrupts are on the form
`TC{0..=11}` instead of a perhaps expected `TC{0..=3}_CHANNEL{0..=2}`.
For convenience, consult the below table when a task or
[`rtic_monotonic::Monotonic`] shall be bound to a [`Channel`]
interrupt:

- [`Channel<Tc0, Ch0, Generate<_, _>>`] : [`crate::pac::Interrupt::TC0`];
- [`Channel<Tc0, Ch1, Generate<_, _>>`] : [`crate::pac::Interrupt::TC1`];
- [`Channel<Tc0, Ch2, Generate<_, _>>`] : [`crate::pac::Interrupt::TC2`];
- [`Channel<Tc1, Ch0, Generate<_, _>>`] : [`crate::pac::Interrupt::TC3`];
- [`Channel<Tc1, Ch1, Generate<_, _>>`] : [`crate::pac::Interrupt::TC4`];
- [`Channel<Tc1, Ch2, Generate<_, _>>`] : [`crate::pac::Interrupt::TC5`];
- [`Channel<Tc2, Ch0, Generate<_, _>>`] : [`crate::pac::Interrupt::TC6`];
- [`Channel<Tc2, Ch1, Generate<_, _>>`] : [`crate::pac::Interrupt::TC7`];
- [`Channel<Tc2, Ch2, Generate<_, _>>`] : [`crate::pac::Interrupt::TC8`];
- [`Channel<Tc3, Ch0, Generate<_, _>>`] : [`crate::pac::Interrupt::TC9`];
- [`Channel<Tc3, Ch1, Generate<_, _>>`] : [`crate::pac::Interrupt::TC10`]; and
- [`Channel<Tc3, Ch2, Generate<_, _>>`] : [`crate::pac::Interrupt::TC11`].

[`Channel<_, _, Inactive>`]s do not generate interrupts.
*/

use crate::clocks::{Clock, Hertz, HostClock, PeripheralIdentifier};
use crate::ehal::blocking::delay;
use crate::ehal::timer;
pub use crate::fugit::{ExtU32, RateExtU32};
use crate::fugit::{
    MicrosDurationU32 as MicrosDuration, MillisDurationU32 as MillisDuration,
    TimerDurationU32 as Duration, TimerInstantU32 as Instant,
};
use crate::generics::{self, CountDownError};
use crate::pac::tc0::{
    tc_channel::{
        cmr_capture_mode::{LDRASELECT_A, LDRBSELECT_A, SBSMPLRSELECT_A},
        cmr_waveform_mode::TCCLKSSELECT_A as TCCLKS,
        sr::R as StatusRegister,
    },
    RegisterBlock, TC_CHANNEL as ChannelRegisterBlock,
};
use crate::pio::*;

use core::marker::PhantomData;

use paste::paste;

#[doc(hidden)]
pub struct HwChannelClock(TCCLKS);
impl HwChannelClock {
    /// C.f. p. 1481, Clock Selection
    fn prescaler(&self) -> u32 {
        match self.0 {
            TCCLKS::TIMER_CLOCK2 => 8,
            TCCLKS::TIMER_CLOCK3 => 32,
            TCCLKS::TIMER_CLOCK4 => 128,
            _ => 1,
        }
    }
}

const CHANNELS_PER_TC: usize = 3;

#[doc(hidden)]
pub trait TcMeta {
    const PIDS: [PeripheralIdentifier; CHANNELS_PER_TC];
    const REG: *const RegisterBlock;
}

/// Trait for mutable hardware register access.
///
/// # Safety
///
/// Implementors of this trait modifies the hardware state.
unsafe trait RegisterAccess<M: TcMeta, I: ChannelId> {
    #[inline(always)]
    fn reg(&self) -> &RegisterBlock {
        unsafe { &*M::REG }
    }

    #[inline(always)]
    fn channel(&self) -> &ChannelRegisterBlock {
        let tc = self.reg();
        match I::DYN {
            DynChannelId::Ch0 => &tc.tc_channel0,
            DynChannelId::Ch1 => &tc.tc_channel1,
            DynChannelId::Ch2 => &tc.tc_channel2,
        }
    }
}

/// Trait for synchronizing the start of all channels in the [`Tc`].
trait SyncChannels<M: TcMeta> {
    /// Start the clocks of all [`Channel`]s in the [`Tc`] at the same
    /// time. Calling this function is required if channels are
    /// [`Channel::chain`]ed.
    ///
    /// # Safety
    ///
    /// Modifies the hardware state of all [`Channel`]s.
    #[inline(always)]
    unsafe fn sync_start_channels() {
        { &*M::REG }.bcr.write(|w| w.sync().set_bit());
    }
}

/// Valid [`Channel`] input clock.
pub trait ChannelClock: Clock {
    #[doc(hidden)]
    const SOURCE: HwChannelClock;

    /// Frequency of the [`Channel`] input clock.
    fn freq(&self) -> Hertz {
        Clock::freq(self)
    }
}
impl ChannelClock for HostClock {
    const SOURCE: HwChannelClock = HwChannelClock(TCCLKS::TIMER_CLOCK4);
}

impl<M: TcMeta, I: ChannelId, C: ChannelClock, const FREQ_HZ: u32> Clock
    for Channel<M, I, Generate<C, FREQ_HZ>>
{
    fn freq(&self) -> Hertz {
        Hertz::from_raw(FREQ_HZ) / C::SOURCE.prescaler()
    }
}

impl<M: TcMeta, C: ChannelClock, const FREQ_HZ: u32> ChannelClock
    for Channel<M, Ch0, Generate<C, FREQ_HZ>>
{
    const SOURCE: HwChannelClock = HwChannelClock(TCCLKS::XC0);
}

impl<M: TcMeta, C: ChannelClock, const FREQ_HZ: u32> ChannelClock
    for Channel<M, Ch1, Generate<C, FREQ_HZ>>
{
    const SOURCE: HwChannelClock = HwChannelClock(TCCLKS::XC1);
}

impl<M: TcMeta, C: ChannelClock, const FREQ_HZ: u32> ChannelClock
    for Channel<M, Ch2, Generate<C, FREQ_HZ>>
{
    const SOURCE: HwChannelClock = HwChannelClock(TCCLKS::XC2);
}

#[doc(hidden)]
pub enum DynChannelId {
    Ch0,
    Ch1,
    Ch2,
}
impl DynChannelId {
    /// C.f §50.7.16, External Clock Signal [0..=2] Selection
    fn external_feedback_field(&self, dependent: &Self) -> u8 {
        use DynChannelId::*;
        match (dependent, self) {
            (Ch0, Ch1) => 2,
            (Ch0, Ch2) => 3,
            (Ch1, Ch0) => 2,
            (Ch1, Ch2) => 3,
            (Ch2, Ch0) => 2,
            (Ch2, Ch1) => 3,
            _ => unreachable!(),
        }
    }
}

#[doc(hidden)]
pub trait ChannelId {
    const DYN: DynChannelId;
}
/// Identifier for the first [`Channel`] in a [`Tc`].
pub enum Ch0 {}
/// Identifier for the second [`Channel`] in a [`Tc`].
pub enum Ch1 {}
/// Identifier for the third [`Channel`] in a [`Tc`].
pub enum Ch2 {}
impl ChannelId for Ch0 {
    const DYN: DynChannelId = DynChannelId::Ch0;
}
impl ChannelId for Ch1 {
    const DYN: DynChannelId = DynChannelId::Ch1;
}
impl ChannelId for Ch2 {
    const DYN: DynChannelId = DynChannelId::Ch2;
}

/// The current state of the [`Channel`].
pub trait ChannelState: generics::Sealed {}
/// The [`Channel`] is inactive and does not generate a signal.
pub enum Inactive {}
/// The [`Channel`] is active and is generating a signal, with an input clock of `FREQ_HZ`Hz.
pub struct Generate<C: ChannelClock, const FREQ_HZ: u32>(PhantomData<C>);
/// The [`Channel`] is active and is capturing input signals.
pub struct Capture<C: ChannelClock, const FREQ_HZ: u32>(PhantomData<C>);

impl generics::Sealed for Inactive {}
impl ChannelState for Inactive {}
impl<C: ChannelClock, const FREQ_HZ: u32> generics::Sealed for Generate<C, FREQ_HZ> {}
impl<C: ChannelClock, const FREQ_HZ: u32> ChannelState for Generate<C, FREQ_HZ> {}
impl<C: ChannelClock, const FREQ_HZ: u32> generics::Sealed for Capture<C, FREQ_HZ> {}
impl<C: ChannelClock, const FREQ_HZ: u32> ChannelState for Capture<C, FREQ_HZ> {}

/// Possible [`Channel`] configuration errors.
///
/// For [`TcError::PrescalerOverflow`] and
/// [`TcError::PrescalerInvalid`], the prescaler is calculated via
///
/// ```no_run
/// # fn func() -> Result<(), <u16 as TryFrom<u32>>::Error> {
/// # let input = 0;
/// # let wanted = 1;
/// let pres: u16 = ((input as f32 / wanted as f32 + 0.5) as u32).try_into()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum TcError {
    /// Calculated prescaler does not fit in a [`u16`].
    ///
    /// Solution: increase the wanted frequency.
    PrescalerOverflow {
        /// The [`Channel`] input frequency.
        input: Hertz,
        /// The wanted [`Channel`] output frequency.
        wanted: Hertz,
    },
    /// Calculated prescaler is below 2. A prescaler of 0 would noop
    /// the channel's clock generation functionality, but a prescaler
    /// of 1 would yield half of the expected frequency.
    ///
    /// Solution: decrease the wanted frequency.
    PrescalerInvalid {
        /// The [`Channel`] input frequency.
        input: Hertz,
        /// The wanted [`Channel`] output frequency.
        wanted: Hertz,
    },
    /// [`Channel`] input clock frequency does not match type argument.
    ///
    /// On this error, the second type argument in
    /// ```no_run
    /// # use atsamx7x_hal as hal;
    /// # use hal::pio::*;
    /// # use hal::clocks::*;
    /// # use hal::efc::*;
    /// # use hal::tc::*;
    /// # let pac = hal::pac::Peripherals::take().unwrap();
    /// # let (slck, mut mck) = Tokens::new((pac.PMC, pac.SUPC, pac.UTMI), &pac.WDT.into()).por_state(&mut Efc::new(pac.EFC, VddioLevel::V3));
    /// let tc = Tc::new_tc0(pac.TC0, &mut mck);
    /// let ch = tc.channel_0.generate::<15_000_000>(&mck).unwrap();
    /// ```
    /// is incorrect: a [`HostClock`] can only be clocked at 12MHz, 8MHz, or 4MHz.
    InputClockFreqMismatch {
        /// The frequency expected by the caller.
        expected: Hertz,
        /// The internally resolved frequency of the input clock.
        actual: Hertz,
    },
}

enum CompareRegister {
    #[allow(dead_code)]
    Ra(u16),
    #[allow(dead_code)]
    Rb(u16),
    Rc(u16),
}

/// A [`Tc`] [`Channel`] and its current state.
///
/// An [`Inactive`] [`Channel`] can be transformed into a [`Channel`]
/// that [`Generate`]s a signal. If a [`Channel`] is
/// [`Channel::chain`]ed, it can be transformed further into a
/// [`Monotonic`].
pub struct Channel<M: TcMeta, I: ChannelId, S: ChannelState> {
    _id: PhantomData<I>,
    _meta: PhantomData<M>,
    _state: PhantomData<S>,
}
impl<M: TcMeta, I: ChannelId, S: ChannelState> generics::Sealed for Channel<M, I, S> {}
unsafe impl<M: TcMeta, I: ChannelId, S: ChannelState> RegisterAccess<M, I> for Channel<M, I, S> {}
impl<M: TcMeta, I: ChannelId, S: ChannelState> Channel<M, I, S> {
    /// Create a new [`Channel`].
    ///
    /// # Safety
    ///
    /// A [`Channel`] can modify hardware, but can be created without
    /// consuming a corresponding singleton.
    const unsafe fn new<So: ChannelState>() -> Self {
        Self {
            _meta: PhantomData,
            _id: PhantomData,
            _state: PhantomData,
        }
    }

    const fn transform<So: ChannelState>(_: Channel<M, I, So>) -> Self {
        Self {
            _meta: PhantomData,
            _id: PhantomData,
            _state: PhantomData,
        }
    }

    /// Disable the [`Channel`]'s input clock.
    fn disable(&mut self) {
        self.channel().ccr.write(|w| w.clkdis().set_bit());
    }

    /// Enable the [`Channel`]'s input clock.
    fn enable(&mut self) {
        self.channel().ccr.write(|w| {
            w.clkdis().clear_bit();
            w.clken().set_bit();

            w
        });
    }

    /// Reset the [`Channel`]s counter and start its input clock.
    fn reset_enable(&mut self) {
        self.channel().ccr.write(|w| {
            w.clkdis().clear_bit();
            w.clken().set_bit();
            w.swtrg().set_bit();

            w
        });
    }

    #[inline]
    fn set_compare(&mut self, val: CompareRegister) {
        match val {
            CompareRegister::Ra(v) => self
                .channel()
                .ra
                .write(|w| unsafe { w.ra().bits(v.into()) }),
            CompareRegister::Rb(v) => self
                .channel()
                .rb
                .write(|w| unsafe { w.rb().bits(v.into()) }),
            CompareRegister::Rc(v) => self
                .channel()
                .rc
                .write(|w| unsafe { w.rc().bits(v.into()) }),
        };
    }

    /// Configures the channel for waveform mode (an signal is generated).
    ///
    /// # Safety
    ///
    /// After this call, the [`Channel`] could be considered in a
    /// state between [`Inactive`] and [`Generate`]. A subsequent call
    /// to [`Channel::tranform`] to [`Generate`] should be performed.
    unsafe fn generate_inner<C: ChannelClock>(&self, _clk: &C) {
        self.channel().cmr_waveform_mode().modify(|_, w| {
            w.wave().set_bit();
            w.tcclks().variant(C::SOURCE.0);

            w
        });
    }

    #[inline(always)]
    fn read_status(&mut self) -> StatusRegister {
        self.channel().sr.read()
    }
}

impl<M: TcMeta, I: ChannelId> Channel<M, I, Inactive> {
    /// Transform the [`Channel`] into one that [`Generate`]s a
    /// signal, driven by the [`HostClock`].
    pub fn generate<const FREQ_HZ: u32>(
        #[allow(unused_mut)] mut self,
        mck: &HostClock,
    ) -> Result<Channel<M, I, Generate<HostClock, FREQ_HZ>>, TcError> {
        let expected = Hertz::from_raw(FREQ_HZ);
        let actual = Clock::freq(mck);
        if actual != expected {
            return Err(TcError::InputClockFreqMismatch { expected, actual });
        }

        unsafe {
            self.generate_inner(mck);
        }

        Ok(Channel::transform(self))
    }

    /// Transforms the [`Channel`] into an input [`Capture`]
    /// [`Channel`], using a [`ChannelInputPin<_, _, A>`].
    pub fn capture<const FREQ_HZ: u32>(
        #[allow(unused_mut)] mut self,
        mck: &HostClock,
        _input: impl ChannelInputPin<M, I, A>,
    ) -> Result<Channel<M, I, Capture<HostClock, FREQ_HZ>>, TcError> {
        let expected = Hertz::from_raw(FREQ_HZ);
        let actual = Clock::freq(mck) / <HostClock as ChannelClock>::SOURCE.prescaler();
        if actual != expected {
            return Err(TcError::InputClockFreqMismatch { expected, actual });
        }

        // NOTE: we write to WAVEFORM because of TCCLKS type variant
        // limitations, but it has the same effct as writing these
        // registers to CAPTURE.
        self.channel().cmr_waveform_mode().modify(|_, w| {
            w.wave().clear_bit();
            w.tcclks().variant(<HostClock as ChannelClock>::SOURCE.0);

            w
        });

        Ok(Channel::transform(self))
    }
}

/// A Timer Counter (TC) peripheral, containing three [`Channel`]s.
pub struct Tc<M: TcMeta> {
    _meta: PhantomData<M>,
    /// The peripheral's first [`Channel`].
    pub channel_0: Channel<M, Ch0, Inactive>,
    /// The peripheral's second [`Channel`].
    pub channel_1: Channel<M, Ch1, Inactive>,
    /// The peripheral's third [`Channel`].
    pub channel_2: Channel<M, Ch2, Inactive>,
}

impl<M: TcMeta> Tc<M> {
    fn new(mck: &mut HostClock) -> Self {
        for pid in M::PIDS {
            mck.enable_peripheral(pid);
        }

        // Safe: the TC block has been consumed.
        let mut tc = Self {
            _meta: PhantomData,
            channel_0: unsafe { Channel::new::<Inactive>() },
            channel_1: unsafe { Channel::new::<Inactive>() },
            channel_2: unsafe { Channel::new::<Inactive>() },
        };

        // Ensure channels are disabled.
        tc.channel_0.disable();
        tc.channel_1.disable();
        tc.channel_2.disable();

        tc
    }
}

/// Denotes whether a [`Pin`] is a `TIOAx` or `TIOBx` pin.
pub trait PinIdentifier: generics::Sealed {}
/// Denotes that the [`Pin`] is a `TIOAx` pin.
pub enum A {}
/// Denotes that the [`Pin`] is a `TIOBx` pin.
pub enum B {}
impl generics::Sealed for A {}
impl PinIdentifier for A {}
impl generics::Sealed for B {}
impl PinIdentifier for B {}

/// [`Pin`] that acts as an input for a [`Channel`].
pub trait ChannelInputPin<M: TcMeta, I: ChannelId, P: PinIdentifier>: generics::Sealed {}
#[doc(hidden)]
pub trait ChannelOutputPin<M: TcMeta, I: ChannelId, P: PinIdentifier>: generics::Sealed {}
#[doc(hidden)]
pub trait ChannelClockPin<M: TcMeta, I: ChannelId>: generics::Sealed {}

macro_rules! impl_tc {
    (
        $(
            $( #[$cfg:meta] )?
            $Tc:ident: {
                $(
                    $Ch:ident: {
                        $( #[$cfgAPin:meta] )?
                        TIOA: $APin:ty,
                        $( #[$cfgBPin:meta] )?
                        TIOB: $BPin:ty,
                        $( #[$cfgClkPin:meta] )?
                        TCLK: $ClkPin:ty,
                    },
                )+
            },
        )+
    ) => {
        paste! {
            $(
                $( #[$cfg] )?
                mod [<$Tc:lower _impl>] {
                    use super::*;
                    use crate::pac::[<$Tc:upper>];

                    #[doc = "Identifier for [`Channel`]s in the [`" [<$Tc:upper>] "`]."]
                    pub enum $Tc {}

                    impl TcMeta for $Tc {
                        const REG: *const RegisterBlock = [<$Tc:upper>]::ptr();
                        const PIDS: [PeripheralIdentifier; CHANNELS_PER_TC] = [
                            PeripheralIdentifier::[<$Tc:upper _CHANNEL0>],
                            PeripheralIdentifier::[<$Tc:upper _CHANNEL1>],
                            PeripheralIdentifier::[<$Tc:upper _CHANNEL2>],
                        ];
                    }

                    $(
                            $( #[$cfgAPin] )?
                            impl ChannelInputPin<$Tc, $Ch, A> for $APin {}
                            $( #[$cfgAPin] )?
                            impl ChannelOutputPin<$Tc, $Ch, A> for $APin {}
                            $( #[$cfgBPin] )?
                            impl ChannelInputPin<$Tc, $Ch, B> for $BPin {}
                            $( #[$cfgBPin] )?
                            impl ChannelOutputPin<$Tc, $Ch, B> for $BPin {}
                            $( #[$cfgClkPin] )?
                            impl ChannelClockPin<$Tc, $Ch> for $ClkPin {}
                    )+

                    impl Tc<$Tc> {
                        #[doc = "Create a new [`Tc`] from a [`" [<$Tc:upper>] "`]."]
                        pub fn [<new_ $Tc:lower>](
                            _tc: [<$Tc:upper>],
                            mck: &mut HostClock,
                        ) -> Self {
                            Self::new(mck)
                        }
                    }
                }
                $( #[$cfg] )?
                pub use [<$Tc:lower _impl>]::*;
            )+
        }
    };
}

#[rustfmt::skip]
impl_tc!(
    Tc0: {
        Ch0: {
            // TIOA0
            #[cfg(not(feature = "pins-64"))]
            TIOA: Pin<PA0, PeripheralB>,
            // TIOB0
            #[cfg(not(feature = "pins-64"))]
            TIOB: Pin<PA1, PeripheralB>,
            // TCLK0
            TCLK: Pin<PA4, PeripheralB>,
        },
        Ch1: {
            // TIOA1
            #[cfg(not(feature = "pins-64"))]
            TIOA: Pin<PA15, PeripheralB>,
            // TIOB1
            #[cfg(not(feature = "pins-64"))]
            TIOB: Pin<PA16, PeripheralB>,
            // TCLK1
            #[cfg(not(feature = "pins-64"))]
            TCLK: Pin<PA28, PeripheralB>,
        },
        Ch2: {
            // TIOA2
            #[cfg(not(feature = "pins-64"))]
            TIOA: Pin<PA26, PeripheralB>,
            // TIOB2
            TIOB: Pin<PA27, PeripheralB>,
            // TCLK2
            #[cfg(feature = "pins-144")]
            TCLK: Pin<PA29, PeripheralB>,
        },
    },
    Tc1: {
        Ch0: {
            // TIOA3
            #[cfg(feature = "pins-144")]
            TIOA: Pin<PC23, PeripheralB>,
            // TIOB3
            #[cfg(feature = "pins-144")]
            TIOB: Pin<PC24, PeripheralB>,
            // TCLK3
            #[cfg(feature = "pins-144")]
            TCLK: Pin<PC25, PeripheralB>,
        },
        Ch1: {
            // TIOA4
            #[cfg(feature = "pins-144")]
            TIOA: Pin<PC26, PeripheralB>,
            // TIOB4
            #[cfg(feature = "pins-144")]
            TIOB: Pin<PC27, PeripheralB>,
            // TCLK4
            #[cfg(feature = "pins-144")]
            TCLK: Pin<PC28, PeripheralB>,
        },
        Ch2: {
            // TIOA5
            #[cfg(feature = "pins-144")]
            TIOA: Pin<PC29, PeripheralB>,
            // TIOB5
            #[cfg(feature = "pins-144")]
            TIOB: Pin<PC30, PeripheralB>,
            // TCLK5
            #[cfg(feature = "pins-144")]
            TCLK: Pin<PC31, PeripheralB>,
        },
    },
    Tc2: {
        Ch0: {
            // TIOA6
            #[cfg(feature = "pins-144")]
            TIOA: Pin<PC5, PeripheralB>,
            // TIOB6
            #[cfg(feature = "pins-144")]
            TIOB: Pin<PC6, PeripheralB>,
            // TCLK6
            #[cfg(feature = "pins-144")]
            TCLK: Pin<PC7, PeripheralB>,
        },
        Ch1: {
            // TIOA7
            #[cfg(feature = "pins-144")]
            TIOA: Pin<PC8, PeripheralB>,
            // TIOB7
            #[cfg(feature = "pins-144")]
            TIOB: Pin<PC9, PeripheralB>,
            // TCLK7
            #[cfg(feature = "pins-144")]
            TCLK: Pin<PC10, PeripheralB>,
        },
        Ch2: {
            // TIOA8
            #[cfg(feature = "pins-144")]
            TIOA: Pin<PC11, PeripheralB>,
            #[cfg(feature = "pins-144")]
            // TIOB8
            TIOB: Pin<PC12, PeripheralB>,
            // TCLK8
            #[cfg(feature = "pins-144")]
            TCLK: Pin<PC14, PeripheralB>,
        },
    },
    Tc3: {
        Ch0: {
            // TIOA9
            #[cfg(feature = "pins-144")]
            TIOA: Pin<PE0, PeripheralB>,
            // TIOB9
            #[cfg(feature = "pins-144")]
            TIOB: Pin<PE1, PeripheralB>,
            // TCLK9
            #[cfg(feature = "pins-144")]
            TCLK: Pin<PE2, PeripheralB>,
        },
        Ch1: {
            // TIOA10
            #[cfg(feature = "pins-144")]
            TIOA: Pin<PE3, PeripheralB>,
            // TIOB10
            #[cfg(feature = "pins-144")]
            TIOB: Pin<PE4, PeripheralB>,
            // TCLK10
            #[cfg(feature = "pins-144")]
            TCLK: Pin<PE5, PeripheralB>,
        },
        Ch2: {
            // TIOA11
            TIOA: Pin<PD21, PeripheralC>,
            // TIOB11
            TIOB: Pin<PD22, PeripheralC>,
            // TCLK11
            TCLK: Pin<PD24, PeripheralC>,
        },
    },
);

mod generate;
pub use generate::*;
mod capture;
pub use capture::*;
