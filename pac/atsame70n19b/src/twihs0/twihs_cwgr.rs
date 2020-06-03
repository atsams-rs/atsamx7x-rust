#[doc = "Reader of register TWIHS_CWGR"]
pub type R = crate::R<u32, super::TWIHS_CWGR>;
#[doc = "Writer for register TWIHS_CWGR"]
pub type W = crate::W<u32, super::TWIHS_CWGR>;
#[doc = "Register TWIHS_CWGR `reset()`'s with value 0"]
impl crate::ResetValue for super::TWIHS_CWGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLDIV`"]
pub type CLDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLDIV`"]
pub struct CLDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `CHDIV`"]
pub type CHDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CHDIV`"]
pub struct CHDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CHDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CKDIV`"]
pub type CKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKDIV`"]
pub struct CKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOLD`"]
pub type HOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOLD`"]
pub struct HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> HOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Clock Low Divider"]
    #[inline(always)]
    pub fn cldiv(&self) -> CLDIV_R {
        CLDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock High Divider"]
    #[inline(always)]
    pub fn chdiv(&self) -> CHDIV_R {
        CHDIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Clock Divider"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CKDIV_R {
        CKDIV_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 24:29 - TWD Hold Time Versus TWCK Falling"]
    #[inline(always)]
    pub fn hold(&self) -> HOLD_R {
        HOLD_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Low Divider"]
    #[inline(always)]
    pub fn cldiv(&mut self) -> CLDIV_W {
        CLDIV_W { w: self }
    }
    #[doc = "Bits 8:15 - Clock High Divider"]
    #[inline(always)]
    pub fn chdiv(&mut self) -> CHDIV_W {
        CHDIV_W { w: self }
    }
    #[doc = "Bits 16:18 - Clock Divider"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CKDIV_W {
        CKDIV_W { w: self }
    }
    #[doc = "Bits 24:29 - TWD Hold Time Versus TWCK Falling"]
    #[inline(always)]
    pub fn hold(&mut self) -> HOLD_W {
        HOLD_W { w: self }
    }
}
