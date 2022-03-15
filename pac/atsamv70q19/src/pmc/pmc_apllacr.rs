#[doc = "Register `PMC_APLLACR` reader"]
pub struct R(crate::R<PMC_APLLACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_APLLACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_APLLACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_APLLACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC_APLLACR` writer"]
pub struct W(crate::W<PMC_APLLACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_APLLACR_SPEC>;
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
impl From<crate::W<PMC_APLLACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_APLLACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCOFLTSEL` reader - DCO Filter Selection"]
pub struct DCOFLTSEL_R(crate::FieldReader<u8, u8>);
impl DCOFLTSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DCOFLTSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCOFLTSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCOFLTSEL` writer - DCO Filter Selection"]
pub struct DCOFLTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DCOFLTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `FLTSEL` reader - PLL Filter Selection"]
pub struct FLTSEL_R(crate::FieldReader<u8, u8>);
impl FLTSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLTSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLTSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLTSEL` writer - PLL Filter Selection"]
pub struct FLTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLTSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `BIAS` reader - Bias Voltage Selection"]
pub struct BIAS_R(crate::FieldReader<u8, u8>);
impl BIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIAS` writer - Bias Voltage Selection"]
pub struct BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
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
        BIAS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DCO Filter Selection"]
    #[inline(always)]
    pub fn dcofltsel(&mut self) -> DCOFLTSEL_W {
        DCOFLTSEL_W { w: self }
    }
    #[doc = "Bits 4:7 - PLL Filter Selection"]
    #[inline(always)]
    pub fn fltsel(&mut self) -> FLTSEL_W {
        FLTSEL_W { w: self }
    }
    #[doc = "Bits 8:9 - Bias Voltage Selection"]
    #[inline(always)]
    pub fn bias(&mut self) -> BIAS_W {
        BIAS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Audio PLL Analog Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_apllacr](index.html) module"]
pub struct PMC_APLLACR_SPEC;
impl crate::RegisterSpec for PMC_APLLACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_apllacr::R](R) reader structure"]
impl crate::Readable for PMC_APLLACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc_apllacr::W](W) writer structure"]
impl crate::Writable for PMC_APLLACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC_APLLACR to value 0"]
impl crate::Resettable for PMC_APLLACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
