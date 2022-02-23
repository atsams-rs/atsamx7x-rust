#[doc = "Register `QSPI_VERSION` reader"]
pub struct R(crate::R<QSPI_VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QSPI_VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QSPI_VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QSPI_VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VERSION` reader - Hardware Module Version"]
pub struct VERSION_R(crate::FieldReader<u16, u16>);
impl VERSION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VERSION_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MFN` reader - Metal Fix Number"]
pub struct MFN_R(crate::FieldReader<u8, u8>);
impl MFN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MFN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11 - Hardware Module Version"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:18 - Metal Fix Number"]
    #[inline(always)]
    pub fn mfn(&self) -> MFN_R {
        MFN_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
#[doc = "Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_version](index.html) module"]
pub struct QSPI_VERSION_SPEC;
impl crate::RegisterSpec for QSPI_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qspi_version::R](R) reader structure"]
impl crate::Readable for QSPI_VERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QSPI_VERSION to value 0"]
impl crate::Resettable for QSPI_VERSION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
