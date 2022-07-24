#[doc = "Register `IDR` writer"]
pub struct W(crate::W<IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RHC` writer - Region Hash Completed Interrupt Disable"]
pub type RHC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RDM` writer - Region Digest Mismatch Interrupt Disable"]
pub type RDM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RBE` writer - Region Bus Error Interrupt Disable"]
pub type RBE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RWC` writer - Region Wrap Condition Detected Interrupt Disable"]
pub type RWC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `REC` writer - Region End bit Condition detected Interrupt Disable"]
pub type REC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RSU` writer - Region Status Updated Interrupt Disable"]
pub type RSU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDR_SPEC, u8, u8, 4, O>;
#[doc = "Field `URAD` writer - Undefined Register Access Detection Interrupt Disable"]
pub type URAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:3 - Region Hash Completed Interrupt Disable"]
    #[inline(always)]
    pub fn rhc(&mut self) -> RHC_W<0> {
        RHC_W::new(self)
    }
    #[doc = "Bits 4:7 - Region Digest Mismatch Interrupt Disable"]
    #[inline(always)]
    pub fn rdm(&mut self) -> RDM_W<4> {
        RDM_W::new(self)
    }
    #[doc = "Bits 8:11 - Region Bus Error Interrupt Disable"]
    #[inline(always)]
    pub fn rbe(&mut self) -> RBE_W<8> {
        RBE_W::new(self)
    }
    #[doc = "Bits 12:15 - Region Wrap Condition Detected Interrupt Disable"]
    #[inline(always)]
    pub fn rwc(&mut self) -> RWC_W<12> {
        RWC_W::new(self)
    }
    #[doc = "Bits 16:19 - Region End bit Condition detected Interrupt Disable"]
    #[inline(always)]
    pub fn rec(&mut self) -> REC_W<16> {
        REC_W::new(self)
    }
    #[doc = "Bits 20:23 - Region Status Updated Interrupt Disable"]
    #[inline(always)]
    pub fn rsu(&mut self) -> RSU_W<20> {
        RSU_W::new(self)
    }
    #[doc = "Bit 24 - Undefined Register Access Detection Interrupt Disable"]
    #[inline(always)]
    pub fn urad(&mut self) -> URAD_W<24> {
        URAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idr::W](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
