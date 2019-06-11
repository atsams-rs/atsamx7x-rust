#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CKGR_PLLAR {
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
#[doc = "Possible values of the field `DIVA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVAR {
    #[doc = "Divider output is 0 and PLLA is disabled."]
    _0,
    #[doc = "Divider is bypassed (divide by 1) and PLLA is enabled."]
    BYPASS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DIVAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIVAR::_0 => 0,
            DIVAR::BYPASS => 1,
            DIVAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIVAR {
        match value {
            0 => DIVAR::_0,
            1 => DIVAR::BYPASS,
            i => DIVAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DIVAR::_0
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline]
    pub fn is_bypass(&self) -> bool {
        *self == DIVAR::BYPASS
    }
}
#[doc = r" Value of the field"]
pub struct PLLACOUNTR {
    bits: u8,
}
impl PLLACOUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MULAR {
    bits: u16,
}
impl MULAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ONER {
    bits: bool,
}
impl ONER {
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
#[doc = "Values that can be written to the field `DIVA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVAW {
    #[doc = "Divider output is 0 and PLLA is disabled."]
    _0,
    #[doc = "Divider is bypassed (divide by 1) and PLLA is enabled."]
    BYPASS,
}
impl DIVAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIVAW::_0 => 0,
            DIVAW::BYPASS => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVAW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVAW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divider output is 0 and PLLA is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIVAW::_0)
    }
    #[doc = "Divider is bypassed (divide by 1) and PLLA is enabled."]
    #[inline]
    pub fn bypass(self) -> &'a mut W {
        self.variant(DIVAW::BYPASS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PLLACOUNTW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLACOUNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MULAW<'a> {
    w: &'a mut W,
}
impl<'a> _MULAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ONEW<'a> {
    w: &'a mut W,
}
impl<'a> _ONEW<'a> {
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
        const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:7 - PLLA Front End Divider"]
    #[inline]
    pub fn diva(&self) -> DIVAR {
        DIVAR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:13 - PLLA Counter"]
    #[inline]
    pub fn pllacount(&self) -> PLLACOUNTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PLLACOUNTR { bits }
    }
    #[doc = "Bits 16:26 - PLLA Multiplier"]
    #[inline]
    pub fn mula(&self) -> MULAR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MULAR { bits }
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline]
    pub fn one(&self) -> ONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ONER { bits }
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
    #[doc = "Bits 0:7 - PLLA Front End Divider"]
    #[inline]
    pub fn diva(&mut self) -> _DIVAW {
        _DIVAW { w: self }
    }
    #[doc = "Bits 8:13 - PLLA Counter"]
    #[inline]
    pub fn pllacount(&mut self) -> _PLLACOUNTW {
        _PLLACOUNTW { w: self }
    }
    #[doc = "Bits 16:26 - PLLA Multiplier"]
    #[inline]
    pub fn mula(&mut self) -> _MULAW {
        _MULAW { w: self }
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline]
    pub fn one(&mut self) -> _ONEW {
        _ONEW { w: self }
    }
}
