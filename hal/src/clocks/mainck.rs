use super::*;

/// Main "RC" oscillator frequency selection.
pub use crate::target_device::pmc::ckgr_mor::MOSCRCF_A as InternalRcFreq;

/// The source of the [`MainClock`].
///
/// Refer to §60.2.1.
pub trait MainClockSource {}

impl MainClockSource for InternalRC {}
impl MainClockSource for ExternalNormal {}
impl MainClockSource for ExternalBypass {}

/// MAINCK, sourced from the [`InternalRC`], or an external clock:
/// [`ExternalNormal`] or [`ExternalBypass`].
pub struct MainClock<S: MainClockSource> {
    freq: Megahertz,
    source: PhantomData<S>,
}

impl<S: MainClockSource> Clock for MainClock<S> {
    fn freq(&self) -> Hertz {
        self.freq.convert()
    }
}

impl Token<MainClock<InternalRC>> {
    /// Configure [`MainClock`] for the [`InternalRC`] source.
    pub fn configure_internal(
        self,
        freq: InternalRcFreq,
    ) -> Result<MainClock<InternalRC>, ClockError> {
        self.pmc().ckgr_mor.modify(|_, w| {
            w.key().passwd();
            w.moscsel().clear_bit();
            w.moscrcen().set_bit();
            w.moscrcf().variant(freq);
            w
        });

        // TODO hande note for MOSCRCF unhandled (p. 276;
        // first table, second row)

        // Wait until clock is stable.
        while self.pmc().pmc_sr.read().moscrcs().bit_is_clear() {}

        let freq = Megahertz::from_raw(match freq {
            InternalRcFreq::_4_MHZ => 4,
            InternalRcFreq::_8_MHZ => 8,
            InternalRcFreq::_12_MHZ => 12,
        });

        Ok(MainClock {
            freq,
            source: PhantomData,
        })
    }

    /// Configure [`MainClock`] for the [`ExternalNormal`] source.
    pub fn configure_external_normal(
        self,
        freq: Megahertz,
    ) -> Result<MainClock<ExternalNormal>, ClockError> {
        // Clock signal frequency needs to be between 3 and
        // 20MHz (§30.2).
        if freq.to_MHz() < 3 || freq.to_MHz() > 20 {
            return Err(ClockError::InvalidConfiguration);
        }

        // Enable the external oscillator and wait for it to
        // stabilize.
        self.pmc().ckgr_mor.modify(|_, w| {
            w.key().passwd();
            w.moscxten().set_bit();
            unsafe {
                w.moscxtst().bits(COMMON_WAIT_UNTIL_STABLE_62_MILLISECS);
            }
            w
        });
        while self.pmc().pmc_sr.read().moscxts().bit_is_clear() {}

        // Switch over to the main oscillator.
        self.pmc().ckgr_mor.modify(|_, w| {
            w.key().passwd();
            w.moscsel().set_bit();
            w
        });
        while self.pmc().pmc_sr.read().moscsels().bit_is_clear() {}

        // TODO check MAINCK frequency (§31.17; step 5).

        Ok(MainClock {
            freq,
            source: PhantomData,
        })
    }
    /// Configure the [`MainClock`] for the [`ExternalBypass`] mode.
    pub fn configure_external_bypass(
        self,
        freq: Megahertz,
    ) -> Result<MainClock<ExternalBypass>, ClockError> {
        // Crystal frequency needs to be between 3 and 20MHz
        // (§30.2).
        if freq.to_MHz() < 3 || freq.to_MHz() > 20 {
            return Err(ClockError::InvalidConfiguration);
        }

        // Bypass the main crystal oscillator and disable it.
        self.pmc().ckgr_mor.modify(|_, w| {
            w.key().passwd();
            w.moscxtby().set_bit();
            w.moscxten().clear_bit();
            unsafe {
                w.moscxtst().bits(COMMON_WAIT_UNTIL_STABLE_62_MILLISECS);
            }
            w
        });

        // Wait until oscillator is stable.
        while self.pmc().pmc_sr.read().moscxts().bit_is_clear() {}

        // Switch over to the external clock.
        self.pmc().ckgr_mor.modify(|_, w| {
            w.key().passwd();
            w.moscsel().set_bit();
            w
        });
        while self.pmc().pmc_sr.read().moscsels().bit_is_clear() {}

        Ok(MainClock {
            freq,
            source: PhantomData,
        })
    }
}
