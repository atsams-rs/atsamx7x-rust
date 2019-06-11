#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CALIB {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TENMSR {
    bits: u32,
}
impl TENMSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `SKEW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKEWR {
    #[doc = "10ms calibration value is exact"]
    VALUE_0,
    #[doc = "10ms calibration value is inexact, because of the clock frequency"]
    VALUE_1,
}
impl SKEWR {
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
            SKEWR::VALUE_0 => false,
            SKEWR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SKEWR {
        match value {
            false => SKEWR::VALUE_0,
            true => SKEWR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == SKEWR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
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
impl NOREFR {
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
            NOREFR::VALUE_0 => false,
            NOREFR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NOREFR {
        match value {
            false => NOREFR::VALUE_0,
            true => NOREFR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == NOREFR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == NOREFR::VALUE_1
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:23 - Reload value to use for 10ms timing"]
    #[inline]
    pub fn tenms(&self) -> TENMSR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        TENMSR { bits }
    }
    #[doc = "Bit 30 - Indicates whether the TENMS value is exact"]
    #[inline]
    pub fn skew(&self) -> SKEWR {
        SKEWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Indicates whether the device provides a reference clock to the processor"]
    #[inline]
    pub fn noref(&self) -> NOREFR {
        NOREFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
