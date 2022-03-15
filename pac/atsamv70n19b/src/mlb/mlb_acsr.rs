#[doc = "Register `MLB_ACSR[%s]` reader"]
pub struct R(crate::R<MLB_ACSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MLB_ACSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MLB_ACSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MLB_ACSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MLB_ACSR[%s]` writer"]
pub struct W(crate::W<MLB_ACSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MLB_ACSR_SPEC>;
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
impl From<crate::W<MLB_ACSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MLB_ACSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHS` reader - Interrupt Status for Logical Channels \\[31:0\\]
(cleared by writing a 1)"]
pub struct CHS_R(crate::FieldReader<u32, u32>);
impl CHS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CHS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHS` writer - Interrupt Status for Logical Channels \\[31:0\\]
(cleared by writing a 1)"]
pub struct CHS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt Status for Logical Channels \\[31:0\\]
(cleared by writing a 1)"]
    #[inline(always)]
    pub fn chs(&self) -> CHS_R {
        CHS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Status for Logical Channels \\[31:0\\]
(cleared by writing a 1)"]
    #[inline(always)]
    pub fn chs(&mut self) -> CHS_W {
        CHS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Channel Status 0 Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlb_acsr](index.html) module"]
pub struct MLB_ACSR_SPEC;
impl crate::RegisterSpec for MLB_ACSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mlb_acsr::R](R) reader structure"]
impl crate::Readable for MLB_ACSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mlb_acsr::W](W) writer structure"]
impl crate::Writable for MLB_ACSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MLB_ACSR[%s]
to value 0"]
impl crate::Resettable for MLB_ACSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
