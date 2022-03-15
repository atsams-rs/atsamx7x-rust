#[doc = "Register `MCAN_BTP` reader"]
pub struct R(crate::R<MCAN_BTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_BTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_BTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_BTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCAN_BTP` writer"]
pub struct W(crate::W<MCAN_BTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCAN_BTP_SPEC>;
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
impl From<crate::W<MCAN_BTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCAN_BTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SJW` reader - (Re) Synchronization Jump Width"]
pub struct SJW_R(crate::FieldReader<u8, u8>);
impl SJW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SJW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SJW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SJW` writer - (Re) Synchronization Jump Width"]
pub struct SJW_W<'a> {
    w: &'a mut W,
}
impl<'a> SJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `TSEG2` reader - Time Segment After Sample Point"]
pub struct TSEG2_R(crate::FieldReader<u8, u8>);
impl TSEG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSEG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEG2` writer - Time Segment After Sample Point"]
pub struct TSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `TSEG1` reader - Time Segment Before Sample Point"]
pub struct TSEG1_R(crate::FieldReader<u8, u8>);
impl TSEG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TSEG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSEG1` writer - Time Segment Before Sample Point"]
pub struct TSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `BRP` reader - Baud Rate Prescaler"]
pub struct BRP_R(crate::FieldReader<u16, u16>);
impl BRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        BRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRP` writer - Baud Rate Prescaler"]
pub struct BRP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Time Segment After Sample Point"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn tseg1(&self) -> TSEG1_R {
        TSEG1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:25 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W {
        SJW_W { w: self }
    }
    #[doc = "Bits 4:7 - Time Segment After Sample Point"]
    #[inline(always)]
    pub fn tseg2(&mut self) -> TSEG2_W {
        TSEG2_W { w: self }
    }
    #[doc = "Bits 8:13 - Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn tseg1(&mut self) -> TSEG1_W {
        TSEG1_W { w: self }
    }
    #[doc = "Bits 16:25 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W {
        BRP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit Timing and Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_btp](index.html) module"]
pub struct MCAN_BTP_SPEC;
impl crate::RegisterSpec for MCAN_BTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_btp::R](R) reader structure"]
impl crate::Readable for MCAN_BTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcan_btp::W](W) writer structure"]
impl crate::Writable for MCAN_BTP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCAN_BTP to value 0"]
impl crate::Resettable for MCAN_BTP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
