#[doc = "Register `SCH` reader"]
pub struct R(crate::R<SCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCH` writer"]
pub struct W(crate::W<SCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCH_SPEC>;
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
impl From<crate::W<SCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC` reader - 1588 Timer Second Comparison Value"]
pub type SEC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SEC` writer - 1588 Timer Second Comparison Value"]
pub type SEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 1588 Timer Second Comparison Value"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 1588 Timer Second Comparison Value"]
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
#[doc = "1588 Timer Second Comparison High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sch](index.html) module"]
pub struct SCH_SPEC;
impl crate::RegisterSpec for SCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sch::R](R) reader structure"]
impl crate::Readable for SCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sch::W](W) writer structure"]
impl crate::Writable for SCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCH to value 0"]
impl crate::Resettable for SCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
