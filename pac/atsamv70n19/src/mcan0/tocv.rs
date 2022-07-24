#[doc = "Register `TOCV` reader"]
pub struct R(crate::R<TOCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOCV` writer"]
pub struct W(crate::W<TOCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOCV_SPEC>;
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
impl From<crate::W<TOCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOCV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOC` reader - Timeout Counter (cleared on write)"]
pub type TOC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOC` writer - Timeout Counter (cleared on write)"]
pub type TOC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOCV_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Timeout Counter (cleared on write)"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timeout Counter (cleared on write)"]
    #[inline(always)]
    pub fn toc(&mut self) -> TOC_W<0> {
        TOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timeout Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tocv](index.html) module"]
pub struct TOCV_SPEC;
impl crate::RegisterSpec for TOCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tocv::R](R) reader structure"]
impl crate::Readable for TOCV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tocv::W](W) writer structure"]
impl crate::Writable for TOCV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOCV to value 0"]
impl crate::Resettable for TOCV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
