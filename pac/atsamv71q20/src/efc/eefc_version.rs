#[doc = "Register `EEFC_VERSION` reader"]
pub struct R(crate::R<EEFC_VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEFC_VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEFC_VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEFC_VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VERSION` reader - Version of the Hardware Module"]
pub type VERSION_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MFN` reader - Metal Fix Number"]
pub type MFN_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - Version of the Hardware Module"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:18 - Metal Fix Number"]
    #[inline(always)]
    pub fn mfn(&self) -> MFN_R {
        MFN_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "EEFC Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefc_version](index.html) module"]
pub struct EEFC_VERSION_SPEC;
impl crate::RegisterSpec for EEFC_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eefc_version::R](R) reader structure"]
impl crate::Readable for EEFC_VERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EEFC_VERSION to value 0"]
impl crate::Resettable for EEFC_VERSION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
