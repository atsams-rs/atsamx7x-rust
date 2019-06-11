#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TC_CMR {
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
pub struct LDBSTOPR {
    bits: bool,
}
impl LDBSTOPR {
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
pub struct LDBDISR {
    bits: bool,
}
impl LDBDISR {
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
#[doc = "Possible values of the field `ETRGEDG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETRGEDGR {
    #[doc = "The clock is not gated by an external signal."]
    NONE,
    #[doc = "Rising edge"]
    RISING,
    #[doc = "Falling edge"]
    FALLING,
    #[doc = "Each edge"]
    EDGE,
}
impl ETRGEDGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ETRGEDGR::NONE => 0,
            ETRGEDGR::RISING => 1,
            ETRGEDGR::FALLING => 2,
            ETRGEDGR::EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ETRGEDGR {
        match value {
            0 => ETRGEDGR::NONE,
            1 => ETRGEDGR::RISING,
            2 => ETRGEDGR::FALLING,
            3 => ETRGEDGR::EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == ETRGEDGR::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == ETRGEDGR::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == ETRGEDGR::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline]
    pub fn is_edge(&self) -> bool {
        *self == ETRGEDGR::EDGE
    }
}
#[doc = r" Value of the field"]
pub struct ABETRGR {
    bits: bool,
}
impl ABETRGR {
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
pub struct CPCTRGR {
    bits: bool,
}
impl CPCTRGR {
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
#[doc = "Possible values of the field `LDRA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDRAR {
    #[doc = "None"]
    NONE,
    #[doc = "Rising edge of TIOAx"]
    RISING,
    #[doc = "Falling edge of TIOAx"]
    FALLING,
    #[doc = "Each edge of TIOAx"]
    EDGE,
}
impl LDRAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LDRAR::NONE => 0,
            LDRAR::RISING => 1,
            LDRAR::FALLING => 2,
            LDRAR::EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LDRAR {
        match value {
            0 => LDRAR::NONE,
            1 => LDRAR::RISING,
            2 => LDRAR::FALLING,
            3 => LDRAR::EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == LDRAR::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == LDRAR::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == LDRAR::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline]
    pub fn is_edge(&self) -> bool {
        *self == LDRAR::EDGE
    }
}
#[doc = "Possible values of the field `LDRB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDRBR {
    #[doc = "None"]
    NONE,
    #[doc = "Rising edge of TIOAx"]
    RISING,
    #[doc = "Falling edge of TIOAx"]
    FALLING,
    #[doc = "Each edge of TIOAx"]
    EDGE,
}
impl LDRBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LDRBR::NONE => 0,
            LDRBR::RISING => 1,
            LDRBR::FALLING => 2,
            LDRBR::EDGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LDRBR {
        match value {
            0 => LDRBR::NONE,
            1 => LDRBR::RISING,
            2 => LDRBR::FALLING,
            3 => LDRBR::EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == LDRBR::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline]
    pub fn is_rising(&self) -> bool {
        *self == LDRBR::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline]
    pub fn is_falling(&self) -> bool {
        *self == LDRBR::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline]
    pub fn is_edge(&self) -> bool {
        *self == LDRBR::EDGE
    }
}
#[doc = "Possible values of the field `SBSMPLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBSMPLRR {
    #[doc = "Load a Capture Register each selected edge"]
    ONE,
    #[doc = "Load a Capture Register every 2 selected edges"]
    HALF,
    #[doc = "Load a Capture Register every 4 selected edges"]
    FOURTH,
    #[doc = "Load a Capture Register every 8 selected edges"]
    EIGHTH,
    #[doc = "Load a Capture Register every 16 selected edges"]
    SIXTEENTH,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SBSMPLRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SBSMPLRR::ONE => 0,
            SBSMPLRR::HALF => 1,
            SBSMPLRR::FOURTH => 2,
            SBSMPLRR::EIGHTH => 3,
            SBSMPLRR::SIXTEENTH => 4,
            SBSMPLRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SBSMPLRR {
        match value {
            0 => SBSMPLRR::ONE,
            1 => SBSMPLRR::HALF,
            2 => SBSMPLRR::FOURTH,
            3 => SBSMPLRR::EIGHTH,
            4 => SBSMPLRR::SIXTEENTH,
            i => SBSMPLRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline]
    pub fn is_one(&self) -> bool {
        *self == SBSMPLRR::ONE
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline]
    pub fn is_half(&self) -> bool {
        *self == SBSMPLRR::HALF
    }
    #[doc = "Checks if the value of the field is `FOURTH`"]
    #[inline]
    pub fn is_fourth(&self) -> bool {
        *self == SBSMPLRR::FOURTH
    }
    #[doc = "Checks if the value of the field is `EIGHTH`"]
    #[inline]
    pub fn is_eighth(&self) -> bool {
        *self == SBSMPLRR::EIGHTH
    }
    #[doc = "Checks if the value of the field is `SIXTEENTH`"]
    #[inline]
    pub fn is_sixteenth(&self) -> bool {
        *self == SBSMPLRR::SIXTEENTH
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
pub struct _LDBSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _LDBSTOPW<'a> {
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
pub struct _LDBDISW<'a> {
    w: &'a mut W,
}
impl<'a> _LDBDISW<'a> {
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
#[doc = "Values that can be written to the field `ETRGEDG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETRGEDGW {
    #[doc = "The clock is not gated by an external signal."]
    NONE,
    #[doc = "Rising edge"]
    RISING,
    #[doc = "Falling edge"]
    FALLING,
    #[doc = "Each edge"]
    EDGE,
}
impl ETRGEDGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ETRGEDGW::NONE => 0,
            ETRGEDGW::RISING => 1,
            ETRGEDGW::FALLING => 2,
            ETRGEDGW::EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ETRGEDGW<'a> {
    w: &'a mut W,
}
impl<'a> _ETRGEDGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ETRGEDGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The clock is not gated by an external signal."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(ETRGEDGW::NONE)
    }
    #[doc = "Rising edge"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(ETRGEDGW::RISING)
    }
    #[doc = "Falling edge"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(ETRGEDGW::FALLING)
    }
    #[doc = "Each edge"]
    #[inline]
    pub fn edge(self) -> &'a mut W {
        self.variant(ETRGEDGW::EDGE)
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
#[doc = r" Proxy"]
pub struct _ABETRGW<'a> {
    w: &'a mut W,
}
impl<'a> _ABETRGW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CPCTRGW<'a> {
    w: &'a mut W,
}
impl<'a> _CPCTRGW<'a> {
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
        const OFFSET: u8 = 14;
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
#[doc = "Values that can be written to the field `LDRA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDRAW {
    #[doc = "None"]
    NONE,
    #[doc = "Rising edge of TIOAx"]
    RISING,
    #[doc = "Falling edge of TIOAx"]
    FALLING,
    #[doc = "Each edge of TIOAx"]
    EDGE,
}
impl LDRAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LDRAW::NONE => 0,
            LDRAW::RISING => 1,
            LDRAW::FALLING => 2,
            LDRAW::EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDRAW<'a> {
    w: &'a mut W,
}
impl<'a> _LDRAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDRAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "None"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(LDRAW::NONE)
    }
    #[doc = "Rising edge of TIOAx"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(LDRAW::RISING)
    }
    #[doc = "Falling edge of TIOAx"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(LDRAW::FALLING)
    }
    #[doc = "Each edge of TIOAx"]
    #[inline]
    pub fn edge(self) -> &'a mut W {
        self.variant(LDRAW::EDGE)
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
#[doc = "Values that can be written to the field `LDRB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDRBW {
    #[doc = "None"]
    NONE,
    #[doc = "Rising edge of TIOAx"]
    RISING,
    #[doc = "Falling edge of TIOAx"]
    FALLING,
    #[doc = "Each edge of TIOAx"]
    EDGE,
}
impl LDRBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LDRBW::NONE => 0,
            LDRBW::RISING => 1,
            LDRBW::FALLING => 2,
            LDRBW::EDGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDRBW<'a> {
    w: &'a mut W,
}
impl<'a> _LDRBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDRBW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "None"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(LDRBW::NONE)
    }
    #[doc = "Rising edge of TIOAx"]
    #[inline]
    pub fn rising(self) -> &'a mut W {
        self.variant(LDRBW::RISING)
    }
    #[doc = "Falling edge of TIOAx"]
    #[inline]
    pub fn falling(self) -> &'a mut W {
        self.variant(LDRBW::FALLING)
    }
    #[doc = "Each edge of TIOAx"]
    #[inline]
    pub fn edge(self) -> &'a mut W {
        self.variant(LDRBW::EDGE)
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
#[doc = "Values that can be written to the field `SBSMPLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBSMPLRW {
    #[doc = "Load a Capture Register each selected edge"]
    ONE,
    #[doc = "Load a Capture Register every 2 selected edges"]
    HALF,
    #[doc = "Load a Capture Register every 4 selected edges"]
    FOURTH,
    #[doc = "Load a Capture Register every 8 selected edges"]
    EIGHTH,
    #[doc = "Load a Capture Register every 16 selected edges"]
    SIXTEENTH,
}
impl SBSMPLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SBSMPLRW::ONE => 0,
            SBSMPLRW::HALF => 1,
            SBSMPLRW::FOURTH => 2,
            SBSMPLRW::EIGHTH => 3,
            SBSMPLRW::SIXTEENTH => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBSMPLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SBSMPLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBSMPLRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Load a Capture Register each selected edge"]
    #[inline]
    pub fn one(self) -> &'a mut W {
        self.variant(SBSMPLRW::ONE)
    }
    #[doc = "Load a Capture Register every 2 selected edges"]
    #[inline]
    pub fn half(self) -> &'a mut W {
        self.variant(SBSMPLRW::HALF)
    }
    #[doc = "Load a Capture Register every 4 selected edges"]
    #[inline]
    pub fn fourth(self) -> &'a mut W {
        self.variant(SBSMPLRW::FOURTH)
    }
    #[doc = "Load a Capture Register every 8 selected edges"]
    #[inline]
    pub fn eighth(self) -> &'a mut W {
        self.variant(SBSMPLRW::EIGHTH)
    }
    #[doc = "Load a Capture Register every 16 selected edges"]
    #[inline]
    pub fn sixteenth(self) -> &'a mut W {
        self.variant(SBSMPLRW::SIXTEENTH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline]
    pub fn ldbstop(&self) -> LDBSTOPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LDBSTOPR { bits }
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline]
    pub fn ldbdis(&self) -> LDBDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LDBDISR { bits }
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline]
    pub fn etrgedg(&self) -> ETRGEDGR {
        ETRGEDGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - TIOAx or TIOBx External Trigger Selection"]
    #[inline]
    pub fn abetrg(&self) -> ABETRGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ABETRGR { bits }
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline]
    pub fn cpctrg(&self) -> CPCTRGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CPCTRGR { bits }
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
    #[doc = "Bits 16:17 - RA Loading Edge Selection"]
    #[inline]
    pub fn ldra(&self) -> LDRAR {
        LDRAR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - RB Loading Edge Selection"]
    #[inline]
    pub fn ldrb(&self) -> LDRBR {
        LDRBR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:22 - Loading Edge Subsampling Ratio"]
    #[inline]
    pub fn sbsmplr(&self) -> SBSMPLRR {
        SBSMPLRR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
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
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline]
    pub fn ldbstop(&mut self) -> _LDBSTOPW {
        _LDBSTOPW { w: self }
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline]
    pub fn ldbdis(&mut self) -> _LDBDISW {
        _LDBDISW { w: self }
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline]
    pub fn etrgedg(&mut self) -> _ETRGEDGW {
        _ETRGEDGW { w: self }
    }
    #[doc = "Bit 10 - TIOAx or TIOBx External Trigger Selection"]
    #[inline]
    pub fn abetrg(&mut self) -> _ABETRGW {
        _ABETRGW { w: self }
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline]
    pub fn cpctrg(&mut self) -> _CPCTRGW {
        _CPCTRGW { w: self }
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline]
    pub fn wave(&mut self) -> _WAVEW {
        _WAVEW { w: self }
    }
    #[doc = "Bits 16:17 - RA Loading Edge Selection"]
    #[inline]
    pub fn ldra(&mut self) -> _LDRAW {
        _LDRAW { w: self }
    }
    #[doc = "Bits 18:19 - RB Loading Edge Selection"]
    #[inline]
    pub fn ldrb(&mut self) -> _LDRBW {
        _LDRBW { w: self }
    }
    #[doc = "Bits 20:22 - Loading Edge Subsampling Ratio"]
    #[inline]
    pub fn sbsmplr(&mut self) -> _SBSMPLRW {
        _SBSMPLRW { w: self }
    }
}
