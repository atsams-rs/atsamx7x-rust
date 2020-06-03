#[doc = "Reader of register USBHS_HSTADDR3"]
pub type R = crate::R<u32, super::USBHS_HSTADDR3>;
#[doc = "Writer for register USBHS_HSTADDR3"]
pub type W = crate::W<u32, super::USBHS_HSTADDR3>;
#[doc = "Register USBHS_HSTADDR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::USBHS_HSTADDR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSTADDRP8`"]
pub type HSTADDRP8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTADDRP8`"]
pub struct HSTADDRP8_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `HSTADDRP9`"]
pub type HSTADDRP9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTADDRP9`"]
pub struct HSTADDRP9_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp8(&self) -> HSTADDRP8_R {
        HSTADDRP8_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp9(&self) -> HSTADDRP9_R {
        HSTADDRP9_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp8(&mut self) -> HSTADDRP8_W {
        HSTADDRP8_W { w: self }
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp9(&mut self) -> HSTADDRP9_W {
        HSTADDRP9_W { w: self }
    }
}
