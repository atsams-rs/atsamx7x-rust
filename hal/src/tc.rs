/*!
Timer Counter (TC)
---

The [`Tc`] is a peripheral that contain three [`Channel`]s. Each [`Channel`] can [`Generate`] or `Capture` a signal.
In [`Generate`], PWM signals can be emitted on connected [`Pin`]s, drive another [`Channel`] via [`Channel::chain`], and provide delay/scheduling primitives.
In `Capture`, a [`Channel`] can measure frequencies/intervals, count events, decode quadrature signals, etc.

In the present implementation, only scheduling primitives are exploited for [`Channel::chain`]ed [`Channel`]s in order to acquire [`rtic_monotonic::Monotonic`] and [`ehal::timer::CountDown`] implementations.
If a [`Pin`] is configured out-of-band to correspond to a [`Channel`] I/O, an output clock signal may be observed, but any input edges are ignored.

Refer to §50 for a full description on the capabilities offered by a [`Tc`].

[`Pin`]: crate::pio::Pin
[`ehal::timer::Countdown`]: crate::ehal::timer::CountDown

# Example usage

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
// This Monotonic will overflow after approximately 18.2 hours.
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
use crate::pac::tc0::{
    tc_channel::tc_cmr_waveform_mode::TCCLKS_A as TCCLKS, RegisterBlock,
    TC_CHANNEL as ChannelRegisterBlock,
};
use crate::rtt::CountDownError;

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
    #[inline]
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
pub trait ChannelState {}
/// The [`Channel`] is inactive and does not generate a signal.
pub enum Inactive {}
/// The [`Channel`] is active and is generating a signal, with an input clock of `FREQ_HZ`Hz.
pub struct Generate<C: ChannelClock, const FREQ_HZ: u32>(PhantomData<C>);
impl ChannelState for Inactive {}
impl<C: ChannelClock, const FREQ_HZ: u32> ChannelState for Generate<C, FREQ_HZ> {}

/// Possible [`Channel`] configuration errors.
///
/// For [`TcError::PrescalerOverflow`] and [`TcError::PrescalerNoop`], the prescaler is calculated via
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
    /// Calculated prescaler does not fit in a [`u16`]. Increase the
    /// wanted frequency.
    PrescalerOverflow {
        /// The [`Channel`] input frequency.
        input: Hertz,
        /// The wanted [`Channel`] output frequency.
        wanted: Hertz,
    },
    /// Calculated prescaler is zero, which would noop the channel's
    /// function. Decrease the wanted frequency.
    PrescalerNoop {
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
        self.channel().tc_ccr.write(|w| w.clkdis().set_bit());
    }

    /// Configures the channel for waveform mode (an signal is generated).
    ///
    /// # Safety
    ///
    /// After this call, the [`Channel`] could be considered in a
    /// state between [`Inactive`] and [`Generate`]. A subsequent call
    /// to [`Channel::tranform`] to [`Generate`] should be performed.
    unsafe fn generate_inner<C: ChannelClock>(&self, _clk: &C) {
        self.channel().tc_cmr_waveform_mode().modify(|_, w| {
            w.wave().set_bit();
            w.tcclks().variant(C::SOURCE.0);

            w
        });
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
}
impl<M: TcMeta, I: ChannelId, C: ChannelClock, const FREQ_HZ: u32>
    Channel<M, I, Generate<C, FREQ_HZ>>
{
    /// Transform the [`Channel`] into one driven (chained) by another
    /// [`Channel`], using the output clock of `driver` an an input
    /// clock.
    #[allow(clippy::complexity)]
    pub fn chain<J: ChannelId, Co: ChannelClock, const DRIVER_FREQ_HZ: u32>(
        #[allow(unused_mut)] mut self,
        driver: Channel<M, J, Generate<Co, DRIVER_FREQ_HZ>>,
    ) -> Channel<M, I, Generate<Channel<M, J, Generate<Co, DRIVER_FREQ_HZ>>, FREQ_HZ>>
    where
        Channel<M, J, Generate<Co, DRIVER_FREQ_HZ>>: ChannelClock,
        Self: ChannelClock,
    {
        // Self is generated by an "internal clock", which is defined
        // within itself. Specifying &driver here, as one would
        // perhaps except, would instead generate the clock from that
        // channel's "external clock": HostClock. In the next
        // statement, the two channels are connected.
        unsafe {
            self.generate_inner(&self);
        }

        // Connect driver TIOA output to self's input
        self.reg().tc_bmr.modify(|_, w| {
            let tcxcs = I::DYN.external_feedback_field(&J::DYN);
            match I::DYN {
                DynChannelId::Ch0 => unsafe { w.tc0xc0s().bits(tcxcs) },
                DynChannelId::Ch1 => unsafe { w.tc1xc1s().bits(tcxcs) },
                DynChannelId::Ch2 => unsafe { w.tc2xc2s().bits(tcxcs) },
            };

            w
        });

        // Configure driving channel
        driver.channel().tc_cmr_waveform_mode().modify(|_, w| {
            // noop effects on TIOB, TIOA toggles on RC compare match
            //
            // NOTE: RA and RB compare match affects TIOA and
            // TIOB, respectively. RC compare match affects TIOA
            // and/or TIOB.
            //
            // noop TIOB on software trigger
            w.bswtrg().none();
            // XXX clear TIOA on software trigger
            w.aswtrg().clear();
            // noop on external event
            w.beevt().none();
            w.aeevt().none();
            // no TIOB effect on RC compare match
            w.bcpc().none();
            // XXX TIOA toggle on RC compare match
            w.acpc().toggle();
            // no effect on RA/RB match
            w.bcpb().none();
            w.acpa().none();

            // Use TIOB as external event, but disable triggers:
            // does not reset counter or start its clock.
            //
            // As a side-effect, TIOB is now an input and does not
            // generate a waveform and subsequently no IRQs.
            w.eevt().tiob();
            w.eevtedg().none();
            w.enetrg().clear_bit();

            // Increment counter to RC, then reset it
            w.wavsel().up_rc();

            // Input clock configuration (c.f. Fig. 50-3):
            //
            // Do not disable/stop clock when counter reaches RC.
            w.cpcdis().clear_bit();
            w.cpcstop().clear_bit();
            // Do not gate the clock by an external signal.
            w.burst().none();
            // Do not invert the clock
            w.clki().clear_bit();

            w
        });
        // disable any IRQs
        driver
            .channel()
            .tc_idr
            .write(|w| unsafe { w.bits(u32::MAX) });

        // Enable the input clock
        driver.channel().tc_ccr.write(|w| {
            w.clkdis().clear_bit();
            w.clken().set_bit();

            w
        });

        // Safe: both channels are consumed
        Channel::transform(self)
    }
}

/// A monotonically increasing clock that is [`Channel::chain`]ed with
/// a driving [`Channel`]. Implemented using the [`Channel`]s 16-bit
/// counter. The lowest frequency of this clock if 1Hz, which
/// corresponds to a life-time of approximately 18.2 hours.
pub struct Monotonic<M: TcMeta, I: ChannelId, C: ChannelClock, const FREQ_HZ: u32> {
    channel: Channel<M, I, Generate<C, FREQ_HZ>>,
}

/// A [`ehal::timer::CountDown`] implementation that is
/// [`Channel::chain`]ed with a driving [`Channel`].
///
/// [`ehal::timer::CountDown`]: crate::ehal::timer::CountDown
pub struct Timer<M: TcMeta, I: ChannelId, C: ChannelClock, const FREQ_HZ: u32> {
    channel: Channel<M, I, Generate<C, FREQ_HZ>>,
}

impl<M: TcMeta, I, J, C: ChannelClock, const DRIVER_FREQ_HZ: u32, const FREQ_HZ: u32>
    Channel<M, I, Generate<Channel<M, J, Generate<C, DRIVER_FREQ_HZ>>, FREQ_HZ>>
where
    I: ChannelId,
    J: ChannelId,
    Channel<M, J, Generate<C, DRIVER_FREQ_HZ>>: ChannelClock,
{
    /// Transform the [`Channel`] into a [`rtic_monotonic::Monotonic`]
    /// implementation with a frequency of `MONO_FREQ_HZ`Hz.
    #[allow(clippy::complexity)]
    pub fn into_monotonic<const MONO_FREQ_HZ: u32>(
        self,
    ) -> Result<Monotonic<M, I, Channel<M, J, Generate<C, DRIVER_FREQ_HZ>>, MONO_FREQ_HZ>, TcError>
    {
        let freq = Hertz::from_raw(MONO_FREQ_HZ);

        // Configure driver prescaler, rounding to the closest
        // integer.
        //
        // Safe: driver was consumed in Channel::chain.
        let driver = unsafe { Channel::<M, J, _>::new::<Generate<C, DRIVER_FREQ_HZ>>() };
        // The 16-bit counter is incremented only at each positive
        // input clock edge. When chaining channels, this then results
        // in a static /2 prescaler. Refer to §50.6.2.
        const FEEDBACK_PRESCALER: u32 = 2;
        let input_freq = ChannelClock::freq(&driver).raw() as f32 / FEEDBACK_PRESCALER as f32;
        let pres: u16 = ((input_freq / freq.raw() as f32 + 0.5) as u32)
            .try_into()
            .map_err(|_| TcError::PrescalerOverflow {
                input: Hertz::from_raw(input_freq as u32),
                wanted: freq,
            })?;
        if pres == 0 {
            return Err(TcError::PrescalerNoop {
                input: Hertz::from_raw(input_freq as u32),
                wanted: freq,
            });
        }
        driver
            .channel()
            .tc_rc
            .write(|w| unsafe { w.rc().bits(pres.into()) });

        // Configure self
        self.channel().tc_cmr_waveform_mode().modify(|_, w| {
            // noop effects on TIOA/TIOB
            //
            // NOTE: RA and RB compare match affects TIOA and
            // TIOB, respectively. RC compare match affects TIOA
            // and/or TIOB.
            //
            // noop on software trigger
            w.bswtrg().none();
            w.aswtrg().none();
            // noop on external event
            w.beevt().none();
            w.aeevt().none();
            // no effect on RC compare match
            w.bcpc().none();
            w.acpc().none();
            // no effect on RA/RB match
            w.bcpb().none();
            w.acpa().none();

            // Use TIOB as external event, but disable triggers:
            // does not reset counter or start its clock.
            //
            // As a side-effect, TIOB is now an input and does not
            // generate a waveform and subsequently no IRQs.
            w.eevt().tiob();
            w.eevtedg().none();
            w.enetrg().clear_bit();

            // Increment the counter from 0 to u16::MAX. Do not
            // reset when RC is reached, and disallow counting
            // down.
            w.wavsel().up();

            // Input clock configuration (c.f. Fig. 50-3):
            //
            // Do not disable/stop clock when counter reaches RC.
            w.cpcdis().clear_bit();
            w.cpcstop().clear_bit();
            // Do not gate the clock by an external signal.
            //
            // TODO: burst with driver clock?
            w.burst().none();
            // Do not invert the clock
            w.clki().clear_bit();

            w
        });

        // Enable interrupt on RC compare match
        self.channel().tc_idr.write(|w| unsafe { w.bits(u32::MAX) });
        self.channel().tc_ier.write(|w| w.cpcs().set_bit());

        Ok(Monotonic {
            channel: Channel::transform(self),
        })
    }

    /// Transform the [`Channel`] into a [`timer::CountDown`]
    /// implementation with a frequency of `TIMER_FREQ_HZ`Hz.
    #[allow(clippy::complexity)]
    pub fn into_timer<const TIMER_FREQ_HZ: u32>(
        self,
    ) -> Result<Timer<M, I, Channel<M, J, Generate<C, DRIVER_FREQ_HZ>>, TIMER_FREQ_HZ>, TcError>
    {
        let mono = self.into_monotonic::<TIMER_FREQ_HZ>()?;

        // Disable the input clock on an RC compare. This gives us a
        // oneshot timer.
        mono.channel
            .channel()
            .tc_cmr_waveform_mode()
            .modify(|_, w| w.cpcdis().set_bit());

        // XXX: we let IER.CPCS be set which allows the user to bind a
        // task for when the timer reaches "zero" (RC).

        Ok(Timer {
            channel: mono.channel,
        })
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

macro_rules! impl_tc {
    (
        $(
            $( #[$cfg:meta] )?
            $Tc:ident,
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
    Tc0,
    Tc1,
    Tc2,
    Tc3,
);

impl<M: TcMeta, I: ChannelId, C: ChannelClock, const FREQ_HZ: u32> timer::CountDown
    for Timer<M, I, C, FREQ_HZ>
{
    type Time = Duration<FREQ_HZ>;

    fn start<T>(&mut self, duration: T)
    where
        T: Into<Self::Time>,
    {
        let ticks = duration.into().ticks();
        if ticks > u16::MAX.into() {
            // Internal 16-bit counter must be extended in software to
            // 32-bit.
            todo!()
        }

        self.channel
            .channel()
            .tc_rc
            .write(|w| unsafe { w.rc().bits(ticks) });

        // Enable input clock
        self.channel.channel().tc_ccr.write(|w| {
            w.clkdis().clear_bit();
            w.clken().set_bit();

            w
        });

        // SYNC (software trigger all channels); required when channels are chained.
        self.channel.reg().tc_bcr.write(|w| w.sync().set_bit());

        // Wait until the clock is enabled.
        while self.channel.channel().tc_sr.read().clksta().bit_is_clear() {}
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        if self.channel.channel().tc_sr.read().cpcs().bit_is_clear() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }
}

impl<M: TcMeta, I: ChannelId, C: ChannelClock, const FREQ_HZ: u32> timer::Cancel
    for Timer<M, I, C, FREQ_HZ>
{
    type Error = CountDownError;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        if self.channel.channel().tc_sr.read().clksta().bit_is_clear() {
            return Err(Self::Error::Disabled);
        } else if self.channel.channel().tc_sr.read().cpcs().bit_is_set() {
            return Err(Self::Error::Expired);
        }

        self.channel.disable();
        Ok(())
    }
}

impl<M: TcMeta, I: ChannelId, C: ChannelClock, const FREQ_HZ: u32> delay::DelayUs<u32>
    for Timer<M, I, C, FREQ_HZ>
{
    fn delay_us(&mut self, us: u32) {
        use timer::CountDown;

        self.start(MicrosDuration::from_ticks(us).convert());
        nb::block!(self.wait()).unwrap()
    }
}

impl<M: TcMeta, I: ChannelId, C: ChannelClock, const FREQ_HZ: u32> delay::DelayMs<u32>
    for Timer<M, I, C, FREQ_HZ>
{
    fn delay_ms(&mut self, ms: u32) {
        use timer::CountDown;

        self.start(MillisDuration::from_ticks(ms).convert());
        nb::block!(self.wait()).unwrap()
    }
}

impl<M: TcMeta, I: ChannelId, C: ChannelClock, const FREQ_HZ: u32> rtic_monotonic::Monotonic
    for Monotonic<M, I, C, FREQ_HZ>
{
    type Instant = Instant<FREQ_HZ>;
    type Duration = Duration<FREQ_HZ>;

    /// Resets the counter to zero and start the clock.
    unsafe fn reset(&mut self) {
        // Enable input clock
        self.channel.channel().tc_ccr.write(|w| {
            w.clkdis().clear_bit();
            w.clken().set_bit();

            w
        });

        // SYNC (software trigger all channels); required when channels are chained.
        self.channel.reg().tc_bcr.write(|w| w.sync().set_bit());

        // Wait until the clock is enabled.
        while self.channel.channel().tc_sr.read().clksta().bit_is_clear() {}
    }

    #[inline(always)]
    fn now(&mut self) -> Self::Instant {
        Self::Instant::from_ticks(self.channel.channel().tc_cv.read().cv().bits())
    }

    #[inline(always)]
    fn set_compare(&mut self, instant: Self::Instant) {
        let ticks = instant.duration_since_epoch().ticks();
        if ticks > u16::MAX.into() {
            // Internal 16-bit counter must be extended in software to
            // 32-bit.
            todo!()
        }

        self.channel
            .channel()
            .tc_rc
            .write(|w| unsafe { w.rc().bits(ticks) });
    }

    #[inline(always)]
    fn clear_compare_flag(&mut self) {
        self.channel.channel().tc_sr.read();
    }

    #[inline(always)]
    fn zero() -> Self::Instant {
        Self::Instant::from_ticks(0)
    }
}
