#[doc = "Reader of register SMC_PULSE"]
pub type R = crate::R<u32, super::SMC_PULSE>;
#[doc = "Writer for register SMC_PULSE"]
pub type W = crate::W<u32, super::SMC_PULSE>;
#[doc = "Register SMC_PULSE `reset()`'s with value 0"]
impl crate::ResetValue for super::SMC_PULSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NWE_PULSE`"]
pub type NWE_PULSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NWE_PULSE`"]
pub struct NWE_PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> NWE_PULSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `NCS_WR_PULSE`"]
pub type NCS_WR_PULSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NCS_WR_PULSE`"]
pub struct NCS_WR_PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> NCS_WR_PULSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `NRD_PULSE`"]
pub type NRD_PULSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NRD_PULSE`"]
pub struct NRD_PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> NRD_PULSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `NCS_RD_PULSE`"]
pub type NCS_RD_PULSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NCS_RD_PULSE`"]
pub struct NCS_RD_PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> NCS_RD_PULSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
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
}
