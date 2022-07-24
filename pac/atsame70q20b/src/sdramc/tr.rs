#[doc = "Register `TR` reader"]
pub struct R(crate::R<TR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR` writer"]
pub struct W(crate::W<TR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_SPEC>;
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
impl From<crate::W<TR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - SDRAMC Refresh Timer Count"]
pub type COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT` writer - SDRAMC Refresh Timer Count"]
pub type COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - SDRAMC Refresh Timer Count"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - SDRAMC Refresh Timer Count"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W<0> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAMC Refresh Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr](index.html) module"]
pub struct TR_SPEC;
impl crate::RegisterSpec for TR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr::R](R) reader structure"]
impl crate::Readable for TR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr::W](W) writer structure"]
impl crate::Writable for TR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TR to value 0"]
impl crate::Resettable for TR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
