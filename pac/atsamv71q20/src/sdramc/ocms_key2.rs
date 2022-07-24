#[doc = "Register `OCMS_KEY2` writer"]
pub struct W(crate::W<OCMS_KEY2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCMS_KEY2_SPEC>;
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
impl From<crate::W<OCMS_KEY2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCMS_KEY2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY2` writer - Off-chip Memory Scrambling (OCMS) Key Part 2"]
pub type KEY2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCMS_KEY2_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Off-chip Memory Scrambling (OCMS) Key Part 2"]
    #[inline(always)]
    pub fn key2(&mut self) -> KEY2_W<0> {
        KEY2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAMC OCMS KEY2 Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocms_key2](index.html) module"]
pub struct OCMS_KEY2_SPEC;
impl crate::RegisterSpec for OCMS_KEY2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ocms_key2::W](W) writer structure"]
impl crate::Writable for OCMS_KEY2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OCMS_KEY2 to value 0"]
impl crate::Resettable for OCMS_KEY2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
