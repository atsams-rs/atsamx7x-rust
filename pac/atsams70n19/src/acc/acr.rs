#[doc = "Register `ACR` reader"]
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR` writer"]
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Current Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISEL_A {
    #[doc = "0: Low-power option."]
    LOPW = 0,
    #[doc = "1: High-speed option."]
    HISP = 1,
}
impl From<ISEL_A> for bool {
    #[inline(always)]
    fn from(variant: ISEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISEL` reader - Current Selection"]
pub type ISEL_R = crate::BitReader<ISEL_A>;
impl ISEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISEL_A {
        match self.bits {
            false => ISEL_A::LOPW,
            true => ISEL_A::HISP,
        }
    }
    #[doc = "Checks if the value of the field is `LOPW`"]
    #[inline(always)]
    pub fn is_lopw(&self) -> bool {
        *self == ISEL_A::LOPW
    }
    #[doc = "Checks if the value of the field is `HISP`"]
    #[inline(always)]
    pub fn is_hisp(&self) -> bool {
        *self == ISEL_A::HISP
    }
}
#[doc = "Field `ISEL` writer - Current Selection"]
pub type ISEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, ISEL_A, O>;
impl<'a, const O: u8> ISEL_W<'a, O> {
    #[doc = "Low-power option."]
    #[inline(always)]
    pub fn lopw(self) -> &'a mut W {
        self.variant(ISEL_A::LOPW)
    }
    #[doc = "High-speed option."]
    #[inline(always)]
    pub fn hisp(self) -> &'a mut W {
        self.variant(ISEL_A::HISP)
    }
}
#[doc = "Field `HYST` reader - Hysteresis Selection"]
pub type HYST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HYST` writer - Hysteresis Selection"]
pub type HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Current Selection"]
    #[inline(always)]
    pub fn isel(&self) -> ISEL_R {
        ISEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Hysteresis Selection"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Current Selection"]
    #[inline(always)]
    pub fn isel(&mut self) -> ISEL_W<0> {
        ISEL_W::new(self)
    }
    #[doc = "Bits 1:2 - Hysteresis Selection"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W<1> {
        HYST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](index.html) module"]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr::R](R) reader structure"]
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr::W](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
