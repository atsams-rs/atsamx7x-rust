#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CALIB {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type TENMS_R = crate::FR<u32, u32>;
#[doc = "Possible values of the field `SKEW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKEWR {
    #[doc = "10ms calibration value is exact"]
    VALUE_0,
    #[doc = "10ms calibration value is inexact, because of the clock frequency"]
    VALUE_1,
}
impl crate::ToBits<bool> for SKEWR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SKEWR::VALUE_0 => false,
            SKEWR::VALUE_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SKEW_R = crate::FR<bool, SKEWR>;
impl SKEW_R {
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == SKEWR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == SKEWR::VALUE_1
    }
}
#[doc = "Possible values of the field `NOREF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOREFR {
    #[doc = "The reference clock is provided"]
    VALUE_0,
    #[doc = "The reference clock is not provided"]
    VALUE_1,
}
impl crate::ToBits<bool> for NOREFR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            NOREFR::VALUE_0 => false,
            NOREFR::VALUE_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NOREF_R = crate::FR<bool, NOREFR>;
impl NOREF_R {
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == NOREFR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == NOREFR::VALUE_1
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:23 - Reload value to use for 10ms timing"]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new((self.bits() & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 30 - Indicates whether the TENMS value is exact"]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Indicates whether the device provides a reference clock to the processor"]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
