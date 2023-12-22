/*!
PWM (Pulse-width modulation)
---

This module implements PWM support via the [`Pwm`] and [`Channel`] abstractions. Each [`Pwm`] has 4 [`Channel`]s which can be configured independently.

In the present implementation, it is possible to configure the frequency and duty-rate of a [`Channel`], as well as inverting the high-output and low-output signals.

Refer to §51 for a full description of the PWM peripheral.

# Example usage

```no_run
# use atsamx7x_hal as hal;
# use hal::pio::*;
# use hal::clocks::*;
# use hal::efc::*;
# use hal::pwm::*;
# use hal::fugit::RateExtU32;
# let pac = unsafe{hal::pac::Peripherals::steal()};
# let (slck, mut mck) = Tokens::new((pac.PMC, pac.SUPC, pac.UTMI), &pac.WDT.into()).por_state(&mut Efc::new(pac.EFC, VddioLevel::V3));
use hal::ehal::PwmPin;

let banka = BankA::new(
    pac.PIOA,
    &mut mck,
    &slck,
    BankConfiguration::default(),
);

let pwmh = banka.pa0.into_peripheral(); // high-output signal pin
let pwml = banka.pa19.into_peripheral(); // low-output signal pin
let pwm = Pwm::new_pwm0(pac.PWM0, &mut mck);

let mut ch = pwm.ch0.configure(ChannelConfiguration {
    freq: 30.kHz(),
    duty: Percentage::try_from(0.1).unwrap(),
    invert: false,
});
ch.output_on(pwmh);
ch.output_on(pwml);
ch.enable();

// Changing duty-rate and frequency while the channel is active is glich-free.
ch.set_duty(Percentage::try_from(0.5).unwrap());
ch.set_freq(2.kHz());
```
 */

use core::marker::PhantomData;

use crate::clocks::{Clock, HostClock, PeripheralIdentifier};
use crate::ehal::PwmPin;
use crate::fugit::HertzU32 as Hertz;
use crate::pac::{
    pwm0::{RegisterBlock, PWM_CH_NUM as ChannelRegister},
    PWM0, PWM1,
};
use crate::{generics, pio::*};

use paste::paste;

/// Possible [`Pwm`]/[`Channel`] errors.
#[derive(Debug)]
pub enum PwmError {
    /// [`Percentage`] float outside of the `0.0..=1.0` range.
    InvalidPercentage,
}

/// Hardware metadata for a PWM.
pub trait PwmMeta: generics::Sealed {
    #[doc(hidden)]
    const REG: *const RegisterBlock;
    #[doc(hidden)]
    const PID: PeripheralIdentifier;
}

/// Trait for mutable hardware register access.
///
/// # Safety
///
/// Implementors of this trait must guarantee that they may
/// safely read- and write to the hardware state.
unsafe trait RegisterAccess<M: PwmMeta> {
    #[inline(always)]
    fn reg(&self) -> &RegisterBlock {
        unsafe { &*M::REG }
    }
}

/// A PWM peripheral, each with 4 [`Channel`]s.
pub struct Pwm<M: PwmMeta> {
    _meta: PhantomData<M>,
    /// First [`Channel`].
    pub ch0: Channel<M, Ch0>,
    /// Second [`Channel`].
    pub ch1: Channel<M, Ch1>,
    /// Third [`Channel`].
    pub ch2: Channel<M, Ch2>,
    /// Fourth [`Channel`].
    pub ch3: Channel<M, Ch3>,
}

/// Initial [`Channel`] configuration.
pub struct ChannelConfiguration {
    /// Initial frequency of the [`Channel`].
    pub freq: Hertz,
    /// Initial duty-rate of the [`Channel`].
    pub duty: Percentage,
    /// Whether the high-output and low-output signals should be
    /// inverted.
    ///
    /// # Example
    ///
    /// If disabled, and the [`Channel`] is configured for a duty-rate
    /// of 10%, the high-output signal will have a duty-rate of 10%,
    /// and the low-output signal will have a duty-rate of 90%. If
    /// enabled, the high-output signal will have a duty-rate of 90%,
    /// and the low-output 10%.
    pub invert: bool,
}

impl<M: PwmMeta, I: ChannelId> Channel<M, I> {
    unsafe fn new(clk_freq: Hertz) -> Self {
        Self {
            clk_freq,
            duty: Percentage::try_from(0.0).unwrap(),
            _meta: PhantomData,
            _id: PhantomData,
        }
    }

    #[inline(always)]
    fn reg(&self) -> &ChannelRegister {
        &<Self as RegisterAccess<M>>::reg(self).pwm_ch_num[I::DYN]
    }

    /// Returns `true` if the [`Channel`] is enabled and is currently
    /// emitting signal. Otherwise `false`.
    pub fn is_enabled(&self) -> bool {
        let mask = 1 << I::DYN;
        <Self as RegisterAccess<M>>::reg(self).sr.read().bits() & mask != 0
    }

    /// Apply a wanted [`Channel`] frequency in hardware.
    fn apply_freq(&mut self, freq: Hertz) {
        const CENTER_ALIGNED_PRESCALER: u32 = 2;

        let pres = self.clk_freq / CENTER_ALIGNED_PRESCALER / freq;

        // TODO: Calculate divider for clock generators, not implemented
        //
        // Refer to <https://git.grepit.se/embedded-rust/atsamx7x-hal/-/issues/42>.
        //
        // let (clka, clkb) = {
        //     let clks = <Self as RegisterAccess<M>>::reg(self).pwm_clk.read();

        //     // XXX: Does DIVx==0 require special attention?
        //     let clka = 2u32.pow(clks.prea().bits().into()) * clks.diva().bits() as u32;
        //     let clkb = 2u32.pow(clks.preb().bits().into()) * clks.divb().bits() as u32;

        //     (clka, clkb)
        // };

        if self.is_enabled() {
            // Divider from divider mux
            let div = match self.reg().cmr.read().cpre().bits() {
                11 => panic!("PWM clock generator A is not supported"),
                12 => panic!("PWM clock generator B is not supported"),
                exp => 2u32.pow(exp.into()),
            };

            // Determine period
            let cprd: u16 = ((pres as f32 / div as f32 + 0.5) as u32)
                .try_into()
                .unwrap();

            // Update buffer register for period
            self.reg()
                .cprdupd
                .write(|w| unsafe { w.cprdupd().bits(cprd.into()) });

            // Update buffer register for compare value
            self.apply_duty(self.duty, Some(cprd.into()));
        } else {
            // Find the most suitable prescaler available for the
            // requested frequency. Favour larger prescalers in order
            // to support a larger frequency range.
            //
            // C.f. p. 1599, CPRE field
            let prescalers = (0..=10).map(|exp| 2u32.pow(exp)); // TODO(above) .chain([clka, clkb])
            let (ridx, div) = prescalers
                .clone()
                .rev()
                .enumerate()
                .reduce(|pdiv, div| {
                    if pres % div.1 < pres % pdiv.1 {
                        div
                    } else {
                        pdiv
                    }
                })
                .unwrap();
            let idx = prescalers.count() - ridx - 1;

            self.reg()
                .cmr
                .modify(|_, w| unsafe { w.cpre().bits(idx.try_into().unwrap()) });
            self.reg()
                .cprd
                .modify(|_, w| unsafe { w.cprd().bits(pres / div) });
        };
    }

    /// Apply a wanted [`Channel`] dury rate in hardware. `max`
    /// denotes the new maximum value when it cannot be queried in
    /// hardware yet.
    fn apply_duty(&mut self, duty: Percentage, max: Option<u32>) {
        // New maximum value/period
        let max = max.unwrap_or_else(|| self.reg().cprd.read().cprd().bits());
        let cdty = (duty.0 * max as f32 + 0.5) as u16;

        if self.is_enabled() {
            self.reg()
                .cdtyupd
                .write(|w| unsafe { w.cdtyupd().bits(cdty.into()) });
        } else {
            self.reg()
                .cdty
                .modify(|_, w| unsafe { w.cdty().bits(cdty.into()) });
        }

        self.duty = duty;
    }

    /// Initially configures the [`Channel`] to generate a signal with
    /// a given frequency and duty rate. Does not start the
    /// [`Channel`].
    pub fn configure(mut self, cfg: ChannelConfiguration) -> Self {
        self.reg().cmr.modify(|_, w| {
            // center-align, one event per period (end of period)
            w.calg().set_bit();
            w.ces().clear_bit();
            w.cpol().bit(!cfg.invert);
            // disable dead-time generator
            w.dte().clear_bit();

            w
        });

        self.apply_freq(cfg.freq);
        self.apply_duty(cfg.duty, None);

        self
    }

    /// Emit the [`Channel`]'s signal on a [`ChannelPin`].
    ///
    /// **NOTE:** This function does nothing. Its purpose is to expose
    /// the [`Pin`] type used for a signal emission, for use with
    /// [`Pin::into_peripheral`].
    pub fn output_on(&self, _pin: impl ChannelPin<M, I>) {}

    /// Configures the [`Channel`] frequency. When
    /// [`Channel::is_enabled`], the [`Channel`] will output a
    /// center-aligned signal with this frequency, with the previously
    /// configured duty-rate ([`Channel::set_duty`]).
    ///
    /// Switching frequency while [`Channel::is_enabled`] is
    /// glitch-free.
    ///
    /// # [`Channel`]-internal clock hierarchy optimization
    ///
    /// If the [`Channel`] is disabled before calling this function,
    /// the most optimal prescaler of the input clock for `freq` will
    /// be switched to. Otherwise, if [`Channel::is_enabled`], the
    /// last prescaler will be used.
    pub fn set_freq(&mut self, freq: Hertz) {
        self.apply_freq(freq);
    }
}

unsafe impl<M: PwmMeta> RegisterAccess<M> for Pwm<M> {}

/// An independent [`Pwm`] channel.
pub struct Channel<M: PwmMeta, I: ChannelId> {
    /// Input clock frequency. Required in order to calculate prescaler values.
    clk_freq: Hertz,
    /// Wanted duty rate. Required in order to derive hardware value
    /// from future calls to [`Channel::apply_freq`].
    duty: Percentage,
    _meta: PhantomData<M>,
    _id: PhantomData<I>,
}

unsafe impl<M: PwmMeta, I: ChannelId> RegisterAccess<M> for Channel<M, I> {}

impl<M: PwmMeta> Pwm<M> {
    unsafe fn new(mck: &mut HostClock) -> Self {
        mck.enable_peripheral(M::PID);

        let mut pwm = Self {
            _meta: PhantomData,
            ch0: Channel::new(mck.freq()),
            ch1: Channel::new(mck.freq()),
            ch2: Channel::new(mck.freq()),
            ch3: Channel::new(mck.freq()),
        };

        // Unlock the peripheral
        pwm.unlock();

        pwm
    }

    fn unlock(&mut self) {
        // Unlock user interface as per §51.6.6.1
        self.reg().wpcr.write(|w| {
            w.wpkey().passwd();
            w.wpcmd().disable_sw_prot();
            w.wprg0().set_bit();
            w.wprg1().set_bit();
            w.wprg2().set_bit();
            w.wprg3().set_bit();
            w.wprg4().set_bit();
            w.wprg5().set_bit();

            w
        });
        // Ensure write protection is disabled for all registers.
        debug_assert_eq!(self.reg().wpsr.read().bits() & 0xffff, 0);
    }
}

/// Run-time [`Channel`] identifier.
pub trait ChannelId: generics::Sealed {
    #[doc(hidden)]
    const DYN: usize;
}

macro_rules! impl_ch {
    ($($Num:literal),*) => {
        paste! {
            $(
                /// [`Channel`] identifier.
                pub enum [<Ch $Num>] {}
                impl generics::Sealed for [<Ch $Num>] {}
                impl ChannelId for [<Ch $Num>] {
                    const DYN: usize = $Num;
                }
            )*
        }
    }
}
impl_ch!(0, 1, 2, 3);

/// Valid signal emission pin for a given [`Channel`].
pub trait ChannelPin<M: PwmMeta, I: ChannelId>: generics::Sealed {}
#[doc(hidden)]
pub trait ChannelPinHigh<M: PwmMeta, I: ChannelId>: ChannelPin<M, I> {}
#[doc(hidden)]
pub trait ChannelPinLow<M: PwmMeta, I: ChannelId>: ChannelPin<M, I> {}

macro_rules! impl_pwm {
    (
        $(
            $( #[$cfgPeriph:meta] )?
            $Pwm:ident: {
                $(
                    $Ch:ident: {
                        H: [$(
                            $( #[$cfgHPin:meta] )?
                            $HPin:ty,
                        )+],
                        L: [$(
                            $( #[$cfgLPin:meta] )?
                            $LPin:ty,
                        )+],
                        TRIGGER: [$(
                            $( #[$cfgTPin:meta] )?
                            $TriggerPin:ty,
                        )*],
                        FAULT: [$(
                            $( #[$cfgFPin:meta] )?
                            $FaultPin:ty,
                        )*],
                    },
                )+
            },
        )+
    ) => {
        paste! {
            $(
                $( #[$cfgPeriph] )?
                mod [<$Pwm:lower _impl>] {
                    use super::*;

                    #[doc = "Identifier for [`" [<$Pwm:upper>] "`]."]
                    pub enum $Pwm {}

                    impl generics::Sealed for $Pwm {}
                    impl PwmMeta for $Pwm {
                        const REG: *const RegisterBlock = [<$Pwm:upper>]::ptr();
                        const PID: PeripheralIdentifier = PeripheralIdentifier::[<$Pwm:upper>];
                    }

                    $(
                        $(
                            $( #[$cfgHPin] )?
                            impl ChannelPin<$Pwm, $Ch> for $HPin {}
                            $( #[$cfgHPin] )?
                            impl ChannelPinHigh<$Pwm, $Ch> for $HPin {}
                        )+
                        $(
                            $( #[$cfgLPin] )?
                            impl ChannelPin<$Pwm, $Ch> for $LPin {}
                            $( #[$cfgLPin] )?
                            impl ChannelPinLow<$Pwm, $Ch> for $LPin {}
                        )+
                    )+

                    impl Pwm<$Pwm> {
                        #[doc = "Create a new [`Pwm`] from a [`" [<$Pwm:upper>] "`]."]
                        pub fn [<new_ $Pwm:lower>] (
                            _pwm: [<$Pwm:upper>],
                            mck: &mut HostClock,
                        ) -> Self {
                            unsafe { Self::new(mck) }
                        }
                    }
                }
                $( #[$cfgPeriph] )?
                pub use [<$Pwm:lower _impl>]::*;
            )+
        }
    };
}

impl_pwm!(
    Pwm0: {
        Ch0: {
            // PWMC0_PWMH0
            H: [
                #[cfg(not(feature = "pins-64"))]
                Pin<PA0, PeripheralA>,
                Pin<PA11, PeripheralB>,
                #[cfg(not(feature = "pins-64"))]
                Pin<PA23, PeripheralB>,
                Pin<PB0, PeripheralA>,
                #[cfg(feature = "pins-144")]
                Pin<PC0, PeripheralB>,
                Pin<PD11, PeripheralB>,
                #[cfg(not(feature = "pins-64"))]
                Pin<PD20, PeripheralB>,
            ],
            // PWMC0_PWML0
            L: [
                #[cfg(not(feature = "pins-64"))]
                Pin<PA1, PeripheralA>,
                #[cfg(not(feature = "pins-64"))]
                Pin<PA19, PeripheralB>,
                #[cfg(feature = "reconfigurable-system-pins")]
                Pin<PB5, PeripheralB>,
                Pin<PD10, PeripheralB>,
                Pin<PD24, PeripheralB>,
            ],
            // PWMC0_PWMEXTRG0
            TRIGGER: [
                Pin<PA10, PeripheralB>,
            ],
            // PWMC0_PWMFI0
            FAULT: [
                Pin<PA9, PeripheralC>,
            ],
        },
        Ch1: {
            // PWMC0_PWMH1
            H: [
                #[cfg(not(feature = "pins-64"))]
                Pin<PA2, PeripheralA>,
                Pin<PA12, PeripheralB>,
                Pin<PA24, PeripheralB>,
                Pin<PB1, PeripheralA>,
                Pin<PD21, PeripheralB>,
            ],
            // PWMC0_PWML1
            L: [
                #[cfg(not(feature = "pins-64"))]
                Pin<PA20, PeripheralB>,
                #[cfg(not(feature = "pins-64"))]
                Pin<PA30, PeripheralA>,
                #[cfg(feature = "reconfigurable-system-pins")]
                Pin<PB12, PeripheralA>,
                #[cfg(feature = "pins-144")]
                Pin<PC1, PeripheralB>,
                #[cfg(feature = "pins-144")]
                Pin<PC18, PeripheralB>,
                Pin<PD25, PeripheralB>,
            ],
            // PWMC0_PWMEXTRG1
            TRIGGER: [
                Pin<PA22, PeripheralB>,
            ],
            // PWMC0_PWMFI1
            FAULT: [
                Pin<PD8, PeripheralB>,
            ],
        },
        Ch2: {
            // PWMC0_PWMH2
            H: [
                Pin<PA13, PeripheralB>,
                #[cfg(not(feature = "pins-64"))]
                Pin<PA25, PeripheralB>,
                #[cfg(feature = "reconfigurable-system-pins")]
                Pin<PB4, PeripheralB>,
                #[cfg(feature = "pins-144")]
                Pin<PC19, PeripheralB>,
                Pin<PD22, PeripheralB>,
            ],
            // PWMC0_PWML2
            L: [
                #[cfg(not(feature = "pins-64"))]
                Pin<PA16, PeripheralC>,
                Pin<PA13, PeripheralA>,
                #[cfg(feature = "pins-144")]
                Pin<PC2, PeripheralB>,
                #[cfg(feature = "pins-144")]
                Pin<PC20, PeripheralB>,
                Pin<PD26, PeripheralB>,
            ],
            // PWMC0_PWMEXTRG2
            TRIGGER: [],
            // PWMC0_PWMFI2
            FAULT: [
                Pin<PD9, PeripheralB>,
            ],
        },
        Ch3: {
            // PWMC0_PWMH3
            H: [
                Pin<PA7, PeripheralB>,
                Pin<PA14, PeripheralB>,
                #[cfg(not(feature = "pins-64"))]
                Pin<PA17, PeripheralC>,
                #[cfg(feature = "pins-144")]
                Pin<PC13, PeripheralB>,
                #[cfg(feature = "pins-144")]
                Pin<PC21, PeripheralB>,
                #[cfg(feature = "pins-144")]
                Pin<PD23, PeripheralB>,
            ],
            // PWMC0_PWML3
            L: [
                #[cfg(not(feature = "pins-64"))]
                Pin<PA15, PeripheralC>,
                #[cfg(feature = "pins-144")]
                Pin<PC3, PeripheralB>,
                #[cfg(feature = "pins-144")]
                Pin<PC15, PeripheralB>,
                #[cfg(feature = "pins-144")]
                Pin<PC22, PeripheralB>,
                #[cfg(not(feature = "pins-64"))]
                Pin<PD27, PeripheralB>,
            ],
            // PWMC0_PWMEXTRG3
            TRIGGER: [],
            // PWMC0_PWMFI3
            FAULT: [],
        },
    },

    Pwm1: {
        Ch0: {
            // PWMC1_PWMH0
            H: [
                Pin<PA12, PeripheralB>,
                Pin<PD1, PeripheralB>,
            ],
            // PWMC1_PWML0
            L: [
                Pin<PA11, PeripheralC>,
                Pin<PD0, PeripheralB>,
            ],
            // PWMC1_PWMEXTRG0
            TRIGGER: [
                Pin<PA30, PeripheralB>,
            ],
            // PWMC1_PWMFI0
            FAULT: [
                Pin<PA21, PeriperalC>,
            ],
        },
        Ch1: {
            // PWMC1_PWMH1
            H: [
                Pin<PA14, PeripheralC>,
                Pin<PD3, PeripheralB>,
            ],
            // PWMC1_PWML1
            L: [
                Pin<PA13, PeripheralC>,
                Pin<PD2, PeripheralB>,
            ],
            // PWMC1_PWMEXTRG1
            TRIGGER: [
                Pin<PA18, PeripheralA>,
            ],
            // PWMC1_PWMFI1
            FAULT: [
                Pin<PA26, PeripheralD>,
            ],
        },
        Ch2: {
            // PWMC1_PWMH2
            H: [
                #[cfg(not(feature = "pins-64"))]
                Pin<PA31, PeripheralD>,
                Pin<PD5, PeripheralB>,
            ],
            // PWMC1_PWML2
            L: [
                #[cfg(not(feature = "pins-64"))]
                Pin<PA23, PeripheralD>,
                Pin<PD4, PeripheralB>,
            ],
            // PWMC1_PWMEXTRG2
            TRIGGER: [
            ],
            // PWMC1_PWMFI2
            FAULT: [
                Pin<PA28, PeripheralD>,
            ],
        },
        Ch3: {
            // PWMC1_PWMH3
            H: [
                Pin<PA8, PeripheralA>,
                Pin<PD7, PeripheralB>,
            ],
            // PWMC1_PWML3
            L: [
                Pin<PA5, PeripheralA>,
                Pin<PD6, PeripheralB>,
            ],
            // PWMC1_PWMEXTRG3
            TRIGGER: [],
            // PWMC1_PWMFI3
            FAULT: [],
        },
    },
);

/// A representation of 0% to 100%; for duty rate representations.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Percentage(f32);

impl TryFrom<f32> for Percentage {
    type Error = PwmError;

    fn try_from(f: f32) -> Result<Self, Self::Error> {
        if !(0.0..=1.0).contains(&f) {
            Err(Self::Error::InvalidPercentage)
        } else {
            Ok(Percentage(f))
        }
    }
}

impl<M: PwmMeta, I: ChannelId> PwmPin for Channel<M, I> {
    /// Duty rate: from 0% to 100%.
    type Duty = Percentage;

    /// Disables the signal generation of the [`Channel`].
    fn disable(&mut self) {
        let pwm = <Self as RegisterAccess<M>>::reg(self);
        let mask = 1 << I::DYN;
        pwm.dis.write(|w| unsafe { w.bits(mask) });
        while self.is_enabled() {}
    }

    /// Enables the signal generation of the [`Channel`].
    fn enable(&mut self) {
        let pwm = <Self as RegisterAccess<M>>::reg(self);
        let mask = 1 << I::DYN;
        pwm.ena.write(|w| unsafe { w.bits(mask) });
        while !self.is_enabled() {}
    }

    /// Returns the duty rate of the [`Channel`].
    fn get_duty(&self) -> Self::Duty {
        let period = self.reg().cprd.read().cprd().bits();
        let duty = self.reg().cdty.read().cdty().bits();

        // Configuring duty and period required setting valid values for
        // Percentages, should never fail
        Self::Duty::try_from(duty as f32 / period as f32).unwrap()
    }

    /// Returns the maximum duty rate: 100%.
    fn get_max_duty(&self) -> Self::Duty {
        Self::Duty::try_from(1.0).unwrap()
    }

    /// Configures the [`Channel`] duty rate. When
    /// [`Channel::is_enabled`], the [`Channel`] will output a
    /// center-aligned signal with this duty rate, using the
    /// previously configured frequency ([`Channel::set_freq`]).
    ///
    /// Switching duty-rate while [`Channel::is_enabled`] is
    /// glitch-free.
    fn set_duty(&mut self, duty: Self::Duty) {
        self.apply_duty(duty, None);
    }
}
