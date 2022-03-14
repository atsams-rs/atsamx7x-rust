#[doc = "Register `SDRAMC_OCMS` reader"]
pub struct R(crate::R<SDRAMC_OCMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDRAMC_OCMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDRAMC_OCMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDRAMC_OCMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDRAMC_OCMS` writer"]
pub struct W(crate::W<SDRAMC_OCMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDRAMC_OCMS_SPEC>;
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
impl From<crate::W<SDRAMC_OCMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDRAMC_OCMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDR_SE` reader - SDRAM Memory Controller Scrambling Enable"]
pub struct SDR_SE_R(crate::FieldReader<bool, bool>);
impl SDR_SE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SDR_SE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDR_SE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDR_SE` writer - SDRAM Memory Controller Scrambling Enable"]
pub struct SDR_SE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDR_SE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SDRAM Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn sdr_se(&self) -> SDR_SE_R {
        SDR_SE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDRAM Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn sdr_se(&mut self) -> SDR_SE_W {
        SDR_SE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAMC OCMS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramc_ocms](index.html) module"]
pub struct SDRAMC_OCMS_SPEC;
impl crate::RegisterSpec for SDRAMC_OCMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdramc_ocms::R](R) reader structure"]
impl crate::Readable for SDRAMC_OCMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdramc_ocms::W](W) writer structure"]
impl crate::Writable for SDRAMC_OCMS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDRAMC_OCMS to value 0"]
impl crate::Resettable for SDRAMC_OCMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
