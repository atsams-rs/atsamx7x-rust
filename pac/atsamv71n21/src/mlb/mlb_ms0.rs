#[doc = "Register `MLB_MS0` reader"]
pub struct R(crate::R<MLB_MS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MLB_MS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MLB_MS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MLB_MS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MLB_MS0` writer"]
pub struct W(crate::W<MLB_MS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MLB_MS0_SPEC>;
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
impl From<crate::W<MLB_MS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MLB_MS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCS` reader - MediaLB Channel Status \\[31:0\\]
(cleared by writing a 0)"]
pub struct MCS_R(crate::FieldReader<u32, u32>);
impl MCS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCS` writer - MediaLB Channel Status \\[31:0\\]
(cleared by writing a 0)"]
pub struct MCS_W<'a> {
    w: &'a mut W,
}
impl<'a> MCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - MediaLB Channel Status \\[31:0\\]
(cleared by writing a 0)"]
    #[inline(always)]
    pub fn mcs(&self) -> MCS_R {
        MCS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MediaLB Channel Status \\[31:0\\]
(cleared by writing a 0)"]
    #[inline(always)]
    pub fn mcs(&mut self) -> MCS_W {
        MCS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MediaLB Channel Status 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlb_ms0](index.html) module"]
pub struct MLB_MS0_SPEC;
impl crate::RegisterSpec for MLB_MS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mlb_ms0::R](R) reader structure"]
impl crate::Readable for MLB_MS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mlb_ms0::W](W) writer structure"]
impl crate::Writable for MLB_MS0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MLB_MS0 to value 0"]
impl crate::Resettable for MLB_MS0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
