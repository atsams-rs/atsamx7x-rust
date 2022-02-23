#[doc = "Register `GMAC_MID` reader"]
pub struct R(crate::R<GMAC_MID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GMAC_MID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GMAC_MID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GMAC_MID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MREV` reader - Module Revision"]
pub struct MREV_R(crate::FieldReader<u16, u16>);
impl MREV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MREV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MREV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MID` reader - Module Identification Number"]
pub struct MID_R(crate::FieldReader<u16, u16>);
impl MID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Module ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_mid](index.html) module"]
pub struct GMAC_MID_SPEC;
impl crate::RegisterSpec for GMAC_MID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gmac_mid::R](R) reader structure"]
impl crate::Readable for GMAC_MID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GMAC_MID to value 0"]
impl crate::Resettable for GMAC_MID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
