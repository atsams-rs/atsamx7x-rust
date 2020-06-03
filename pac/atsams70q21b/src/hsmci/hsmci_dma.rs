#[doc = "Reader of register HSMCI_DMA"]
pub type R = crate::R<u32, super::HSMCI_DMA>;
#[doc = "Writer for register HSMCI_DMA"]
pub type W = crate::W<u32, super::HSMCI_DMA>;
#[doc = "Register HSMCI_DMA `reset()`'s with value 0"]
impl crate::ResetValue for super::HSMCI_DMA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA Channel Read and Write Chunk Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHKSIZE_A {
    #[doc = "0: 1 data available"]
    _1 = 0,
    #[doc = "1: 2 data available"]
    _2 = 1,
    #[doc = "2: 4 data available"]
    _4 = 2,
    #[doc = "3: 8 data available"]
    _8 = 3,
    #[doc = "4: 16 data available"]
    _16 = 4,
}
impl From<CHKSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHKSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHKSIZE`"]
pub type CHKSIZE_R = crate::R<u8, CHKSIZE_A>;
impl CHKSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CHKSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CHKSIZE_A::_1),
            1 => Val(CHKSIZE_A::_2),
            2 => Val(CHKSIZE_A::_4),
            3 => Val(CHKSIZE_A::_8),
            4 => Val(CHKSIZE_A::_16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHKSIZE_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == CHKSIZE_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == CHKSIZE_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == CHKSIZE_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == CHKSIZE_A::_16
    }
}
#[doc = "Write proxy for field `CHKSIZE`"]
pub struct CHKSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHKSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHKSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 data available"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHKSIZE_A::_1)
    }
    #[doc = "2 data available"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CHKSIZE_A::_2)
    }
    #[doc = "4 data available"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(CHKSIZE_A::_4)
    }
    #[doc = "8 data available"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(CHKSIZE_A::_8)
    }
    #[doc = "16 data available"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(CHKSIZE_A::_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - DMA Channel Read and Write Chunk Size"]
    #[inline(always)]
    pub fn chksize(&self) -> CHKSIZE_R {
        CHKSIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - DMA Hardware Handshaking Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - DMA Channel Read and Write Chunk Size"]
    #[inline(always)]
    pub fn chksize(&mut self) -> CHKSIZE_W {
        CHKSIZE_W { w: self }
    }
    #[doc = "Bit 8 - DMA Hardware Handshaking Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
}
