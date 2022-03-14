#[doc = "Register `MLB_ACMR[%s]` reader"]
pub struct R(crate::R<MLB_ACMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MLB_ACMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MLB_ACMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MLB_ACMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MLB_ACMR[%s]` writer"]
pub struct W(crate::W<MLB_ACMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MLB_ACMR_SPEC>;
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
impl From<crate::W<MLB_ACMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MLB_ACMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHM` reader - Bitwise Channel Mask Bits 31 to 0"]
pub struct CHM_R(crate::FieldReader<u32, u32>);
impl CHM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CHM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHM` writer - Bitwise Channel Mask Bits 31 to 0"]
pub struct CHM_W<'a> {
    w: &'a mut W,
}
impl<'a> CHM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Bitwise Channel Mask Bits 31 to 0"]
    #[inline(always)]
    pub fn chm(&self) -> CHM_R {
        CHM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bitwise Channel Mask Bits 31 to 0"]
    #[inline(always)]
    pub fn chm(&mut self) -> CHM_W {
        CHM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Channel Mask 0 Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlb_acmr](index.html) module"]
pub struct MLB_ACMR_SPEC;
impl crate::RegisterSpec for MLB_ACMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mlb_acmr::R](R) reader structure"]
impl crate::Readable for MLB_ACMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mlb_acmr::W](W) writer structure"]
impl crate::Writable for MLB_ACMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MLB_ACMR[%s]
to value 0"]
impl crate::Resettable for MLB_ACMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
