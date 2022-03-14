#[doc = "Register `MCAN_FBTP` reader"]
pub struct R(crate::R<MCAN_FBTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCAN_FBTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCAN_FBTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCAN_FBTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCAN_FBTP` writer"]
pub struct W(crate::W<MCAN_FBTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCAN_FBTP_SPEC>;
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
impl From<crate::W<MCAN_FBTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCAN_FBTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSJW` reader - Fast (Re) Synchronization Jump Width"]
pub struct FSJW_R(crate::FieldReader<u8, u8>);
impl FSJW_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FSJW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSJW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSJW` writer - Fast (Re) Synchronization Jump Width"]
pub struct FSJW_W<'a> {
    w: &'a mut W,
}
impl<'a> FSJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `FTSEG2` reader - Fast Time Segment After Sample Point"]
pub struct FTSEG2_R(crate::FieldReader<u8, u8>);
impl FTSEG2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FTSEG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTSEG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTSEG2` writer - Fast Time Segment After Sample Point"]
pub struct FTSEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> FTSEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `FTSEG1` reader - Fast Time Segment Before Sample Point"]
pub struct FTSEG1_R(crate::FieldReader<u8, u8>);
impl FTSEG1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FTSEG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTSEG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTSEG1` writer - Fast Time Segment Before Sample Point"]
pub struct FTSEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> FTSEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `FBRP` reader - Fast Baud Rate Prescaler"]
pub struct FBRP_R(crate::FieldReader<u8, u8>);
impl FBRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FBRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBRP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBRP` writer - Fast Baud Rate Prescaler"]
pub struct FBRP_W<'a> {
    w: &'a mut W,
}
impl<'a> FBRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Transceiver Delay Compensation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDC_A {
    #[doc = "0: Transceiver Delay Compensation disabled."]
    DISABLED = 0,
    #[doc = "1: Transceiver Delay Compensation enabled."]
    ENABLED = 1,
}
impl From<TDC_A> for bool {
    #[inline(always)]
    fn from(variant: TDC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDC` reader - Transceiver Delay Compensation"]
pub struct TDC_R(crate::FieldReader<bool, TDC_A>);
impl TDC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDC_A {
        match self.bits {
            false => TDC_A::DISABLED,
            true => TDC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TDC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TDC_A::ENABLED
    }
}
impl core::ops::Deref for TDC_R {
    type Target = crate::FieldReader<bool, TDC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDC` writer - Transceiver Delay Compensation"]
pub struct TDC_W<'a> {
    w: &'a mut W,
}
impl<'a> TDC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Transceiver Delay Compensation disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TDC_A::DISABLED)
    }
    #[doc = "Transceiver Delay Compensation enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TDC_A::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `TDCO` reader - Transceiver Delay Compensation Offset"]
pub struct TDCO_R(crate::FieldReader<u8, u8>);
impl TDCO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TDCO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDCO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDCO` writer - Transceiver Delay Compensation Offset"]
pub struct TDCO_W<'a> {
    w: &'a mut W,
}
impl<'a> TDCO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Fast (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn fsjw(&self) -> FSJW_R {
        FSJW_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Fast Time Segment After Sample Point"]
    #[inline(always)]
    pub fn ftseg2(&self) -> FTSEG2_R {
        FTSEG2_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Fast Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn ftseg1(&self) -> FTSEG1_R {
        FTSEG1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Fast Baud Rate Prescaler"]
    #[inline(always)]
    pub fn fbrp(&self) -> FBRP_R {
        FBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Transceiver Delay Compensation"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdco(&self) -> TDCO_R {
        TDCO_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Fast (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn fsjw(&mut self) -> FSJW_W {
        FSJW_W { w: self }
    }
    #[doc = "Bits 4:6 - Fast Time Segment After Sample Point"]
    #[inline(always)]
    pub fn ftseg2(&mut self) -> FTSEG2_W {
        FTSEG2_W { w: self }
    }
    #[doc = "Bits 8:11 - Fast Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn ftseg1(&mut self) -> FTSEG1_W {
        FTSEG1_W { w: self }
    }
    #[doc = "Bits 16:20 - Fast Baud Rate Prescaler"]
    #[inline(always)]
    pub fn fbrp(&mut self) -> FBRP_W {
        FBRP_W { w: self }
    }
    #[doc = "Bit 23 - Transceiver Delay Compensation"]
    #[inline(always)]
    pub fn tdc(&mut self) -> TDC_W {
        TDC_W { w: self }
    }
    #[doc = "Bits 24:28 - Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdco(&mut self) -> TDCO_W {
        TDCO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fast Bit Timing and Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_fbtp](index.html) module"]
pub struct MCAN_FBTP_SPEC;
impl crate::RegisterSpec for MCAN_FBTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcan_fbtp::R](R) reader structure"]
impl crate::Readable for MCAN_FBTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcan_fbtp::W](W) writer structure"]
impl crate::Writable for MCAN_FBTP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCAN_FBTP to value 0"]
impl crate::Resettable for MCAN_FBTP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
