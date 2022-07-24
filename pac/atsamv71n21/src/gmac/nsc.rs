#[doc = "Register `NSC` reader"]
pub struct R(crate::R<NSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NSC` writer"]
pub struct W(crate::W<NSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSC_SPEC>;
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
impl From<crate::W<NSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NANOSEC` reader - 1588 Timer Nanosecond Comparison Value"]
pub type NANOSEC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NANOSEC` writer - 1588 Timer Nanosecond Comparison Value"]
pub type NANOSEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NSC_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:21 - 1588 Timer Nanosecond Comparison Value"]
    #[inline(always)]
    pub fn nanosec(&self) -> NANOSEC_R {
        NANOSEC_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - 1588 Timer Nanosecond Comparison Value"]
    #[inline(always)]
    pub fn nanosec(&mut self) -> NANOSEC_W<0> {
        NANOSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Nanosecond Comparison Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsc](index.html) module"]
pub struct NSC_SPEC;
impl crate::RegisterSpec for NSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nsc::R](R) reader structure"]
impl crate::Readable for NSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nsc::W](W) writer structure"]
impl crate::Writable for NSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NSC to value 0"]
impl crate::Resettable for NSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
