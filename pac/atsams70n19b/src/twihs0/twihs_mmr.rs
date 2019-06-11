#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TWIHS_MMR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
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
impl IADRSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IADRSZR::NONE => 0,
            IADRSZR::_1_BYTE => 1,
            IADRSZR::_2_BYTE => 2,
            IADRSZR::_3_BYTE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IADRSZR {
        match value {
            0 => IADRSZR::NONE,
            1 => IADRSZR::_1_BYTE,
            2 => IADRSZR::_2_BYTE,
            3 => IADRSZR::_3_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == IADRSZR::NONE
    }
    #[doc = "Checks if the value of the field is `_1_BYTE`"]
    #[inline]
    pub fn is_1_byte(&self) -> bool {
        *self == IADRSZR::_1_BYTE
    }
    #[doc = "Checks if the value of the field is `_2_BYTE`"]
    #[inline]
    pub fn is_2_byte(&self) -> bool {
        *self == IADRSZR::_2_BYTE
    }
    #[doc = "Checks if the value of the field is `_3_BYTE`"]
    #[inline]
    pub fn is_3_byte(&self) -> bool {
        *self == IADRSZR::_3_BYTE
    }
}
#[doc = r" Value of the field"]
pub struct MREADR {
    bits: bool,
}
impl MREADR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DADRR {
    bits: u8,
}
impl DADRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IADRSZW::NONE => 0,
            IADRSZW::_1_BYTE => 1,
            IADRSZW::_2_BYTE => 2,
            IADRSZW::_3_BYTE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IADRSZW<'a> {
    w: &'a mut W,
}
impl<'a> _IADRSZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IADRSZW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No internal device address"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(IADRSZW::NONE)
    }
    #[doc = "One-byte internal device address"]
    #[inline]
    pub fn _1_byte(self) -> &'a mut W {
        self.variant(IADRSZW::_1_BYTE)
    }
    #[doc = "Two-byte internal device address"]
    #[inline]
    pub fn _2_byte(self) -> &'a mut W {
        self.variant(IADRSZW::_2_BYTE)
    }
    #[doc = "Three-byte internal device address"]
    #[inline]
    pub fn _3_byte(self) -> &'a mut W {
        self.variant(IADRSZW::_3_BYTE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MREADW<'a> {
    w: &'a mut W,
}
impl<'a> _MREADW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DADRW<'a> {
    w: &'a mut W,
}
impl<'a> _DADRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 8:9 - Internal Device Address Size"]
    #[inline]
    pub fn iadrsz(&self) -> IADRSZR {
        IADRSZR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Master Read Direction"]
    #[inline]
    pub fn mread(&self) -> MREADR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MREADR { bits }
    }
    #[doc = "Bits 16:22 - Device Address"]
    #[inline]
    pub fn dadr(&self) -> DADRR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DADRR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:9 - Internal Device Address Size"]
    #[inline]
    pub fn iadrsz(&mut self) -> _IADRSZW {
        _IADRSZW { w: self }
    }
    #[doc = "Bit 12 - Master Read Direction"]
    #[inline]
    pub fn mread(&mut self) -> _MREADW {
        _MREADW { w: self }
    }
    #[doc = "Bits 16:22 - Device Address"]
    #[inline]
    pub fn dadr(&mut self) -> _DADRW {
        _DADRW { w: self }
    }
}
