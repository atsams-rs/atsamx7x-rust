#[doc = "Register `IVR[%s]` writer"]
pub struct W(crate::W<IVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IVR_SPEC>;
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
impl From<crate::W<IVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IV` writer - Initialization Vector"]
pub type IV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IVR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Initialization Vector"]
    #[inline(always)]
    pub fn iv(&mut self) -> IV_W<0> {
        IV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Initialization Vector Register 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivr](index.html) module"]
pub struct IVR_SPEC;
impl crate::RegisterSpec for IVR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ivr::W](W) writer structure"]
impl crate::Writable for IVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IVR[%s]
to value 0"]
impl crate::Resettable for IVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
