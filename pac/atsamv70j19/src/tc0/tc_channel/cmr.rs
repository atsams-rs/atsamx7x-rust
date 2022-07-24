#[doc = "Register `CMR` reader"]
pub struct R(crate::R<CMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMR` writer"]
pub struct W(crate::W<CMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMR_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `TCCLKS` reader - Clock Selection"]
pub type TCCLKS_R = crate::FieldReader<u8, TCCLKS_A>;
impl TCCLKS_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `TCCLKS` writer - Clock Selection"]
pub type TCCLKS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CMR_SPEC, u8, TCCLKS_A, 3, O>;
impl<'a, const O: u8> TCCLKS_W<'a, O> {
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
}
#[doc = "Field `CLKI` reader - Clock Invert"]
pub type CLKI_R = crate::BitReader<bool>;
#[doc = "Field `CLKI` writer - Clock Invert"]
pub type CLKI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_SPEC, bool, O>;
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
#[doc = "Field `BURST` reader - Burst Signal Selection"]
pub type BURST_R = crate::FieldReader<u8, BURST_A>;
impl BURST_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `BURST` writer - Burst Signal Selection"]
pub type BURST_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CMR_SPEC, u8, BURST_A, 2, O>;
impl<'a, const O: u8> BURST_W<'a, O> {
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
}
#[doc = "Field `LDBSTOP` reader - Counter Clock Stopped with RB Loading"]
pub type LDBSTOP_R = crate::BitReader<bool>;
#[doc = "Field `LDBSTOP` writer - Counter Clock Stopped with RB Loading"]
pub type LDBSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_SPEC, bool, O>;
#[doc = "Field `LDBDIS` reader - Counter Clock Disable with RB Loading"]
pub type LDBDIS_R = crate::BitReader<bool>;
#[doc = "Field `LDBDIS` writer - Counter Clock Disable with RB Loading"]
pub type LDBDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_SPEC, bool, O>;
#[doc = "External Trigger Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETRGEDG_A {
    #[doc = "0: The clock is not gated by an external signal."]
    NONE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Each edge"]
    EDGE = 3,
}
impl From<ETRGEDG_A> for u8 {
    #[inline(always)]
    fn from(variant: ETRGEDG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ETRGEDG` reader - External Trigger Edge Selection"]
pub type ETRGEDG_R = crate::FieldReader<u8, ETRGEDG_A>;
impl ETRGEDG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETRGEDG_A {
        match self.bits {
            0 => ETRGEDG_A::NONE,
            1 => ETRGEDG_A::RISING,
            2 => ETRGEDG_A::FALLING,
            3 => ETRGEDG_A::EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ETRGEDG_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ETRGEDG_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ETRGEDG_A::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == ETRGEDG_A::EDGE
    }
}
#[doc = "Field `ETRGEDG` writer - External Trigger Edge Selection"]
pub type ETRGEDG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CMR_SPEC, u8, ETRGEDG_A, 2, O>;
impl<'a, const O: u8> ETRGEDG_W<'a, O> {
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ETRGEDG_A::NONE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(ETRGEDG_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(ETRGEDG_A::FALLING)
    }
    #[doc = "Each edge"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(ETRGEDG_A::EDGE)
    }
}
#[doc = "Field `ABETRG` reader - TIOAx or TIOBx External Trigger Selection"]
pub type ABETRG_R = crate::BitReader<bool>;
#[doc = "Field `ABETRG` writer - TIOAx or TIOBx External Trigger Selection"]
pub type ABETRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_SPEC, bool, O>;
#[doc = "Field `CPCTRG` reader - RC Compare Trigger Enable"]
pub type CPCTRG_R = crate::BitReader<bool>;
#[doc = "Field `CPCTRG` writer - RC Compare Trigger Enable"]
pub type CPCTRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_SPEC, bool, O>;
#[doc = "Field `WAVE` reader - Waveform Mode"]
pub type WAVE_R = crate::BitReader<bool>;
#[doc = "Field `WAVE` writer - Waveform Mode"]
pub type WAVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMR_SPEC, bool, O>;
#[doc = "RA Loading Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LDRA_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: Rising edge of TIOAx"]
    RISING = 1,
    #[doc = "2: Falling edge of TIOAx"]
    FALLING = 2,
    #[doc = "3: Each edge of TIOAx"]
    EDGE = 3,
}
impl From<LDRA_A> for u8 {
    #[inline(always)]
    fn from(variant: LDRA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LDRA` reader - RA Loading Edge Selection"]
pub type LDRA_R = crate::FieldReader<u8, LDRA_A>;
impl LDRA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDRA_A {
        match self.bits {
            0 => LDRA_A::NONE,
            1 => LDRA_A::RISING,
            2 => LDRA_A::FALLING,
            3 => LDRA_A::EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LDRA_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == LDRA_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == LDRA_A::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == LDRA_A::EDGE
    }
}
#[doc = "Field `LDRA` writer - RA Loading Edge Selection"]
pub type LDRA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CMR_SPEC, u8, LDRA_A, 2, O>;
impl<'a, const O: u8> LDRA_W<'a, O> {
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(LDRA_A::NONE)
    }
    #[doc = "Rising edge of TIOAx"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(LDRA_A::RISING)
    }
    #[doc = "Falling edge of TIOAx"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(LDRA_A::FALLING)
    }
    #[doc = "Each edge of TIOAx"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(LDRA_A::EDGE)
    }
}
#[doc = "RB Loading Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LDRB_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: Rising edge of TIOAx"]
    RISING = 1,
    #[doc = "2: Falling edge of TIOAx"]
    FALLING = 2,
    #[doc = "3: Each edge of TIOAx"]
    EDGE = 3,
}
impl From<LDRB_A> for u8 {
    #[inline(always)]
    fn from(variant: LDRB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LDRB` reader - RB Loading Edge Selection"]
pub type LDRB_R = crate::FieldReader<u8, LDRB_A>;
impl LDRB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDRB_A {
        match self.bits {
            0 => LDRB_A::NONE,
            1 => LDRB_A::RISING,
            2 => LDRB_A::FALLING,
            3 => LDRB_A::EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LDRB_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == LDRB_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == LDRB_A::FALLING
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == LDRB_A::EDGE
    }
}
#[doc = "Field `LDRB` writer - RB Loading Edge Selection"]
pub type LDRB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CMR_SPEC, u8, LDRB_A, 2, O>;
impl<'a, const O: u8> LDRB_W<'a, O> {
    #[doc = "None"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(LDRB_A::NONE)
    }
    #[doc = "Rising edge of TIOAx"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(LDRB_A::RISING)
    }
    #[doc = "Falling edge of TIOAx"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(LDRB_A::FALLING)
    }
    #[doc = "Each edge of TIOAx"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(LDRB_A::EDGE)
    }
}
#[doc = "Loading Edge Subsampling Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SBSMPLR_A {
    #[doc = "0: Load a Capture Register each selected edge"]
    ONE = 0,
    #[doc = "1: Load a Capture Register every 2 selected edges"]
    HALF = 1,
    #[doc = "2: Load a Capture Register every 4 selected edges"]
    FOURTH = 2,
    #[doc = "3: Load a Capture Register every 8 selected edges"]
    EIGHTH = 3,
    #[doc = "4: Load a Capture Register every 16 selected edges"]
    SIXTEENTH = 4,
}
impl From<SBSMPLR_A> for u8 {
    #[inline(always)]
    fn from(variant: SBSMPLR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SBSMPLR` reader - Loading Edge Subsampling Ratio"]
pub type SBSMPLR_R = crate::FieldReader<u8, SBSMPLR_A>;
impl SBSMPLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SBSMPLR_A> {
        match self.bits {
            0 => Some(SBSMPLR_A::ONE),
            1 => Some(SBSMPLR_A::HALF),
            2 => Some(SBSMPLR_A::FOURTH),
            3 => Some(SBSMPLR_A::EIGHTH),
            4 => Some(SBSMPLR_A::SIXTEENTH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == SBSMPLR_A::ONE
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == SBSMPLR_A::HALF
    }
    #[doc = "Checks if the value of the field is `FOURTH`"]
    #[inline(always)]
    pub fn is_fourth(&self) -> bool {
        *self == SBSMPLR_A::FOURTH
    }
    #[doc = "Checks if the value of the field is `EIGHTH`"]
    #[inline(always)]
    pub fn is_eighth(&self) -> bool {
        *self == SBSMPLR_A::EIGHTH
    }
    #[doc = "Checks if the value of the field is `SIXTEENTH`"]
    #[inline(always)]
    pub fn is_sixteenth(&self) -> bool {
        *self == SBSMPLR_A::SIXTEENTH
    }
}
#[doc = "Field `SBSMPLR` writer - Loading Edge Subsampling Ratio"]
pub type SBSMPLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMR_SPEC, u8, SBSMPLR_A, 3, O>;
impl<'a, const O: u8> SBSMPLR_W<'a, O> {
    #[doc = "Load a Capture Register each selected edge"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(SBSMPLR_A::ONE)
    }
    #[doc = "Load a Capture Register every 2 selected edges"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(SBSMPLR_A::HALF)
    }
    #[doc = "Load a Capture Register every 4 selected edges"]
    #[inline(always)]
    pub fn fourth(self) -> &'a mut W {
        self.variant(SBSMPLR_A::FOURTH)
    }
    #[doc = "Load a Capture Register every 8 selected edges"]
    #[inline(always)]
    pub fn eighth(self) -> &'a mut W {
        self.variant(SBSMPLR_A::EIGHTH)
    }
    #[doc = "Load a Capture Register every 16 selected edges"]
    #[inline(always)]
    pub fn sixteenth(self) -> &'a mut W {
        self.variant(SBSMPLR_A::SIXTEENTH)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    pub fn tcclks(&self) -> TCCLKS_R {
        TCCLKS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    pub fn clki(&self) -> CLKI_R {
        CLKI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline(always)]
    pub fn ldbstop(&self) -> LDBSTOP_R {
        LDBSTOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline(always)]
    pub fn ldbdis(&self) -> LDBDIS_R {
        LDBDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline(always)]
    pub fn etrgedg(&self) -> ETRGEDG_R {
        ETRGEDG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - TIOAx or TIOBx External Trigger Selection"]
    #[inline(always)]
    pub fn abetrg(&self) -> ABETRG_R {
        ABETRG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline(always)]
    pub fn cpctrg(&self) -> CPCTRG_R {
        CPCTRG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - RA Loading Edge Selection"]
    #[inline(always)]
    pub fn ldra(&self) -> LDRA_R {
        LDRA_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - RB Loading Edge Selection"]
    #[inline(always)]
    pub fn ldrb(&self) -> LDRB_R {
        LDRB_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Loading Edge Subsampling Ratio"]
    #[inline(always)]
    pub fn sbsmplr(&self) -> SBSMPLR_R {
        SBSMPLR_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    pub fn tcclks(&mut self) -> TCCLKS_W<0> {
        TCCLKS_W::new(self)
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    pub fn clki(&mut self) -> CLKI_W<3> {
        CLKI_W::new(self)
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    pub fn burst(&mut self) -> BURST_W<4> {
        BURST_W::new(self)
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline(always)]
    pub fn ldbstop(&mut self) -> LDBSTOP_W<6> {
        LDBSTOP_W::new(self)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline(always)]
    pub fn ldbdis(&mut self) -> LDBDIS_W<7> {
        LDBDIS_W::new(self)
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline(always)]
    pub fn etrgedg(&mut self) -> ETRGEDG_W<8> {
        ETRGEDG_W::new(self)
    }
    #[doc = "Bit 10 - TIOAx or TIOBx External Trigger Selection"]
    #[inline(always)]
    pub fn abetrg(&mut self) -> ABETRG_W<10> {
        ABETRG_W::new(self)
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline(always)]
    pub fn cpctrg(&mut self) -> CPCTRG_W<14> {
        CPCTRG_W::new(self)
    }
    #[doc = "Bit 15 - Waveform Mode"]
    #[inline(always)]
    pub fn wave(&mut self) -> WAVE_W<15> {
        WAVE_W::new(self)
    }
    #[doc = "Bits 16:17 - RA Loading Edge Selection"]
    #[inline(always)]
    pub fn ldra(&mut self) -> LDRA_W<16> {
        LDRA_W::new(self)
    }
    #[doc = "Bits 18:19 - RB Loading Edge Selection"]
    #[inline(always)]
    pub fn ldrb(&mut self) -> LDRB_W<18> {
        LDRB_W::new(self)
    }
    #[doc = "Bits 20:22 - Loading Edge Subsampling Ratio"]
    #[inline(always)]
    pub fn sbsmplr(&mut self) -> SBSMPLR_W<20> {
        SBSMPLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Mode Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmr](index.html) module"]
pub struct CMR_SPEC;
impl crate::RegisterSpec for CMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmr::R](R) reader structure"]
impl crate::Readable for CMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmr::W](W) writer structure"]
impl crate::Writable for CMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMR to value 0"]
impl crate::Resettable for CMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
