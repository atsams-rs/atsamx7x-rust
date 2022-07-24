#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` writer - ICM Controller Enable Register"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `RAWRMDIS` writer - Region Monitoring Disabled Raw Status"]
pub type RAWRMDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RMDIS` writer - Region Monitoring Disabled Status"]
pub type RMDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SR_SPEC, u8, u8, 4, O>;
impl W {
    #[doc = "Bit 0 - ICM Controller Enable Register"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 8:11 - Region Monitoring Disabled Raw Status"]
    #[inline(always)]
    pub fn rawrmdis(&mut self) -> RAWRMDIS_W<8> {
        RAWRMDIS_W::new(self)
    }
    #[doc = "Bits 12:15 - Region Monitoring Disabled Status"]
    #[inline(always)]
    pub fn rmdis(&mut self) -> RMDIS_W<12> {
        RMDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
