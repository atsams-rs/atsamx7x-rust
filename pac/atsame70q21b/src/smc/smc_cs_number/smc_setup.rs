#[doc = "Reader of register SMC_SETUP"]
pub type R = crate::R<u32, super::SMC_SETUP>;
#[doc = "Writer for register SMC_SETUP"]
pub type W = crate::W<u32, super::SMC_SETUP>;
#[doc = "Register SMC_SETUP `reset()`'s with value 0"]
impl crate::ResetValue for super::SMC_SETUP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NWE_SETUP`"]
pub type NWE_SETUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NWE_SETUP`"]
pub struct NWE_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> NWE_SETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `NCS_WR_SETUP`"]
pub type NCS_WR_SETUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NCS_WR_SETUP`"]
pub struct NCS_WR_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> NCS_WR_SETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `NRD_SETUP`"]
pub type NRD_SETUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NRD_SETUP`"]
pub struct NRD_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> NRD_SETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `NCS_RD_SETUP`"]
pub type NCS_RD_SETUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NCS_RD_SETUP`"]
pub struct NCS_RD_SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> NCS_RD_SETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
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
}
