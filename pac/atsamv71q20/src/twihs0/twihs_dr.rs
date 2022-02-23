#[doc = "Register `TWIHS_DR` reader"]
pub struct R(crate::R<TWIHS_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWIHS_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWIHS_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWIHS_DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SWEN` reader - SleepWalking Enable"]
pub struct SWEN_R(crate::FieldReader<bool, bool>);
impl SWEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKRQ` reader - Clock Request"]
pub struct CLKRQ_R(crate::FieldReader<bool, bool>);
impl CLKRQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWMATCH` reader - SleepWalking Match"]
pub struct SWMATCH_R(crate::FieldReader<bool, bool>);
impl SWMATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWMATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWMATCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRP` reader - Transfer Pending"]
pub struct TRP_R(crate::FieldReader<bool, bool>);
impl TRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - SleepWalking Enable"]
    #[inline(always)]
    pub fn swen(&self) -> SWEN_R {
        SWEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock Request"]
    #[inline(always)]
    pub fn clkrq(&self) -> CLKRQ_R {
        CLKRQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SleepWalking Match"]
    #[inline(always)]
    pub fn swmatch(&self) -> SWMATCH_R {
        SWMATCH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transfer Pending"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "Debug Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_dr](index.html) module"]
pub struct TWIHS_DR_SPEC;
impl crate::RegisterSpec for TWIHS_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twihs_dr::R](R) reader structure"]
impl crate::Readable for TWIHS_DR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TWIHS_DR to value 0"]
impl crate::Resettable for TWIHS_DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
