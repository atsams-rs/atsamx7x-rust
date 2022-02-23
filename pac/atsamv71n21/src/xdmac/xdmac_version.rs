#[doc = "Register `XDMAC_VERSION` reader"]
pub struct R(crate::R<XDMAC_VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XDMAC_VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XDMAC_VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XDMAC_VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XDMAC_VERSION` writer"]
pub struct W(crate::W<XDMAC_VERSION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XDMAC_VERSION_SPEC>;
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
impl From<crate::W<XDMAC_VERSION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XDMAC_VERSION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VERSION` reader - Version of the Hardware Module"]
pub struct VERSION_R(crate::FieldReader<u16, u16>);
impl VERSION_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VERSION_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VERSION` writer - Version of the Hardware Module"]
pub struct VERSION_W<'a> {
    w: &'a mut W,
}
impl<'a> VERSION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `MFN` reader - Metal Fix Number"]
pub struct MFN_R(crate::FieldReader<u8, u8>);
impl MFN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MFN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MFN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MFN` writer - Metal Fix Number"]
pub struct MFN_W<'a> {
    w: &'a mut W,
}
impl<'a> MFN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Version of the Hardware Module"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:18 - Metal Fix Number"]
    #[inline(always)]
    pub fn mfn(&self) -> MFN_R {
        MFN_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Version of the Hardware Module"]
    #[inline(always)]
    pub fn version(&mut self) -> VERSION_W {
        VERSION_W { w: self }
    }
    #[doc = "Bits 16:18 - Metal Fix Number"]
    #[inline(always)]
    pub fn mfn(&mut self) -> MFN_W {
        MFN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XDMAC Version Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_version](index.html) module"]
pub struct XDMAC_VERSION_SPEC;
impl crate::RegisterSpec for XDMAC_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xdmac_version::R](R) reader structure"]
impl crate::Readable for XDMAC_VERSION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xdmac_version::W](W) writer structure"]
impl crate::Writable for XDMAC_VERSION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XDMAC_VERSION to value 0"]
impl crate::Resettable for XDMAC_VERSION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
