#[doc = "Reader of register TC_CMR_WAVEFORM_MODE"]
pub type R = crate::R<u32, super::TC_CMR_WAVEFORM_MODE>;
#[doc = "Writer for register TC_CMR_WAVEFORM_MODE"]
pub type W = crate::W<u32, super::TC_CMR_WAVEFORM_MODE>;
#[doc = "Register TC_CMR_WAVEFORM_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::TC_CMR_WAVEFORM_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TCCLKS_A {
    #[doc = "0: Clock selected: internal PCK6 clock signal (from PMC)"]
    TIMER_CLOCK1 = 0,
    #[doc = "1: Clock selected: internal MCK/8 clock signal (from PMC)"]
    TIMER_CLOCK2 = 1,
    #[doc = "2: Clock selected: internal MCK/32 clock signal (from PMC)"]
    TIMER_CLOCK3 = 2,
    #[doc = "3: Clock selected: internal MCK/128 clock signal (from PMC)"]
    TIMER_CLOCK4 = 3,
    #[doc = "4: Clock selected: internal SLCK clock signal (from PMC)"]
    TIMER_CLOCK5 = 4,
    #[doc = "5: Clock selected: XC0"]
    XC0 = 5,
    #[doc = "6: Clock selected: XC1"]
    XC1 = 6,
    #[doc = "7: Clock selected: XC2"]
    XC2 = 7,
}
impl From<TCCLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCLKS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TCCLKS`"]
pub type TCCLKS_R = crate::R<u8, TCCLKS_A>;
impl TCCLKS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCCLKS_A {
        match self.bits {
            0 => TCCLKS_A::TIMER_CLOCK1,
            1 => TCCLKS_A::TIMER_CLOCK2,
            2 => TCCLKS_A::TIMER_CLOCK3,
            3 => TCCLKS_A::TIMER_CLOCK4,
            4 => TCCLKS_A::TIMER_CLOCK5,
            5 => TCCLKS_A::XC0,
            6 => TCCLKS_A::XC1,
            7 => TCCLKS_A::XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK1`"]
    #[inline(always)]
    pub fn is_timer_clock1(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK1
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK2`"]
    #[inline(always)]
    pub fn is_timer_clock2(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK2
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK3`"]
    #[inline(always)]
    pub fn is_timer_clock3(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK3
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK4`"]
    #[inline(always)]
    pub fn is_timer_clock4(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK4
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK5`"]
    #[inline(always)]
    pub fn is_timer_clock5(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK5
    }
    #[doc = "Checks if the value of the field is `XC0`"]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == TCCLKS_A::XC0
    }
    #[doc = "Checks if the value of the field is `XC1`"]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == TCCLKS_A::XC1
    }
    #[doc = "Checks if the value of the field is `XC2`"]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == TCCLKS_A::XC2
    }
}
#[doc = "Write proxy for field `TCCLKS`"]
pub struct TCCLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCCLKS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock selected: internal PCK6 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock1(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK1)
    }
    #[doc = "Clock selected: internal MCK/8 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock2(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK2)
    }
    #[doc = "Clock selected: internal MCK/32 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock3(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK3)
    }
    #[doc = "Clock selected: internal MCK/128 clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock4(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK4)
    }
    #[doc = "Clock selected: internal SLCK clock signal (from PMC)"]
    #[inline(always)]
    pub fn timer_clock5(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK5)
    }
    #[doc = "Clock selected: XC0"]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut W {
        self.variant(TCCLKS_A::XC0)
    }
    #[doc = "Clock selected: XC1"]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut W {
        self.variant(TCCLKS_A::XC1)
    }
    #[doc = "Clock selected: XC2"]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut W {
        self.variant(TCCLKS_A::XC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `CLKI`"]
pub type CLKI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKI`"]
pub struct CLKI_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKI_W<'a> {
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
#[doc = "Burst Signal Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BURST_A {
    #[doc = "0: The clock is not gated by an external signal."]
    NONE = 0,
    #[doc = "1: XC0 is ANDed with the selected clock."]
    XC0 = 1,
    #[doc = "2: XC1 is ANDed with the selected clock."]
    XC1 = 2,
    #[doc = "3: XC2 is ANDed with the selected clock."]
    XC2 = 3,
}
impl From<BURST_A> for u8 {
    #[inline(always)]
    fn from(variant: BURST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BURST`"]
pub type BURST_R = crate::R<u8, BURST_A>;
impl BURST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURST_A {
        match self.bits {
            0 => BURST_A::NONE,
            1 => BURST_A::XC0,
            2 => BURST_A::XC1,
            3 => BURST_A::XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BURST_A::NONE
    }
    #[doc = "Checks if the value of the field is `XC0`"]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == BURST_A::XC0
    }
    #[doc = "Checks if the value of the field is `XC1`"]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == BURST_A::XC1
    }
    #[doc = "Checks if the value of the field is `XC2`"]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == BURST_A::XC2
    }
}
#[doc = "Write proxy for field `BURST`"]
pub struct BURST_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURST_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BURST_A::NONE)
    }
    #[doc = "XC0 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut W {
        self.variant(BURST_A::XC0)
    }
    #[doc = "XC1 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut W {
        self.variant(BURST_A::XC1)
    }
    #[doc = "XC2 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut W {
        self.variant(BURST_A::XC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `CPCSTOP`"]
pub type CPCSTOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPCSTOP`"]
pub struct CPCSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CPCSTOP_W<'a> {
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
#[doc = "Reader of field `CPCDIS`"]
pub type CPCDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPCDIS`"]
pub struct CPCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPCDIS_W<'a> {
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
#[doc = "External Event Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EEVTEDG_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Each edges"]
    EDGE = 3,
}
impl From<EEVTEDG_A> for u8 {
    #[inline(always)]
    fn from(variant: EEVTEDG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EEVTEDG`"]
pub type EEVTEDG_R = crate::R<u8, EEVTEDG_A>;
impl EEVTEDG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEVTEDG_A {
        match self.bits {
            0 => EEVTEDG_A::NONE,
            1 => EEVTEDG_A::RISING,
            2 => EEVTEDG_A::FALLING,
            3 => EEVTEDG_A::EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == EEVTEDG_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == EEVTEDG_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == EEVTEDG_A::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == EEVTEDG_A::EDGE
    }
}
#[doc = "Write proxy for field `EEVTEDG`"]
pub struct EEVTEDG_W<'a> {
    w: &'a mut W,
}
impl<'a> EEVTEDG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEVTEDG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(EEVTEDG_A::NONE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EEVTEDG_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EEVTEDG_A::FALLING)
    }
    #[doc = "Each edges"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(EEVTEDG_A::EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "External Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EEVT_A {
    #[doc = "0: TIOB"]
    TIOB = 0,
    #[doc = "1: XC0"]
    XC0 = 1,
    #[doc = "2: XC1"]
    XC1 = 2,
    #[doc = "3: XC2"]
    XC2 = 3,
}
impl From<EEVT_A> for u8 {
    #[inline(always)]
    fn from(variant: EEVT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EEVT`"]
pub type EEVT_R = crate::R<u8, EEVT_A>;
impl EEVT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEVT_A {
        match self.bits {
            0 => EEVT_A::TIOB,
            1 => EEVT_A::XC0,
            2 => EEVT_A::XC1,
            3 => EEVT_A::XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIOB`"]
    #[inline(always)]
    pub fn is_tiob(&self) -> bool {
        *self == EEVT_A::TIOB
    }
    #[doc = "Checks if the value of the field is `XC0`"]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == EEVT_A::XC0
    }
    #[doc = "Checks if the value of the field is `XC1`"]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == EEVT_A::XC1
    }
    #[doc = "Checks if the value of the field is `XC2`"]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == EEVT_A::XC2
    }
}
#[doc = "Write proxy for field `EEVT`"]
pub struct EEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> EEVT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEVT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "TIOB"]
    #[inline(always)]
    pub fn tiob(self) -> &'a mut W {
        self.variant(EEVT_A::TIOB)
    }
    #[doc = "XC0"]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut W {
        self.variant(EEVT_A::XC0)
    }
    #[doc = "XC1"]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut W {
        self.variant(EEVT_A::XC1)
    }
    #[doc = "XC2"]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut W {
        self.variant(EEVT_A::XC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `ENETRG`"]
pub type ENETRG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENETRG`"]
pub struct ENETRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ENETRG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Waveform Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAVSEL_A {
    #[doc = "0: UP mode without automatic trigger on RC Compare"]
    UP = 0,
    #[doc = "1: UPDOWN mode without automatic trigger on RC Compare"]
    UPDOWN = 1,
    #[doc = "2: UP mode with automatic trigger on RC Compare"]
    UP_RC = 2,
    #[doc = "3: UPDOWN mode with automatic trigger on RC Compare"]
    UPDOWN_RC = 3,
}
impl From<WAVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WAVSEL`"]
pub type WAVSEL_R = crate::R<u8, WAVSEL_A>;
impl WAVSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAVSEL_A {
        match self.bits {
            0 => WAVSEL_A::UP,
            1 => WAVSEL_A::UPDOWN,
            2 => WAVSEL_A::UP_RC,
            3 => WAVSEL_A::UPDOWN_RC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == WAVSEL_A::UP
    }
    #[doc = "Checks if the value of the field is `UPDOWN`"]
    #[inline(always)]
    pub fn is_updown(&self) -> bool {
        *self == WAVSEL_A::UPDOWN
    }
    #[doc = "Checks if the value of the field is `UP_RC`"]
    #[inline(always)]
    pub fn is_up_rc(&self) -> bool {
        *self == WAVSEL_A::UP_RC
    }
    #[doc = "Checks if the value of the field is `UPDOWN_RC`"]
    #[inline(always)]
    pub fn is_updown_rc(&self) -> bool {
        *self == WAVSEL_A::UPDOWN_RC
    }
}
#[doc = "Write proxy for field `WAVSEL`"]
pub struct WAVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAVSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "UP mode without automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(WAVSEL_A::UP)
    }
    #[doc = "UPDOWN mode without automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn updown(self) -> &'a mut W {
        self.variant(WAVSEL_A::UPDOWN)
    }
    #[doc = "UP mode with automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn up_rc(self) -> &'a mut W {
        self.variant(WAVSEL_A::UP_RC)
    }
    #[doc = "UPDOWN mode with automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn updown_rc(self) -> &'a mut W {
        self.variant(WAVSEL_A::UPDOWN_RC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `WAVE`"]
pub type WAVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAVE`"]
pub struct WAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE_W<'a> {
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
#[doc = "RA Compare Effect on TIOAx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACPA_A {
    #[doc = "0: NONE"]
    NONE = 0,
    #[doc = "1: SET"]
    SET = 1,
    #[doc = "2: CLEAR"]
    CLEAR = 2,
    #[doc = "3: TOGGLE"]
    TOGGLE = 3,
}
impl From<ACPA_A> for u8 {
    #[inline(always)]
    fn from(variant: ACPA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ACPA`"]
pub type ACPA_R = crate::R<u8, ACPA_A>;
impl ACPA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACPA_A {
        match self.bits {
            0 => ACPA_A::NONE,
            1 => ACPA_A::SET,
            2 => ACPA_A::CLEAR,
            3 => ACPA_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ACPA_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ACPA_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ACPA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == ACPA_A::TOGGLE
    }
}
#[doc = "Write proxy for field `ACPA`"]
pub struct ACPA_W<'a> {
    w: &'a mut W,
}
impl<'a> ACPA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACPA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ACPA_A::NONE)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ACPA_A::SET)
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ACPA_A::CLEAR)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ACPA_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "RC Compare Effect on TIOAx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACPC_A {
    #[doc = "0: NONE"]
    NONE = 0,
    #[doc = "1: SET"]
    SET = 1,
    #[doc = "2: CLEAR"]
    CLEAR = 2,
    #[doc = "3: TOGGLE"]
    TOGGLE = 3,
}
impl From<ACPC_A> for u8 {
    #[inline(always)]
    fn from(variant: ACPC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ACPC`"]
pub type ACPC_R = crate::R<u8, ACPC_A>;
impl ACPC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACPC_A {
        match self.bits {
            0 => ACPC_A::NONE,
            1 => ACPC_A::SET,
            2 => ACPC_A::CLEAR,
            3 => ACPC_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ACPC_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ACPC_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ACPC_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == ACPC_A::TOGGLE
    }
}
#[doc = "Write proxy for field `ACPC`"]
pub struct ACPC_W<'a> {
    w: &'a mut W,
}
impl<'a> ACPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACPC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ACPC_A::NONE)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ACPC_A::SET)
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ACPC_A::CLEAR)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ACPC_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "External Event Effect on TIOAx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AEEVT_A {
    #[doc = "0: NONE"]
    NONE = 0,
    #[doc = "1: SET"]
    SET = 1,
    #[doc = "2: CLEAR"]
    CLEAR = 2,
    #[doc = "3: TOGGLE"]
    TOGGLE = 3,
}
impl From<AEEVT_A> for u8 {
    #[inline(always)]
    fn from(variant: AEEVT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AEEVT`"]
pub type AEEVT_R = crate::R<u8, AEEVT_A>;
impl AEEVT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AEEVT_A {
        match self.bits {
            0 => AEEVT_A::NONE,
            1 => AEEVT_A::SET,
            2 => AEEVT_A::CLEAR,
            3 => AEEVT_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == AEEVT_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == AEEVT_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == AEEVT_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == AEEVT_A::TOGGLE
    }
}
#[doc = "Write proxy for field `AEEVT`"]
pub struct AEEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> AEEVT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AEEVT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(AEEVT_A::NONE)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(AEEVT_A::SET)
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(AEEVT_A::CLEAR)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(AEEVT_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Software Trigger Effect on TIOAx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ASWTRG_A {
    #[doc = "0: NONE"]
    NONE = 0,
    #[doc = "1: SET"]
    SET = 1,
    #[doc = "2: CLEAR"]
    CLEAR = 2,
    #[doc = "3: TOGGLE"]
    TOGGLE = 3,
}
impl From<ASWTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: ASWTRG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ASWTRG`"]
pub type ASWTRG_R = crate::R<u8, ASWTRG_A>;
impl ASWTRG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASWTRG_A {
        match self.bits {
            0 => ASWTRG_A::NONE,
            1 => ASWTRG_A::SET,
            2 => ASWTRG_A::CLEAR,
            3 => ASWTRG_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ASWTRG_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ASWTRG_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ASWTRG_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == ASWTRG_A::TOGGLE
    }
}
#[doc = "Write proxy for field `ASWTRG`"]
pub struct ASWTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ASWTRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASWTRG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ASWTRG_A::NONE)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ASWTRG_A::SET)
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ASWTRG_A::CLEAR)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ASWTRG_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "RB Compare Effect on TIOBx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BCPB_A {
    #[doc = "0: NONE"]
    NONE = 0,
    #[doc = "1: SET"]
    SET = 1,
    #[doc = "2: CLEAR"]
    CLEAR = 2,
    #[doc = "3: TOGGLE"]
    TOGGLE = 3,
}
impl From<BCPB_A> for u8 {
    #[inline(always)]
    fn from(variant: BCPB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BCPB`"]
pub type BCPB_R = crate::R<u8, BCPB_A>;
impl BCPB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCPB_A {
        match self.bits {
            0 => BCPB_A::NONE,
            1 => BCPB_A::SET,
            2 => BCPB_A::CLEAR,
            3 => BCPB_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BCPB_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == BCPB_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == BCPB_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == BCPB_A::TOGGLE
    }
}
#[doc = "Write proxy for field `BCPB`"]
pub struct BCPB_W<'a> {
    w: &'a mut W,
}
impl<'a> BCPB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCPB_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BCPB_A::NONE)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BCPB_A::SET)
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCPB_A::CLEAR)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BCPB_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "RC Compare Effect on TIOBx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BCPC_A {
    #[doc = "0: NONE"]
    NONE = 0,
    #[doc = "1: SET"]
    SET = 1,
    #[doc = "2: CLEAR"]
    CLEAR = 2,
    #[doc = "3: TOGGLE"]
    TOGGLE = 3,
}
impl From<BCPC_A> for u8 {
    #[inline(always)]
    fn from(variant: BCPC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BCPC`"]
pub type BCPC_R = crate::R<u8, BCPC_A>;
impl BCPC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCPC_A {
        match self.bits {
            0 => BCPC_A::NONE,
            1 => BCPC_A::SET,
            2 => BCPC_A::CLEAR,
            3 => BCPC_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BCPC_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == BCPC_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == BCPC_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == BCPC_A::TOGGLE
    }
}
#[doc = "Write proxy for field `BCPC`"]
pub struct BCPC_W<'a> {
    w: &'a mut W,
}
impl<'a> BCPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCPC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BCPC_A::NONE)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BCPC_A::SET)
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCPC_A::CLEAR)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BCPC_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "External Event Effect on TIOBx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BEEVT_A {
    #[doc = "0: NONE"]
    NONE = 0,
    #[doc = "1: SET"]
    SET = 1,
    #[doc = "2: CLEAR"]
    CLEAR = 2,
    #[doc = "3: TOGGLE"]
    TOGGLE = 3,
}
impl From<BEEVT_A> for u8 {
    #[inline(always)]
    fn from(variant: BEEVT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BEEVT`"]
pub type BEEVT_R = crate::R<u8, BEEVT_A>;
impl BEEVT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEEVT_A {
        match self.bits {
            0 => BEEVT_A::NONE,
            1 => BEEVT_A::SET,
            2 => BEEVT_A::CLEAR,
            3 => BEEVT_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BEEVT_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == BEEVT_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == BEEVT_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == BEEVT_A::TOGGLE
    }
}
#[doc = "Write proxy for field `BEEVT`"]
pub struct BEEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> BEEVT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BEEVT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BEEVT_A::NONE)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BEEVT_A::SET)
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BEEVT_A::CLEAR)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BEEVT_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Software Trigger Effect on TIOBx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BSWTRG_A {
    #[doc = "0: NONE"]
    NONE = 0,
    #[doc = "1: SET"]
    SET = 1,
    #[doc = "2: CLEAR"]
    CLEAR = 2,
    #[doc = "3: TOGGLE"]
    TOGGLE = 3,
}
impl From<BSWTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: BSWTRG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BSWTRG`"]
pub type BSWTRG_R = crate::R<u8, BSWTRG_A>;
impl BSWTRG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSWTRG_A {
        match self.bits {
            0 => BSWTRG_A::NONE,
            1 => BSWTRG_A::SET,
            2 => BSWTRG_A::CLEAR,
            3 => BSWTRG_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BSWTRG_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == BSWTRG_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == BSWTRG_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == BSWTRG_A::TOGGLE
    }
}
#[doc = "Write proxy for field `BSWTRG`"]
pub struct BSWTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> BSWTRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BSWTRG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BSWTRG_A::NONE)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSWTRG_A::SET)
    }
    #[doc = "CLEAR"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BSWTRG_A::CLEAR)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BSWTRG_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    pub fn tcclks(&self) -> TCCLKS_R {
        TCCLKS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    pub fn clki(&self) -> CLKI_R {
        CLKI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RC Compare"]
    #[inline(always)]
    pub fn cpcstop(&self) -> CPCSTOP_R {
        CPCSTOP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RC Loading"]
    #[inline(always)]
    pub fn cpcdis(&self) -> CPCDIS_R {
        CPCDIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - External Event Edge Selection"]
    #[inline(always)]
    pub fn eevtedg(&self) -> EEVTEDG_R {
        EEVTEDG_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - External Event Selection"]
    #[inline(always)]
    pub fn eevt(&self) -> EEVT_R {
        EEVT_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - External Event Trigger Enable"]
    #[inline(always)]
    pub fn enetrg(&self) -> ENETRG_R {
        ENETRG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - Waveform Selection"]
    #[inline(always)]
    pub fn wavsel(&self) -> WAVSEL_R {
        WAVSEL_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - RA Compare Effect on TIOAx"]
    #[inline(always)]
    pub fn acpa(&self) -> ACPA_R {
        ACPA_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - RC Compare Effect on TIOAx"]
    #[inline(always)]
    pub fn acpc(&self) -> ACPC_R {
        ACPC_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - External Event Effect on TIOAx"]
    #[inline(always)]
    pub fn aeevt(&self) -> AEEVT_R {
        AEEVT_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Software Trigger Effect on TIOAx"]
    #[inline(always)]
    pub fn aswtrg(&self) -> ASWTRG_R {
        ASWTRG_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - RB Compare Effect on TIOBx"]
    #[inline(always)]
    pub fn bcpb(&self) -> BCPB_R {
        BCPB_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - RC Compare Effect on TIOBx"]
    #[inline(always)]
    pub fn bcpc(&self) -> BCPC_R {
        BCPC_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - External Event Effect on TIOBx"]
    #[inline(always)]
    pub fn beevt(&self) -> BEEVT_R {
        BEEVT_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Software Trigger Effect on TIOBx"]
    #[inline(always)]
    pub fn bswtrg(&self) -> BSWTRG_R {
        BSWTRG_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    pub fn tcclks(&mut self) -> TCCLKS_W {
        TCCLKS_W { w: self }
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    pub fn clki(&mut self) -> CLKI_W {
        CLKI_W { w: self }
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    pub fn burst(&mut self) -> BURST_W {
        BURST_W { w: self }
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RC Compare"]
    #[inline(always)]
    pub fn cpcstop(&mut self) -> CPCSTOP_W {
        CPCSTOP_W { w: self }
    }
    #[doc = "Bit 7 - Counter Clock Disable with RC Loading"]
    #[inline(always)]
    pub fn cpcdis(&mut self) -> CPCDIS_W {
        CPCDIS_W { w: self }
    }
    #[doc = "Bits 8:9 - External Event Edge Selection"]
    #[inline(always)]
    pub fn eevtedg(&mut self) -> EEVTEDG_W {
        EEVTEDG_W { w: self }
    }
    #[doc = "Bits 10:11 - External Event Selection"]
    #[inline(always)]
    pub fn eevt(&mut self) -> EEVT_W {
        EEVT_W { w: self }
    }
    #[doc = "Bit 12 - External Event Trigger Enable"]
    #[inline(always)]
    pub fn enetrg(&mut self) -> ENETRG_W {
        ENETRG_W { w: self }
    }
    #[doc = "Bits 13:14 - Waveform Selection"]
    #[inline(always)]
    pub fn wavsel(&mut self) -> WAVSEL_W {
        WAVSEL_W { w: self }
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    pub fn wave(&mut self) -> WAVE_W {
        WAVE_W { w: self }
    }
    #[doc = "Bits 16:17 - RA Compare Effect on TIOAx"]
    #[inline(always)]
    pub fn acpa(&mut self) -> ACPA_W {
        ACPA_W { w: self }
    }
    #[doc = "Bits 18:19 - RC Compare Effect on TIOAx"]
    #[inline(always)]
    pub fn acpc(&mut self) -> ACPC_W {
        ACPC_W { w: self }
    }
    #[doc = "Bits 20:21 - External Event Effect on TIOAx"]
    #[inline(always)]
    pub fn aeevt(&mut self) -> AEEVT_W {
        AEEVT_W { w: self }
    }
    #[doc = "Bits 22:23 - Software Trigger Effect on TIOAx"]
    #[inline(always)]
    pub fn aswtrg(&mut self) -> ASWTRG_W {
        ASWTRG_W { w: self }
    }
    #[doc = "Bits 24:25 - RB Compare Effect on TIOBx"]
    #[inline(always)]
    pub fn bcpb(&mut self) -> BCPB_W {
        BCPB_W { w: self }
    }
    #[doc = "Bits 26:27 - RC Compare Effect on TIOBx"]
    #[inline(always)]
    pub fn bcpc(&mut self) -> BCPC_W {
        BCPC_W { w: self }
    }
    #[doc = "Bits 28:29 - External Event Effect on TIOBx"]
    #[inline(always)]
    pub fn beevt(&mut self) -> BEEVT_W {
        BEEVT_W { w: self }
    }
    #[doc = "Bits 30:31 - Software Trigger Effect on TIOBx"]
    #[inline(always)]
    pub fn bswtrg(&mut self) -> BSWTRG_W {
        BSWTRG_W { w: self }
    }
}
