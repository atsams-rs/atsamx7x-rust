#[doc = "Register `FBTP` reader"]
pub struct R(crate::R<FBTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBTP` writer"]
pub struct W(crate::W<FBTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBTP_SPEC>;
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
impl From<crate::W<FBTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSJW` reader - Fast (Re) Synchronization Jump Width"]
pub type FSJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSJW` writer - Fast (Re) Synchronization Jump Width"]
pub type FSJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBTP_SPEC, u8, u8, 2, O>;
#[doc = "Field `FTSEG2` reader - Fast Time Segment After Sample Point"]
pub type FTSEG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FTSEG2` writer - Fast Time Segment After Sample Point"]
pub type FTSEG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBTP_SPEC, u8, u8, 3, O>;
#[doc = "Field `FTSEG1` reader - Fast Time Segment Before Sample Point"]
pub type FTSEG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FTSEG1` writer - Fast Time Segment Before Sample Point"]
pub type FTSEG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBTP_SPEC, u8, u8, 4, O>;
#[doc = "Field `FBRP` reader - Fast Baud Rate Prescaler"]
pub type FBRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FBRP` writer - Fast Baud Rate Prescaler"]
pub type FBRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBTP_SPEC, u8, u8, 5, O>;
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
pub type TDC_R = crate::BitReader<TDC_A>;
impl TDC_R {
    #[doc = "Get enumerated values variant"]
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
        *self == TDC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TDC_A::ENABLED
    }
}
#[doc = "Field `TDC` writer - Transceiver Delay Compensation"]
pub type TDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBTP_SPEC, TDC_A, O>;
impl<'a, const O: u8> TDC_W<'a, O> {
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
}
#[doc = "Field `TDCO` reader - Transceiver Delay Compensation Offset"]
pub type TDCO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDCO` writer - Transceiver Delay Compensation Offset"]
pub type TDCO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBTP_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:1 - Fast (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn fsjw(&self) -> FSJW_R {
        FSJW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Fast Time Segment After Sample Point"]
    #[inline(always)]
    pub fn ftseg2(&self) -> FTSEG2_R {
        FTSEG2_R::new(((self.bits >> 4) & 7) as u8)
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
        TDC_R::new(((self.bits >> 23) & 1) != 0)
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
    pub fn fsjw(&mut self) -> FSJW_W<0> {
        FSJW_W::new(self)
    }
    #[doc = "Bits 4:6 - Fast Time Segment After Sample Point"]
    #[inline(always)]
    pub fn ftseg2(&mut self) -> FTSEG2_W<4> {
        FTSEG2_W::new(self)
    }
    #[doc = "Bits 8:11 - Fast Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn ftseg1(&mut self) -> FTSEG1_W<8> {
        FTSEG1_W::new(self)
    }
    #[doc = "Bits 16:20 - Fast Baud Rate Prescaler"]
    #[inline(always)]
    pub fn fbrp(&mut self) -> FBRP_W<16> {
        FBRP_W::new(self)
    }
    #[doc = "Bit 23 - Transceiver Delay Compensation"]
    #[inline(always)]
    pub fn tdc(&mut self) -> TDC_W<23> {
        TDC_W::new(self)
    }
    #[doc = "Bits 24:28 - Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdco(&mut self) -> TDCO_W<24> {
        TDCO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fast Bit Timing and Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbtp](index.html) module"]
pub struct FBTP_SPEC;
impl crate::RegisterSpec for FBTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbtp::R](R) reader structure"]
impl crate::Readable for FBTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbtp::W](W) writer structure"]
impl crate::Writable for FBTP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FBTP to value 0"]
impl crate::Resettable for FBTP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
