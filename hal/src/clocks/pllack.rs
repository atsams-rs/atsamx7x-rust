use super::*;

impl Clock for PllaClock {
    fn freq(&self) -> Hertz {
        self.freq
    }
}

/// The source of the [`PllaClock`].
pub trait PllaSource: Clock {}
impl<S: MainClockSource> PllaSource for MainClock<S> {}

impl Token<PllaClock> {
    /// Configures PLLACK and returns a corresponding clock token.
    /// This method corresponds to Step 6 of 31.17 Recommended Programming Sequence.
    pub fn configure(
        self,
        source: &impl PllaSource,
        PllaConfig { div, mult }: PllaConfig,
    ) -> Result<PllaClock, ClockError> {
        if !(2..=62).contains(&mult) {
            return Err(ClockError::InvalidPllaCk);
        }
        // Datasheet error, sec 31.17 (step 6) & 31.20.10 differ.
        if div == 0 || div > 127 {
            return Err(ClockError::InvalidPllaCk);
        }
        // TODO: Ensure valid requested output frequency.

        // Configure PLLA and wait for lock.
        self.pmc().ckgr_pllar().modify(|_, w| {
            w.one().set_bit();
            unsafe {
                w.mula().bits(mult as u16 - 1); // HW adds 1
                w.diva().bits(div);
            }
            w
        });
        while self.pmc().sr().read().locka().bit_is_clear() {}

        Ok(PllaClock {
            // Per datasheet sec. 31.20.10, chip adds 1 to the multiplier.
            freq: (source.freq().convert() / div as u32) * (mult + 1) as u32,
        })
    }
}

/// [`PllaClock`] configuration.
///
/// Output frequency is `plla <- (input_freq / div) * mult`.
pub struct PllaConfig {
    /// The input clock divider.
    pub div: u8,
    /// The output clock multiplier.
    pub mult: u8,
}

/// PLLACK, driven by [`MainClock`].
pub struct PllaClock {
    freq: Hertz,
}
impl generics::Sealed for PllaClock {}
