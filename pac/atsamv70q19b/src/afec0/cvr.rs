#[doc = "Register `CVR` reader"]
pub struct R(crate::R<CVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CVR` writer"]
pub struct W(crate::W<CVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CVR_SPEC>;
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
impl From<crate::W<CVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSETCORR` reader - Offset Correction"]
pub type OFFSETCORR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OFFSETCORR` writer - Offset Correction"]
pub type OFFSETCORR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CVR_SPEC, u16, u16, 16, O>;
#[doc = "Field `GAINCORR` reader - Gain Correction"]
pub type GAINCORR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GAINCORR` writer - Gain Correction"]
pub type GAINCORR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CVR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Offset Correction"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OFFSETCORR_R {
        OFFSETCORR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Gain Correction"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GAINCORR_R {
        GAINCORR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Offset Correction"]
    #[inline(always)]
    pub fn offsetcorr(&mut self) -> OFFSETCORR_W<0> {
        OFFSETCORR_W::new(self)
    }
    #[doc = "Bits 16:31 - Gain Correction"]
    #[inline(always)]
    pub fn gaincorr(&mut self) -> GAINCORR_W<16> {
        GAINCORR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Correction Values Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cvr](index.html) module"]
pub struct CVR_SPEC;
impl crate::RegisterSpec for CVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cvr::R](R) reader structure"]
impl crate::Readable for CVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cvr::W](W) writer structure"]
impl crate::Writable for CVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CVR to value 0"]
impl crate::Resettable for CVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
