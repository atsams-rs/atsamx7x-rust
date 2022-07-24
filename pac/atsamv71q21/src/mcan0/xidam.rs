#[doc = "Register `XIDAM` reader"]
pub struct R(crate::R<XIDAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XIDAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XIDAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XIDAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XIDAM` writer"]
pub struct W(crate::W<XIDAM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XIDAM_SPEC>;
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
impl From<crate::W<XIDAM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XIDAM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EIDM` reader - Extended ID Mask"]
pub type EIDM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EIDM` writer - Extended ID Mask"]
pub type EIDM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XIDAM_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:28 - Extended ID Mask"]
    #[inline(always)]
    pub fn eidm(&self) -> EIDM_R {
        EIDM_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:28 - Extended ID Mask"]
    #[inline(always)]
    pub fn eidm(&mut self) -> EIDM_W<0> {
        EIDM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended ID AND Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xidam](index.html) module"]
pub struct XIDAM_SPEC;
impl crate::RegisterSpec for XIDAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xidam::R](R) reader structure"]
impl crate::Readable for XIDAM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xidam::W](W) writer structure"]
impl crate::Writable for XIDAM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XIDAM to value 0"]
impl crate::Resettable for XIDAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
