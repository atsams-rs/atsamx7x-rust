#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWM_CMPV {
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
#[doc = r"Reader of the field"]
pub type CV_R = crate::FR<u32, u32>;
#[doc = r"Proxy"]
pub struct _CVW<'a> {
    w: &'a mut W,
}
impl<'a> _CVW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
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
impl crate::ToBits<bool> for CVMR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CVMR::COMPARE_AT_INCREMENT => false,
            CVMR::COMPARE_AT_DECREMENT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CVM_R = crate::FR<bool, CVMR>;
impl CVM_R {
    #[doc = "Checks if the value of the field is `COMPARE_AT_INCREMENT`"]
    #[inline(always)]
    pub fn is_compare_at_increment(&self) -> bool {
        *self == CVMR::COMPARE_AT_INCREMENT
    }
    #[doc = "Checks if the value of the field is `COMPARE_AT_DECREMENT`"]
    #[inline(always)]
    pub fn is_compare_at_decrement(&self) -> bool {
        *self == CVMR::COMPARE_AT_DECREMENT
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
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CVMW::COMPARE_AT_INCREMENT => false,
            CVMW::COMPARE_AT_DECREMENT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CVMW<'a> {
    w: &'a mut W,
}
impl<'a> _CVMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CVMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare when counter is incrementing"]
    #[inline(always)]
    pub fn compare_at_increment(self) -> &'a mut W {
        self.variant(CVMW::COMPARE_AT_INCREMENT)
    }
    #[doc = "Compare when counter is decrementing"]
    #[inline(always)]
    pub fn compare_at_decrement(self) -> &'a mut W {
        self.variant(CVMW::COMPARE_AT_DECREMENT)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new((self.bits() & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    pub fn cvm(&self) -> CVM_R {
        CVM_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    pub fn cv(&mut self) -> _CVW {
        _CVW { w: self }
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    pub fn cvm(&mut self) -> _CVMW {
        _CVMW { w: self }
    }
}
