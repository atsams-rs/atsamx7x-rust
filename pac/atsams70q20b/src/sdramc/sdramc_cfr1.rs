#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDRAMC_CFR1 {
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
pub struct TMRDR {
    bits: u8,
}
impl TMRDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `UNAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALR {
    #[doc = "Unaligned access is not supported."]
    UNSUPPORTED,
    #[doc = "Unaligned access is supported."]
    SUPPORTED,
}
impl UNALR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            UNALR::UNSUPPORTED => false,
            UNALR::SUPPORTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UNALR {
        match value {
            false => UNALR::UNSUPPORTED,
            true => UNALR::SUPPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNSUPPORTED`"]
    #[inline]
    pub fn is_unsupported(&self) -> bool {
        *self == UNALR::UNSUPPORTED
    }
    #[doc = "Checks if the value of the field is `SUPPORTED`"]
    #[inline]
    pub fn is_supported(&self) -> bool {
        *self == UNALR::SUPPORTED
    }
}
#[doc = r" Proxy"]
pub struct _TMRDW<'a> {
    w: &'a mut W,
}
impl<'a> _TMRDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UNAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNALW {
    #[doc = "Unaligned access is not supported."]
    UNSUPPORTED,
    #[doc = "Unaligned access is supported."]
    SUPPORTED,
}
impl UNALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UNALW::UNSUPPORTED => false,
            UNALW::SUPPORTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UNALW<'a> {
    w: &'a mut W,
}
impl<'a> _UNALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UNALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Unaligned access is not supported."]
    #[inline]
    pub fn unsupported(self) -> &'a mut W {
        self.variant(UNALW::UNSUPPORTED)
    }
    #[doc = "Unaligned access is supported."]
    #[inline]
    pub fn supported(self) -> &'a mut W {
        self.variant(UNALW::SUPPORTED)
    }
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
    #[doc = "Bits 0:3 - Load Mode Register Command to Active or Refresh Command"]
    #[inline]
    pub fn tmrd(&self) -> TMRDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMRDR { bits }
    }
    #[doc = "Bit 8 - Support Unaligned Access"]
    #[inline]
    pub fn unal(&self) -> UNALR {
        UNALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:3 - Load Mode Register Command to Active or Refresh Command"]
    #[inline]
    pub fn tmrd(&mut self) -> _TMRDW {
        _TMRDW { w: self }
    }
    #[doc = "Bit 8 - Support Unaligned Access"]
    #[inline]
    pub fn unal(&mut self) -> _UNALW {
        _UNALW { w: self }
    }
}
