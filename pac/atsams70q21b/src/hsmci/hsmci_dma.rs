#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HSMCI_DMA {
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
#[doc = "Possible values of the field `CHKSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHKSIZER {
    #[doc = "1 data available"]
    _1,
    #[doc = "2 data available"]
    _2,
    #[doc = "4 data available"]
    _4,
    #[doc = "8 data available"]
    _8,
    #[doc = "16 data available"]
    _16,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CHKSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHKSIZER::_1 => 0,
            CHKSIZER::_2 => 1,
            CHKSIZER::_4 => 2,
            CHKSIZER::_8 => 3,
            CHKSIZER::_16 => 4,
            CHKSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHKSIZER {
        match value {
            0 => CHKSIZER::_1,
            1 => CHKSIZER::_2,
            2 => CHKSIZER::_4,
            3 => CHKSIZER::_8,
            4 => CHKSIZER::_16,
            i => CHKSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CHKSIZER::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline]
    pub fn is_2(&self) -> bool {
        *self == CHKSIZER::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline]
    pub fn is_4(&self) -> bool {
        *self == CHKSIZER::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline]
    pub fn is_8(&self) -> bool {
        *self == CHKSIZER::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline]
    pub fn is_16(&self) -> bool {
        *self == CHKSIZER::_16
    }
}
#[doc = r" Value of the field"]
pub struct DMAENR {
    bits: bool,
}
impl DMAENR {
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
#[doc = "Values that can be written to the field `CHKSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHKSIZEW {
    #[doc = "1 data available"]
    _1,
    #[doc = "2 data available"]
    _2,
    #[doc = "4 data available"]
    _4,
    #[doc = "8 data available"]
    _8,
    #[doc = "16 data available"]
    _16,
}
impl CHKSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHKSIZEW::_1 => 0,
            CHKSIZEW::_2 => 1,
            CHKSIZEW::_4 => 2,
            CHKSIZEW::_8 => 3,
            CHKSIZEW::_16 => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHKSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _CHKSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHKSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 data available"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHKSIZEW::_1)
    }
    #[doc = "2 data available"]
    #[inline]
    pub fn _2(self) -> &'a mut W {
        self.variant(CHKSIZEW::_2)
    }
    #[doc = "4 data available"]
    #[inline]
    pub fn _4(self) -> &'a mut W {
        self.variant(CHKSIZEW::_4)
    }
    #[doc = "8 data available"]
    #[inline]
    pub fn _8(self) -> &'a mut W {
        self.variant(CHKSIZEW::_8)
    }
    #[doc = "16 data available"]
    #[inline]
    pub fn _16(self) -> &'a mut W {
        self.variant(CHKSIZEW::_16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
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
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 4:6 - DMA Channel Read and Write Chunk Size"]
    #[inline]
    pub fn chksize(&self) -> CHKSIZER {
        CHKSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - DMA Hardware Handshaking Enable"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMAENR { bits }
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
    #[doc = "Bits 4:6 - DMA Channel Read and Write Chunk Size"]
    #[inline]
    pub fn chksize(&mut self) -> _CHKSIZEW {
        _CHKSIZEW { w: self }
    }
    #[doc = "Bit 8 - DMA Hardware Handshaking Enable"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
}
