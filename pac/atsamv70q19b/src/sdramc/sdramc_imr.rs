#[doc = "Register `SDRAMC_IMR` reader"]
pub struct R(crate::R<SDRAMC_IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRAMC_IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRAMC_IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRAMC_IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RES` reader - Refresh Error Interrupt Mask"]
pub struct RES_R(crate::FieldReader<bool, bool>);
impl RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Refresh Error Interrupt Mask"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "SDRAMC Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramc_imr](index.html) module"]
pub struct SDRAMC_IMR_SPEC;
impl crate::RegisterSpec for SDRAMC_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdramc_imr::R](R) reader structure"]
impl crate::Readable for SDRAMC_IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDRAMC_IMR to value 0"]
impl crate::Resettable for SDRAMC_IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
