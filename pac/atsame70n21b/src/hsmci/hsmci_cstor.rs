#[doc = "Reader of register HSMCI_CSTOR"]
pub type R = crate::R<u32, super::HSMCI_CSTOR>;
#[doc = "Writer for register HSMCI_CSTOR"]
pub type W = crate::W<u32, super::HSMCI_CSTOR>;
#[doc = "Register HSMCI_CSTOR `reset()`'s with value 0"]
impl crate::ResetValue for super::HSMCI_CSTOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSTOCYC`"]
pub type CSTOCYC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSTOCYC`"]
pub struct CSTOCYC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTOCYC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Completion Signal Timeout Multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSTOMUL_A {
    #[doc = "0: CSTOCYC x 1"]
    _1 = 0,
    #[doc = "1: CSTOCYC x 16"]
    _16 = 1,
    #[doc = "2: CSTOCYC x 128"]
    _128 = 2,
    #[doc = "3: CSTOCYC x 256"]
    _256 = 3,
    #[doc = "4: CSTOCYC x 1024"]
    _1024 = 4,
    #[doc = "5: CSTOCYC x 4096"]
    _4096 = 5,
    #[doc = "6: CSTOCYC x 65536"]
    _65536 = 6,
    #[doc = "7: CSTOCYC x 1048576"]
    _1048576 = 7,
}
impl From<CSTOMUL_A> for u8 {
    #[inline(always)]
    fn from(variant: CSTOMUL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CSTOMUL`"]
pub type CSTOMUL_R = crate::R<u8, CSTOMUL_A>;
impl CSTOMUL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSTOMUL_A {
        match self.bits {
            0 => CSTOMUL_A::_1,
            1 => CSTOMUL_A::_16,
            2 => CSTOMUL_A::_128,
            3 => CSTOMUL_A::_256,
            4 => CSTOMUL_A::_1024,
            5 => CSTOMUL_A::_4096,
            6 => CSTOMUL_A::_65536,
            7 => CSTOMUL_A::_1048576,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CSTOMUL_A::_1
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == CSTOMUL_A::_16
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == CSTOMUL_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == CSTOMUL_A::_256
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == CSTOMUL_A::_1024
    }
    #[doc = "Checks if the value of the field is `_4096`"]
    #[inline(always)]
    pub fn is_4096(&self) -> bool {
        *self == CSTOMUL_A::_4096
    }
    #[doc = "Checks if the value of the field is `_65536`"]
    #[inline(always)]
    pub fn is_65536(&self) -> bool {
        *self == CSTOMUL_A::_65536
    }
    #[doc = "Checks if the value of the field is `_1048576`"]
    #[inline(always)]
    pub fn is_1048576(&self) -> bool {
        *self == CSTOMUL_A::_1048576
    }
}
#[doc = "Write proxy for field `CSTOMUL`"]
pub struct CSTOMUL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTOMUL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSTOMUL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CSTOCYC x 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CSTOMUL_A::_1)
    }
    #[doc = "CSTOCYC x 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(CSTOMUL_A::_16)
    }
    #[doc = "CSTOCYC x 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(CSTOMUL_A::_128)
    }
    #[doc = "CSTOCYC x 256"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(CSTOMUL_A::_256)
    }
    #[doc = "CSTOCYC x 1024"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(CSTOMUL_A::_1024)
    }
    #[doc = "CSTOCYC x 4096"]
    #[inline(always)]
    pub fn _4096(self) -> &'a mut W {
        self.variant(CSTOMUL_A::_4096)
    }
    #[doc = "CSTOCYC x 65536"]
    #[inline(always)]
    pub fn _65536(self) -> &'a mut W {
        self.variant(CSTOMUL_A::_65536)
    }
    #[doc = "CSTOCYC x 1048576"]
    #[inline(always)]
    pub fn _1048576(self) -> &'a mut W {
        self.variant(CSTOMUL_A::_1048576)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Completion Signal Timeout Cycle Number"]
    #[inline(always)]
    pub fn cstocyc(&self) -> CSTOCYC_R {
        CSTOCYC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Completion Signal Timeout Multiplier"]
    #[inline(always)]
    pub fn cstomul(&self) -> CSTOMUL_R {
        CSTOMUL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Completion Signal Timeout Cycle Number"]
    #[inline(always)]
    pub fn cstocyc(&mut self) -> CSTOCYC_W {
        CSTOCYC_W { w: self }
    }
    #[doc = "Bits 4:6 - Completion Signal Timeout Multiplier"]
    #[inline(always)]
    pub fn cstomul(&mut self) -> CSTOMUL_W {
        CSTOMUL_W { w: self }
    }
}
