#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USBHS_SR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type RDERRI_R = crate::FR<bool, bool>;
#[doc = "Possible values of the field `SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPEEDR {
    #[doc = "Full-Speed mode"]
    FULL_SPEED,
    #[doc = "High-Speed mode"]
    HIGH_SPEED,
    #[doc = "Low-Speed mode"]
    LOW_SPEED,
}
impl crate::ToBits<u8> for SPEEDR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SPEEDR::FULL_SPEED => 0,
            SPEEDR::HIGH_SPEED => 1,
            SPEEDR::LOW_SPEED => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SPEED_R = crate::FR<u8, SPEEDR>;
impl SPEED_R {
    #[doc = "Checks if the value of the field is `FULL_SPEED`"]
    #[inline(always)]
    pub fn is_full_speed(&self) -> bool {
        *self == SPEEDR::FULL_SPEED
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == SPEEDR::HIGH_SPEED
    }
    #[doc = "Checks if the value of the field is `LOW_SPEED`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == SPEEDR::LOW_SPEED
    }
}
#[doc = r"Reader of the field"]
pub type CLKUSABLE_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt (Host mode only)"]
    #[inline(always)]
    pub fn rderri(&self) -> RDERRI_R {
        RDERRI_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Speed Status (Device mode only)"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits() >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - UTMI Clock Usable"]
    #[inline(always)]
    pub fn clkusable(&self) -> CLKUSABLE_R {
        CLKUSABLE_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
}
