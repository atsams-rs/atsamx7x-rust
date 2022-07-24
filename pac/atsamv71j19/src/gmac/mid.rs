#[doc = "Register `MID` reader"]
pub struct R(crate::R<MID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MREV` reader - Module Revision"]
pub type MREV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MID` reader - Module Identification Number"]
pub type MID_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Module Revision"]
    #[inline(always)]
    pub fn mrev(&self) -> MREV_R {
        MREV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Module Identification Number"]
    #[inline(always)]
    pub fn mid(&self) -> MID_R {
        MID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Module ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mid](index.html) module"]
pub struct MID_SPEC;
impl crate::RegisterSpec for MID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mid::R](R) reader structure"]
impl crate::Readable for MID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MID to value 0"]
impl crate::Resettable for MID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
