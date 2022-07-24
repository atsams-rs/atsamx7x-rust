#[doc = "Register `SCL` reader"]
pub struct R(crate::R<SCL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCL` writer"]
pub struct W(crate::W<SCL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCL_SPEC>;
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
impl From<crate::W<SCL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC` reader - 1588 Timer Second Comparison Value"]
pub type SEC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SEC` writer - 1588 Timer Second Comparison Value"]
pub type SEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 1588 Timer Second Comparison Value"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1588 Timer Second Comparison Value"]
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W<0> {
        SEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Second Comparison Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl](index.html) module"]
pub struct SCL_SPEC;
impl crate::RegisterSpec for SCL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scl::R](R) reader structure"]
impl crate::Readable for SCL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scl::W](W) writer structure"]
impl crate::Writable for SCL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCL to value 0"]
impl crate::Resettable for SCL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
