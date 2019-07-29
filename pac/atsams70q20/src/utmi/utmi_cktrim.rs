#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UTMI_CKTRIM {
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
#[doc = "Possible values of the field `FREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREQR {
    #[doc = "12 MHz reference clock"]
    XTAL12,
    #[doc = "16 MHz reference clock"]
    XTAL16,
}
impl crate::ToBits<u8> for FREQR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            FREQR::XTAL12 => 0,
            FREQR::XTAL16 => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FREQ_R = crate::FR<u8, FREQR>;
impl FREQ_R {
    #[doc = "Checks if the value of the field is `XTAL12`"]
    #[inline(always)]
    pub fn is_xtal12(&self) -> bool {
        *self == FREQR::XTAL12
    }
    #[doc = "Checks if the value of the field is `XTAL16`"]
    #[inline(always)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FREQW::XTAL12 => 0,
            FREQW::XTAL16 => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FREQW<'a> {
    w: &'a mut W,
}
impl<'a> _FREQW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "12 MHz reference clock"]
    #[inline(always)]
    pub fn xtal12(self) -> &'a mut W {
        self.variant(FREQW::XTAL12)
    }
    #[doc = "16 MHz reference clock"]
    #[inline(always)]
    pub fn xtal16(self) -> &'a mut W {
        self.variant(FREQW::XTAL16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - UTMI Reference Clock Frequency"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits() & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - UTMI Reference Clock Frequency"]
    #[inline(always)]
    pub fn freq(&mut self) -> _FREQW {
        _FREQW { w: self }
    }
}
