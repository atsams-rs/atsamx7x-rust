#[doc = "Reader of register PIO_PCMR"]
pub type R = crate::R<u32, super::PIO_PCMR>;
#[doc = "Writer for register PIO_PCMR"]
pub type W = crate::W<u32, super::PIO_PCMR>;
#[doc = "Register PIO_PCMR `reset()`'s with value 0"]
impl crate::ResetValue for super::PIO_PCMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCEN`"]
pub type PCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PCEN`"]
pub struct PCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PCEN_W<'a> {
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
#[doc = "Parallel Capture Mode Data Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSIZE_A {
    #[doc = "0: The reception data in the PIO_PCRHR is a byte (8-bit)"]
    BYTE = 0,
    #[doc = "1: The reception data in the PIO_PCRHR is a half-word (16-bit)"]
    HALFWORD = 1,
    #[doc = "2: The reception data in the PIO_PCRHR is a word (32-bit)"]
    WORD = 2,
}
impl From<DSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: DSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DSIZE`"]
pub type DSIZE_R = crate::R<u8, DSIZE_A>;
impl DSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DSIZE_A::BYTE),
            1 => Val(DSIZE_A::HALFWORD),
            2 => Val(DSIZE_A::WORD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DSIZE_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == DSIZE_A::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DSIZE_A::WORD
    }
}
#[doc = "Write proxy for field `DSIZE`"]
pub struct DSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The reception data in the PIO_PCRHR is a byte (8-bit)"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(DSIZE_A::BYTE)
    }
    #[doc = "The reception data in the PIO_PCRHR is a half-word (16-bit)"]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut W {
        self.variant(DSIZE_A::HALFWORD)
    }
    #[doc = "The reception data in the PIO_PCRHR is a word (32-bit)"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(DSIZE_A::WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `ALWYS`"]
pub type ALWYS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALWYS`"]
pub struct ALWYS_W<'a> {
    w: &'a mut W,
}
impl<'a> ALWYS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `HALFS`"]
pub type HALFS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HALFS`"]
pub struct HALFS_W<'a> {
    w: &'a mut W,
}
impl<'a> HALFS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `FRSTS`"]
pub type FRSTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRSTS`"]
pub struct FRSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> FRSTS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Parallel Capture Mode Enable"]
    #[inline(always)]
    pub fn pcen(&self) -> PCEN_R {
        PCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Parallel Capture Mode Data Size"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 9 - Parallel Capture Mode Always Sampling"]
    #[inline(always)]
    pub fn alwys(&self) -> ALWYS_R {
        ALWYS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Parallel Capture Mode Half Sampling"]
    #[inline(always)]
    pub fn halfs(&self) -> HALFS_R {
        HALFS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Parallel Capture Mode First Sample"]
    #[inline(always)]
    pub fn frsts(&self) -> FRSTS_R {
        FRSTS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parallel Capture Mode Enable"]
    #[inline(always)]
    pub fn pcen(&mut self) -> PCEN_W {
        PCEN_W { w: self }
    }
    #[doc = "Bits 4:5 - Parallel Capture Mode Data Size"]
    #[inline(always)]
    pub fn dsize(&mut self) -> DSIZE_W {
        DSIZE_W { w: self }
    }
    #[doc = "Bit 9 - Parallel Capture Mode Always Sampling"]
    #[inline(always)]
    pub fn alwys(&mut self) -> ALWYS_W {
        ALWYS_W { w: self }
    }
    #[doc = "Bit 10 - Parallel Capture Mode Half Sampling"]
    #[inline(always)]
    pub fn halfs(&mut self) -> HALFS_W {
        HALFS_W { w: self }
    }
    #[doc = "Bit 11 - Parallel Capture Mode First Sample"]
    #[inline(always)]
    pub fn frsts(&mut self) -> FRSTS_W {
        FRSTS_W { w: self }
    }
}
