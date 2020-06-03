#[doc = "Reader of register USBHS_SR"]
pub type R = crate::R<u32, super::USBHS_SR>;
#[doc = "Reader of field `RDERRI`"]
pub type RDERRI_R = crate::R<bool, bool>;
#[doc = "Speed Status (Device mode only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPEED_A {
    #[doc = "0: Full-Speed mode"]
    FULL_SPEED = 0,
    #[doc = "1: High-Speed mode"]
    HIGH_SPEED = 1,
    #[doc = "2: Low-Speed mode"]
    LOW_SPEED = 2,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPEED`"]
pub type SPEED_R = crate::R<u8, SPEED_A>;
impl SPEED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPEED_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SPEED_A::FULL_SPEED),
            1 => Val(SPEED_A::HIGH_SPEED),
            2 => Val(SPEED_A::LOW_SPEED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FULL_SPEED`"]
    #[inline(always)]
    pub fn is_full_speed(&self) -> bool {
        *self == SPEED_A::FULL_SPEED
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == SPEED_A::HIGH_SPEED
    }
    #[doc = "Checks if the value of the field is `LOW_SPEED`"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == SPEED_A::LOW_SPEED
    }
}
#[doc = "Reader of field `CLKUSABLE`"]
pub type CLKUSABLE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt (Host mode only)"]
    #[inline(always)]
    pub fn rderri(&self) -> RDERRI_R {
        RDERRI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Speed Status (Device mode only)"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - UTMI Clock Usable"]
    #[inline(always)]
    pub fn clkusable(&self) -> CLKUSABLE_R {
        CLKUSABLE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
