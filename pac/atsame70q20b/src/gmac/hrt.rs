#[doc = "Register `HRT` reader"]
pub struct R(crate::R<HRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HRT` writer"]
pub struct W(crate::W<HRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HRT_SPEC>;
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
impl From<crate::W<HRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HRT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Hash Address"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Hash Address"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HRT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Hash Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hash Register Top\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrt](index.html) module"]
pub struct HRT_SPEC;
impl crate::RegisterSpec for HRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrt::R](R) reader structure"]
impl crate::Readable for HRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hrt::W](W) writer structure"]
impl crate::Writable for HRT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HRT to value 0"]
impl crate::Resettable for HRT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
