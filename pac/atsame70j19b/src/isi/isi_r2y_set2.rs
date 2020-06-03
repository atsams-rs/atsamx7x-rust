#[doc = "Reader of register ISI_R2Y_SET2"]
pub type R = crate::R<u32, super::ISI_R2Y_SET2>;
#[doc = "Writer for register ISI_R2Y_SET2"]
pub type W = crate::W<u32, super::ISI_R2Y_SET2>;
#[doc = "Register ISI_R2Y_SET2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ISI_R2Y_SET2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `C6`"]
pub type C6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `C6`"]
pub struct C6_W<'a> {
    w: &'a mut W,
}
impl<'a> C6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `C7`"]
pub type C7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `C7`"]
pub struct C7_W<'a> {
    w: &'a mut W,
}
impl<'a> C7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `C8`"]
pub type C8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `C8`"]
pub struct C8_W<'a> {
    w: &'a mut W,
}
impl<'a> C8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `Boff`"]
pub type BOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Boff`"]
pub struct BOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> BOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C6"]
    #[inline(always)]
    pub fn c6(&self) -> C6_R {
        C6_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C7"]
    #[inline(always)]
    pub fn c7(&self) -> C7_R {
        C7_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C8"]
    #[inline(always)]
    pub fn c8(&self) -> C8_R {
        C8_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Color Space Conversion Blue Component Offset"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C6"]
    #[inline(always)]
    pub fn c6(&mut self) -> C6_W {
        C6_W { w: self }
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C7"]
    #[inline(always)]
    pub fn c7(&mut self) -> C7_W {
        C7_W { w: self }
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C8"]
    #[inline(always)]
    pub fn c8(&mut self) -> C8_W {
        C8_W { w: self }
    }
    #[doc = "Bit 24 - Color Space Conversion Blue Component Offset"]
    #[inline(always)]
    pub fn boff(&mut self) -> BOFF_W {
        BOFF_W { w: self }
    }
}
