#[doc = "Reader of register US_LONL2HDR"]
pub type R = crate::R<u32, super::US_LONL2HDR>;
#[doc = "Writer for register US_LONL2HDR"]
pub type W = crate::W<u32, super::US_LONL2HDR>;
#[doc = "Register US_LONL2HDR `reset()`'s with value 0"]
impl crate::ResetValue for super::US_LONL2HDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLI`"]
pub type BLI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BLI`"]
pub struct BLI_W<'a> {
    w: &'a mut W,
}
impl<'a> BLI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `ALTP`"]
pub type ALTP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALTP`"]
pub struct ALTP_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PB`"]
pub type PB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB`"]
pub struct PB_W<'a> {
    w: &'a mut W,
}
impl<'a> PB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - LON Backlog Increment"]
    #[inline(always)]
    pub fn bli(&self) -> BLI_R {
        BLI_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - LON Alternate Path Bit"]
    #[inline(always)]
    pub fn altp(&self) -> ALTP_R {
        ALTP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LON Priority Bit"]
    #[inline(always)]
    pub fn pb(&self) -> PB_R {
        PB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - LON Backlog Increment"]
    #[inline(always)]
    pub fn bli(&mut self) -> BLI_W {
        BLI_W { w: self }
    }
    #[doc = "Bit 6 - LON Alternate Path Bit"]
    #[inline(always)]
    pub fn altp(&mut self) -> ALTP_W {
        ALTP_W { w: self }
    }
    #[doc = "Bit 7 - LON Priority Bit"]
    #[inline(always)]
    pub fn pb(&mut self) -> PB_W {
        PB_W { w: self }
    }
}
