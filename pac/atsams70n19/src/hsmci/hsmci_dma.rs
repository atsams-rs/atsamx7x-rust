#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HSMCI_DMA {
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
}
impl crate::ToBits<u8> for CHKSIZER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CHKSIZER::_1 => 0,
            CHKSIZER::_2 => 1,
            CHKSIZER::_4 => 2,
            CHKSIZER::_8 => 3,
            CHKSIZER::_16 => 4,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CHKSIZE_R = crate::FR<u8, CHKSIZER>;
impl CHKSIZE_R {
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHKSIZER::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == CHKSIZER::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == CHKSIZER::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == CHKSIZER::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == CHKSIZER::_16
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
    #[inline(always)]
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
#[doc = r"Proxy"]
pub struct _CHKSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _CHKSIZEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHKSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 data available"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHKSIZEW::_1)
    }
    #[doc = "2 data available"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(CHKSIZEW::_2)
    }
    #[doc = "4 data available"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(CHKSIZEW::_4)
    }
    #[doc = "8 data available"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(CHKSIZEW::_8)
    }
    #[doc = "16 data available"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(CHKSIZEW::_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DMAEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
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
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 4:6 - DMA Channel Read and Write Chunk Size"]
    #[inline(always)]
    pub fn chksize(&self) -> CHKSIZE_R {
        CHKSIZE_R::new(((self.bits() >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - DMA Hardware Handshaking Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 4:6 - DMA Channel Read and Write Chunk Size"]
    #[inline(always)]
    pub fn chksize(&mut self) -> _CHKSIZEW {
        _CHKSIZEW { w: self }
    }
    #[doc = "Bit 8 - DMA Hardware Handshaking Enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
}
