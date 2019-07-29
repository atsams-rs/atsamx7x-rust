#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RTC_MR {
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
pub type HRMOD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _HRMODW<'a> {
    w: &'a mut W,
}
impl<'a> _HRMODW<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PERSIAN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PERSIANW<'a> {
    w: &'a mut W,
}
impl<'a> _PERSIANW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type NEGPPM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _NEGPPMW<'a> {
    w: &'a mut W,
}
impl<'a> _NEGPPMW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CORRECTION_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CORRECTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _CORRECTIONW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type HIGHPPM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _HIGHPPMW<'a> {
    w: &'a mut W,
}
impl<'a> _HIGHPPMW<'a> {
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
#[doc = "Possible values of the field `OUT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT0R {
    #[doc = "No waveform, stuck at '0'"]
    NO_WAVE,
    #[doc = "1 Hz square wave"]
    FREQ1HZ,
    #[doc = "32 Hz square wave"]
    FREQ32HZ,
    #[doc = "64 Hz square wave"]
    FREQ64HZ,
    #[doc = "512 Hz square wave"]
    FREQ512HZ,
    #[doc = "Output toggles when alarm flag rises"]
    ALARM_TOGGLE,
    #[doc = "Output is a copy of the alarm flag"]
    ALARM_FLAG,
    #[doc = "Duty cycle programmable pulse"]
    PROG_PULSE,
}
impl crate::ToBits<u8> for OUT0R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            OUT0R::NO_WAVE => 0,
            OUT0R::FREQ1HZ => 1,
            OUT0R::FREQ32HZ => 2,
            OUT0R::FREQ64HZ => 3,
            OUT0R::FREQ512HZ => 4,
            OUT0R::ALARM_TOGGLE => 5,
            OUT0R::ALARM_FLAG => 6,
            OUT0R::PROG_PULSE => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OUT0_R = crate::FR<u8, OUT0R>;
impl OUT0_R {
    #[doc = "Checks if the value of the field is `NO_WAVE`"]
    #[inline(always)]
    pub fn is_no_wave(&self) -> bool {
        *self == OUT0R::NO_WAVE
    }
    #[doc = "Checks if the value of the field is `FREQ1HZ`"]
    #[inline(always)]
    pub fn is_freq1hz(&self) -> bool {
        *self == OUT0R::FREQ1HZ
    }
    #[doc = "Checks if the value of the field is `FREQ32HZ`"]
    #[inline(always)]
    pub fn is_freq32hz(&self) -> bool {
        *self == OUT0R::FREQ32HZ
    }
    #[doc = "Checks if the value of the field is `FREQ64HZ`"]
    #[inline(always)]
    pub fn is_freq64hz(&self) -> bool {
        *self == OUT0R::FREQ64HZ
    }
    #[doc = "Checks if the value of the field is `FREQ512HZ`"]
    #[inline(always)]
    pub fn is_freq512hz(&self) -> bool {
        *self == OUT0R::FREQ512HZ
    }
    #[doc = "Checks if the value of the field is `ALARM_TOGGLE`"]
    #[inline(always)]
    pub fn is_alarm_toggle(&self) -> bool {
        *self == OUT0R::ALARM_TOGGLE
    }
    #[doc = "Checks if the value of the field is `ALARM_FLAG`"]
    #[inline(always)]
    pub fn is_alarm_flag(&self) -> bool {
        *self == OUT0R::ALARM_FLAG
    }
    #[doc = "Checks if the value of the field is `PROG_PULSE`"]
    #[inline(always)]
    pub fn is_prog_pulse(&self) -> bool {
        *self == OUT0R::PROG_PULSE
    }
}
#[doc = "Values that can be written to the field `OUT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT0W {
    #[doc = "No waveform, stuck at '0'"]
    NO_WAVE,
    #[doc = "1 Hz square wave"]
    FREQ1HZ,
    #[doc = "32 Hz square wave"]
    FREQ32HZ,
    #[doc = "64 Hz square wave"]
    FREQ64HZ,
    #[doc = "512 Hz square wave"]
    FREQ512HZ,
    #[doc = "Output toggles when alarm flag rises"]
    ALARM_TOGGLE,
    #[doc = "Output is a copy of the alarm flag"]
    ALARM_FLAG,
    #[doc = "Duty cycle programmable pulse"]
    PROG_PULSE,
}
impl OUT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            OUT0W::NO_WAVE => 0,
            OUT0W::FREQ1HZ => 1,
            OUT0W::FREQ32HZ => 2,
            OUT0W::FREQ64HZ => 3,
            OUT0W::FREQ512HZ => 4,
            OUT0W::ALARM_TOGGLE => 5,
            OUT0W::ALARM_FLAG => 6,
            OUT0W::PROG_PULSE => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _OUT0W<'a> {
    w: &'a mut W,
}
impl<'a> _OUT0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No waveform, stuck at '0'"]
    #[inline(always)]
    pub fn no_wave(self) -> &'a mut W {
        self.variant(OUT0W::NO_WAVE)
    }
    #[doc = "1 Hz square wave"]
    #[inline(always)]
    pub fn freq1hz(self) -> &'a mut W {
        self.variant(OUT0W::FREQ1HZ)
    }
    #[doc = "32 Hz square wave"]
    #[inline(always)]
    pub fn freq32hz(self) -> &'a mut W {
        self.variant(OUT0W::FREQ32HZ)
    }
    #[doc = "64 Hz square wave"]
    #[inline(always)]
    pub fn freq64hz(self) -> &'a mut W {
        self.variant(OUT0W::FREQ64HZ)
    }
    #[doc = "512 Hz square wave"]
    #[inline(always)]
    pub fn freq512hz(self) -> &'a mut W {
        self.variant(OUT0W::FREQ512HZ)
    }
    #[doc = "Output toggles when alarm flag rises"]
    #[inline(always)]
    pub fn alarm_toggle(self) -> &'a mut W {
        self.variant(OUT0W::ALARM_TOGGLE)
    }
    #[doc = "Output is a copy of the alarm flag"]
    #[inline(always)]
    pub fn alarm_flag(self) -> &'a mut W {
        self.variant(OUT0W::ALARM_FLAG)
    }
    #[doc = "Duty cycle programmable pulse"]
    #[inline(always)]
    pub fn prog_pulse(self) -> &'a mut W {
        self.variant(OUT0W::PROG_PULSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `OUT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT1R {
    #[doc = "No waveform, stuck at '0'"]
    NO_WAVE,
    #[doc = "1 Hz square wave"]
    FREQ1HZ,
    #[doc = "32 Hz square wave"]
    FREQ32HZ,
    #[doc = "64 Hz square wave"]
    FREQ64HZ,
    #[doc = "512 Hz square wave"]
    FREQ512HZ,
    #[doc = "Output toggles when alarm flag rises"]
    ALARM_TOGGLE,
    #[doc = "Output is a copy of the alarm flag"]
    ALARM_FLAG,
    #[doc = "Duty cycle programmable pulse"]
    PROG_PULSE,
}
impl crate::ToBits<u8> for OUT1R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            OUT1R::NO_WAVE => 0,
            OUT1R::FREQ1HZ => 1,
            OUT1R::FREQ32HZ => 2,
            OUT1R::FREQ64HZ => 3,
            OUT1R::FREQ512HZ => 4,
            OUT1R::ALARM_TOGGLE => 5,
            OUT1R::ALARM_FLAG => 6,
            OUT1R::PROG_PULSE => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OUT1_R = crate::FR<u8, OUT1R>;
impl OUT1_R {
    #[doc = "Checks if the value of the field is `NO_WAVE`"]
    #[inline(always)]
    pub fn is_no_wave(&self) -> bool {
        *self == OUT1R::NO_WAVE
    }
    #[doc = "Checks if the value of the field is `FREQ1HZ`"]
    #[inline(always)]
    pub fn is_freq1hz(&self) -> bool {
        *self == OUT1R::FREQ1HZ
    }
    #[doc = "Checks if the value of the field is `FREQ32HZ`"]
    #[inline(always)]
    pub fn is_freq32hz(&self) -> bool {
        *self == OUT1R::FREQ32HZ
    }
    #[doc = "Checks if the value of the field is `FREQ64HZ`"]
    #[inline(always)]
    pub fn is_freq64hz(&self) -> bool {
        *self == OUT1R::FREQ64HZ
    }
    #[doc = "Checks if the value of the field is `FREQ512HZ`"]
    #[inline(always)]
    pub fn is_freq512hz(&self) -> bool {
        *self == OUT1R::FREQ512HZ
    }
    #[doc = "Checks if the value of the field is `ALARM_TOGGLE`"]
    #[inline(always)]
    pub fn is_alarm_toggle(&self) -> bool {
        *self == OUT1R::ALARM_TOGGLE
    }
    #[doc = "Checks if the value of the field is `ALARM_FLAG`"]
    #[inline(always)]
    pub fn is_alarm_flag(&self) -> bool {
        *self == OUT1R::ALARM_FLAG
    }
    #[doc = "Checks if the value of the field is `PROG_PULSE`"]
    #[inline(always)]
    pub fn is_prog_pulse(&self) -> bool {
        *self == OUT1R::PROG_PULSE
    }
}
#[doc = "Values that can be written to the field `OUT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUT1W {
    #[doc = "No waveform, stuck at '0'"]
    NO_WAVE,
    #[doc = "1 Hz square wave"]
    FREQ1HZ,
    #[doc = "32 Hz square wave"]
    FREQ32HZ,
    #[doc = "64 Hz square wave"]
    FREQ64HZ,
    #[doc = "512 Hz square wave"]
    FREQ512HZ,
    #[doc = "Output toggles when alarm flag rises"]
    ALARM_TOGGLE,
    #[doc = "Output is a copy of the alarm flag"]
    ALARM_FLAG,
    #[doc = "Duty cycle programmable pulse"]
    PROG_PULSE,
}
impl OUT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            OUT1W::NO_WAVE => 0,
            OUT1W::FREQ1HZ => 1,
            OUT1W::FREQ32HZ => 2,
            OUT1W::FREQ64HZ => 3,
            OUT1W::FREQ512HZ => 4,
            OUT1W::ALARM_TOGGLE => 5,
            OUT1W::ALARM_FLAG => 6,
            OUT1W::PROG_PULSE => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _OUT1W<'a> {
    w: &'a mut W,
}
impl<'a> _OUT1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No waveform, stuck at '0'"]
    #[inline(always)]
    pub fn no_wave(self) -> &'a mut W {
        self.variant(OUT1W::NO_WAVE)
    }
    #[doc = "1 Hz square wave"]
    #[inline(always)]
    pub fn freq1hz(self) -> &'a mut W {
        self.variant(OUT1W::FREQ1HZ)
    }
    #[doc = "32 Hz square wave"]
    #[inline(always)]
    pub fn freq32hz(self) -> &'a mut W {
        self.variant(OUT1W::FREQ32HZ)
    }
    #[doc = "64 Hz square wave"]
    #[inline(always)]
    pub fn freq64hz(self) -> &'a mut W {
        self.variant(OUT1W::FREQ64HZ)
    }
    #[doc = "512 Hz square wave"]
    #[inline(always)]
    pub fn freq512hz(self) -> &'a mut W {
        self.variant(OUT1W::FREQ512HZ)
    }
    #[doc = "Output toggles when alarm flag rises"]
    #[inline(always)]
    pub fn alarm_toggle(self) -> &'a mut W {
        self.variant(OUT1W::ALARM_TOGGLE)
    }
    #[doc = "Output is a copy of the alarm flag"]
    #[inline(always)]
    pub fn alarm_flag(self) -> &'a mut W {
        self.variant(OUT1W::ALARM_FLAG)
    }
    #[doc = "Duty cycle programmable pulse"]
    #[inline(always)]
    pub fn prog_pulse(self) -> &'a mut W {
        self.variant(OUT1W::PROG_PULSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `THIGH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THIGHR {
    #[doc = "31.2 ms"]
    H_31MS,
    #[doc = "15.6 ms"]
    H_16MS,
    #[doc = "3.91 ms"]
    H_4MS,
    #[doc = "976 us"]
    H_976US,
    #[doc = "488 us"]
    H_488US,
    #[doc = "122 us"]
    H_122US,
    #[doc = "30.5 us"]
    H_30US,
    #[doc = "15.2 us"]
    H_15US,
}
impl crate::ToBits<u8> for THIGHR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            THIGHR::H_31MS => 0,
            THIGHR::H_16MS => 1,
            THIGHR::H_4MS => 2,
            THIGHR::H_976US => 3,
            THIGHR::H_488US => 4,
            THIGHR::H_122US => 5,
            THIGHR::H_30US => 6,
            THIGHR::H_15US => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type THIGH_R = crate::FR<u8, THIGHR>;
impl THIGH_R {
    #[doc = "Checks if the value of the field is `H_31MS`"]
    #[inline(always)]
    pub fn is_h_31ms(&self) -> bool {
        *self == THIGHR::H_31MS
    }
    #[doc = "Checks if the value of the field is `H_16MS`"]
    #[inline(always)]
    pub fn is_h_16ms(&self) -> bool {
        *self == THIGHR::H_16MS
    }
    #[doc = "Checks if the value of the field is `H_4MS`"]
    #[inline(always)]
    pub fn is_h_4ms(&self) -> bool {
        *self == THIGHR::H_4MS
    }
    #[doc = "Checks if the value of the field is `H_976US`"]
    #[inline(always)]
    pub fn is_h_976us(&self) -> bool {
        *self == THIGHR::H_976US
    }
    #[doc = "Checks if the value of the field is `H_488US`"]
    #[inline(always)]
    pub fn is_h_488us(&self) -> bool {
        *self == THIGHR::H_488US
    }
    #[doc = "Checks if the value of the field is `H_122US`"]
    #[inline(always)]
    pub fn is_h_122us(&self) -> bool {
        *self == THIGHR::H_122US
    }
    #[doc = "Checks if the value of the field is `H_30US`"]
    #[inline(always)]
    pub fn is_h_30us(&self) -> bool {
        *self == THIGHR::H_30US
    }
    #[doc = "Checks if the value of the field is `H_15US`"]
    #[inline(always)]
    pub fn is_h_15us(&self) -> bool {
        *self == THIGHR::H_15US
    }
}
#[doc = "Values that can be written to the field `THIGH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THIGHW {
    #[doc = "31.2 ms"]
    H_31MS,
    #[doc = "15.6 ms"]
    H_16MS,
    #[doc = "3.91 ms"]
    H_4MS,
    #[doc = "976 us"]
    H_976US,
    #[doc = "488 us"]
    H_488US,
    #[doc = "122 us"]
    H_122US,
    #[doc = "30.5 us"]
    H_30US,
    #[doc = "15.2 us"]
    H_15US,
}
impl THIGHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            THIGHW::H_31MS => 0,
            THIGHW::H_16MS => 1,
            THIGHW::H_4MS => 2,
            THIGHW::H_976US => 3,
            THIGHW::H_488US => 4,
            THIGHW::H_122US => 5,
            THIGHW::H_30US => 6,
            THIGHW::H_15US => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _THIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _THIGHW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THIGHW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "31.2 ms"]
    #[inline(always)]
    pub fn h_31ms(self) -> &'a mut W {
        self.variant(THIGHW::H_31MS)
    }
    #[doc = "15.6 ms"]
    #[inline(always)]
    pub fn h_16ms(self) -> &'a mut W {
        self.variant(THIGHW::H_16MS)
    }
    #[doc = "3.91 ms"]
    #[inline(always)]
    pub fn h_4ms(self) -> &'a mut W {
        self.variant(THIGHW::H_4MS)
    }
    #[doc = "976 us"]
    #[inline(always)]
    pub fn h_976us(self) -> &'a mut W {
        self.variant(THIGHW::H_976US)
    }
    #[doc = "488 us"]
    #[inline(always)]
    pub fn h_488us(self) -> &'a mut W {
        self.variant(THIGHW::H_488US)
    }
    #[doc = "122 us"]
    #[inline(always)]
    pub fn h_122us(self) -> &'a mut W {
        self.variant(THIGHW::H_122US)
    }
    #[doc = "30.5 us"]
    #[inline(always)]
    pub fn h_30us(self) -> &'a mut W {
        self.variant(THIGHW::H_30US)
    }
    #[doc = "15.2 us"]
    #[inline(always)]
    pub fn h_15us(self) -> &'a mut W {
        self.variant(THIGHW::H_15US)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `TPERIOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPERIODR {
    #[doc = "1 second"]
    P_1S,
    #[doc = "500 ms"]
    P_500MS,
    #[doc = "250 ms"]
    P_250MS,
    #[doc = "125 ms"]
    P_125MS,
}
impl crate::ToBits<u8> for TPERIODR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TPERIODR::P_1S => 0,
            TPERIODR::P_500MS => 1,
            TPERIODR::P_250MS => 2,
            TPERIODR::P_125MS => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TPERIOD_R = crate::FR<u8, TPERIODR>;
impl TPERIOD_R {
    #[doc = "Checks if the value of the field is `P_1S`"]
    #[inline(always)]
    pub fn is_p_1s(&self) -> bool {
        *self == TPERIODR::P_1S
    }
    #[doc = "Checks if the value of the field is `P_500MS`"]
    #[inline(always)]
    pub fn is_p_500ms(&self) -> bool {
        *self == TPERIODR::P_500MS
    }
    #[doc = "Checks if the value of the field is `P_250MS`"]
    #[inline(always)]
    pub fn is_p_250ms(&self) -> bool {
        *self == TPERIODR::P_250MS
    }
    #[doc = "Checks if the value of the field is `P_125MS`"]
    #[inline(always)]
    pub fn is_p_125ms(&self) -> bool {
        *self == TPERIODR::P_125MS
    }
}
#[doc = "Values that can be written to the field `TPERIOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPERIODW {
    #[doc = "1 second"]
    P_1S,
    #[doc = "500 ms"]
    P_500MS,
    #[doc = "250 ms"]
    P_250MS,
    #[doc = "125 ms"]
    P_125MS,
}
impl TPERIODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TPERIODW::P_1S => 0,
            TPERIODW::P_500MS => 1,
            TPERIODW::P_250MS => 2,
            TPERIODW::P_125MS => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TPERIODW<'a> {
    w: &'a mut W,
}
impl<'a> _TPERIODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPERIODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 second"]
    #[inline(always)]
    pub fn p_1s(self) -> &'a mut W {
        self.variant(TPERIODW::P_1S)
    }
    #[doc = "500 ms"]
    #[inline(always)]
    pub fn p_500ms(self) -> &'a mut W {
        self.variant(TPERIODW::P_500MS)
    }
    #[doc = "250 ms"]
    #[inline(always)]
    pub fn p_250ms(self) -> &'a mut W {
        self.variant(TPERIODW::P_250MS)
    }
    #[doc = "125 ms"]
    #[inline(always)]
    pub fn p_125ms(self) -> &'a mut W {
        self.variant(TPERIODW::P_125MS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - 12-/24-hour Mode"]
    #[inline(always)]
    pub fn hrmod(&self) -> HRMOD_R {
        HRMOD_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - PERSIAN Calendar"]
    #[inline(always)]
    pub fn persian(&self) -> PERSIAN_R {
        PERSIAN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NEGative PPM Correction"]
    #[inline(always)]
    pub fn negppm(&self) -> NEGPPM_R {
        NEGPPM_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Slow Clock Correction"]
    #[inline(always)]
    pub fn correction(&self) -> CORRECTION_R {
        CORRECTION_R::new(((self.bits() >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - HIGH PPM Correction"]
    #[inline(always)]
    pub fn highppm(&self) -> HIGHPPM_R {
        HIGHPPM_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - RTCOUT0 OutputSource Selection"]
    #[inline(always)]
    pub fn out0(&self) -> OUT0_R {
        OUT0_R::new(((self.bits() >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - RTCOUT1 Output Source Selection"]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits() >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - High Duration of the Output Pulse"]
    #[inline(always)]
    pub fn thigh(&self) -> THIGH_R {
        THIGH_R::new(((self.bits() >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:29 - Period of the Output Pulse"]
    #[inline(always)]
    pub fn tperiod(&self) -> TPERIOD_R {
        TPERIOD_R::new(((self.bits() >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - 12-/24-hour Mode"]
    #[inline(always)]
    pub fn hrmod(&mut self) -> _HRMODW {
        _HRMODW { w: self }
    }
    #[doc = "Bit 1 - PERSIAN Calendar"]
    #[inline(always)]
    pub fn persian(&mut self) -> _PERSIANW {
        _PERSIANW { w: self }
    }
    #[doc = "Bit 4 - NEGative PPM Correction"]
    #[inline(always)]
    pub fn negppm(&mut self) -> _NEGPPMW {
        _NEGPPMW { w: self }
    }
    #[doc = "Bits 8:14 - Slow Clock Correction"]
    #[inline(always)]
    pub fn correction(&mut self) -> _CORRECTIONW {
        _CORRECTIONW { w: self }
    }
    #[doc = "Bit 15 - HIGH PPM Correction"]
    #[inline(always)]
    pub fn highppm(&mut self) -> _HIGHPPMW {
        _HIGHPPMW { w: self }
    }
    #[doc = "Bits 16:18 - RTCOUT0 OutputSource Selection"]
    #[inline(always)]
    pub fn out0(&mut self) -> _OUT0W {
        _OUT0W { w: self }
    }
    #[doc = "Bits 20:22 - RTCOUT1 Output Source Selection"]
    #[inline(always)]
    pub fn out1(&mut self) -> _OUT1W {
        _OUT1W { w: self }
    }
    #[doc = "Bits 24:26 - High Duration of the Output Pulse"]
    #[inline(always)]
    pub fn thigh(&mut self) -> _THIGHW {
        _THIGHW { w: self }
    }
    #[doc = "Bits 28:29 - Period of the Output Pulse"]
    #[inline(always)]
    pub fn tperiod(&mut self) -> _TPERIODW {
        _TPERIODW { w: self }
    }
}
