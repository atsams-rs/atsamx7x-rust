#[doc = "Register `SDRAMC_ISR` reader"]
pub struct R(crate::R<SDRAMC_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRAMC_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRAMC_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRAMC_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RES` reader - Refresh Error Status (cleared on read)"]
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
    #[doc = "Bit 0 - Refresh Error Status (cleared on read)"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "SDRAMC Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramc_isr](index.html) module"]
pub struct SDRAMC_ISR_SPEC;
impl crate::RegisterSpec for SDRAMC_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdramc_isr::R](R) reader structure"]
impl crate::Readable for SDRAMC_ISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDRAMC_ISR to value 0"]
impl crate::Resettable for SDRAMC_ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
