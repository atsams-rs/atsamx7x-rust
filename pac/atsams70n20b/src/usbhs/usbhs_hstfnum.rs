#[doc = "Reader of register USBHS_HSTFNUM"]
pub type R = crate::R<u32, super::USBHS_HSTFNUM>;
#[doc = "Writer for register USBHS_HSTFNUM"]
pub type W = crate::W<u32, super::USBHS_HSTFNUM>;
#[doc = "Register USBHS_HSTFNUM `reset()`'s with value 0"]
impl crate::ResetValue for super::USBHS_HSTFNUM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MFNUM`"]
pub type MFNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MFNUM`"]
pub struct MFNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> MFNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `FNUM`"]
pub type FNUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FNUM`"]
pub struct FNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> FNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 3)) | (((value as u32) & 0x07ff) << 3);
        self.w
    }
}
#[doc = "Reader of field `FLENHIGH`"]
pub type FLENHIGH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLENHIGH`"]
pub struct FLENHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLENHIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&self) -> MFNUM_R {
        MFNUM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new(((self.bits >> 3) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:23 - Frame Length"]
    #[inline(always)]
    pub fn flenhigh(&self) -> FLENHIGH_R {
        FLENHIGH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Micro Frame Number"]
    #[inline(always)]
    pub fn mfnum(&mut self) -> MFNUM_W {
        MFNUM_W { w: self }
    }
    #[doc = "Bits 3:13 - Frame Number"]
    #[inline(always)]
    pub fn fnum(&mut self) -> FNUM_W {
        FNUM_W { w: self }
    }
    #[doc = "Bits 16:23 - Frame Length"]
    #[inline(always)]
    pub fn flenhigh(&mut self) -> FLENHIGH_W {
        FLENHIGH_W { w: self }
    }
}
