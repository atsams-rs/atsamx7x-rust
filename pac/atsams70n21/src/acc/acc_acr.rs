#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACC_ACR {
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
#[doc = "Possible values of the field `ISEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISELR {
    #[doc = "Low-power option."]
    LOPW,
    #[doc = "High-speed option."]
    HISP,
}
impl ISELR {
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
            ISELR::LOPW => false,
            ISELR::HISP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ISELR {
        match value {
            false => ISELR::LOPW,
            true => ISELR::HISP,
        }
    }
    #[doc = "Checks if the value of the field is `LOPW`"]
    #[inline]
    pub fn is_lopw(&self) -> bool {
        *self == ISELR::LOPW
    }
    #[doc = "Checks if the value of the field is `HISP`"]
    #[inline]
    pub fn is_hisp(&self) -> bool {
        *self == ISELR::HISP
    }
}
#[doc = r" Value of the field"]
pub struct HYSTR {
    bits: u8,
}
impl HYSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `ISEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISELW {
    #[doc = "Low-power option."]
    LOPW,
    #[doc = "High-speed option."]
    HISP,
}
impl ISELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ISELW::LOPW => false,
            ISELW::HISP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ISELW<'a> {
    w: &'a mut W,
}
impl<'a> _ISELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ISELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low-power option."]
    #[inline]
    pub fn lopw(self) -> &'a mut W {
        self.variant(ISELW::LOPW)
    }
    #[doc = "High-speed option."]
    #[inline]
    pub fn hisp(self) -> &'a mut W {
        self.variant(ISELW::HISP)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HYSTW<'a> {
    w: &'a mut W,
}
impl<'a> _HYSTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Current Selection"]
    #[inline]
    pub fn isel(&self) -> ISELR {
        ISELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - Hysteresis Selection"]
    #[inline]
    pub fn hyst(&self) -> HYSTR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        HYSTR { bits }
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
    #[doc = "Bit 0 - Current Selection"]
    #[inline]
    pub fn isel(&mut self) -> _ISELW {
        _ISELW { w: self }
    }
    #[doc = "Bits 1:2 - Hysteresis Selection"]
    #[inline]
    pub fn hyst(&mut self) -> _HYSTW {
        _HYSTW { w: self }
    }
}
