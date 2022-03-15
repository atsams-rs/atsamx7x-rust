#[doc = "Register `MLB_HCER[%s]` reader"]
pub struct R(crate::R<MLB_HCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MLB_HCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MLB_HCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MLB_HCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CERR` reader - Bitwise Channel Error Bit \\[31:0\\]"]
pub struct CERR_R(crate::FieldReader<u32, u32>);
impl CERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CERR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Error Bit \\[31:0\\]"]
    #[inline(always)]
    pub fn cerr(&self) -> CERR_R {
        CERR_R::new(self.bits)
    }
}
#[doc = "HBI Channel Error 0 Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlb_hcer](index.html) module"]
pub struct MLB_HCER_SPEC;
impl crate::RegisterSpec for MLB_HCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mlb_hcer::R](R) reader structure"]
impl crate::Readable for MLB_HCER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MLB_HCER[%s]
to value 0"]
impl crate::Resettable for MLB_HCER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
