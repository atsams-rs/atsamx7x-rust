#[doc = "Register `APLLACR` reader"]
pub struct R(crate::R<APLLACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APLLACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APLLACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APLLACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APLLACR` writer"]
pub struct W(crate::W<APLLACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APLLACR_SPEC>;
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
impl From<crate::W<APLLACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APLLACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCOFLTSEL` reader - DCO Filter Selection"]
pub type DCOFLTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCOFLTSEL` writer - DCO Filter Selection"]
pub type DCOFLTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APLLACR_SPEC, u8, u8, 4, O>;
#[doc = "Field `FLTSEL` reader - PLL Filter Selection"]
pub type FLTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLTSEL` writer - PLL Filter Selection"]
pub type FLTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APLLACR_SPEC, u8, u8, 4, O>;
#[doc = "Field `BIAS` reader - Bias Voltage Selection"]
pub type BIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIAS` writer - Bias Voltage Selection"]
pub type BIAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, APLLACR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - DCO Filter Selection"]
    #[inline(always)]
    pub fn dcofltsel(&self) -> DCOFLTSEL_R {
        DCOFLTSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PLL Filter Selection"]
    #[inline(always)]
    pub fn fltsel(&self) -> FLTSEL_R {
        FLTSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Bias Voltage Selection"]
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DCO Filter Selection"]
    #[inline(always)]
    pub fn dcofltsel(&mut self) -> DCOFLTSEL_W<0> {
        DCOFLTSEL_W::new(self)
    }
    #[doc = "Bits 4:7 - PLL Filter Selection"]
    #[inline(always)]
    pub fn fltsel(&mut self) -> FLTSEL_W<4> {
        FLTSEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Bias Voltage Selection"]
    #[inline(always)]
    pub fn bias(&mut self) -> BIAS_W<8> {
        BIAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Audio PLL Analog Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apllacr](index.html) module"]
pub struct APLLACR_SPEC;
impl crate::RegisterSpec for APLLACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apllacr::R](R) reader structure"]
impl crate::Readable for APLLACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apllacr::W](W) writer structure"]
impl crate::Writable for APLLACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APLLACR to value 0"]
impl crate::Resettable for APLLACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
