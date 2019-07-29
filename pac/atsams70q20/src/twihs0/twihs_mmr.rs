#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TWIHS_MMR {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `IADRSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IADRSZR {
    #[doc = "No internal device address"]
    NONE,
    #[doc = "One-byte internal device address"]
    _1_BYTE,
    #[doc = "Two-byte internal device address"]
    _2_BYTE,
    #[doc = "Three-byte internal device address"]
    _3_BYTE,
}
impl crate::ToBits<u8> for IADRSZR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            IADRSZR::NONE => 0,
            IADRSZR::_1_BYTE => 1,
            IADRSZR::_2_BYTE => 2,
            IADRSZR::_3_BYTE => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type IADRSZ_R = crate::FR<u8, IADRSZR>;
impl IADRSZ_R {
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == IADRSZR::NONE
    }
    #[doc = "Checks if the value of the field is `_1_BYTE`"]
    #[inline(always)]
    pub fn is_1_byte(&self) -> bool {
        *self == IADRSZR::_1_BYTE
    }
    #[doc = "Checks if the value of the field is `_2_BYTE`"]
    #[inline(always)]
    pub fn is_2_byte(&self) -> bool {
        *self == IADRSZR::_2_BYTE
    }
    #[doc = "Checks if the value of the field is `_3_BYTE`"]
    #[inline(always)]
    pub fn is_3_byte(&self) -> bool {
        *self == IADRSZR::_3_BYTE
    }
}
#[doc = "Values that can be written to the field `IADRSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IADRSZW {
    #[doc = "No internal device address"]
    NONE,
    #[doc = "One-byte internal device address"]
    _1_BYTE,
    #[doc = "Two-byte internal device address"]
    _2_BYTE,
    #[doc = "Three-byte internal device address"]
    _3_BYTE,
}
impl IADRSZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            IADRSZW::NONE => 0,
            IADRSZW::_1_BYTE => 1,
            IADRSZW::_2_BYTE => 2,
            IADRSZW::_3_BYTE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _IADRSZW<'a> {
    w: &'a mut W,
}
impl<'a> _IADRSZW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IADRSZW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No internal device address"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(IADRSZW::NONE)
    }
    #[doc = "One-byte internal device address"]
    #[inline(always)]
    pub fn _1_byte(self) -> &'a mut W {
        self.variant(IADRSZW::_1_BYTE)
    }
    #[doc = "Two-byte internal device address"]
    #[inline(always)]
    pub fn _2_byte(self) -> &'a mut W {
        self.variant(IADRSZW::_2_BYTE)
    }
    #[doc = "Three-byte internal device address"]
    #[inline(always)]
    pub fn _3_byte(self) -> &'a mut W {
        self.variant(IADRSZW::_3_BYTE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MREAD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _MREADW<'a> {
    w: &'a mut W,
}
impl<'a> _MREADW<'a> {
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
#[doc = r"Reader of the field"]
pub type DADR_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DADRW<'a> {
    w: &'a mut W,
}
impl<'a> _DADRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 8:9 - Internal Device Address Size"]
    #[inline(always)]
    pub fn iadrsz(&self) -> IADRSZ_R {
        IADRSZ_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Master Read Direction"]
    #[inline(always)]
    pub fn mread(&self) -> MREAD_R {
        MREAD_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - Device Address"]
    #[inline(always)]
    pub fn dadr(&self) -> DADR_R {
        DADR_R::new(((self.bits() >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:9 - Internal Device Address Size"]
    #[inline(always)]
    pub fn iadrsz(&mut self) -> _IADRSZW {
        _IADRSZW { w: self }
    }
    #[doc = "Bit 12 - Master Read Direction"]
    #[inline(always)]
    pub fn mread(&mut self) -> _MREADW {
        _MREADW { w: self }
    }
    #[doc = "Bits 16:22 - Device Address"]
    #[inline(always)]
    pub fn dadr(&mut self) -> _DADRW {
        _DADRW { w: self }
    }
}
