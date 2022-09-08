use super::*;
use crate::efc::Efc;
use crate::pac::pmc::mckr::CSSSELECT_A as HCC_CSS;
use crate::pac::pmc::mckr::MDIVSELECT_A as MCK_DIV;
use crate::pac::pmc::mckr::PRESSELECT_A as HCC_PRES;

/// Common [`HostClock`] and [`ProcessorClock`] prescaler.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum HccPrescaler {
    Div1 = 1,
    Div2 = 2,
    Div3 = 3,
    Div4 = 4,
    Div8 = 8,
    Div16 = 16,
    Div32 = 32,
    Div64 = 64,
}

impl From<HccPrescaler> for HCC_PRES {
    fn from(pres: HccPrescaler) -> Self {
        use HccPrescaler::*;
        use HCC_PRES::*;

        match pres {
            Div1 => CLK_1,
            Div2 => CLK_2,
            Div3 => CLK_3,
            Div4 => CLK_4,
            Div8 => CLK_8,
            Div16 => CLK_16,
            Div32 => CLK_32,
            Div64 => CLK_64,
        }
    }
}

/// [`HostClock`] divider, applied after [`HccPrescaler`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(missing_docs)]
pub enum MckDivider {
    Div1 = 1,
    Div2 = 2,
    Div3 = 3,
    Div4 = 4,
}

impl From<MckDivider> for MCK_DIV {
    fn from(pres: MckDivider) -> Self {
        use MckDivider::*;
        use MCK_DIV::*;

        match pres {
            Div1 => EQ_PCK,
            Div2 => PCK_DIV2,
            Div3 => PCK_DIV3,
            Div4 => PCK_DIV4,
        }
    }
}

/// MCK, driven by [`SlowClock`], [`MainClock`], [`UpllDivClock`], or
/// [`PllaClock`].
pub struct HostClock {
    freq: Hertz,
}
impl generics::Sealed for HostClock {}

/// HCLK, driven by the same clock as [`HostClock`].
pub struct ProcessorClock {
    freq: Hertz,
}
impl generics::Sealed for ProcessorClock {}

/// [`HostClock`] and [`ProcessorClock`] prescaler and divider configuration.
///
/// ```no_compile
/// hclk <- input_clock / pres
/// mck  <- input_clock / pres / div
/// ```
pub struct HostClockConfig {
    /// Prescaler that affects [`HostClock`] and [`ProcessorClock`].
    pub pres: HccPrescaler,
    /// Divider that only affects the [`HostClock`].
    pub div: MckDivider,
}

impl ProcessorClock {
    /// Return the (calculated) frequency of the ARMv7-M SysTick
    /// clock.
    pub fn systick_freq(&self) -> Hertz {
        // §31.3 would suggest that SysTick is HCLK/2, but experiments
        // show that it is equal to HCLK.
        self.freq
    }
}

/// Model of the hierarchy's Host Clock Controller (HCC), from which
/// [`HostClock`] and [`ProcessorClock`] are sourced.
pub struct HostClockController;

impl HostClockController {
    /// Create a new HCC from the required clock [`Token`]s.
    pub fn new(_hclk: Token<ProcessorClock>, _mck: Token<HostClock>) -> Self {
        Self
    }
}

unsafe impl RegisterAccess for HostClockController {}
unsafe impl RegisterAccess for HostClock {}

impl HostClockController {
    /// Configures HCLK and MCK and returns corresponding Clock Tokens.
    /// This method corresponds to Step 7 in 31.17.
    pub fn configure<SRC: HccClockSource>(
        self,
        source: &SRC,
        efc: &mut Efc,
        HostClockConfig { pres, div }: HostClockConfig,
    ) -> Result<(ProcessorClock, HostClock), ClockError> {
        // Ensure we use the correct amount of wait states for flash
        // access for the new HCLK frequency.
        efc.set_wait_states(source.freq().convert() / (pres as u32))?;

        let freq = source.freq();

        let source = SRC::HCC_CSS;
        match source {
            HCC_CSS::PLLA_CLK | HCC_CSS::UPLL_CLK => {
                self.pmc().mckr.modify(|_, w| w.pres().variant(pres.into()));
                while self.pmc().sr.read().mckrdy().bit_is_clear() {}

                self.pmc().mckr.modify(|_, w| w.mdiv().variant(div.into()));
                while self.pmc().sr.read().mckrdy().bit_is_clear() {}

                self.pmc().mckr.modify(|_, w| w.css().variant(source));
                while self.pmc().sr.read().mckrdy().bit_is_clear() {}
            }
            HCC_CSS::MAIN_CLK | HCC_CSS::SLOW_CLK => {
                self.pmc().mckr.modify(|_, w| w.css().variant(source));
                while self.pmc().sr.read().mckrdy().bit_is_clear() {}

                self.pmc().mckr.modify(|_, w| w.pres().variant(pres.into()));
                while self.pmc().sr.read().mckrdy().bit_is_clear() {}

                self.pmc().mckr.modify(|_, w| w.mdiv().variant(div.into()));
                while self.pmc().sr.read().mckrdy().bit_is_clear() {}
            }
        }

        Ok((
            ProcessorClock {
                freq: freq / (pres as u32),
            },
            HostClock {
                freq: freq / (pres as u32) / (div as u32),
            },
        ))
    }
}

impl HostClock {
    pub(crate) fn enable_peripheral(&mut self, pid: PeripheralIdentifier) {
        if !pid.requires_enable() {
            return;
        }

        match pid as u32 {
            7..=31 => {
                let mask = 1 << pid as u32;
                self.pmc().pcer0.write(|w| unsafe { w.bits(mask) });
                while self.pmc().pcsr0.read().bits() & mask == 0 {}
            }
            32..=62 => {
                let mask = 1 << (pid as u32 - 32);
                self.pmc().pcer1.write(|w| unsafe { w.bits(mask) });
                while self.pmc().pcsr1.read().bits() & mask == 0 {}
            }
            _ => unimplemented!(),
        }
    }
}

impl Clock for ProcessorClock {
    fn freq(&self) -> Hertz {
        self.freq
    }
}

impl Clock for HostClock {
    fn freq(&self) -> Hertz {
        self.freq
    }
}

/// The source of the [`HostClock`] and [`ProcessorClock`].
pub trait HccClockSource: Clock {
    #[doc(hidden)]
    const HCC_CSS: HCC_CSS;
}

impl<S: SlowClockSource> HccClockSource for SlowClock<S> {
    const HCC_CSS: HCC_CSS = HCC_CSS::SLOW_CLK;
}
impl<S: MainClockSource> HccClockSource for MainClock<S> {
    const HCC_CSS: HCC_CSS = HCC_CSS::MAIN_CLK;
}
impl HccClockSource for PllaClock {
    const HCC_CSS: HCC_CSS = HCC_CSS::PLLA_CLK;
}
impl HccClockSource for UpllDivClock {
    const HCC_CSS: HCC_CSS = HCC_CSS::UPLL_CLK;
}
