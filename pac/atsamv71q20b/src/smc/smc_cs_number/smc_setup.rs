#[doc = "Register `SMC_SETUP` reader"]
pub struct R(crate::R<SMC_SETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMC_SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMC_SETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMC_SETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMC_SETUP` writer"]
pub struct W(crate::W<SMC_SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMC_SETUP_SPEC>;
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
impl From<crate::W<SMC_SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMC_SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NWE_SETUP` reader - NWE Setup Length"]
pub struct NWE_SETUP_R(crate::FieldReader<u8, u8>);
impl NWE_SETUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NWE_SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NWE_SETUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NWE_SETUP` writer - NWE Setup Length"]
pub struct NWE_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> NWE_SETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `NCS_WR_SETUP` reader - NCS Setup Length in WRITE Access"]
pub struct NCS_WR_SETUP_R(crate::FieldReader<u8, u8>);
impl NCS_WR_SETUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NCS_WR_SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCS_WR_SETUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCS_WR_SETUP` writer - NCS Setup Length in WRITE Access"]
pub struct NCS_WR_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> NCS_WR_SETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `NRD_SETUP` reader - NRD Setup Length"]
pub struct NRD_SETUP_R(crate::FieldReader<u8, u8>);
impl NRD_SETUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NRD_SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRD_SETUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRD_SETUP` writer - NRD Setup Length"]
pub struct NRD_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> NRD_SETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `NCS_RD_SETUP` reader - NCS Setup Length in READ Access"]
pub struct NCS_RD_SETUP_R(crate::FieldReader<u8, u8>);
impl NCS_RD_SETUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NCS_RD_SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NCS_RD_SETUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NCS_RD_SETUP` writer - NCS Setup Length in READ Access"]
pub struct NCS_RD_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> NCS_RD_SETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - NWE Setup Length"]
    #[inline(always)]
    pub fn nwe_setup(&self) -> NWE_SETUP_R {
        NWE_SETUP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - NCS Setup Length in WRITE Access"]
    #[inline(always)]
    pub fn ncs_wr_setup(&self) -> NCS_WR_SETUP_R {
        NCS_WR_SETUP_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - NRD Setup Length"]
    #[inline(always)]
    pub fn nrd_setup(&self) -> NRD_SETUP_R {
        NRD_SETUP_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - NCS Setup Length in READ Access"]
    #[inline(always)]
    pub fn ncs_rd_setup(&self) -> NCS_RD_SETUP_R {
        NCS_RD_SETUP_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - NWE Setup Length"]
    #[inline(always)]
    pub fn nwe_setup(&mut self) -> NWE_SETUP_W {
        NWE_SETUP_W { w: self }
    }
    #[doc = "Bits 8:13 - NCS Setup Length in WRITE Access"]
    #[inline(always)]
    pub fn ncs_wr_setup(&mut self) -> NCS_WR_SETUP_W {
        NCS_WR_SETUP_W { w: self }
    }
    #[doc = "Bits 16:21 - NRD Setup Length"]
    #[inline(always)]
    pub fn nrd_setup(&mut self) -> NRD_SETUP_W {
        NRD_SETUP_W { w: self }
    }
    #[doc = "Bits 24:29 - NCS Setup Length in READ Access"]
    #[inline(always)]
    pub fn ncs_rd_setup(&mut self) -> NCS_RD_SETUP_W {
        NCS_RD_SETUP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Setup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smc_setup](index.html) module"]
pub struct SMC_SETUP_SPEC;
impl crate::RegisterSpec for SMC_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smc_setup::R](R) reader structure"]
impl crate::Readable for SMC_SETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smc_setup::W](W) writer structure"]
impl crate::Writable for SMC_SETUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMC_SETUP to value 0"]
impl crate::Resettable for SMC_SETUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
