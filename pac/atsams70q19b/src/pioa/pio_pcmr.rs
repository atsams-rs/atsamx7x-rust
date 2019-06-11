#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIO_PCMR {
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
#[doc = r" Value of the field"]
pub struct PCENR {
    bits: bool,
}
impl PCENR {
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
#[doc = "Possible values of the field `DSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSIZER {
    #[doc = "The reception data in the PIO_PCRHR is a byte (8-bit)"]
    BYTE,
    #[doc = "The reception data in the PIO_PCRHR is a half-word (16-bit)"]
    HALFWORD,
    #[doc = "The reception data in the PIO_PCRHR is a word (32-bit)"]
    WORD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSIZER::BYTE => 0,
            DSIZER::HALFWORD => 1,
            DSIZER::WORD => 2,
            DSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSIZER {
        match value {
            0 => DSIZER::BYTE,
            1 => DSIZER::HALFWORD,
            2 => DSIZER::WORD,
            i => DSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline]
    pub fn is_byte(&self) -> bool {
        *self == DSIZER::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline]
    pub fn is_halfword(&self) -> bool {
        *self == DSIZER::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline]
    pub fn is_word(&self) -> bool {
        *self == DSIZER::WORD
    }
}
#[doc = r" Value of the field"]
pub struct ALWYSR {
    bits: bool,
}
impl ALWYSR {
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
pub struct HALFSR {
    bits: bool,
}
impl HALFSR {
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
pub struct FRSTSR {
    bits: bool,
}
impl FRSTSR {
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
#[doc = r" Proxy"]
pub struct _PCENW<'a> {
    w: &'a mut W,
}
impl<'a> _PCENW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSIZEW {
    #[doc = "The reception data in the PIO_PCRHR is a byte (8-bit)"]
    BYTE,
    #[doc = "The reception data in the PIO_PCRHR is a half-word (16-bit)"]
    HALFWORD,
    #[doc = "The reception data in the PIO_PCRHR is a word (32-bit)"]
    WORD,
}
impl DSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSIZEW::BYTE => 0,
            DSIZEW::HALFWORD => 1,
            DSIZEW::WORD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _DSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The reception data in the PIO_PCRHR is a byte (8-bit)"]
    #[inline]
    pub fn byte(self) -> &'a mut W {
        self.variant(DSIZEW::BYTE)
    }
    #[doc = "The reception data in the PIO_PCRHR is a half-word (16-bit)"]
    #[inline]
    pub fn halfword(self) -> &'a mut W {
        self.variant(DSIZEW::HALFWORD)
    }
    #[doc = "The reception data in the PIO_PCRHR is a word (32-bit)"]
    #[inline]
    pub fn word(self) -> &'a mut W {
        self.variant(DSIZEW::WORD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ALWYSW<'a> {
    w: &'a mut W,
}
impl<'a> _ALWYSW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HALFSW<'a> {
    w: &'a mut W,
}
impl<'a> _HALFSW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FRSTSW<'a> {
    w: &'a mut W,
}
impl<'a> _FRSTSW<'a> {
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
        const OFFSET: u8 = 11;
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
    #[doc = "Bit 0 - Parallel Capture Mode Enable"]
    #[inline]
    pub fn pcen(&self) -> PCENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PCENR { bits }
    }
    #[doc = "Bits 4:5 - Parallel Capture Mode Data Size"]
    #[inline]
    pub fn dsize(&self) -> DSIZER {
        DSIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 9 - Parallel Capture Mode Always Sampling"]
    #[inline]
    pub fn alwys(&self) -> ALWYSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALWYSR { bits }
    }
    #[doc = "Bit 10 - Parallel Capture Mode Half Sampling"]
    #[inline]
    pub fn halfs(&self) -> HALFSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HALFSR { bits }
    }
    #[doc = "Bit 11 - Parallel Capture Mode First Sample"]
    #[inline]
    pub fn frsts(&self) -> FRSTSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FRSTSR { bits }
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
    #[doc = "Bit 0 - Parallel Capture Mode Enable"]
    #[inline]
    pub fn pcen(&mut self) -> _PCENW {
        _PCENW { w: self }
    }
    #[doc = "Bits 4:5 - Parallel Capture Mode Data Size"]
    #[inline]
    pub fn dsize(&mut self) -> _DSIZEW {
        _DSIZEW { w: self }
    }
    #[doc = "Bit 9 - Parallel Capture Mode Always Sampling"]
    #[inline]
    pub fn alwys(&mut self) -> _ALWYSW {
        _ALWYSW { w: self }
    }
    #[doc = "Bit 10 - Parallel Capture Mode Half Sampling"]
    #[inline]
    pub fn halfs(&mut self) -> _HALFSW {
        _HALFSW { w: self }
    }
    #[doc = "Bit 11 - Parallel Capture Mode First Sample"]
    #[inline]
    pub fn frsts(&mut self) -> _FRSTSW {
        _FRSTSW { w: self }
    }
}
