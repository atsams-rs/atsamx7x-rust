#[doc = "Register `US_VERSION` reader"]
pub struct R(crate::R<US_VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VERSION` reader - Hardware Module Version"]
pub type VERSION_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MFN` reader - Metal Fix Number"]
pub type MFN_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - Hardware Module Version"]
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
#[doc = "Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_version](index.html) module"]
pub struct US_VERSION_SPEC;
impl crate::RegisterSpec for US_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_version::R](R) reader structure"]
impl crate::Readable for US_VERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets US_VERSION to value 0"]
impl crate::Resettable for US_VERSION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
