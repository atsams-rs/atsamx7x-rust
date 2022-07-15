use super::*;

impl Clock for UpllClock {
    fn freq(&self) -> Hertz {
        Self::FREQ.convert()
    }
}

impl Clock for UpllDivClock {
    fn freq(&self) -> Hertz {
        self.freq.convert()
    }
}

impl Token<UpllClock> {
    /// Configures UPLLCK
    pub fn configure(self, source: &impl UpllSource) -> Result<UpllClock, ClockError> {
        use crate::target_device::utmi::utmi_cktrim::FREQ_A as FREQ;

        let freq = match source.freq().to_MHz() {
            12 => FREQ::XTAL12,
            16 => FREQ::XTAL16,
            _ => return Err(ClockError::InvalidConfiguration),
        };

        // Configure the UTMI PLL clock and wait for lock.
        self.utmi()
            .utmi_cktrim
            .modify(|_, w| w.freq().variant(freq));
        self.pmc().ckgr_uckr.modify(|_, w| {
            w.upllen().set_bit();
            unsafe {
                w.upllcount().bits(COMMON_WAIT_UNTIL_STABLE_62_MILLISECS);
            }
            w
        });
        while self.pmc().pmc_sr.read().locku().bit_is_clear() {}

        Ok(UpllClock)
    }
}

impl Token<UpllDivClock> {
    /// Configures UPLLCKDIV
    pub fn configure(self, source: &impl UpllDivSource, div: UpllDivider) -> UpllDivClock {
        self.pmc()
            .pmc_mckr
            .modify(|_, w| w.uplldiv2().bit(div == UpllDivider::Div2));

        UpllDivClock {
            freq: source.freq().convert()
                / match div {
                    UpllDivider::Div1 => 1,
                    UpllDivider::Div2 => 2,
                },
        }
    }
}

/// Output divider for [`UpllDivClock`].
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum UpllDivider {
    /// UPLLCK is divided by 1: input and output frequencies are
    /// equal.
    Div1,
    /// UPLLCK is diveded by 2: output frequency is half of input
    /// frequency.
    Div2,
}

/// UPLLCK, driven by [`MainClock<ExternalNormal>`] or
/// [`MainClock<ExternalBypass>`].
pub struct UpllClock;

impl UpllClock {
    pub(crate) const FREQ: Megahertz = Megahertz::from_raw(480);
}

/// UPLLCKDIV, driven by [`UpllClock`].
pub struct UpllDivClock {
    freq: Megahertz,
}

/// The source of the [`UpllDivClock`].
pub trait UpllDivSource: Clock {}
impl UpllDivSource for UpllClock {}

/// The source of the [`UpllClock`].
pub trait UpllSource: Clock {}
impl UpllSource for MainClock<ExternalNormal> {}
impl UpllSource for MainClock<ExternalBypass> {}
