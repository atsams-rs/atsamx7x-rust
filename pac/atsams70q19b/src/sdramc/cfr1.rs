#[doc = "Register `CFR1` reader"]
pub struct R(crate::R<CFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFR1` writer"]
pub struct W(crate::W<CFR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFR1_SPEC>;
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
impl From<crate::W<CFR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMRD` reader - Load Mode Register Command to Active or Refresh Command"]
pub type TMRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMRD` writer - Load Mode Register Command to Active or Refresh Command"]
pub type TMRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFR1_SPEC, u8, u8, 4, O>;
#[doc = "Support Unaligned Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNAL_A {
    #[doc = "0: Unaligned access is not supported."]
    UNSUPPORTED = 0,
    #[doc = "1: Unaligned access is supported."]
    SUPPORTED = 1,
}
impl From<UNAL_A> for bool {
    #[inline(always)]
    fn from(variant: UNAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNAL` reader - Support Unaligned Access"]
pub type UNAL_R = crate::BitReader<UNAL_A>;
impl UNAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNAL_A {
        match self.bits {
            false => UNAL_A::UNSUPPORTED,
            true => UNAL_A::SUPPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNSUPPORTED`"]
    #[inline(always)]
    pub fn is_unsupported(&self) -> bool {
        *self == UNAL_A::UNSUPPORTED
    }
    #[doc = "Checks if the value of the field is `SUPPORTED`"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == UNAL_A::SUPPORTED
    }
}
#[doc = "Field `UNAL` writer - Support Unaligned Access"]
pub type UNAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFR1_SPEC, UNAL_A, O>;
impl<'a, const O: u8> UNAL_W<'a, O> {
    #[doc = "Unaligned access is not supported."]
    #[inline(always)]
    pub fn unsupported(self) -> &'a mut W {
        self.variant(UNAL_A::UNSUPPORTED)
    }
    #[doc = "Unaligned access is supported."]
    #[inline(always)]
    pub fn supported(self) -> &'a mut W {
        self.variant(UNAL_A::SUPPORTED)
    }
}
impl R {
    #[doc = "Bits 0:3 - Load Mode Register Command to Active or Refresh Command"]
    #[inline(always)]
    pub fn tmrd(&self) -> TMRD_R {
        TMRD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Support Unaligned Access"]
    #[inline(always)]
    pub fn unal(&self) -> UNAL_R {
        UNAL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Load Mode Register Command to Active or Refresh Command"]
    #[inline(always)]
    pub fn tmrd(&mut self) -> TMRD_W<0> {
        TMRD_W::new(self)
    }
    #[doc = "Bit 8 - Support Unaligned Access"]
    #[inline(always)]
    pub fn unal(&mut self) -> UNAL_W<8> {
        UNAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAMC Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfr1](index.html) module"]
pub struct CFR1_SPEC;
impl crate::RegisterSpec for CFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfr1::R](R) reader structure"]
impl crate::Readable for CFR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfr1::W](W) writer structure"]
impl crate::Writable for CFR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFR1 to value 0"]
impl crate::Resettable for CFR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
