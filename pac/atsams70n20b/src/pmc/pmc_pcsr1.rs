#[doc = "Reader of register PMC_PCSR1"]
pub type R = crate::R<u32, super::PMC_PCSR1>;
#[doc = "Reader of field `PID32`"]
pub type PID32_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID33`"]
pub type PID33_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID34`"]
pub type PID34_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID40`"]
pub type PID40_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID41`"]
pub type PID41_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID43`"]
pub type PID43_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID44`"]
pub type PID44_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID45`"]
pub type PID45_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID46`"]
pub type PID46_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID47`"]
pub type PID47_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID48`"]
pub type PID48_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID49`"]
pub type PID49_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID50`"]
pub type PID50_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID51`"]
pub type PID51_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID52`"]
pub type PID52_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID56`"]
pub type PID56_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID57`"]
pub type PID57_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID58`"]
pub type PID58_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID59`"]
pub type PID59_R = crate::R<bool, bool>;
#[doc = "Reader of field `PID60`"]
pub type PID60_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Peripheral Clock 32 Status"]
    #[inline(always)]
    pub fn pid32(&self) -> PID32_R {
        PID32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Status"]
    #[inline(always)]
    pub fn pid33(&self) -> PID33_R {
        PID33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Status"]
    #[inline(always)]
    pub fn pid34(&self) -> PID34_R {
        PID34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Peripheral Clock 40 Status"]
    #[inline(always)]
    pub fn pid40(&self) -> PID40_R {
        PID40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Peripheral Clock 41 Status"]
    #[inline(always)]
    pub fn pid41(&self) -> PID41_R {
        PID41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Peripheral Clock 43 Status"]
    #[inline(always)]
    pub fn pid43(&self) -> PID43_R {
        PID43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Peripheral Clock 44 Status"]
    #[inline(always)]
    pub fn pid44(&self) -> PID44_R {
        PID44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Peripheral Clock 45 Status"]
    #[inline(always)]
    pub fn pid45(&self) -> PID45_R {
        PID45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Peripheral Clock 46 Status"]
    #[inline(always)]
    pub fn pid46(&self) -> PID46_R {
        PID46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Peripheral Clock 47 Status"]
    #[inline(always)]
    pub fn pid47(&self) -> PID47_R {
        PID47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Peripheral Clock 48 Status"]
    #[inline(always)]
    pub fn pid48(&self) -> PID48_R {
        PID48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Peripheral Clock 49 Status"]
    #[inline(always)]
    pub fn pid49(&self) -> PID49_R {
        PID49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Peripheral Clock 50 Status"]
    #[inline(always)]
    pub fn pid50(&self) -> PID50_R {
        PID50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Peripheral Clock 51 Status"]
    #[inline(always)]
    pub fn pid51(&self) -> PID51_R {
        PID51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Peripheral Clock 52 Status"]
    #[inline(always)]
    pub fn pid52(&self) -> PID52_R {
        PID52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Peripheral Clock 56 Status"]
    #[inline(always)]
    pub fn pid56(&self) -> PID56_R {
        PID56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Peripheral Clock 57 Status"]
    #[inline(always)]
    pub fn pid57(&self) -> PID57_R {
        PID57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Peripheral Clock 58 Status"]
    #[inline(always)]
    pub fn pid58(&self) -> PID58_R {
        PID58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Peripheral Clock 59 Status"]
    #[inline(always)]
    pub fn pid59(&self) -> PID59_R {
        PID59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Peripheral Clock 60 Status"]
    #[inline(always)]
    pub fn pid60(&self) -> PID60_R {
        PID60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
