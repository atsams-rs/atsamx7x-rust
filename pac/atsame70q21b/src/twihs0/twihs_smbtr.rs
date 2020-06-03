#[doc = "Reader of register TWIHS_SMBTR"]
pub type R = crate::R<u32, super::TWIHS_SMBTR>;
#[doc = "Writer for register TWIHS_SMBTR"]
pub type W = crate::W<u32, super::TWIHS_SMBTR>;
#[doc = "Register TWIHS_SMBTR `reset()`'s with value 0"]
impl crate::ResetValue for super::TWIHS_SMBTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRESC`"]
pub type PRESC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRESC`"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `TLOWS`"]
pub type TLOWS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TLOWS`"]
pub struct TLOWS_W<'a> {
    w: &'a mut W,
}
impl<'a> TLOWS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `TLOWM`"]
pub type TLOWM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TLOWM`"]
pub struct TLOWM_W<'a> {
    w: &'a mut W,
}
impl<'a> TLOWM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `THMAX`"]
pub type THMAX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `THMAX`"]
pub struct THMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> THMAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - SMBus Clock Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Slave Clock Stretch Maximum Cycles"]
    #[inline(always)]
    pub fn tlows(&self) -> TLOWS_R {
        TLOWS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Master Clock Stretch Maximum Cycles"]
    #[inline(always)]
    pub fn tlowm(&self) -> TLOWM_R {
        TLOWM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clock High Maximum Cycles"]
    #[inline(always)]
    pub fn thmax(&self) -> THMAX_R {
        THMAX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SMBus Clock Prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    #[doc = "Bits 8:15 - Slave Clock Stretch Maximum Cycles"]
    #[inline(always)]
    pub fn tlows(&mut self) -> TLOWS_W {
        TLOWS_W { w: self }
    }
    #[doc = "Bits 16:23 - Master Clock Stretch Maximum Cycles"]
    #[inline(always)]
    pub fn tlowm(&mut self) -> TLOWM_W {
        TLOWM_W { w: self }
    }
    #[doc = "Bits 24:31 - Clock High Maximum Cycles"]
    #[inline(always)]
    pub fn thmax(&mut self) -> THMAX_W {
        THMAX_W { w: self }
    }
}
