#[doc = "Reader of register ISI_Y2R_SET1"]
pub type R = crate::R<u32, super::ISI_Y2R_SET1>;
#[doc = "Writer for register ISI_Y2R_SET1"]
pub type W = crate::W<u32, super::ISI_Y2R_SET1>;
#[doc = "Register ISI_Y2R_SET1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ISI_Y2R_SET1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `C4`"]
pub type C4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `C4`"]
pub struct C4_W<'a> {
    w: &'a mut W,
}
impl<'a> C4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Reader of field `Yoff`"]
pub type YOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Yoff`"]
pub struct YOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> YOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `Croff`"]
pub type CROFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Croff`"]
pub struct CROFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CROFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `Cboff`"]
pub type CBOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Cboff`"]
pub struct CBOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CBOFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Color Space Conversion Matrix Coefficient C4"]
    #[inline(always)]
    pub fn c4(&self) -> C4_R {
        C4_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 12 - Color Space Conversion Luminance Default Offset"]
    #[inline(always)]
    pub fn yoff(&self) -> YOFF_R {
        YOFF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Color Space Conversion Red Chrominance Default Offset"]
    #[inline(always)]
    pub fn croff(&self) -> CROFF_R {
        CROFF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Color Space Conversion Blue Chrominance Default Offset"]
    #[inline(always)]
    pub fn cboff(&self) -> CBOFF_R {
        CBOFF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Color Space Conversion Matrix Coefficient C4"]
    #[inline(always)]
    pub fn c4(&mut self) -> C4_W {
        C4_W { w: self }
    }
    #[doc = "Bit 12 - Color Space Conversion Luminance Default Offset"]
    #[inline(always)]
    pub fn yoff(&mut self) -> YOFF_W {
        YOFF_W { w: self }
    }
    #[doc = "Bit 13 - Color Space Conversion Red Chrominance Default Offset"]
    #[inline(always)]
    pub fn croff(&mut self) -> CROFF_W {
        CROFF_W { w: self }
    }
    #[doc = "Bit 14 - Color Space Conversion Blue Chrominance Default Offset"]
    #[inline(always)]
    pub fn cboff(&mut self) -> CBOFF_W {
        CBOFF_W { w: self }
    }
}
