use super::*;
use crate::target_device::pmc::pmc_pck::CSS_A as PCK_CSS;

impl<I: PckId> Clock for Pck<I> {
    fn freq(&self) -> Hertz {
        self.freq
    }
}

impl<I: PckId> Token<Pck<I>> {
    /// Configures PCKx and returns a token. Corresponds to Step 8 in
    /// 31.17
    pub fn configure<SRC: PckSource>(self, source: &SRC, pres: u8) -> Pck<I> {
        self.pmc().pmc_pck[I::ID as usize].write(|w| unsafe {
            w.pres().bits(pres);
            w.css().bits(SRC::PCK_CSS as u8)
        });

        // PCK fields are in the second byte of the SCER and SCSR
        // registers.
        const PCK_REG_OFFSET: u8 = 8;
        let mask = 1 << (I::ID + PCK_REG_OFFSET);

        self.pmc().pmc_scer.write(|w| unsafe { w.bits(mask) });
        while (self.pmc().pmc_scsr.read().bits() & mask) == 0 {}
        Pck {
            id: PhantomData,
            freq: source.freq() / (pres as u32),
        }
    }
}

/// Set of [`Token`]s for all device [`Pck`]s.
#[allow(missing_docs)]
pub struct PckTokens {
    pub pck0: Token<Pck<Pck0>>,
    pub pck1: Token<Pck<Pck1>>,
    pub pck2: Token<Pck<Pck2>>,
    pub pck3: Token<Pck<Pck3>>,
    pub pck4: Token<Pck<Pck4>>,
    pub pck5: Token<Pck<Pck5>>,
    pub pck6: Token<Pck<Pck6>>,
    pub pck7: Token<Pck<Pck7>>,
}

impl PckTokens {
    pub(crate) fn default() -> Self {
        Self {
            pck0: Token::default(),
            pck1: Token::default(),
            pck2: Token::default(),
            pck3: Token::default(),
            pck4: Token::default(),
            pck5: Token::default(),
            pck6: Token::default(),
            pck7: Token::default(),
        }
    }
}

/// [`Pck`] identifier.
pub trait PckId {
    #[doc(hidden)]
    const ID: u8;
}

macro_rules! impl_pck {
    ($($Id:literal,)+) => {
        paste! {
            $(
                #[doc = "Identifier for PCK" $Id "."]
                pub enum [<Pck $Id>] {}
                impl PckId for [<Pck $Id>] {
                    const ID: u8 = $Id;
                }
            )+
        }
    }
}

impl_pck!(0, 1, 2, 3, 4, 5, 6, 7,);

/// PCK, driven by [`SlowClock`] or [`MainClock`].
pub struct Pck<I: PckId> {
    id: PhantomData<I>,
    freq: Hertz,
}

#[doc(hidden)]
pub trait PckSource: Clock {
    const PCK_CSS: PCK_CSS;
}

impl<S: SlowClockSource> PckSource for SlowClock<S> {
    const PCK_CSS: PCK_CSS = PCK_CSS::SLOW_CLK;
}
impl<S: MainClockSource> PckSource for MainClock<S> {
    const PCK_CSS: PCK_CSS = PCK_CSS::MAIN_CLK;
}
impl PckSource for UpllDivClock {
    const PCK_CSS: PCK_CSS = PCK_CSS::UPLL_CLK;
}
impl PckSource for PllaClock {
    const PCK_CSS: PCK_CSS = PCK_CSS::PLLA_CLK;
}
impl PckSource for HostClock {
    const PCK_CSS: PCK_CSS = PCK_CSS::MCK;
}
