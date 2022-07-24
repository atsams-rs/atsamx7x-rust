#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RHC` reader - Region Hash Completed Interrupt Mask"]
pub type RHC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDM` reader - Region Digest Mismatch Interrupt Mask"]
pub type RDM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RBE` reader - Region Bus Error Interrupt Mask"]
pub type RBE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RWC` reader - Region Wrap Condition Detected Interrupt Mask"]
pub type RWC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REC` reader - Region End bit Condition Detected Interrupt Mask"]
pub type REC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSU` reader - Region Status Updated Interrupt Mask"]
pub type RSU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `URAD` reader - Undefined Register Access Detection Interrupt Mask"]
pub type URAD_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Region Hash Completed Interrupt Mask"]
    #[inline(always)]
    pub fn rhc(&self) -> RHC_R {
        RHC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Region Digest Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn rdm(&self) -> RDM_R {
        RDM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Region Bus Error Interrupt Mask"]
    #[inline(always)]
    pub fn rbe(&self) -> RBE_R {
        RBE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Region Wrap Condition Detected Interrupt Mask"]
    #[inline(always)]
    pub fn rwc(&self) -> RWC_R {
        RWC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Region End bit Condition Detected Interrupt Mask"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Region Status Updated Interrupt Mask"]
    #[inline(always)]
    pub fn rsu(&self) -> RSU_R {
        RSU_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Undefined Register Access Detection Interrupt Mask"]
    #[inline(always)]
    pub fn urad(&self) -> URAD_R {
        URAD_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
