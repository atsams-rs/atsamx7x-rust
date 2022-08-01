/*!
Real-Time Timer (RTT)
---

This module implements [`Monotonic`] and [`delay`] and
[`timer::CountDown`] from [`embedded-hal`](crate::ehal) upon the
device's 32-bit timer counter, prescaled by a 16-bit counter, and
driven by the [`SlowClock`] (32.768 kHz).

The range of the timer ranges from 109 hours (at 10.9227 kHz; best
accuracy) to 272 years (at ~2 Hz; worst accuracy).

A timer with the best accuracy and integer frequency can be acquired
via [`Rtt::new_8192Hz`].

# Usage example

```no_run
# use atsamx7x_hal as hal;
# use hal::pio::*;
# use hal::clocks::*;
# use hal::efc::*;
# use hal::rtt::*;
# use hal::fugit::RateExtU32;
# use rtic_monotonic::*;
# let pac = hal::pac::Peripherals::take().unwrap();
# let (slck, mut mck) = Tokens::new((pac.PMC, pac.SUPC, pac.UTMI), &pac.WDT.into()).por_state(&mut Efc::new(pac.EFC, VddioLevel::V3));

let mono: Mono<100> = Rtt::new(pac.RTT, &slck, 100.Hz()).unwrap().into_monotonic();
```
or
```no_run
# use atsamx7x_hal as hal;
# use hal::pio::*;
# use hal::clocks::*;
# use hal::efc::*;
# use hal::rtt::*;
# use hal::fugit::RateExtU32;
# use rtic_monotonic::Monotonic;
# let pac = hal::pac::Peripherals::take().unwrap();
# let (slck, mut mck) = Tokens::new((pac.PMC, pac.SUPC, pac.UTMI), &pac.WDT.into()).por_state(&mut Efc::new(pac.EFC, VddioLevel::V3));

use hal::ehal::{blocking::delay::DelayMs, timer::{CountDown, Cancel}};

let mut timer: Timer<100> = Rtt::new(pac.RTT, &slck, 100.Hz()).unwrap().into_timer();
timer.delay_ms(100);
timer.start(10.secs());
timer.reset();
// or
timer.cancel().unwrap();
// and when it is no longer needed
timer.disable();
```
*/

use crate::clocks::{Clock, SlowClock, SlowClockSource};
use crate::ehal::{blocking::delay, timer};
use crate::pac::{rtt::rtt_sr::R as StatusRegister, RTT};
pub use fugit::{ExtU32, RateExtU32};
use fugit::{
    HertzU32 as Hertz, MicrosDurationU32 as MicrosDuration, MillisDurationU32 as MillisDuration,
    TimerDurationU32 as Duration, TimerInstantU32 as Instant,
};
use rtic_monotonic::Monotonic;

/// Possible [`Rtt::new`] errors.
#[derive(Debug)]
pub enum RttError {
    /// The calculated prescaler is below the minimum required by
    /// hardware: [`Rtt::MINIMUM_PRESCALER`]. Increase the timer
    /// frequency.
    InvalidPrescaler,
    /// The calculated prescaler cannot be represented by a [`u16`].
    /// Decrease the timer frequency.
    PrescalerOverflow,
    /// Type-level and run-time-level frequency information does not
    /// match. [`Rtt::new`] can be called as follows:
    /// ```no_run
    /// # use atsamx7x_hal as hal;
    /// # use hal::pio::*;
    /// # use hal::clocks::*;
    /// # use hal::efc::*;
    /// # use hal::rtt::*;
    /// # use hal::fugit::RateExtU32;
    /// # let pac = hal::pac::Peripherals::take().unwrap();
    /// # let (slck, mut mck) = Tokens::new((pac.PMC, pac.SUPC, pac.UTMI), &pac.WDT.into()).por_state(&mut Efc::new(pac.EFC, VddioLevel::V3));
    /// let rtt: Rtt<100> = Rtt::new(pac.RTT, &slck, 100.Hz()).unwrap();
    /// ```
    FrequencyMismatch,
}

/// Possible [`timer::Cancel::cancel`] errors.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum CountDownError {
    /// The [`Timer`] is disabled and was thus never started.
    Disabled,
    /// The [`Timer`] countdown has already expired.
    Expired,
}

/// [`RTT`] abstraction.
pub struct Rtt<const FREQ_HZ: u32> {
    rtt: RTT,
}

/// A monotonically increasing counter, for use with
/// [RTIC](https://rtic.rs).
pub struct Mono<const FREQ_HZ: u32> {
    rtt: Rtt<FREQ_HZ>,
}

/// A general-purpose timer.
pub struct Timer<const FREQ_HZ: u32> {
    rtt: Rtt<FREQ_HZ>,
}

impl Rtt<1> {
    /// Configure the [`Rtt`] for a tick-rate of 1Hz.
    ///
    /// # Overflow
    ///
    /// This [`Rtt`] will overflow after 136 years.
    #[allow(non_snake_case)]
    pub fn new_1Hz<S: SlowClockSource>(rtt: RTT, clk: &SlowClock<S>) -> Self {
        // NOTE(unwrap): 1Hz is a valid frequency.
        Self::new(rtt, clk, 1.Hz()).unwrap()
    }
}

impl Rtt<8192> {
    /// Configure the [`Rtt`] for a tick-rate of 8192Hz.
    ///
    /// # Overflow
    ///
    /// This [`Rtt`] will overflow after 145 hours.
    #[allow(non_snake_case)]
    pub fn new_8192Hz<S: SlowClockSource>(rtt: RTT, clk: &SlowClock<S>) -> Self {
        // NOTE(unwrap): 8192Hz is a valid frequency.
        Self::new(rtt, clk, 8192.Hz()).unwrap()
    }
}

impl<const FREQ_HZ: u32> Rtt<FREQ_HZ> {
    /// C.f. p. 230, paragraph 5.
    pub const MINIMUM_PRESCALER: u16 = 3;

    /// Consumes the [`Rtt`], creating a monotonically increasing
    /// timer that implements [`Monotonic`].
    pub fn into_monotonic(self) -> Mono<FREQ_HZ> {
        Mono { rtt: self }
    }

    /// Consumes the [`Rtt`], creating a general-purpose timer that
    /// implements [`timer`] and [`delay`] from
    /// [`embedded-hal`](crate::ehal).
    pub fn into_timer(self) -> Timer<FREQ_HZ> {
        Timer { rtt: self }
    }

    /// Configure the [`Rtt`] at a tick-rate of `freq`Hz.
    pub fn new<S: SlowClockSource>(
        rtt: RTT,
        clk: &SlowClock<S>,
        freq: Hertz,
    ) -> Result<Self, RttError> {
        if FREQ_HZ != freq.to_Hz() {
            return Err(RttError::FrequencyMismatch);
        }

        // find the prescaler
        let pres: u16 = (clk.freq() / freq)
            .try_into()
            .map_err(|_| RttError::PrescalerOverflow)?;
        if pres < Self::MINIMUM_PRESCALER {
            return Err(RttError::InvalidPrescaler);
        }

        rtt.rtt_mr.modify(|_, w| {
            // feed via prescaled 32.768kHz clock
            w.rtc1hz().clear_bit();

            // no presclear roll-over interrupt; dont interrupt on
            // each timer tick.
            w.rttincien().clear_bit();

            // disable compare interrupt for now until needed.
            w.almien().clear_bit();

            unsafe {
                w.rtpres().bits(pres);
            }

            w
        });

        let mut rtt = Rtt { rtt };
        rtt.enable();

        Ok(rtt)
    }

    /// Reset the [`Rtt`], starting the counting from the beginnning
    /// if enabled.
    #[inline(always)]
    fn reset(&mut self) {
        self.rtt.rtt_mr.modify(|_, w| w.rttrst().set_bit());
    }

    /// Disable the [`Rtt`].
    #[inline(always)]
    fn disable(&mut self) {
        self.rtt.rtt_mr.modify(|_, w| w.rttdis().set_bit());
    }

    /// Enable the [`Rtt`].
    #[inline(always)]
    fn enable(&mut self) {
        self.rtt.rtt_mr.modify(|_, w| w.rttdis().clear_bit());
    }

    /// Returns whether the [`Rtt`] is enabled.
    fn is_enabled(&self) -> bool {
        self.rtt.rtt_mr.read().rttdis().bit_is_clear()
    }

    /// Read the Current Real-Time Value (CRTV)
    #[inline(always)]
    fn current_tick(&self) -> u32 {
        // CRTV can be asynchronously updated with the host clock, so
        // we must thus read the value twice for a correct value (C.f.
        // p. 230, paragraph 7).
        let crtv = || self.rtt.rtt_vr.read().crtv().bits();
        let mut prev = crtv();
        loop {
            let cur = crtv();
            if prev == cur {
                return prev;
            }
            prev = cur;
        }
    }

    fn set_alarm_tick(&mut self, tick: u32) {
        self.modify(|rtt| {
            // Set new value
            rtt.rtt.rtt_ar.write(|w| unsafe {
                // Note: interrupt is triggered when ALMV + 1 is reached.
                // Subtract one to trigger AT `tick` value
                w.bits(tick.saturating_sub(1))
            });
        });
    }

    /// Modifies the [`Rtt`] via `f`, preventing several executions of
    /// the interrupt handler (c.f. p. 230, paragraph 5) by disabling
    /// the alarm interrupt. The [`Rtt`] status register is also read
    /// (and clearing the flags as a side-effect).
    fn modify<F: FnOnce(&mut Self)>(&mut self, f: F) -> StatusRegister {
        // Disable interrupt before changing/clearing the alarm.
        self.rtt.rtt_mr.modify(|_r, w| w.almien().clear_bit());

        // Clear the alarm and execute calling code
        let sr = self.rtt.rtt_sr.read();
        f(self);

        // Re-enable alarm interrupt
        self.rtt.rtt_mr.modify(|_r, w| w.almien().set_bit());
        sr
    }

    /// Reads the status of the [`Rtt`] (and clearing the flags as a
    /// side-effect), while executing `F` without [`Rtt`]
    /// interruptions.
    #[inline(always)]
    fn read_status(&mut self) -> StatusRegister {
        self.modify(|_| {})
    }
}

impl<const FREQ_HZ: u32> Mono<FREQ_HZ> {
    /// Calculates the time remaining until the 32-bit counter overflows.
    pub fn time_until_overflow(&self) -> <Self as Monotonic>::Duration {
        <Self as Monotonic>::Duration::from_ticks(u32::MAX - self.rtt.current_tick())
    }
}

impl<const FREQ_HZ: u32> Monotonic for Mono<FREQ_HZ> {
    type Instant = Instant<FREQ_HZ>;
    type Duration = Duration<FREQ_HZ>;

    /// Resets the counter to zero.
    unsafe fn reset(&mut self) {
        self.rtt.reset();
    }

    #[inline(always)]
    fn now(&mut self) -> Self::Instant {
        Self::Instant::from_ticks(self.rtt.current_tick())
    }

    #[inline(always)]
    fn set_compare(&mut self, instant: Self::Instant) {
        self.rtt
            .set_alarm_tick(instant.duration_since_epoch().ticks());
    }

    #[inline(always)]
    fn clear_compare_flag(&mut self) {
        self.rtt.read_status();
    }

    #[inline(always)]
    fn zero() -> Self::Instant {
        Self::Instant::from_ticks(0)
    }
}

impl<const FREQ_HZ: u32> timer::CountDown for Timer<FREQ_HZ> {
    type Time = Duration<FREQ_HZ>;

    fn start<T>(&mut self, duration: T)
    where
        T: Into<Self::Time>,
    {
        self.rtt.set_alarm_tick(duration.into().ticks());
        self.rtt.reset();
        self.rtt.enable();
    }

    fn wait(&mut self) -> nb::Result<(), void::Void> {
        if self.rtt.read_status().alms().bit_is_clear() {
            Err(nb::Error::WouldBlock)
        } else {
            self.rtt.disable();
            Ok(())
        }
    }
}

impl<const FREQ_HZ: u32> Timer<FREQ_HZ> {
    /// Resets the [`Timer`], making it start counting from the
    /// beginnning again.
    pub fn reset(&mut self) {
        self.rtt.read_status();
        self.rtt.reset();
    }

    /// Disables the [`Timer`].
    pub fn disable(&mut self) {
        self.rtt.disable();
    }
}

impl<const FREQ_HZ: u32> timer::Cancel for Timer<FREQ_HZ> {
    type Error = CountDownError;

    fn cancel(&mut self) -> Result<(), Self::Error> {
        if !self.rtt.is_enabled() {
            return Err(Self::Error::Disabled);
        } else if self.rtt.read_status().alms().bit_is_set() {
            return Err(Self::Error::Expired);
        }

        self.rtt.disable();
        Ok(())
    }
}

impl<const FREQ_HZ: u32> delay::DelayUs<u32> for Timer<FREQ_HZ> {
    fn delay_us(&mut self, us: u32) {
        use timer::CountDown;

        self.start(MicrosDuration::from_ticks(us).convert());
        nb::block!(self.wait()).unwrap()
    }
}

impl<const FREQ_HZ: u32> delay::DelayMs<u32> for Timer<FREQ_HZ> {
    fn delay_ms(&mut self, ms: u32) {
        use timer::CountDown;

        self.start(MillisDuration::from_ticks(ms).convert());
        nb::block!(self.wait()).unwrap()
    }
}
