#[doc = "Register `OCMS_KEY1` writer"]
pub struct W(crate::W<OCMS_KEY1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCMS_KEY1_SPEC>;
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
impl From<crate::W<OCMS_KEY1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCMS_KEY1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY1` writer - Off-chip Memory Scrambling (OCMS) Key Part 1"]
pub type KEY1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OCMS_KEY1_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Off-chip Memory Scrambling (OCMS) Key Part 1"]
    #[inline(always)]
    pub fn key1(&mut self) -> KEY1_W<0> {
        KEY1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAMC OCMS KEY1 Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocms_key1](index.html) module"]
pub struct OCMS_KEY1_SPEC;
impl crate::RegisterSpec for OCMS_KEY1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ocms_key1::W](W) writer structure"]
impl crate::Writable for OCMS_KEY1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OCMS_KEY1 to value 0"]
impl crate::Resettable for OCMS_KEY1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
