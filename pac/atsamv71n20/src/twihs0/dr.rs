#[doc = "Register `DR` reader"]
pub struct R(crate::R<DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SWEN` reader - SleepWalking Enable"]
pub type SWEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKRQ` reader - Clock Request"]
pub type CLKRQ_R = crate::BitReader<bool>;
#[doc = "Field `SWMATCH` reader - SleepWalking Match"]
pub type SWMATCH_R = crate::BitReader<bool>;
#[doc = "Field `TRP` reader - Transfer Pending"]
pub type TRP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - SleepWalking Enable"]
    #[inline(always)]
    pub fn swen(&self) -> SWEN_R {
        SWEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Request"]
    #[inline(always)]
    pub fn clkrq(&self) -> CLKRQ_R {
        CLKRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SleepWalking Match"]
    #[inline(always)]
    pub fn swmatch(&self) -> SWMATCH_R {
        SWMATCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer Pending"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Debug Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](index.html) module"]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr::R](R) reader structure"]
impl crate::Readable for DR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
