//! Analog Front-End Controller (AFEC; i.e., ADC)
//! ---
//!
//! This module provides an abstraction of the system's AFECs (ADCs).
//! While the feature-set is expansive, the present implementation
//! configures the created [`Afec`] for 12-bit samples of single-ended
//! inputs of single sample-and-hold mode. After its creation the
//! wanted channels are enabled, after which a sample can be
//! performed. Sampling is done in channel number order, and when
//! concluded, a [`Samples`] iterator of all enabled channels is
//! returned.
//!
//! This implementation presumes that VREFP is 3.3V.
//!
//! # Example usage
//!
//! ```
//! let mut efc = Efc::new(ctx.device.EFC, VddioLevel::V3);
//!
//! let mut pmc = hal::pmc::Pmc::new(ctx.device.PMC, &ctx.device.WDT.into());
//! let mainck = pmc
//!     .get_mainck(MainCkSource::InternalRC(MainRcFreq::_12_MHZ))
//!     .unwrap();
//! let (_, mck) = pmc
//!     .get_hclk(
//!         HostClockConfig {
//!             pres: MckPrescaler::CLK_1,
//!             div: MckDivider::EQ_PCK,
//!         },
//!         &mainck,
//!         &mut efc,
//!     )
//!     .unwrap();
//!
//! let banka = hal::pio::BankA::new(ctx.device.PIOA, &mut pmc, BankConfiguration::default());
//! let mut afec = Afec::new_afec0(ctx.device.AFEC0, &mut pmc, &mck);
//! afec.configure_channel(banka.pa17.into_input(PullDir::PullUp));
//!
//! for s in ctx.local.afec.sample() {
//!     rprintln!("Ch: {} = {:.2}V", s.channel, s.voltage); // e.g.: "Ch: 6 = 1.75V"
//! }
//! ```

use crate::pio::*;
use crate::pmc::{HostClock, PeripheralIdentifier, Pmc};
use crate::target_device::{afec0::RegisterBlock, AFEC0, AFEC1};

use core::marker::PhantomData;

pub type Channel = u8;

/// Type denoting [`Pin`]s handled by [`Afec0`]
pub trait Afec0ChannelPin: ChannelPin {}
/// Type denoting [`Pin`]s handled by [`Afec0`]
pub trait Afec1ChannelPin: ChannelPin {}
/// [`Afec`] [`Pin`] metadata
pub trait ChannelPin {
    /// The numerical channel that this [`Pin`] corresponds to.
    const CHANNEL: Channel;
}

macro_rules! impl_channel_pins {
    (
        $(
            $Trait:ident {
                $(
                    $( #[$cfg:meta] )?
                    ($Pin:ident, $Channel:literal),
                )+
            }
        )+
    ) => {
        $(
            $(
                $( #[$cfg] )?
                impl $Trait for Pin<$Pin, Input> {}

                $( #[$cfg] )?
                impl ChannelPin for Pin<$Pin, Input> {
                    const CHANNEL: Channel = $Channel;
                }
            )+
        )+
    };
}

impl_channel_pins!(
    Afec0ChannelPin {
        // Channel 11 is dedicated to the temperature sensor.

        #[cfg(not(feature = "pins-64"))]
        (PA17, 6),
        #[cfg(not(feature = "pins-64"))]
        (PA18, 7),
        #[cfg(not(feature = "pins-64"))]
        (PA19, 8),
        #[cfg(not(feature = "pins-64"))]
        (PA20, 9),
        (PA21, 1),
        (PB0, 10),
        (PB2, 5),
        (PB3, 2),
        #[cfg(not(feature = "pins-64"))]
        (PD30, 0),
        #[cfg(feature = "pins-144")]
        (PE4, 4),
        #[cfg(feature = "pins-144")]
        (PE5, 3),
    }

    Afec1ChannelPin {
        (PB1, 0),
        #[cfg(feature = "pins-144")]
        (PC0, 9),
        #[cfg(feature = "pins-144")]
        (PC12, 3),
        #[cfg(feature = "pins-144")]
        (PC13, 1),
        #[cfg(feature = "pins-144")]
        (PC15, 2),
        #[cfg(feature = "pins-144")]
        (PC26, 7),
        #[cfg(feature = "pins-144")]
        (PC27, 8),
        #[cfg(feature = "pins-144")]
        (PC29, 4),
        #[cfg(feature = "pins-144")]
        (PC30, 5),
        #[cfg(feature = "pins-144")]
        (PC31, 6),
        #[cfg(feature = "pins-144")]
        (PE0, 11),
        #[cfg(feature = "pins-144")]
        (PE3, 10),
    }
);

/// Metadata for an AFEC peripheral
pub trait AfecMeta {
    const REG: *const RegisterBlock;
    const PID: PeripheralIdentifier;
}

/// Type-level enum denoting [`AFEC0`]
pub enum Afec0 {}
/// Type-level enum denoting [`AFEC1`]
pub enum Afec1 {}

impl AfecMeta for Afec0 {
    const REG: *const RegisterBlock = AFEC0::ptr();
    const PID: PeripheralIdentifier = PeripheralIdentifier::AFEC0;
}
impl AfecMeta for Afec1 {
    const REG: *const RegisterBlock = AFEC1::ptr();
    const PID: PeripheralIdentifier = PeripheralIdentifier::AFEC1;
}

/// AFEC peripheral abstraction
pub struct Afec<A: AfecMeta> {
    assoc: PhantomData<A>,
}

/// Possible [`Afec`] errors
#[derive(Copy, Clone, Debug)]
pub enum AfecError {
    /// Unable to find a valid prescaler for a maximum of 40MHz,
    /// downstream from the used [`HostClock`].
    ImpossibleFreq,
}

/// Result component of an [`Afec::sample`].
#[derive(Copy, Clone, Debug)]
pub struct Sample {
    /// The numerical channel that was measured.
    pub channel: Channel,
    /// The measured voltage on this channel.
    pub voltage: f32,
}

const NUM_CHANNELS: usize = 12;

/// Iterator of [`Sample`]s for enabled and measured channels, from [`Afec::sample`].
pub struct Samples([Option<Sample>; NUM_CHANNELS]);
impl Iterator for Samples {
    type Item = Sample;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .iter_mut()
            .find(|s| s.is_some())
            .and_then(|s| s.take())
    }
}

impl<A: AfecMeta> Afec<A> {
    fn new(pmc: &mut Pmc, mck: &HostClock) -> Result<Self, AfecError> {
        pmc.enable_peripherals(&[A::PID]).unwrap();
        let mut afec = Self { assoc: PhantomData };
        afec.reset();
        afec.configure(mck)?;

        Ok(afec)
    }

    fn configure(&mut self, mck: &HostClock) -> Result<(), AfecError> {
        // Find a prescaler that is below the maximum allowed
        // frequency of 40MHz (c.f. §58.8.3).
        const AFEC_MAX_FREQ_MHZ: u32 = 40;
        let calc_afec_freq = |p| (mck.freq() / (p + 1) as u32).to_MHz();
        let pres: u8 = (1u8..)
            .find(|p| calc_afec_freq(*p) < AFEC_MAX_FREQ_MHZ)
            .ok_or(AfecError::ImpossibleFreq)?;

        self.reg().afec_mr.modify(|_, w| {
            w.one().set_bit();

            // use default conversion order of channels (0..=1, in
            // order)
            w.useq().clear_bit();

            // 9 ADC clocks between start command and analog channel
            // selection.
            const DEFAULT_CHANNEL_SELECTION_PERIOD: u8 = 0x3;
            unsafe {
                w.transfer().bits(DEFAULT_CHANNEL_SELECTION_PERIOD);
            }

            // Normal mode: wait for a trigger.
            w.freerun().clear_bit();

            // The AFE and reference voltage circuitry are kept on
            // between conversions.
            w.fwup().off();
            w.sleep().set_bit();

            // Require software trigger
            w.trgen().clear_bit();

            unsafe {
                w.prescal().bits(pres);
            }

            w.startup().sut512();

            w
        });

        self.reg().afec_emr.modify(|_, w| {
            w.signmode().all_unsigned();
            w.stm().clear_bit();
            w.tag().clear_bit();
            w.res().no_average();
            w
        });

        // Configure all channels for single-ended mode.
        self.reg().afec_diffr.write(|w| unsafe { w.bits(0x0) });

        // Configure all channels for single sample-and-hold mode
        self.reg().afec_shmr.write(|w| unsafe { w.bits(0x0) });

        // Configure all channels for a gain of 1
        self.reg().afec_cgr.modify(|_, w| unsafe { w.bits(0x0) });

        // Enable programmable gain amplifiers (PGAs; required prior
        // to any conversion)
        self.reg().afec_acr.modify(|_, w| {
            // C.f. §58.8.5
            if calc_afec_freq(pres) < AFEC_MAX_FREQ_MHZ / 2 {
                unsafe {
                    w.ibctl().bits(0b10);
                }
            } else {
                unsafe {
                    w.ibctl().bits(0b11);
                }
            }

            w.pga0en().set_bit();
            w.pga1en().set_bit();
            w
        });

        self.reg().afec_ier.write(|w| unsafe { w.bits(u32::MAX) });

        Ok(())
    }

    /// Sample all channels previously configured. Returns a
    /// [`Samples`] with up to 12 samples channels.
    pub fn sample(&mut self) -> Samples {
        // start the conversion
        self.reg().afec_cr.write(|w| w.start().set_bit());

        // Common mask for enabled channels (CHSR; §52.7.8) and status
        // bits (ISR; §52.7.13).
        const CHANNELS_MASK: u32 = 0xfff;

        // wait until all enabled channels have been sampled
        let enabled_mask = self.reg().afec_chsr.read().bits() & CHANNELS_MASK;
        loop {
            let isr = self.reg().afec_isr.read();
            // We disregard the COMPE (comparison error) and GOVRE
            // (general overflow error) flags because we are not use a
            // comparison trigger, and read our data from CDR instead
            // of LCDR, respectively.

            if (isr.bits() & CHANNELS_MASK) == enabled_mask {
                // all enabled channels have been sampled
                break;
            }
        }

        // read all channel samples
        let mut samples: [Option<Sample>; NUM_CHANNELS] = [None; NUM_CHANNELS];
        for ch in 0..(NUM_CHANNELS as u8) {
            samples[ch as usize] = if enabled_mask & (1 << ch) != 0 {
                self.reg()
                    .afec_cselr
                    .write(|w| unsafe { w.csel().bits(ch) });
                let code = self.reg().afec_cdr.read().data().bits();

                Some(Sample {
                    channel: ch,
                    voltage: Self::code_to_voltage(code),
                })
            } else {
                None
            }
        }
        assert_eq!(self.reg().afec_isr.read().bits() & CHANNELS_MASK, 0);

        Samples(samples)
    }

    fn code_to_voltage(code: u16) -> f32 {
        const GAIN: f32 = 1.0;
        const VREF: f32 = 3.3;
        const VDAC: f32 = VREF / 2.0;

        // via §58.8.4.2, solved for Vin
        ((code as f32 - 2047f32) / ((4096f32 / VREF) * GAIN)) + VDAC
    }

    #[inline]
    fn reg(&self) -> &RegisterBlock {
        unsafe { &*A::REG }
    }

    /// Resets the controller and its configuration.
    #[inline]
    fn reset(&mut self) {
        // Simulate a hardware reset for the AFEC
        self.reg().afec_cr.write(|w| w.swrst().set_bit());

        // Reset configuration
        self.reg().afec_mr.reset();
    }

    #[inline]
    fn enable_channel(&mut self, ch: Channel) {
        self.reg().afec_cher.write(|w| unsafe { w.bits(1 << ch) });

        // set a no-compensation channel offset
        const DAC_NO_COMPENSATION: u16 = 512;
        self.reg()
            .afec_cselr
            .write(|w| unsafe { w.csel().bits(ch) });
        self.reg()
            .afec_cocr
            .write(|w| unsafe { w.aoff().bits(DAC_NO_COMPENSATION) });
    }
}

impl Afec<Afec0> {
    /// Create a new [`Afec`].
    pub fn new_afec0(_afec: AFEC0, pmc: &mut Pmc, mck: &HostClock) -> Result<Self, AfecError> {
        Self::new(pmc, mck)
    }

    #[inline]
    pub fn configure_temp_sensor(&mut self) {
        const TEMP_SENSOR_CHANNEL: Channel = 11;
        self.enable_channel(TEMP_SENSOR_CHANNEL)
    }

    #[inline]
    /// Enable and configure sampling for the given [`ChannelPin`].
    pub fn configure_channel<P: Afec0ChannelPin + ChannelPin>(&mut self, _pin: P) {
        self.enable_channel(P::CHANNEL)
    }
}

impl Afec<Afec1> {
    /// Create a new [`Afec`].
    pub fn new_afec1(_afec: AFEC1, pmc: &mut Pmc, mck: &HostClock) -> Result<Self, AfecError> {
        Self::new(pmc, mck)
    }

    #[inline]
    /// Enable and configure sampling for the given [`ChannelPin`].
    pub fn configure_channel<P: Afec0ChannelPin + ChannelPin>(&mut self, _pin: P) {
        self.enable_channel(P::CHANNEL)
    }
}
