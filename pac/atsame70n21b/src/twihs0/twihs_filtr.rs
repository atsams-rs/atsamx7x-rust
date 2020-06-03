#[doc = "Reader of register TWIHS_FILTR"]
pub type R = crate::R<u32, super::TWIHS_FILTR>;
#[doc = "Writer for register TWIHS_FILTR"]
pub type W = crate::W<u32, super::TWIHS_FILTR>;
#[doc = "Register TWIHS_FILTR `reset()`'s with value 0"]
impl crate::ResetValue for super::TWIHS_FILTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FILT`"]
pub type FILT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FILT`"]
pub struct FILT_W<'a> {
    w: &'a mut W,
}
impl<'a> FILT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `PADFEN`"]
pub type PADFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PADFEN`"]
pub struct PADFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PADFEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `PADFCFG`"]
pub type PADFCFG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PADFCFG`"]
pub struct PADFCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PADFCFG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `THRES`"]
pub type THRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `THRES`"]
pub struct THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RX Digital Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FILT_R {
        FILT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PAD Filter Enable"]
    #[inline(always)]
    pub fn padfen(&self) -> PADFEN_R {
        PADFEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PAD Filter Config"]
    #[inline(always)]
    pub fn padfcfg(&self) -> PADFCFG_R {
        PADFCFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Digital Filter Threshold"]
    #[inline(always)]
    pub fn thres(&self) -> THRES_R {
        THRES_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RX Digital Filter"]
    #[inline(always)]
    pub fn filt(&mut self) -> FILT_W {
        FILT_W { w: self }
    }
    #[doc = "Bit 1 - PAD Filter Enable"]
    #[inline(always)]
    pub fn padfen(&mut self) -> PADFEN_W {
        PADFEN_W { w: self }
    }
    #[doc = "Bit 2 - PAD Filter Config"]
    #[inline(always)]
    pub fn padfcfg(&mut self) -> PADFCFG_W {
        PADFCFG_W { w: self }
    }
    #[doc = "Bits 8:10 - Digital Filter Threshold"]
    #[inline(always)]
    pub fn thres(&mut self) -> THRES_W {
        THRES_W { w: self }
    }
}
