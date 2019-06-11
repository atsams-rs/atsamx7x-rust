#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UTMI_CKTRIM {
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
#[doc = "Possible values of the field `FREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQR {
    #[doc = "12 MHz reference clock"]
    XTAL12,
    #[doc = "16 MHz reference clock"]
    XTAL16,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FREQR::XTAL12 => 0,
            FREQR::XTAL16 => 1,
            FREQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FREQR {
        match value {
            0 => FREQR::XTAL12,
            1 => FREQR::XTAL16,
            i => FREQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `XTAL12`"]
    #[inline]
    pub fn is_xtal12(&self) -> bool {
        *self == FREQR::XTAL12
    }
    #[doc = "Checks if the value of the field is `XTAL16`"]
    #[inline]
    pub fn is_xtal16(&self) -> bool {
        *self == FREQR::XTAL16
    }
}
#[doc = "Values that can be written to the field `FREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQW {
    #[doc = "12 MHz reference clock"]
    XTAL12,
    #[doc = "16 MHz reference clock"]
    XTAL16,
}
impl FREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FREQW::XTAL12 => 0,
            FREQW::XTAL16 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FREQW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FREQW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "12 MHz reference clock"]
    #[inline]
    pub fn xtal12(self) -> &'a mut W {
        self.variant(FREQW::XTAL12)
    }
    #[doc = "16 MHz reference clock"]
    #[inline]
    pub fn xtal16(self) -> &'a mut W {
        self.variant(FREQW::XTAL16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
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
    #[doc = "Bits 0:1 - UTMI Reference Clock Frequency"]
    #[inline]
    pub fn freq(&self) -> FREQR {
        FREQR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:1 - UTMI Reference Clock Frequency"]
    #[inline]
    pub fn freq(&mut self) -> _FREQW {
        _FREQW { w: self }
    }
}
