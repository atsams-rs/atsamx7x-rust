#[doc = "Register `SMC_PULSE` reader"]
pub struct R(crate::R<SMC_PULSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMC_PULSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMC_PULSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMC_PULSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMC_PULSE` writer"]
pub struct W(crate::W<SMC_PULSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMC_PULSE_SPEC>;
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
impl From<crate::W<SMC_PULSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMC_PULSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NWE_PULSE` reader - NWE Pulse Length"]
pub struct NWE_PULSE_R(crate::FieldReader<u8, u8>);
impl NWE_PULSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NWE_PULSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NWE_PULSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NWE_PULSE` writer - NWE Pulse Length"]
pub struct NWE_PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> NWE_PULSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `NCS_WR_PULSE` reader - NCS Pulse Length in WRITE Access"]
pub struct NCS_WR_PULSE_R(crate::FieldReader<u8, u8>);
impl NCS_WR_PULSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NCS_WR_PULSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCS_WR_PULSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCS_WR_PULSE` writer - NCS Pulse Length in WRITE Access"]
pub struct NCS_WR_PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> NCS_WR_PULSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `NRD_PULSE` reader - NRD Pulse Length"]
pub struct NRD_PULSE_R(crate::FieldReader<u8, u8>);
impl NRD_PULSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NRD_PULSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRD_PULSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRD_PULSE` writer - NRD Pulse Length"]
pub struct NRD_PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> NRD_PULSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `NCS_RD_PULSE` reader - NCS Pulse Length in READ Access"]
pub struct NCS_RD_PULSE_R(crate::FieldReader<u8, u8>);
impl NCS_RD_PULSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NCS_RD_PULSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCS_RD_PULSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCS_RD_PULSE` writer - NCS Pulse Length in READ Access"]
pub struct NCS_RD_PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> NCS_RD_PULSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | ((value as u32 & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - NWE Pulse Length"]
    #[inline(always)]
    pub fn nwe_pulse(&self) -> NWE_PULSE_R {
        NWE_PULSE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - NCS Pulse Length in WRITE Access"]
    #[inline(always)]
    pub fn ncs_wr_pulse(&self) -> NCS_WR_PULSE_R {
        NCS_WR_PULSE_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - NRD Pulse Length"]
    #[inline(always)]
    pub fn nrd_pulse(&self) -> NRD_PULSE_R {
        NRD_PULSE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - NCS Pulse Length in READ Access"]
    #[inline(always)]
    pub fn ncs_rd_pulse(&self) -> NCS_RD_PULSE_R {
        NCS_RD_PULSE_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - NWE Pulse Length"]
    #[inline(always)]
    pub fn nwe_pulse(&mut self) -> NWE_PULSE_W {
        NWE_PULSE_W { w: self }
    }
    #[doc = "Bits 8:14 - NCS Pulse Length in WRITE Access"]
    #[inline(always)]
    pub fn ncs_wr_pulse(&mut self) -> NCS_WR_PULSE_W {
        NCS_WR_PULSE_W { w: self }
    }
    #[doc = "Bits 16:22 - NRD Pulse Length"]
    #[inline(always)]
    pub fn nrd_pulse(&mut self) -> NRD_PULSE_W {
        NRD_PULSE_W { w: self }
    }
    #[doc = "Bits 24:30 - NCS Pulse Length in READ Access"]
    #[inline(always)]
    pub fn ncs_rd_pulse(&mut self) -> NCS_RD_PULSE_W {
        NCS_RD_PULSE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Pulse Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smc_pulse](index.html) module"]
pub struct SMC_PULSE_SPEC;
impl crate::RegisterSpec for SMC_PULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smc_pulse::R](R) reader structure"]
impl crate::Readable for SMC_PULSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smc_pulse::W](W) writer structure"]
impl crate::Writable for SMC_PULSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMC_PULSE to value 0"]
impl crate::Resettable for SMC_PULSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
