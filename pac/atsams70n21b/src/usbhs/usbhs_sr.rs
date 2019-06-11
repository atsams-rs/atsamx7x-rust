#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USBHS_SR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RDERRIR {
    bits: bool,
}
impl RDERRIR {
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
#[doc = "Possible values of the field `SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEEDR {
    #[doc = "Full-Speed mode"]
    FULL_SPEED,
    #[doc = "High-Speed mode"]
    HIGH_SPEED,
    #[doc = "Low-Speed mode"]
    LOW_SPEED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SPEEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SPEEDR::FULL_SPEED => 0,
            SPEEDR::HIGH_SPEED => 1,
            SPEEDR::LOW_SPEED => 2,
            SPEEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SPEEDR {
        match value {
            0 => SPEEDR::FULL_SPEED,
            1 => SPEEDR::HIGH_SPEED,
            2 => SPEEDR::LOW_SPEED,
            i => SPEEDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FULL_SPEED`"]
    #[inline]
    pub fn is_full_speed(&self) -> bool {
        *self == SPEEDR::FULL_SPEED
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED`"]
    #[inline]
    pub fn is_high_speed(&self) -> bool {
        *self == SPEEDR::HIGH_SPEED
    }
    #[doc = "Checks if the value of the field is `LOW_SPEED`"]
    #[inline]
    pub fn is_low_speed(&self) -> bool {
        *self == SPEEDR::LOW_SPEED
    }
}
#[doc = r" Value of the field"]
pub struct CLKUSABLER {
    bits: bool,
}
impl CLKUSABLER {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt (Host mode only)"]
    #[inline]
    pub fn rderri(&self) -> RDERRIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RDERRIR { bits }
    }
    #[doc = "Bits 12:13 - Speed Status (Device mode only)"]
    #[inline]
    pub fn speed(&self) -> SPEEDR {
        SPEEDR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - UTMI Clock Usable"]
    #[inline]
    pub fn clkusable(&self) -> CLKUSABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKUSABLER { bits }
    }
}
