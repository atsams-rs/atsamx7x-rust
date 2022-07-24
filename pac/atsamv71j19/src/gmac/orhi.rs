#[doc = "Register `ORHI` reader"]
pub struct R(crate::R<ORHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ORHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ORHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ORHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXO` reader - Received Octets"]
pub type RXO_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Received Octets"]
    #[inline(always)]
    pub fn rxo(&self) -> RXO_R {
        RXO_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Octets Received High Received Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [orhi](index.html) module"]
pub struct ORHI_SPEC;
impl crate::RegisterSpec for ORHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [orhi::R](R) reader structure"]
impl crate::Readable for ORHI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ORHI to value 0"]
impl crate::Resettable for ORHI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
