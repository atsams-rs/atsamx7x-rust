#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TC_CMR_WAVEFORM_MODE {
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
#[doc = "Possible values of the field `TCCLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCCLKSR {
    #[doc = "Clock selected: internal PCK6 clock signal (from PMC)"]
    TIMER_CLOCK1,
    #[doc = "Clock selected: internal MCK/8 clock signal (from PMC)"]
    TIMER_CLOCK2,
    #[doc = "Clock selected: internal MCK/32 clock signal (from PMC)"]
    TIMER_CLOCK3,
    #[doc = "Clock selected: internal MCK/128 clock signal (from PMC)"]
    TIMER_CLOCK4,
    #[doc = "Clock selected: internal SLCK clock signal (from PMC)"]
    TIMER_CLOCK5,
    #[doc = "Clock selected: XC0"]
    XC0,
    #[doc = "Clock selected: XC1"]
    XC1,
    #[doc = "Clock selected: XC2"]
    XC2,
}
impl TCCLKSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TCCLKSR::TIMER_CLOCK1 => 0,
            TCCLKSR::TIMER_CLOCK2 => 1,
            TCCLKSR::TIMER_CLOCK3 => 2,
            TCCLKSR::TIMER_CLOCK4 => 3,
            TCCLKSR::TIMER_CLOCK5 => 4,
            TCCLKSR::XC0 => 5,
            TCCLKSR::XC1 => 6,
            TCCLKSR::XC2 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TCCLKSR {
        match value {
            0 => TCCLKSR::TIMER_CLOCK1,
            1 => TCCLKSR::TIMER_CLOCK2,
            2 => TCCLKSR::TIMER_CLOCK3,
            3 => TCCLKSR::TIMER_CLOCK4,
            4 => TCCLKSR::TIMER_CLOCK5,
            5 => TCCLKSR::XC0,
            6 => TCCLKSR::XC1,
            7 => TCCLKSR::XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK1`"]
    #[inline]
    pub fn is_timer_clock1(&self) -> bool {
        *self == TCCLKSR::TIMER_CLOCK1
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK2`"]
    #[inline]
    pub fn is_timer_clock2(&self) -> bool {
        *self == TCCLKSR::TIMER_CLOCK2
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK3`"]
    #[inline]
    pub fn is_timer_clock3(&self) -> bool {
        *self == TCCLKSR::TIMER_CLOCK3
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK4`"]
    #[inline]
    pub fn is_timer_clock4(&self) -> bool {
        *self == TCCLKSR::TIMER_CLOCK4
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK5`"]
    #[inline]
    pub fn is_timer_clock5(&self) -> bool {
        *self == TCCLKSR::TIMER_CLOCK5
    }
    #[doc = "Checks if the value of the field is `XC0`"]
    #[inline]
    pub fn is_xc0(&self) -> bool {
        *self == TCCLKSR::XC0
    }
    #[doc = "Checks if the value of the field is `XC1`"]
    #[inline]
    pub fn is_xc1(&self) -> bool {
        *self == TCCLKSR::XC1
    }
    #[doc = "Checks if the value of the field is `XC2`"]
    #[inline]
    pub fn is_xc2(&self) -> bool {
        *self == TCCLKSR::XC2
    }
}
#[doc = r" Value of the field"]
pub struct CLKIR {
    bits: bool,
}
impl CLKIR {
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
#[doc = "Possible values of the field `BURST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTR {
    #[doc = "The clock is not gated by an external signal."]
    NONE,
    #[doc = "XC0 is ANDed with the selected clock."]
    XC0,
    #[doc = "XC1 is ANDed with the selected clock."]
    XC1,
    #[doc = "XC2 is ANDed with the selected clock."]
    XC2,
}
impl BURSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BURSTR::NONE => 0,
            BURSTR::XC0 => 1,
            BURSTR::XC1 => 2,
            BURSTR::XC2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BURSTR {
        match value {
            0 => BURSTR::NONE,
            1 => BURSTR::XC0,
            2 => BURSTR::XC1,
            3 => BURSTR::XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == BURSTR::NONE
    }
    #[doc = "Checks if the value of the field is `XC0`"]
    #[inline]
    pub fn is_xc0(&self) -> bool {
        *self == BURSTR::XC0
    }
    #[doc = "Checks if the value of the field is `XC1`"]
    #[inline]
    pub fn is_xc1(&self) -> bool {
        *self == BURSTR::XC1
    }
    #[doc = "Checks if the value of the field is `XC2`"]
    #[inline]
    pub fn is_xc2(&self) -> bool {
        *self == BURSTR::XC2
    }
}
#[doc = r" Value of the field"]
pub struct CPCSTOPR {
    bits: bool,
}
impl CPCSTOPR {
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
#[doc = r" Value of the field"]
pub struct CPCDISR {
    bits: bool,
}
impl CPCDISR {
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
#[doc = "Possible values of the field `EEVTEDG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEVTEDGR {
    #[doc = "None"]
    NONE,
    #[doc = "Rising edge"]
    RISING,
    #[doc = "Falling edge"]
    FALLING,
    #[doc = "Each edges"]
    EDGE,
}
impl EEVTEDGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EEVTEDGR::NONE => 0,
            EEVTEDGR::RISING => 1,
            EEVTEDGR::FALLING => 2,
            EEVTEDGR::EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EEVTEDGR {
        match value {
            0 => EEVTEDGR::NONE,
            1 => EEVTEDGR::RISING,
            2 => EEVTEDGR::FALLING,
            3 => EEVTEDGR::EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == EEVTEDGR::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == EEVTEDGR::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == EEVTEDGR::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline]
    pub fn is_edge(&self) -> bool {
        *self == EEVTEDGR::EDGE
    }
}
#[doc = "Possible values of the field `EEVT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEVTR {
    #[doc = "TIOB"]
    TIOB,
    #[doc = "XC0"]
    XC0,
    #[doc = "XC1"]
    XC1,
    #[doc = "XC2"]
    XC2,
}
impl EEVTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EEVTR::TIOB => 0,
            EEVTR::XC0 => 1,
            EEVTR::XC1 => 2,
            EEVTR::XC2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EEVTR {
        match value {
            0 => EEVTR::TIOB,
            1 => EEVTR::XC0,
            2 => EEVTR::XC1,
            3 => EEVTR::XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIOB`"]
    #[inline]
    pub fn is_tiob(&self) -> bool {
        *self == EEVTR::TIOB
    }
    #[doc = "Checks if the value of the field is `XC0`"]
    #[inline]
    pub fn is_xc0(&self) -> bool {
        *self == EEVTR::XC0
    }
    #[doc = "Checks if the value of the field is `XC1`"]
    #[inline]
    pub fn is_xc1(&self) -> bool {
        *self == EEVTR::XC1
    }
    #[doc = "Checks if the value of the field is `XC2`"]
    #[inline]
    pub fn is_xc2(&self) -> bool {
        *self == EEVTR::XC2
    }
}
#[doc = r" Value of the field"]
pub struct ENETRGR {
    bits: bool,
}
impl ENETRGR {
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
#[doc = "Possible values of the field `WAVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVSELR {
    #[doc = "UP mode without automatic trigger on RC Compare"]
    UP,
    #[doc = "UPDOWN mode without automatic trigger on RC Compare"]
    UPDOWN,
    #[doc = "UP mode with automatic trigger on RC Compare"]
    UP_RC,
    #[doc = "UPDOWN mode with automatic trigger on RC Compare"]
    UPDOWN_RC,
}
impl WAVSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WAVSELR::UP => 0,
            WAVSELR::UPDOWN => 1,
            WAVSELR::UP_RC => 2,
            WAVSELR::UPDOWN_RC => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WAVSELR {
        match value {
            0 => WAVSELR::UP,
            1 => WAVSELR::UPDOWN,
            2 => WAVSELR::UP_RC,
            3 => WAVSELR::UPDOWN_RC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline]
    pub fn is_up(&self) -> bool {
        *self == WAVSELR::UP
    }
    #[doc = "Checks if the value of the field is `UPDOWN`"]
    #[inline]
    pub fn is_updown(&self) -> bool {
        *self == WAVSELR::UPDOWN
    }
    #[doc = "Checks if the value of the field is `UP_RC`"]
    #[inline]
    pub fn is_up_rc(&self) -> bool {
        *self == WAVSELR::UP_RC
    }
    #[doc = "Checks if the value of the field is `UPDOWN_RC`"]
    #[inline]
    pub fn is_updown_rc(&self) -> bool {
        *self == WAVSELR::UPDOWN_RC
    }
}
#[doc = r" Value of the field"]
pub struct WAVER {
    bits: bool,
}
impl WAVER {
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
#[doc = "Possible values of the field `ACPA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACPAR {
    #[doc = "NONE"]
    NONE,
    #[doc = "SET"]
    SET,
    #[doc = "CLEAR"]
    CLEAR,
    #[doc = "TOGGLE"]
    TOGGLE,
}
impl ACPAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACPAR::NONE => 0,
            ACPAR::SET => 1,
            ACPAR::CLEAR => 2,
            ACPAR::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACPAR {
        match value {
            0 => ACPAR::NONE,
            1 => ACPAR::SET,
            2 => ACPAR::CLEAR,
            3 => ACPAR::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ACPAR::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == ACPAR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == ACPAR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == ACPAR::TOGGLE
    }
}
#[doc = "Possible values of the field `ACPC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACPCR {
    #[doc = "NONE"]
    NONE,
    #[doc = "SET"]
    SET,
    #[doc = "CLEAR"]
    CLEAR,
    #[doc = "TOGGLE"]
    TOGGLE,
}
impl ACPCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ACPCR::NONE => 0,
            ACPCR::SET => 1,
            ACPCR::CLEAR => 2,
            ACPCR::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ACPCR {
        match value {
            0 => ACPCR::NONE,
            1 => ACPCR::SET,
            2 => ACPCR::CLEAR,
            3 => ACPCR::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ACPCR::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == ACPCR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == ACPCR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == ACPCR::TOGGLE
    }
}
#[doc = "Possible values of the field `AEEVT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AEEVTR {
    #[doc = "NONE"]
    NONE,
    #[doc = "SET"]
    SET,
    #[doc = "CLEAR"]
    CLEAR,
    #[doc = "TOGGLE"]
    TOGGLE,
}
impl AEEVTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AEEVTR::NONE => 0,
            AEEVTR::SET => 1,
            AEEVTR::CLEAR => 2,
            AEEVTR::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AEEVTR {
        match value {
            0 => AEEVTR::NONE,
            1 => AEEVTR::SET,
            2 => AEEVTR::CLEAR,
            3 => AEEVTR::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == AEEVTR::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == AEEVTR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == AEEVTR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == AEEVTR::TOGGLE
    }
}
#[doc = "Possible values of the field `ASWTRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASWTRGR {
    #[doc = "NONE"]
    NONE,
    #[doc = "SET"]
    SET,
    #[doc = "CLEAR"]
    CLEAR,
    #[doc = "TOGGLE"]
    TOGGLE,
}
impl ASWTRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ASWTRGR::NONE => 0,
            ASWTRGR::SET => 1,
            ASWTRGR::CLEAR => 2,
            ASWTRGR::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ASWTRGR {
        match value {
            0 => ASWTRGR::NONE,
            1 => ASWTRGR::SET,
            2 => ASWTRGR::CLEAR,
            3 => ASWTRGR::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ASWTRGR::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == ASWTRGR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == ASWTRGR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == ASWTRGR::TOGGLE
    }
}
#[doc = "Possible values of the field `BCPB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCPBR {
    #[doc = "NONE"]
    NONE,
    #[doc = "SET"]
    SET,
    #[doc = "CLEAR"]
    CLEAR,
    #[doc = "TOGGLE"]
    TOGGLE,
}
impl BCPBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BCPBR::NONE => 0,
            BCPBR::SET => 1,
            BCPBR::CLEAR => 2,
            BCPBR::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BCPBR {
        match value {
            0 => BCPBR::NONE,
            1 => BCPBR::SET,
            2 => BCPBR::CLEAR,
            3 => BCPBR::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == BCPBR::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == BCPBR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == BCPBR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == BCPBR::TOGGLE
    }
}
#[doc = "Possible values of the field `BCPC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCPCR {
    #[doc = "NONE"]
    NONE,
    #[doc = "SET"]
    SET,
    #[doc = "CLEAR"]
    CLEAR,
    #[doc = "TOGGLE"]
    TOGGLE,
}
impl BCPCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BCPCR::NONE => 0,
            BCPCR::SET => 1,
            BCPCR::CLEAR => 2,
            BCPCR::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BCPCR {
        match value {
            0 => BCPCR::NONE,
            1 => BCPCR::SET,
            2 => BCPCR::CLEAR,
            3 => BCPCR::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == BCPCR::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == BCPCR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == BCPCR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == BCPCR::TOGGLE
    }
}
#[doc = "Possible values of the field `BEEVT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEEVTR {
    #[doc = "NONE"]
    NONE,
    #[doc = "SET"]
    SET,
    #[doc = "CLEAR"]
    CLEAR,
    #[doc = "TOGGLE"]
    TOGGLE,
}
impl BEEVTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BEEVTR::NONE => 0,
            BEEVTR::SET => 1,
            BEEVTR::CLEAR => 2,
            BEEVTR::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BEEVTR {
        match value {
            0 => BEEVTR::NONE,
            1 => BEEVTR::SET,
            2 => BEEVTR::CLEAR,
            3 => BEEVTR::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == BEEVTR::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == BEEVTR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == BEEVTR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == BEEVTR::TOGGLE
    }
}
#[doc = "Possible values of the field `BSWTRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSWTRGR {
    #[doc = "NONE"]
    NONE,
    #[doc = "SET"]
    SET,
    #[doc = "CLEAR"]
    CLEAR,
    #[doc = "TOGGLE"]
    TOGGLE,
}
impl BSWTRGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BSWTRGR::NONE => 0,
            BSWTRGR::SET => 1,
            BSWTRGR::CLEAR => 2,
            BSWTRGR::TOGGLE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BSWTRGR {
        match value {
            0 => BSWTRGR::NONE,
            1 => BSWTRGR::SET,
            2 => BSWTRGR::CLEAR,
            3 => BSWTRGR::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == BSWTRGR::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline]
    pub fn is_set(&self) -> bool {
        *self == BSWTRGR::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline]
    pub fn is_clear(&self) -> bool {
        *self == BSWTRGR::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline]
    pub fn is_toggle(&self) -> bool {
        *self == BSWTRGR::TOGGLE
    }
}
#[doc = "Values that can be written to the field `TCCLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCCLKSW {
    #[doc = "Clock selected: internal PCK6 clock signal (from PMC)"]
    TIMER_CLOCK1,
    #[doc = "Clock selected: internal MCK/8 clock signal (from PMC)"]
    TIMER_CLOCK2,
    #[doc = "Clock selected: internal MCK/32 clock signal (from PMC)"]
    TIMER_CLOCK3,
    #[doc = "Clock selected: internal MCK/128 clock signal (from PMC)"]
    TIMER_CLOCK4,
    #[doc = "Clock selected: internal SLCK clock signal (from PMC)"]
    TIMER_CLOCK5,
    #[doc = "Clock selected: XC0"]
    XC0,
    #[doc = "Clock selected: XC1"]
    XC1,
    #[doc = "Clock selected: XC2"]
    XC2,
}
impl TCCLKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TCCLKSW::TIMER_CLOCK1 => 0,
            TCCLKSW::TIMER_CLOCK2 => 1,
            TCCLKSW::TIMER_CLOCK3 => 2,
            TCCLKSW::TIMER_CLOCK4 => 3,
            TCCLKSW::TIMER_CLOCK5 => 4,
            TCCLKSW::XC0 => 5,
            TCCLKSW::XC1 => 6,
            TCCLKSW::XC2 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCCLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _TCCLKSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCCLKSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Clock selected: internal PCK6 clock signal (from PMC)"]
    #[inline]
    pub fn timer_clock1(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_CLOCK1)
    }
    #[doc = "Clock selected: internal MCK/8 clock signal (from PMC)"]
    #[inline]
    pub fn timer_clock2(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_CLOCK2)
    }
    #[doc = "Clock selected: internal MCK/32 clock signal (from PMC)"]
    #[inline]
    pub fn timer_clock3(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_CLOCK3)
    }
    #[doc = "Clock selected: internal MCK/128 clock signal (from PMC)"]
    #[inline]
    pub fn timer_clock4(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_CLOCK4)
    }
    #[doc = "Clock selected: internal SLCK clock signal (from PMC)"]
    #[inline]
    pub fn timer_clock5(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_CLOCK5)
    }
    #[doc = "Clock selected: XC0"]
    #[inline]
    pub fn xc0(self) -> &'a mut W {
        self.variant(TCCLKSW::XC0)
    }
    #[doc = "Clock selected: XC1"]
    #[inline]
    pub fn xc1(self) -> &'a mut W {
        self.variant(TCCLKSW::XC1)
    }
    #[doc = "Clock selected: XC2"]
    #[inline]
    pub fn xc2(self) -> &'a mut W {
        self.variant(TCCLKSW::XC2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKIW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKIW<'a> {
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BURST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURSTW {
    #[doc = "The clock is not gated by an external signal."]
    NONE,
    #[doc = "XC0 is ANDed with the selected clock."]
    XC0,
    #[doc = "XC1 is ANDed with the selected clock."]
    XC1,
    #[doc = "XC2 is ANDed with the selected clock."]
    XC2,
}
impl BURSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BURSTW::NONE => 0,
            BURSTW::XC0 => 1,
            BURSTW::XC1 => 2,
            BURSTW::XC2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BURSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BURSTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The clock is not gated by an external signal."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(BURSTW::NONE)
    }
    #[doc = "XC0 is ANDed with the selected clock."]
    #[inline]
    pub fn xc0(self) -> &'a mut W {
        self.variant(BURSTW::XC0)
    }
    #[doc = "XC1 is ANDed with the selected clock."]
    #[inline]
    pub fn xc1(self) -> &'a mut W {
        self.variant(BURSTW::XC1)
    }
    #[doc = "XC2 is ANDed with the selected clock."]
    #[inline]
    pub fn xc2(self) -> &'a mut W {
        self.variant(BURSTW::XC2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CPCSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _CPCSTOPW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CPCDISW<'a> {
    w: &'a mut W,
}
impl<'a> _CPCDISW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EEVTEDG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEVTEDGW {
    #[doc = "None"]
    NONE,
    #[doc = "Rising edge"]
    RISING,
    #[doc = "Falling edge"]
    FALLING,
    #[doc = "Each edges"]
    EDGE,
}
impl EEVTEDGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EEVTEDGW::NONE => 0,
            EEVTEDGW::RISING => 1,
            EEVTEDGW::FALLING => 2,
            EEVTEDGW::EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEVTEDGW<'a> {
    w: &'a mut W,
}
impl<'a> _EEVTEDGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEVTEDGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "None"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(EEVTEDGW::NONE)
    }
    #[doc = "Rising edge"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(EEVTEDGW::RISING)
    }
    #[doc = "Falling edge"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(EEVTEDGW::FALLING)
    }
    #[doc = "Each edges"]
    #[inline]
    pub fn edge(self) -> &'a mut W {
        self.variant(EEVTEDGW::EDGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EEVT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EEVTW {
    #[doc = "TIOB"]
    TIOB,
    #[doc = "XC0"]
    XC0,
    #[doc = "XC1"]
    XC1,
    #[doc = "XC2"]
    XC2,
}
impl EEVTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EEVTW::TIOB => 0,
            EEVTW::XC0 => 1,
            EEVTW::XC1 => 2,
            EEVTW::XC2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EEVTW<'a> {
    w: &'a mut W,
}
impl<'a> _EEVTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EEVTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "TIOB"]
    #[inline]
    pub fn tiob(self) -> &'a mut W {
        self.variant(EEVTW::TIOB)
    }
    #[doc = "XC0"]
    #[inline]
    pub fn xc0(self) -> &'a mut W {
        self.variant(EEVTW::XC0)
    }
    #[doc = "XC1"]
    #[inline]
    pub fn xc1(self) -> &'a mut W {
        self.variant(EEVTW::XC1)
    }
    #[doc = "XC2"]
    #[inline]
    pub fn xc2(self) -> &'a mut W {
        self.variant(EEVTW::XC2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENETRGW<'a> {
    w: &'a mut W,
}
impl<'a> _ENETRGW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WAVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVSELW {
    #[doc = "UP mode without automatic trigger on RC Compare"]
    UP,
    #[doc = "UPDOWN mode without automatic trigger on RC Compare"]
    UPDOWN,
    #[doc = "UP mode with automatic trigger on RC Compare"]
    UP_RC,
    #[doc = "UPDOWN mode with automatic trigger on RC Compare"]
    UPDOWN_RC,
}
impl WAVSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WAVSELW::UP => 0,
            WAVSELW::UPDOWN => 1,
            WAVSELW::UP_RC => 2,
            WAVSELW::UPDOWN_RC => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAVSELW<'a> {
    w: &'a mut W,
}
impl<'a> _WAVSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAVSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "UP mode without automatic trigger on RC Compare"]
    #[inline]
    pub fn up(self) -> &'a mut W {
        self.variant(WAVSELW::UP)
    }
    #[doc = "UPDOWN mode without automatic trigger on RC Compare"]
    #[inline]
    pub fn updown(self) -> &'a mut W {
        self.variant(WAVSELW::UPDOWN)
    }
    #[doc = "UP mode with automatic trigger on RC Compare"]
    #[inline]
    pub fn up_rc(self) -> &'a mut W {
        self.variant(WAVSELW::UP_RC)
    }
    #[doc = "UPDOWN mode with automatic trigger on RC Compare"]
    #[inline]
    pub fn updown_rc(self) -> &'a mut W {
        self.variant(WAVSELW::UPDOWN_RC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WAVEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAVEW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACPA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACPAW {
    #[doc = "NONE"]
    NONE,
    #[doc = "SET"]
    SET,
    #[doc = "CLEAR"]
    CLEAR,
    #[doc = "TOGGLE"]
    TOGGLE,
}
impl ACPAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACPAW::NONE => 0,
            ACPAW::SET => 1,
            ACPAW::CLEAR => 2,
            ACPAW::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACPAW<'a> {
    w: &'a mut W,
}
impl<'a> _ACPAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACPAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "NONE"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ACPAW::NONE)
    }
    #[doc = "SET"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ACPAW::SET)
    }
    #[doc = "CLEAR"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ACPAW::CLEAR)
    }
    #[doc = "TOGGLE"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ACPAW::TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACPC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACPCW {
    #[doc = "NONE"]
    NONE,
    #[doc = "SET"]
    SET,
    #[doc = "CLEAR"]
    CLEAR,
    #[doc = "TOGGLE"]
    TOGGLE,
}
impl ACPCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ACPCW::NONE => 0,
            ACPCW::SET => 1,
            ACPCW::CLEAR => 2,
            ACPCW::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACPCW<'a> {
    w: &'a mut W,
}
impl<'a> _ACPCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACPCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "NONE"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ACPCW::NONE)
    }
    #[doc = "SET"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ACPCW::SET)
    }
    #[doc = "CLEAR"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ACPCW::CLEAR)
    }
    #[doc = "TOGGLE"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ACPCW::TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AEEVT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AEEVTW {
    #[doc = "NONE"]
    NONE,
    #[doc = "SET"]
    SET,
    #[doc = "CLEAR"]
    CLEAR,
    #[doc = "TOGGLE"]
    TOGGLE,
}
impl AEEVTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AEEVTW::NONE => 0,
            AEEVTW::SET => 1,
            AEEVTW::CLEAR => 2,
            AEEVTW::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AEEVTW<'a> {
    w: &'a mut W,
}
impl<'a> _AEEVTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AEEVTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "NONE"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(AEEVTW::NONE)
    }
    #[doc = "SET"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(AEEVTW::SET)
    }
    #[doc = "CLEAR"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(AEEVTW::CLEAR)
    }
    #[doc = "TOGGLE"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(AEEVTW::TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASWTRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASWTRGW {
    #[doc = "NONE"]
    NONE,
    #[doc = "SET"]
    SET,
    #[doc = "CLEAR"]
    CLEAR,
    #[doc = "TOGGLE"]
    TOGGLE,
}
impl ASWTRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ASWTRGW::NONE => 0,
            ASWTRGW::SET => 1,
            ASWTRGW::CLEAR => 2,
            ASWTRGW::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASWTRGW<'a> {
    w: &'a mut W,
}
impl<'a> _ASWTRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASWTRGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "NONE"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ASWTRGW::NONE)
    }
    #[doc = "SET"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(ASWTRGW::SET)
    }
    #[doc = "CLEAR"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(ASWTRGW::CLEAR)
    }
    #[doc = "TOGGLE"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ASWTRGW::TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BCPB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCPBW {
    #[doc = "NONE"]
    NONE,
    #[doc = "SET"]
    SET,
    #[doc = "CLEAR"]
    CLEAR,
    #[doc = "TOGGLE"]
    TOGGLE,
}
impl BCPBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BCPBW::NONE => 0,
            BCPBW::SET => 1,
            BCPBW::CLEAR => 2,
            BCPBW::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCPBW<'a> {
    w: &'a mut W,
}
impl<'a> _BCPBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCPBW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "NONE"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(BCPBW::NONE)
    }
    #[doc = "SET"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(BCPBW::SET)
    }
    #[doc = "CLEAR"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCPBW::CLEAR)
    }
    #[doc = "TOGGLE"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BCPBW::TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BCPC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BCPCW {
    #[doc = "NONE"]
    NONE,
    #[doc = "SET"]
    SET,
    #[doc = "CLEAR"]
    CLEAR,
    #[doc = "TOGGLE"]
    TOGGLE,
}
impl BCPCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BCPCW::NONE => 0,
            BCPCW::SET => 1,
            BCPCW::CLEAR => 2,
            BCPCW::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BCPCW<'a> {
    w: &'a mut W,
}
impl<'a> _BCPCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BCPCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "NONE"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(BCPCW::NONE)
    }
    #[doc = "SET"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(BCPCW::SET)
    }
    #[doc = "CLEAR"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCPCW::CLEAR)
    }
    #[doc = "TOGGLE"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BCPCW::TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BEEVT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEEVTW {
    #[doc = "NONE"]
    NONE,
    #[doc = "SET"]
    SET,
    #[doc = "CLEAR"]
    CLEAR,
    #[doc = "TOGGLE"]
    TOGGLE,
}
impl BEEVTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BEEVTW::NONE => 0,
            BEEVTW::SET => 1,
            BEEVTW::CLEAR => 2,
            BEEVTW::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BEEVTW<'a> {
    w: &'a mut W,
}
impl<'a> _BEEVTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BEEVTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "NONE"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(BEEVTW::NONE)
    }
    #[doc = "SET"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(BEEVTW::SET)
    }
    #[doc = "CLEAR"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(BEEVTW::CLEAR)
    }
    #[doc = "TOGGLE"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BEEVTW::TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BSWTRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BSWTRGW {
    #[doc = "NONE"]
    NONE,
    #[doc = "SET"]
    SET,
    #[doc = "CLEAR"]
    CLEAR,
    #[doc = "TOGGLE"]
    TOGGLE,
}
impl BSWTRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BSWTRGW::NONE => 0,
            BSWTRGW::SET => 1,
            BSWTRGW::CLEAR => 2,
            BSWTRGW::TOGGLE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BSWTRGW<'a> {
    w: &'a mut W,
}
impl<'a> _BSWTRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BSWTRGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "NONE"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(BSWTRGW::NONE)
    }
    #[doc = "SET"]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(BSWTRGW::SET)
    }
    #[doc = "CLEAR"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(BSWTRGW::CLEAR)
    }
    #[doc = "TOGGLE"]
    #[inline]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BSWTRGW::TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline]
    pub fn tcclks(&self) -> TCCLKSR {
        TCCLKSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline]
    pub fn clki(&self) -> CLKIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKIR { bits }
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline]
    pub fn burst(&self) -> BURSTR {
        BURSTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RC Compare"]
    #[inline]
    pub fn cpcstop(&self) -> CPCSTOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CPCSTOPR { bits }
    }
    #[doc = "Bit 7 - Counter Clock Disable with RC Loading"]
    #[inline]
    pub fn cpcdis(&self) -> CPCDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CPCDISR { bits }
    }
    #[doc = "Bits 8:9 - External Event Edge Selection"]
    #[inline]
    pub fn eevtedg(&self) -> EEVTEDGR {
        EEVTEDGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - External Event Selection"]
    #[inline]
    pub fn eevt(&self) -> EEVTR {
        EEVTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - External Event Trigger Enable"]
    #[inline]
    pub fn enetrg(&self) -> ENETRGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENETRGR { bits }
    }
    #[doc = "Bits 13:14 - Waveform Selection"]
    #[inline]
    pub fn wavsel(&self) -> WAVSELR {
        WAVSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline]
    pub fn wave(&self) -> WAVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WAVER { bits }
    }
    #[doc = "Bits 16:17 - RA Compare Effect on TIOAx"]
    #[inline]
    pub fn acpa(&self) -> ACPAR {
        ACPAR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - RC Compare Effect on TIOAx"]
    #[inline]
    pub fn acpc(&self) -> ACPCR {
        ACPCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - External Event Effect on TIOAx"]
    #[inline]
    pub fn aeevt(&self) -> AEEVTR {
        AEEVTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Software Trigger Effect on TIOAx"]
    #[inline]
    pub fn aswtrg(&self) -> ASWTRGR {
        ASWTRGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - RB Compare Effect on TIOBx"]
    #[inline]
    pub fn bcpb(&self) -> BCPBR {
        BCPBR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - RC Compare Effect on TIOBx"]
    #[inline]
    pub fn bcpc(&self) -> BCPCR {
        BCPCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - External Event Effect on TIOBx"]
    #[inline]
    pub fn beevt(&self) -> BEEVTR {
        BEEVTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Software Trigger Effect on TIOBx"]
    #[inline]
    pub fn bswtrg(&self) -> BSWTRGR {
        BSWTRGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline]
    pub fn tcclks(&mut self) -> _TCCLKSW {
        _TCCLKSW { w: self }
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline]
    pub fn clki(&mut self) -> _CLKIW {
        _CLKIW { w: self }
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline]
    pub fn burst(&mut self) -> _BURSTW {
        _BURSTW { w: self }
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RC Compare"]
    #[inline]
    pub fn cpcstop(&mut self) -> _CPCSTOPW {
        _CPCSTOPW { w: self }
    }
    #[doc = "Bit 7 - Counter Clock Disable with RC Loading"]
    #[inline]
    pub fn cpcdis(&mut self) -> _CPCDISW {
        _CPCDISW { w: self }
    }
    #[doc = "Bits 8:9 - External Event Edge Selection"]
    #[inline]
    pub fn eevtedg(&mut self) -> _EEVTEDGW {
        _EEVTEDGW { w: self }
    }
    #[doc = "Bits 10:11 - External Event Selection"]
    #[inline]
    pub fn eevt(&mut self) -> _EEVTW {
        _EEVTW { w: self }
    }
    #[doc = "Bit 12 - External Event Trigger Enable"]
    #[inline]
    pub fn enetrg(&mut self) -> _ENETRGW {
        _ENETRGW { w: self }
    }
    #[doc = "Bits 13:14 - Waveform Selection"]
    #[inline]
    pub fn wavsel(&mut self) -> _WAVSELW {
        _WAVSELW { w: self }
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline]
    pub fn wave(&mut self) -> _WAVEW {
        _WAVEW { w: self }
    }
    #[doc = "Bits 16:17 - RA Compare Effect on TIOAx"]
    #[inline]
    pub fn acpa(&mut self) -> _ACPAW {
        _ACPAW { w: self }
    }
    #[doc = "Bits 18:19 - RC Compare Effect on TIOAx"]
    #[inline]
    pub fn acpc(&mut self) -> _ACPCW {
        _ACPCW { w: self }
    }
    #[doc = "Bits 20:21 - External Event Effect on TIOAx"]
    #[inline]
    pub fn aeevt(&mut self) -> _AEEVTW {
        _AEEVTW { w: self }
    }
    #[doc = "Bits 22:23 - Software Trigger Effect on TIOAx"]
    #[inline]
    pub fn aswtrg(&mut self) -> _ASWTRGW {
        _ASWTRGW { w: self }
    }
    #[doc = "Bits 24:25 - RB Compare Effect on TIOBx"]
    #[inline]
    pub fn bcpb(&mut self) -> _BCPBW {
        _BCPBW { w: self }
    }
    #[doc = "Bits 26:27 - RC Compare Effect on TIOBx"]
    #[inline]
    pub fn bcpc(&mut self) -> _BCPCW {
        _BCPCW { w: self }
    }
    #[doc = "Bits 28:29 - External Event Effect on TIOBx"]
    #[inline]
    pub fn beevt(&mut self) -> _BEEVTW {
        _BEEVTW { w: self }
    }
    #[doc = "Bits 30:31 - Software Trigger Effect on TIOBx"]
    #[inline]
    pub fn bswtrg(&mut self) -> _BSWTRGW {
        _BSWTRGW { w: self }
    }
}
