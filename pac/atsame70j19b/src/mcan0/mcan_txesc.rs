#[doc = "Reader of register MCAN_TXESC"]
pub type R = crate::R<u32, super::MCAN_TXESC>;
#[doc = "Writer for register MCAN_TXESC"]
pub type W = crate::W<u32, super::MCAN_TXESC>;
#[doc = "Register MCAN_TXESC `reset()`'s with value 0"]
impl crate::ResetValue for super::MCAN_TXESC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Tx Buffer Data Field Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TBDS_A {
    #[doc = "0: 8-byte data field"]
    _8_BYTE = 0,
    #[doc = "1: 12-byte data field"]
    _12_BYTE = 1,
    #[doc = "2: 16-byte data field"]
    _16_BYTE = 2,
    #[doc = "3: 20-byte data field"]
    _20_BYTE = 3,
    #[doc = "4: 24-byte data field"]
    _24_BYTE = 4,
    #[doc = "5: 32-byte data field"]
    _32_BYTE = 5,
    #[doc = "6: 48-byte data field"]
    _48_BYTE = 6,
    #[doc = "7: 64-byte data field"]
    _64_BYTE = 7,
}
impl From<TBDS_A> for u8 {
    #[inline(always)]
    fn from(variant: TBDS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TBDS`"]
pub type TBDS_R = crate::R<u8, TBDS_A>;
impl TBDS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBDS_A {
        match self.bits {
            0 => TBDS_A::_8_BYTE,
            1 => TBDS_A::_12_BYTE,
            2 => TBDS_A::_16_BYTE,
            3 => TBDS_A::_20_BYTE,
            4 => TBDS_A::_24_BYTE,
            5 => TBDS_A::_32_BYTE,
            6 => TBDS_A::_48_BYTE,
            7 => TBDS_A::_64_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == TBDS_A::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_12_BYTE`"]
    #[inline(always)]
    pub fn is_12_byte(&self) -> bool {
        *self == TBDS_A::_12_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == TBDS_A::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_20_BYTE`"]
    #[inline(always)]
    pub fn is_20_byte(&self) -> bool {
        *self == TBDS_A::_20_BYTE
    }
    #[doc = "Checks if the value of the field is `_24_BYTE`"]
    #[inline(always)]
    pub fn is_24_byte(&self) -> bool {
        *self == TBDS_A::_24_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == TBDS_A::_32_BYTE
    }
    #[doc = "Checks if the value of the field is `_48_BYTE`"]
    #[inline(always)]
    pub fn is_48_byte(&self) -> bool {
        *self == TBDS_A::_48_BYTE
    }
    #[doc = "Checks if the value of the field is `_64_BYTE`"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == TBDS_A::_64_BYTE
    }
}
#[doc = "Write proxy for field `TBDS`"]
pub struct TBDS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBDS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TBDS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "8-byte data field"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(TBDS_A::_8_BYTE)
    }
    #[doc = "12-byte data field"]
    #[inline(always)]
    pub fn _12_byte(self) -> &'a mut W {
        self.variant(TBDS_A::_12_BYTE)
    }
    #[doc = "16-byte data field"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(TBDS_A::_16_BYTE)
    }
    #[doc = "20-byte data field"]
    #[inline(always)]
    pub fn _20_byte(self) -> &'a mut W {
        self.variant(TBDS_A::_20_BYTE)
    }
    #[doc = "24-byte data field"]
    #[inline(always)]
    pub fn _24_byte(self) -> &'a mut W {
        self.variant(TBDS_A::_24_BYTE)
    }
    #[doc = "32-byte data field"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(TBDS_A::_32_BYTE)
    }
    #[doc = "48-byte data field"]
    #[inline(always)]
    pub fn _48_byte(self) -> &'a mut W {
        self.variant(TBDS_A::_48_BYTE)
    }
    #[doc = "64-byte data field"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut W {
        self.variant(TBDS_A::_64_BYTE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size"]
    #[inline(always)]
    pub fn tbds(&self) -> TBDS_R {
        TBDS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size"]
    #[inline(always)]
    pub fn tbds(&mut self) -> TBDS_W {
        TBDS_W { w: self }
    }
}
