#[doc = "Reader of register ISI_R2Y_SET0"]
pub type R = crate::R<u32, super::ISI_R2Y_SET0>;
#[doc = "Writer for register ISI_R2Y_SET0"]
pub type W = crate::W<u32, super::ISI_R2Y_SET0>;
#[doc = "Register ISI_R2Y_SET0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ISI_R2Y_SET0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `C0`"]
pub type C0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `C0`"]
pub struct C0_W<'a> {
    w: &'a mut W,
}
impl<'a> C0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `C1`"]
pub type C1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `C1`"]
pub struct C1_W<'a> {
    w: &'a mut W,
}
impl<'a> C1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `C2`"]
pub type C2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `C2`"]
pub struct C2_W<'a> {
    w: &'a mut W,
}
impl<'a> C2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `Roff`"]
pub type ROFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Roff`"]
pub struct ROFF_W<'a> {
    w: &'a mut W,
}
impl<'a> ROFF_W<'a> {
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
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C0"]
    #[inline(always)]
    pub fn c0(&self) -> C0_R {
        C0_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C1"]
    #[inline(always)]
    pub fn c1(&self) -> C1_R {
        C1_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C2"]
    #[inline(always)]
    pub fn c2(&self) -> C2_R {
        C2_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Color Space Conversion Red Component Offset"]
    #[inline(always)]
    pub fn roff(&self) -> ROFF_R {
        ROFF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C0"]
    #[inline(always)]
    pub fn c0(&mut self) -> C0_W {
        C0_W { w: self }
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C1"]
    #[inline(always)]
    pub fn c1(&mut self) -> C1_W {
        C1_W { w: self }
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C2"]
    #[inline(always)]
    pub fn c2(&mut self) -> C2_W {
        C2_W { w: self }
    }
    #[doc = "Bit 24 - Color Space Conversion Red Component Offset"]
    #[inline(always)]
    pub fn roff(&mut self) -> ROFF_W {
        ROFF_W { w: self }
    }
}
