#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TC_CMR_CAPTURE_MODE {
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
impl crate::ToBits<u8> for TCCLKSR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
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
}
#[doc = r"Reader of the field"]
pub type TCCLKS_R = crate::FR<u8, TCCLKSR>;
impl TCCLKS_R {
    #[doc = "Checks if the value of the field is `TIMER_CLOCK1`"]
    #[inline(always)]
    pub fn is_timer_clock1(&self) -> bool {
        *self == TCCLKSR::TIMER_CLOCK1
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK2`"]
    #[inline(always)]
    pub fn is_timer_clock2(&self) -> bool {
        *self == TCCLKSR::TIMER_CLOCK2
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK3`"]
    #[inline(always)]
    pub fn is_timer_clock3(&self) -> bool {
        *self == TCCLKSR::TIMER_CLOCK3
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK4`"]
    #[inline(always)]
    pub fn is_timer_clock4(&self) -> bool {
        *self == TCCLKSR::TIMER_CLOCK4
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK5`"]
    #[inline(always)]
    pub fn is_timer_clock5(&self) -> bool {
        *self == TCCLKSR::TIMER_CLOCK5
    }
    #[doc = "Checks if the value of the field is `XC0`"]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == TCCLKSR::XC0
    }
    #[doc = "Checks if the value of the field is `XC1`"]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == TCCLKSR::XC1
    }
    #[doc = "Checks if the value of the field is `XC2`"]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == TCCLKSR::XC2
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
    #[inline(always)]
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
#[doc = r"Proxy"]
pub struct _TCCLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _TCCLKSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCCLKSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Clock selected: internal PCK6 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock1(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_CLOCK1)
    }
    #[doc = "Clock selected: internal MCK/8 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock2(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_CLOCK2)
    }
    #[doc = "Clock selected: internal MCK/32 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock3(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_CLOCK3)
    }
    #[doc = "Clock selected: internal MCK/128 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock4(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_CLOCK4)
    }
    #[doc = "Clock selected: internal SLCK clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock5(self) -> &'a mut W {
        self.variant(TCCLKSW::TIMER_CLOCK5)
    }
    #[doc = "Clock selected: XC0"]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut W {
        self.variant(TCCLKSW::XC0)
    }
    #[doc = "Clock selected: XC1"]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut W {
        self.variant(TCCLKSW::XC1)
    }
    #[doc = "Clock selected: XC2"]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut W {
        self.variant(TCCLKSW::XC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CLKI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CLKIW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKIW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
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
impl crate::ToBits<u8> for BURSTR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            BURSTR::NONE => 0,
            BURSTR::XC0 => 1,
            BURSTR::XC1 => 2,
            BURSTR::XC2 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BURST_R = crate::FR<u8, BURSTR>;
impl BURST_R {
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BURSTR::NONE
    }
    #[doc = "Checks if the value of the field is `XC0`"]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == BURSTR::XC0
    }
    #[doc = "Checks if the value of the field is `XC1`"]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == BURSTR::XC1
    }
    #[doc = "Checks if the value of the field is `XC2`"]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == BURSTR::XC2
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            BURSTW::NONE => 0,
            BURSTW::XC0 => 1,
            BURSTW::XC1 => 2,
            BURSTW::XC2 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BURSTW<'a> {
    w: &'a mut W,
}
impl<'a> _BURSTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURSTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BURSTW::NONE)
    }
    #[doc = "XC0 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut W {
        self.variant(BURSTW::XC0)
    }
    #[doc = "XC1 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut W {
        self.variant(BURSTW::XC1)
    }
    #[doc = "XC2 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut W {
        self.variant(BURSTW::XC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LDBSTOP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LDBSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _LDBSTOPW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LDBDIS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LDBDISW<'a> {
    w: &'a mut W,
}
impl<'a> _LDBDISW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
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
impl crate::ToBits<u8> for ETRGEDGR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ETRGEDGR::NONE => 0,
            ETRGEDGR::RISING => 1,
            ETRGEDGR::FALLING => 2,
            ETRGEDGR::EDGE => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ETRGEDG_R = crate::FR<u8, ETRGEDGR>;
impl ETRGEDG_R {
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ETRGEDGR::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ETRGEDGR::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ETRGEDGR::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ETRGEDGR::EDGE
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ETRGEDGW::NONE => 0,
            ETRGEDGW::RISING => 1,
            ETRGEDGW::FALLING => 2,
            ETRGEDGW::EDGE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ETRGEDGW<'a> {
    w: &'a mut W,
}
impl<'a> _ETRGEDGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETRGEDGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ETRGEDGW::NONE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(ETRGEDGW::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(ETRGEDGW::FALLING)
    }
    #[doc = "Each edge"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ETRGEDGW::EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ABETRG_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ABETRGW<'a> {
    w: &'a mut W,
}
impl<'a> _ABETRGW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CPCTRG_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CPCTRGW<'a> {
    w: &'a mut W,
}
impl<'a> _CPCTRGW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type WAVE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WAVEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAVEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
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
impl crate::ToBits<u8> for LDRAR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            LDRAR::NONE => 0,
            LDRAR::RISING => 1,
            LDRAR::FALLING => 2,
            LDRAR::EDGE => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LDRA_R = crate::FR<u8, LDRAR>;
impl LDRA_R {
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LDRAR::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == LDRAR::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == LDRAR::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == LDRAR::EDGE
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            LDRAW::NONE => 0,
            LDRAW::RISING => 1,
            LDRAW::FALLING => 2,
            LDRAW::EDGE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LDRAW<'a> {
    w: &'a mut W,
}
impl<'a> _LDRAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDRAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(LDRAW::NONE)
    }
    #[doc = "Rising edge of TIOAx"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(LDRAW::RISING)
    }
    #[doc = "Falling edge of TIOAx"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(LDRAW::FALLING)
    }
    #[doc = "Each edge of TIOAx"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(LDRAW::EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
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
impl crate::ToBits<u8> for LDRBR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            LDRBR::NONE => 0,
            LDRBR::RISING => 1,
            LDRBR::FALLING => 2,
            LDRBR::EDGE => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LDRB_R = crate::FR<u8, LDRBR>;
impl LDRB_R {
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LDRBR::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == LDRBR::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == LDRBR::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == LDRBR::EDGE
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            LDRBW::NONE => 0,
            LDRBW::RISING => 1,
            LDRBW::FALLING => 2,
            LDRBW::EDGE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LDRBW<'a> {
    w: &'a mut W,
}
impl<'a> _LDRBW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDRBW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(LDRBW::NONE)
    }
    #[doc = "Rising edge of TIOAx"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(LDRBW::RISING)
    }
    #[doc = "Falling edge of TIOAx"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(LDRBW::FALLING)
    }
    #[doc = "Each edge of TIOAx"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(LDRBW::EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
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
}
impl crate::ToBits<u8> for SBSMPLRR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SBSMPLRR::ONE => 0,
            SBSMPLRR::HALF => 1,
            SBSMPLRR::FOURTH => 2,
            SBSMPLRR::EIGHTH => 3,
            SBSMPLRR::SIXTEENTH => 4,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SBSMPLR_R = crate::FR<u8, SBSMPLRR>;
impl SBSMPLR_R {
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == SBSMPLRR::ONE
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == SBSMPLRR::HALF
    }
    #[doc = "Checks if the value of the field is `FOURTH`"]
    #[inline(always)]
    pub fn is_fourth(&self) -> bool {
        *self == SBSMPLRR::FOURTH
    }
    #[doc = "Checks if the value of the field is `EIGHTH`"]
    #[inline(always)]
    pub fn is_eighth(&self) -> bool {
        *self == SBSMPLRR::EIGHTH
    }
    #[doc = "Checks if the value of the field is `SIXTEENTH`"]
    #[inline(always)]
    pub fn is_sixteenth(&self) -> bool {
        *self == SBSMPLRR::SIXTEENTH
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
    #[inline(always)]
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
#[doc = r"Proxy"]
pub struct _SBSMPLRW<'a> {
    w: &'a mut W,
}
impl<'a> _SBSMPLRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBSMPLRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Load a Capture Register each selected edge"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(SBSMPLRW::ONE)
    }
    #[doc = "Load a Capture Register every 2 selected edges"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(SBSMPLRW::HALF)
    }
    #[doc = "Load a Capture Register every 4 selected edges"]
    #[inline(always)]
    pub fn fourth(self) -> &'a mut W {
        self.variant(SBSMPLRW::FOURTH)
    }
    #[doc = "Load a Capture Register every 8 selected edges"]
    #[inline(always)]
    pub fn eighth(self) -> &'a mut W {
        self.variant(SBSMPLRW::EIGHTH)
    }
    #[doc = "Load a Capture Register every 16 selected edges"]
    #[inline(always)]
    pub fn sixteenth(self) -> &'a mut W {
        self.variant(SBSMPLRW::SIXTEENTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    pub fn tcclks(&self) -> TCCLKS_R {
        TCCLKS_R::new((self.bits() & 0x07) as u8)
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    pub fn clki(&self) -> CLKI_R {
        CLKI_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline(always)]
    pub fn ldbstop(&self) -> LDBSTOP_R {
        LDBSTOP_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline(always)]
    pub fn ldbdis(&self) -> LDBDIS_R {
        LDBDIS_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline(always)]
    pub fn etrgedg(&self) -> ETRGEDG_R {
        ETRGEDG_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - TIOAx or TIOBx External Trigger Selection"]
    #[inline(always)]
    pub fn abetrg(&self) -> ABETRG_R {
        ABETRG_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline(always)]
    pub fn cpctrg(&self) -> CPCTRG_R {
        CPCTRG_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - RA Loading Edge Selection"]
    #[inline(always)]
    pub fn ldra(&self) -> LDRA_R {
        LDRA_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - RB Loading Edge Selection"]
    #[inline(always)]
    pub fn ldrb(&self) -> LDRB_R {
        LDRB_R::new(((self.bits() >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:22 - Loading Edge Subsampling Ratio"]
    #[inline(always)]
    pub fn sbsmplr(&self) -> SBSMPLR_R {
        SBSMPLR_R::new(((self.bits() >> 20) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    pub fn tcclks(&mut self) -> _TCCLKSW {
        _TCCLKSW { w: self }
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    pub fn clki(&mut self) -> _CLKIW {
        _CLKIW { w: self }
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    pub fn burst(&mut self) -> _BURSTW {
        _BURSTW { w: self }
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline(always)]
    pub fn ldbstop(&mut self) -> _LDBSTOPW {
        _LDBSTOPW { w: self }
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline(always)]
    pub fn ldbdis(&mut self) -> _LDBDISW {
        _LDBDISW { w: self }
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline(always)]
    pub fn etrgedg(&mut self) -> _ETRGEDGW {
        _ETRGEDGW { w: self }
    }
    #[doc = "Bit 10 - TIOAx or TIOBx External Trigger Selection"]
    #[inline(always)]
    pub fn abetrg(&mut self) -> _ABETRGW {
        _ABETRGW { w: self }
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline(always)]
    pub fn cpctrg(&mut self) -> _CPCTRGW {
        _CPCTRGW { w: self }
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    pub fn wave(&mut self) -> _WAVEW {
        _WAVEW { w: self }
    }
    #[doc = "Bits 16:17 - RA Loading Edge Selection"]
    #[inline(always)]
    pub fn ldra(&mut self) -> _LDRAW {
        _LDRAW { w: self }
    }
    #[doc = "Bits 18:19 - RB Loading Edge Selection"]
    #[inline(always)]
    pub fn ldrb(&mut self) -> _LDRBW {
        _LDRBW { w: self }
    }
    #[doc = "Bits 20:22 - Loading Edge Subsampling Ratio"]
    #[inline(always)]
    pub fn sbsmplr(&mut self) -> _SBSMPLRW {
        _SBSMPLRW { w: self }
    }
}
