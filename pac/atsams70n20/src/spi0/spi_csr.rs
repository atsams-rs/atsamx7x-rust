#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SPI_CSR {
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
pub struct CPOLR {
    bits: bool,
}
impl CPOLR {
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
pub struct NCPHAR {
    bits: bool,
}
impl NCPHAR {
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
pub struct CSNAATR {
    bits: bool,
}
impl CSNAATR {
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
pub struct CSAATR {
    bits: bool,
}
impl CSAATR {
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
#[doc = "Possible values of the field `BITS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITSR {
    #[doc = "8 bits for transfer"]
    _8_BIT,
    #[doc = "9 bits for transfer"]
    _9_BIT,
    #[doc = "10 bits for transfer"]
    _10_BIT,
    #[doc = "11 bits for transfer"]
    _11_BIT,
    #[doc = "12 bits for transfer"]
    _12_BIT,
    #[doc = "13 bits for transfer"]
    _13_BIT,
    #[doc = "14 bits for transfer"]
    _14_BIT,
    #[doc = "15 bits for transfer"]
    _15_BIT,
    #[doc = "16 bits for transfer"]
    _16_BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BITSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BITSR::_8_BIT => 0,
            BITSR::_9_BIT => 1,
            BITSR::_10_BIT => 2,
            BITSR::_11_BIT => 3,
            BITSR::_12_BIT => 4,
            BITSR::_13_BIT => 5,
            BITSR::_14_BIT => 6,
            BITSR::_15_BIT => 7,
            BITSR::_16_BIT => 8,
            BITSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BITSR {
        match value {
            0 => BITSR::_8_BIT,
            1 => BITSR::_9_BIT,
            2 => BITSR::_10_BIT,
            3 => BITSR::_11_BIT,
            4 => BITSR::_12_BIT,
            5 => BITSR::_13_BIT,
            6 => BITSR::_14_BIT,
            7 => BITSR::_15_BIT,
            8 => BITSR::_16_BIT,
            i => BITSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline]
    pub fn is_8_bit(&self) -> bool {
        *self == BITSR::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_9_BIT`"]
    #[inline]
    pub fn is_9_bit(&self) -> bool {
        *self == BITSR::_9_BIT
    }
    #[doc = "Checks if the value of the field is `_10_BIT`"]
    #[inline]
    pub fn is_10_bit(&self) -> bool {
        *self == BITSR::_10_BIT
    }
    #[doc = "Checks if the value of the field is `_11_BIT`"]
    #[inline]
    pub fn is_11_bit(&self) -> bool {
        *self == BITSR::_11_BIT
    }
    #[doc = "Checks if the value of the field is `_12_BIT`"]
    #[inline]
    pub fn is_12_bit(&self) -> bool {
        *self == BITSR::_12_BIT
    }
    #[doc = "Checks if the value of the field is `_13_BIT`"]
    #[inline]
    pub fn is_13_bit(&self) -> bool {
        *self == BITSR::_13_BIT
    }
    #[doc = "Checks if the value of the field is `_14_BIT`"]
    #[inline]
    pub fn is_14_bit(&self) -> bool {
        *self == BITSR::_14_BIT
    }
    #[doc = "Checks if the value of the field is `_15_BIT`"]
    #[inline]
    pub fn is_15_bit(&self) -> bool {
        *self == BITSR::_15_BIT
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline]
    pub fn is_16_bit(&self) -> bool {
        *self == BITSR::_16_BIT
    }
}
#[doc = r" Value of the field"]
pub struct SCBRR {
    bits: u8,
}
impl SCBRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLYBSR {
    bits: u8,
}
impl DLYBSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLYBCTR {
    bits: u8,
}
impl DLYBCTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
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
#[doc = r" Proxy"]
pub struct _NCPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _NCPHAW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSNAATW<'a> {
    w: &'a mut W,
}
impl<'a> _CSNAATW<'a> {
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSAATW<'a> {
    w: &'a mut W,
}
impl<'a> _CSAATW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BITS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITSW {
    #[doc = "8 bits for transfer"]
    _8_BIT,
    #[doc = "9 bits for transfer"]
    _9_BIT,
    #[doc = "10 bits for transfer"]
    _10_BIT,
    #[doc = "11 bits for transfer"]
    _11_BIT,
    #[doc = "12 bits for transfer"]
    _12_BIT,
    #[doc = "13 bits for transfer"]
    _13_BIT,
    #[doc = "14 bits for transfer"]
    _14_BIT,
    #[doc = "15 bits for transfer"]
    _15_BIT,
    #[doc = "16 bits for transfer"]
    _16_BIT,
}
impl BITSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BITSW::_8_BIT => 0,
            BITSW::_9_BIT => 1,
            BITSW::_10_BIT => 2,
            BITSW::_11_BIT => 3,
            BITSW::_12_BIT => 4,
            BITSW::_13_BIT => 5,
            BITSW::_14_BIT => 6,
            BITSW::_15_BIT => 7,
            BITSW::_16_BIT => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BITSW<'a> {
    w: &'a mut W,
}
impl<'a> _BITSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BITSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8 bits for transfer"]
    #[inline]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(BITSW::_8_BIT)
    }
    #[doc = "9 bits for transfer"]
    #[inline]
    pub fn _9_bit(self) -> &'a mut W {
        self.variant(BITSW::_9_BIT)
    }
    #[doc = "10 bits for transfer"]
    #[inline]
    pub fn _10_bit(self) -> &'a mut W {
        self.variant(BITSW::_10_BIT)
    }
    #[doc = "11 bits for transfer"]
    #[inline]
    pub fn _11_bit(self) -> &'a mut W {
        self.variant(BITSW::_11_BIT)
    }
    #[doc = "12 bits for transfer"]
    #[inline]
    pub fn _12_bit(self) -> &'a mut W {
        self.variant(BITSW::_12_BIT)
    }
    #[doc = "13 bits for transfer"]
    #[inline]
    pub fn _13_bit(self) -> &'a mut W {
        self.variant(BITSW::_13_BIT)
    }
    #[doc = "14 bits for transfer"]
    #[inline]
    pub fn _14_bit(self) -> &'a mut W {
        self.variant(BITSW::_14_BIT)
    }
    #[doc = "15 bits for transfer"]
    #[inline]
    pub fn _15_bit(self) -> &'a mut W {
        self.variant(BITSW::_15_BIT)
    }
    #[doc = "16 bits for transfer"]
    #[inline]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(BITSW::_16_BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SCBRW<'a> {
    w: &'a mut W,
}
impl<'a> _SCBRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLYBSW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYBSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DLYBCTW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYBCTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CPOLR { bits }
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline]
    pub fn ncpha(&self) -> NCPHAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NCPHAR { bits }
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
    #[inline]
    pub fn csnaat(&self) -> CSNAATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSNAATR { bits }
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline]
    pub fn csaat(&self) -> CSAATR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSAATR { bits }
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline]
    pub fn bits_(&self) -> BITSR {
        BITSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Serial Clock Bit Rate"]
    #[inline]
    pub fn scbr(&self) -> SCBRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCBRR { bits }
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline]
    pub fn dlybs(&self) -> DLYBSR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYBSR { bits }
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline]
    pub fn dlybct(&self) -> DLYBCTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYBCTR { bits }
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
    #[doc = "Bit 0 - Clock Polarity"]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 1 - Clock Phase"]
    #[inline]
    pub fn ncpha(&mut self) -> _NCPHAW {
        _NCPHAW { w: self }
    }
    #[doc = "Bit 2 - Chip Select Not Active After Transfer (Ignored if CSAAT = 1)"]
    #[inline]
    pub fn csnaat(&mut self) -> _CSNAATW {
        _CSNAATW { w: self }
    }
    #[doc = "Bit 3 - Chip Select Active After Transfer"]
    #[inline]
    pub fn csaat(&mut self) -> _CSAATW {
        _CSAATW { w: self }
    }
    #[doc = "Bits 4:7 - Bits Per Transfer"]
    #[inline]
    pub fn bits_(&mut self) -> _BITSW {
        _BITSW { w: self }
    }
    #[doc = "Bits 8:15 - Serial Clock Bit Rate"]
    #[inline]
    pub fn scbr(&mut self) -> _SCBRW {
        _SCBRW { w: self }
    }
    #[doc = "Bits 16:23 - Delay Before SPCK"]
    #[inline]
    pub fn dlybs(&mut self) -> _DLYBSW {
        _DLYBSW { w: self }
    }
    #[doc = "Bits 24:31 - Delay Between Consecutive Transfers"]
    #[inline]
    pub fn dlybct(&mut self) -> _DLYBCTW {
        _DLYBCTW { w: self }
    }
}
