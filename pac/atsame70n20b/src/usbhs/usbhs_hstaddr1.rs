#[doc = "Reader of register USBHS_HSTADDR1"]
pub type R = crate::R<u32, super::USBHS_HSTADDR1>;
#[doc = "Writer for register USBHS_HSTADDR1"]
pub type W = crate::W<u32, super::USBHS_HSTADDR1>;
#[doc = "Register USBHS_HSTADDR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::USBHS_HSTADDR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HSTADDRP0`"]
pub type HSTADDRP0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTADDRP0`"]
pub struct HSTADDRP0_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `HSTADDRP1`"]
pub type HSTADDRP1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTADDRP1`"]
pub struct HSTADDRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `HSTADDRP2`"]
pub type HSTADDRP2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTADDRP2`"]
pub struct HSTADDRP2_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `HSTADDRP3`"]
pub type HSTADDRP3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSTADDRP3`"]
pub struct HSTADDRP3_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTADDRP3_W<'a> {
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
    pub fn hstaddrp0(&self) -> HSTADDRP0_R {
        HSTADDRP0_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp1(&self) -> HSTADDRP1_R {
        HSTADDRP1_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp2(&self) -> HSTADDRP2_R {
        HSTADDRP2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp3(&self) -> HSTADDRP3_R {
        HSTADDRP3_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp0(&mut self) -> HSTADDRP0_W {
        HSTADDRP0_W { w: self }
    }
    #[doc = "Bits 8:14 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp1(&mut self) -> HSTADDRP1_W {
        HSTADDRP1_W { w: self }
    }
    #[doc = "Bits 16:22 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp2(&mut self) -> HSTADDRP2_W {
        HSTADDRP2_W { w: self }
    }
    #[doc = "Bits 24:30 - USB Host Address"]
    #[inline(always)]
    pub fn hstaddrp3(&mut self) -> HSTADDRP3_W {
        HSTADDRP3_W { w: self }
    }
}
