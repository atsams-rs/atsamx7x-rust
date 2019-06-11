#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWM_CMPV {
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
pub struct CVR {
    bits: u32,
}
impl CVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `CVM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CVMR {
    #[doc = "Compare when counter is incrementing"]
    COMPARE_AT_INCREMENT,
    #[doc = "Compare when counter is decrementing"]
    COMPARE_AT_DECREMENT,
}
impl CVMR {
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
            CVMR::COMPARE_AT_INCREMENT => false,
            CVMR::COMPARE_AT_DECREMENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CVMR {
        match value {
            false => CVMR::COMPARE_AT_INCREMENT,
            true => CVMR::COMPARE_AT_DECREMENT,
        }
    }
    #[doc = "Checks if the value of the field is `COMPARE_AT_INCREMENT`"]
    #[inline]
    pub fn is_compare_at_increment(&self) -> bool {
        *self == CVMR::COMPARE_AT_INCREMENT
    }
    #[doc = "Checks if the value of the field is `COMPARE_AT_DECREMENT`"]
    #[inline]
    pub fn is_compare_at_decrement(&self) -> bool {
        *self == CVMR::COMPARE_AT_DECREMENT
    }
}
#[doc = r" Proxy"]
pub struct _CVW<'a> {
    w: &'a mut W,
}
impl<'a> _CVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CVM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CVMW {
    #[doc = "Compare when counter is incrementing"]
    COMPARE_AT_INCREMENT,
    #[doc = "Compare when counter is decrementing"]
    COMPARE_AT_DECREMENT,
}
impl CVMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CVMW::COMPARE_AT_INCREMENT => false,
            CVMW::COMPARE_AT_DECREMENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CVMW<'a> {
    w: &'a mut W,
}
impl<'a> _CVMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CVMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare when counter is incrementing"]
    #[inline]
    pub fn compare_at_increment(self) -> &'a mut W {
        self.variant(CVMW::COMPARE_AT_INCREMENT)
    }
    #[doc = "Compare when counter is decrementing"]
    #[inline]
    pub fn compare_at_decrement(self) -> &'a mut W {
        self.variant(CVMW::COMPARE_AT_DECREMENT)
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
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline]
    pub fn cv(&self) -> CVR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        CVR { bits }
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline]
    pub fn cvm(&self) -> CVMR {
        CVMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline]
    pub fn cv(&mut self) -> _CVW {
        _CVW { w: self }
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline]
    pub fn cvm(&mut self) -> _CVMW {
        _CVMW { w: self }
    }
}
