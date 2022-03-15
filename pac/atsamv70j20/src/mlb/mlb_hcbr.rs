#[doc = "Register `MLB_HCBR[%s]` reader"]
pub struct R(crate::R<MLB_HCBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MLB_HCBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MLB_HCBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MLB_HCBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHB` reader - Bitwise Channel Busy Bit \\[31:0\\]"]
pub struct CHB_R(crate::FieldReader<u32, u32>);
impl CHB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CHB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Busy Bit \\[31:0\\]"]
    #[inline(always)]
    pub fn chb(&self) -> CHB_R {
        CHB_R::new(self.bits)
    }
}
#[doc = "HBI Channel Busy 0 Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlb_hcbr](index.html) module"]
pub struct MLB_HCBR_SPEC;
impl crate::RegisterSpec for MLB_HCBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mlb_hcbr::R](R) reader structure"]
impl crate::Readable for MLB_HCBR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MLB_HCBR[%s]
to value 0"]
impl crate::Resettable for MLB_HCBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
