#[doc = "Reader of register USBHS_HSTADDR2"]
pub type R = crate::R<u32, super::USBHS_HSTADDR2>;
#[doc = "Writer for register USBHS_HSTADDR2"]
pub type W = crate::W<u32, super::USBHS_HSTADDR2>;
#[doc = "Register USBHS_HSTADDR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::USBHS_HSTADDR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSTADDRP4`"]
pub type HSTADDRP4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTADDRP4`"]
pub struct HSTADDRP4_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `HSTADDRP5`"]
pub type HSTADDRP5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTADDRP5`"]
pub struct HSTADDRP5_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `HSTADDRP6`"]
pub type HSTADDRP6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTADDRP6`"]
pub struct HSTADDRP6_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `HSTADDRP7`"]
pub type HSTADDRP7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTADDRP7`"]
pub struct HSTADDRP7_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp4(&self) -> HSTADDRP4_R {
        HSTADDRP4_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp5(&self) -> HSTADDRP5_R {
        HSTADDRP5_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp6(&self) -> HSTADDRP6_R {
        HSTADDRP6_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp7(&self) -> HSTADDRP7_R {
        HSTADDRP7_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp4(&mut self) -> HSTADDRP4_W {
        HSTADDRP4_W { w: self }
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp5(&mut self) -> HSTADDRP5_W {
        HSTADDRP5_W { w: self }
    }
    #[doc = "Bits 16:22 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp6(&mut self) -> HSTADDRP6_W {
        HSTADDRP6_W { w: self }
    }
    #[doc = "Bits 24:30 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp7(&mut self) -> HSTADDRP7_W {
        HSTADDRP7_W { w: self }
    }
}
