#[doc = "Register `ICM_SR` writer"]
pub struct W(crate::W<ICM_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICM_SR_SPEC>;
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
impl From<crate::W<ICM_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICM_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` writer - ICM Controller Enable Register"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `RAWRMDIS` writer - Region Monitoring Disabled Raw Status"]
pub struct RAWRMDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RAWRMDIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `RMDIS` writer - Region Monitoring Disabled Status"]
pub struct RMDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RMDIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - ICM Controller Enable Register"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 8:11 - Region Monitoring Disabled Raw Status"]
    #[inline(always)]
    pub fn rawrmdis(&mut self) -> RAWRMDIS_W {
        RAWRMDIS_W { w: self }
    }
    #[doc = "Bits 12:15 - Region Monitoring Disabled Status"]
    #[inline(always)]
    pub fn rmdis(&mut self) -> RMDIS_W {
        RMDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icm_sr](index.html) module"]
pub struct ICM_SR_SPEC;
impl crate::RegisterSpec for ICM_SR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icm_sr::W](W) writer structure"]
impl crate::Writable for ICM_SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICM_SR to value 0"]
impl crate::Resettable for ICM_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
