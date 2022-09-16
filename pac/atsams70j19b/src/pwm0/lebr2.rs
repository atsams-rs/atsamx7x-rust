#[doc = "Register `LEBR2` reader"]
pub struct R(crate::R<LEBR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LEBR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LEBR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LEBR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LEBR2` writer"]
pub struct W(crate::W<LEBR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LEBR2_SPEC>;
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
impl From<crate::W<LEBR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LEBR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEBDELAY` reader - Leading-Edge Blanking Delay for TRGINx"]
pub type LEBDELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEBDELAY` writer - Leading-Edge Blanking Delay for TRGINx"]
pub type LEBDELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LEBR2_SPEC, u8, u8, 7, O>;
#[doc = "Field `PWMLFEN` reader - PWML Falling Edge Enable"]
pub type PWMLFEN_R = crate::BitReader<bool>;
#[doc = "Field `PWMLFEN` writer - PWML Falling Edge Enable"]
pub type PWMLFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LEBR2_SPEC, bool, O>;
#[doc = "Field `PWMLREN` reader - PWML Rising Edge Enable"]
pub type PWMLREN_R = crate::BitReader<bool>;
#[doc = "Field `PWMLREN` writer - PWML Rising Edge Enable"]
pub type PWMLREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LEBR2_SPEC, bool, O>;
#[doc = "Field `PWMHFEN` reader - PWMH Falling Edge Enable"]
pub type PWMHFEN_R = crate::BitReader<bool>;
#[doc = "Field `PWMHFEN` writer - PWMH Falling Edge Enable"]
pub type PWMHFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LEBR2_SPEC, bool, O>;
#[doc = "Field `PWMHREN` reader - PWMH Rising Edge Enable"]
pub type PWMHREN_R = crate::BitReader<bool>;
#[doc = "Field `PWMHREN` writer - PWMH Rising Edge Enable"]
pub type PWMHREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LEBR2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - Leading-Edge Blanking Delay for TRGINx"]
    #[inline(always)]
    pub fn lebdelay(&self) -> LEBDELAY_R {
        LEBDELAY_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - PWML Falling Edge Enable"]
    #[inline(always)]
    pub fn pwmlfen(&self) -> PWMLFEN_R {
        PWMLFEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PWML Rising Edge Enable"]
    #[inline(always)]
    pub fn pwmlren(&self) -> PWMLREN_R {
        PWMLREN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PWMH Falling Edge Enable"]
    #[inline(always)]
    pub fn pwmhfen(&self) -> PWMHFEN_R {
        PWMHFEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PWMH Rising Edge Enable"]
    #[inline(always)]
    pub fn pwmhren(&self) -> PWMHREN_R {
        PWMHREN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Leading-Edge Blanking Delay for TRGINx"]
    #[inline(always)]
    pub fn lebdelay(&mut self) -> LEBDELAY_W<0> {
        LEBDELAY_W::new(self)
    }
    #[doc = "Bit 16 - PWML Falling Edge Enable"]
    #[inline(always)]
    pub fn pwmlfen(&mut self) -> PWMLFEN_W<16> {
        PWMLFEN_W::new(self)
    }
    #[doc = "Bit 17 - PWML Rising Edge Enable"]
    #[inline(always)]
    pub fn pwmlren(&mut self) -> PWMLREN_W<17> {
        PWMLREN_W::new(self)
    }
    #[doc = "Bit 18 - PWMH Falling Edge Enable"]
    #[inline(always)]
    pub fn pwmhfen(&mut self) -> PWMHFEN_W<18> {
        PWMHFEN_W::new(self)
    }
    #[doc = "Bit 19 - PWMH Rising Edge Enable"]
    #[inline(always)]
    pub fn pwmhren(&mut self) -> PWMHREN_W<19> {
        PWMHREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Leading-Edge Blanking Register (trg_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lebr2](index.html) module"]
pub struct LEBR2_SPEC;
impl crate::RegisterSpec for LEBR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lebr2::R](R) reader structure"]
impl crate::Readable for LEBR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lebr2::W](W) writer structure"]
impl crate::Writable for LEBR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LEBR2 to value 0"]
impl crate::Resettable for LEBR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
