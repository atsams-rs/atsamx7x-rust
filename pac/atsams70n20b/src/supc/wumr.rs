#[doc = "Register `WUMR` reader"]
pub struct R(crate::R<WUMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUMR` writer"]
pub struct W(crate::W<WUMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUMR_SPEC>;
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
impl From<crate::W<WUMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Supply Monitor Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMEN_A {
    #[doc = "0: The supply monitor detection has no wake-up effect."]
    NOT_ENABLE = 0,
    #[doc = "1: The supply monitor detection forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<SMEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMEN` reader - Supply Monitor Wake-up Enable"]
pub type SMEN_R = crate::BitReader<SMEN_A>;
impl SMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMEN_A {
        match self.bits {
            false => SMEN_A::NOT_ENABLE,
            true => SMEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == SMEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SMEN_A::ENABLE
    }
}
#[doc = "Field `SMEN` writer - Supply Monitor Wake-up Enable"]
pub type SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUMR_SPEC, SMEN_A, O>;
impl<'a, const O: u8> SMEN_W<'a, O> {
    #[doc = "The supply monitor detection has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMEN_A::NOT_ENABLE)
    }
    #[doc = "The supply monitor detection forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMEN_A::ENABLE)
    }
}
#[doc = "Real-time Timer Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTTEN_A {
    #[doc = "0: The RTT alarm signal has no wake-up effect."]
    NOT_ENABLE = 0,
    #[doc = "1: The RTT alarm signal forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<RTTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTTEN` reader - Real-time Timer Wake-up Enable"]
pub type RTTEN_R = crate::BitReader<RTTEN_A>;
impl RTTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTTEN_A {
        match self.bits {
            false => RTTEN_A::NOT_ENABLE,
            true => RTTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == RTTEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTTEN_A::ENABLE
    }
}
#[doc = "Field `RTTEN` writer - Real-time Timer Wake-up Enable"]
pub type RTTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUMR_SPEC, RTTEN_A, O>;
impl<'a, const O: u8> RTTEN_W<'a, O> {
    #[doc = "The RTT alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(RTTEN_A::NOT_ENABLE)
    }
    #[doc = "The RTT alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTTEN_A::ENABLE)
    }
}
#[doc = "Real-time Clock Wake-up Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCEN_A {
    #[doc = "0: The RTC alarm signal has no wake-up effect."]
    NOT_ENABLE = 0,
    #[doc = "1: The RTC alarm signal forces the wake-up of the core power supply."]
    ENABLE = 1,
}
impl From<RTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCEN` reader - Real-time Clock Wake-up Enable"]
pub type RTCEN_R = crate::BitReader<RTCEN_A>;
impl RTCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCEN_A {
        match self.bits {
            false => RTCEN_A::NOT_ENABLE,
            true => RTCEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == RTCEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTCEN_A::ENABLE
    }
}
#[doc = "Field `RTCEN` writer - Real-time Clock Wake-up Enable"]
pub type RTCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUMR_SPEC, RTCEN_A, O>;
impl<'a, const O: u8> RTCEN_W<'a, O> {
    #[doc = "The RTC alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(RTCEN_A::NOT_ENABLE)
    }
    #[doc = "The RTC alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTCEN_A::ENABLE)
    }
}
#[doc = "Low-power Debouncer Enable WKUP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCEN0_A {
    #[doc = "0: The WKUP0 input pin is not connected to the low-power debouncer."]
    NOT_ENABLE = 0,
    #[doc = "1: The WKUP0 input pin is connected to the low-power debouncer and forces a system wake-up."]
    ENABLE = 1,
}
impl From<LPDBCEN0_A> for bool {
    #[inline(always)]
    fn from(variant: LPDBCEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDBCEN0` reader - Low-power Debouncer Enable WKUP0"]
pub type LPDBCEN0_R = crate::BitReader<LPDBCEN0_A>;
impl LPDBCEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBCEN0_A {
        match self.bits {
            false => LPDBCEN0_A::NOT_ENABLE,
            true => LPDBCEN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == LPDBCEN0_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LPDBCEN0_A::ENABLE
    }
}
#[doc = "Field `LPDBCEN0` writer - Low-power Debouncer Enable WKUP0"]
pub type LPDBCEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUMR_SPEC, LPDBCEN0_A, O>;
impl<'a, const O: u8> LPDBCEN0_W<'a, O> {
    #[doc = "The WKUP0 input pin is not connected to the low-power debouncer."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(LPDBCEN0_A::NOT_ENABLE)
    }
    #[doc = "The WKUP0 input pin is connected to the low-power debouncer and forces a system wake-up."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPDBCEN0_A::ENABLE)
    }
}
#[doc = "Low-power Debouncer Enable WKUP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCEN1_A {
    #[doc = "0: The WKUP1 input pin is not connected to the low-power debouncer."]
    NOT_ENABLE = 0,
    #[doc = "1: The WKUP1 input pin is connected to the low-power debouncer and forces a system wake-up."]
    ENABLE = 1,
}
impl From<LPDBCEN1_A> for bool {
    #[inline(always)]
    fn from(variant: LPDBCEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDBCEN1` reader - Low-power Debouncer Enable WKUP1"]
pub type LPDBCEN1_R = crate::BitReader<LPDBCEN1_A>;
impl LPDBCEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBCEN1_A {
        match self.bits {
            false => LPDBCEN1_A::NOT_ENABLE,
            true => LPDBCEN1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == LPDBCEN1_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LPDBCEN1_A::ENABLE
    }
}
#[doc = "Field `LPDBCEN1` writer - Low-power Debouncer Enable WKUP1"]
pub type LPDBCEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUMR_SPEC, LPDBCEN1_A, O>;
impl<'a, const O: u8> LPDBCEN1_W<'a, O> {
    #[doc = "The WKUP1 input pin is not connected to the low-power debouncer."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(LPDBCEN1_A::NOT_ENABLE)
    }
    #[doc = "The WKUP1 input pin is connected to the low-power debouncer and forces a system wake-up."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPDBCEN1_A::ENABLE)
    }
}
#[doc = "Low-power Debouncer Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCCLR_A {
    #[doc = "0: A low-power debounce event does not create an immediate clear on the first half of GPBR registers."]
    NOT_ENABLE = 0,
    #[doc = "1: A low-power debounce event on WKUP0 or WKUP1 generates an immediate clear on the first half of GPBR registers."]
    ENABLE = 1,
}
impl From<LPDBCCLR_A> for bool {
    #[inline(always)]
    fn from(variant: LPDBCCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDBCCLR` reader - Low-power Debouncer Clear"]
pub type LPDBCCLR_R = crate::BitReader<LPDBCCLR_A>;
impl LPDBCCLR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBCCLR_A {
        match self.bits {
            false => LPDBCCLR_A::NOT_ENABLE,
            true => LPDBCCLR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == LPDBCCLR_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LPDBCCLR_A::ENABLE
    }
}
#[doc = "Field `LPDBCCLR` writer - Low-power Debouncer Clear"]
pub type LPDBCCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUMR_SPEC, LPDBCCLR_A, O>;
impl<'a, const O: u8> LPDBCCLR_W<'a, O> {
    #[doc = "A low-power debounce event does not create an immediate clear on the first half of GPBR registers."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(LPDBCCLR_A::NOT_ENABLE)
    }
    #[doc = "A low-power debounce event on WKUP0 or WKUP1 generates an immediate clear on the first half of GPBR registers."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPDBCCLR_A::ENABLE)
    }
}
#[doc = "Wake-up Inputs Debouncer Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WKUPDBC_A {
    #[doc = "0: Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    IMMEDIATE = 0,
    #[doc = "1: WKUPx shall be in its active state for at least 3 SLCK periods"]
    _3_SLCK = 1,
    #[doc = "2: WKUPx shall be in its active state for at least 32 SLCK periods"]
    _32_SLCK = 2,
    #[doc = "3: WKUPx shall be in its active state for at least 512 SLCK periods"]
    _512_SLCK = 3,
    #[doc = "4: WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    _4096_SLCK = 4,
    #[doc = "5: WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    _32768_SLCK = 5,
}
impl From<WKUPDBC_A> for u8 {
    #[inline(always)]
    fn from(variant: WKUPDBC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WKUPDBC` reader - Wake-up Inputs Debouncer Period"]
pub type WKUPDBC_R = crate::FieldReader<u8, WKUPDBC_A>;
impl WKUPDBC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WKUPDBC_A> {
        match self.bits {
            0 => Some(WKUPDBC_A::IMMEDIATE),
            1 => Some(WKUPDBC_A::_3_SLCK),
            2 => Some(WKUPDBC_A::_32_SLCK),
            3 => Some(WKUPDBC_A::_512_SLCK),
            4 => Some(WKUPDBC_A::_4096_SLCK),
            5 => Some(WKUPDBC_A::_32768_SLCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE`"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == WKUPDBC_A::IMMEDIATE
    }
    #[doc = "Checks if the value of the field is `_3_SLCK`"]
    #[inline(always)]
    pub fn is_3_slck(&self) -> bool {
        *self == WKUPDBC_A::_3_SLCK
    }
    #[doc = "Checks if the value of the field is `_32_SLCK`"]
    #[inline(always)]
    pub fn is_32_slck(&self) -> bool {
        *self == WKUPDBC_A::_32_SLCK
    }
    #[doc = "Checks if the value of the field is `_512_SLCK`"]
    #[inline(always)]
    pub fn is_512_slck(&self) -> bool {
        *self == WKUPDBC_A::_512_SLCK
    }
    #[doc = "Checks if the value of the field is `_4096_SLCK`"]
    #[inline(always)]
    pub fn is_4096_slck(&self) -> bool {
        *self == WKUPDBC_A::_4096_SLCK
    }
    #[doc = "Checks if the value of the field is `_32768_SLCK`"]
    #[inline(always)]
    pub fn is_32768_slck(&self) -> bool {
        *self == WKUPDBC_A::_32768_SLCK
    }
}
#[doc = "Field `WKUPDBC` writer - Wake-up Inputs Debouncer Period"]
pub type WKUPDBC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUMR_SPEC, u8, WKUPDBC_A, 3, O>;
impl<'a, const O: u8> WKUPDBC_W<'a, O> {
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(WKUPDBC_A::IMMEDIATE)
    }
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    #[inline(always)]
    pub fn _3_slck(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_3_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    #[inline(always)]
    pub fn _32_slck(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_32_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    #[inline(always)]
    pub fn _512_slck(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_512_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    #[inline(always)]
    pub fn _4096_slck(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_4096_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    #[inline(always)]
    pub fn _32768_slck(self) -> &'a mut W {
        self.variant(WKUPDBC_A::_32768_SLCK)
    }
}
#[doc = "Low-power Debouncer Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPDBC_A {
    #[doc = "0: Disable the low-power debouncers."]
    DISABLE = 0,
    #[doc = "1: WKUP0/1 in active state for at least 2 RTCOUTx clock periods"]
    _2_RTCOUT = 1,
    #[doc = "2: WKUP0/1 in active state for at least 3 RTCOUTx clock periods"]
    _3_RTCOUT = 2,
    #[doc = "3: WKUP0/1 in active state for at least 4 RTCOUTx clock periods"]
    _4_RTCOUT = 3,
    #[doc = "4: WKUP0/1 in active state for at least 5 RTCOUTx clock periods"]
    _5_RTCOUT = 4,
    #[doc = "5: WKUP0/1 in active state for at least 6 RTCOUTx clock periods"]
    _6_RTCOUT = 5,
    #[doc = "6: WKUP0/1 in active state for at least 7 RTCOUTx clock periods"]
    _7_RTCOUT = 6,
    #[doc = "7: WKUP0/1 in active state for at least 8 RTCOUTx clock periods"]
    _8_RTCOUT = 7,
}
impl From<LPDBC_A> for u8 {
    #[inline(always)]
    fn from(variant: LPDBC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPDBC` reader - Low-power Debouncer Period"]
pub type LPDBC_R = crate::FieldReader<u8, LPDBC_A>;
impl LPDBC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBC_A {
        match self.bits {
            0 => LPDBC_A::DISABLE,
            1 => LPDBC_A::_2_RTCOUT,
            2 => LPDBC_A::_3_RTCOUT,
            3 => LPDBC_A::_4_RTCOUT,
            4 => LPDBC_A::_5_RTCOUT,
            5 => LPDBC_A::_6_RTCOUT,
            6 => LPDBC_A::_7_RTCOUT,
            7 => LPDBC_A::_8_RTCOUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LPDBC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `_2_RTCOUT`"]
    #[inline(always)]
    pub fn is_2_rtcout(&self) -> bool {
        *self == LPDBC_A::_2_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_3_RTCOUT`"]
    #[inline(always)]
    pub fn is_3_rtcout(&self) -> bool {
        *self == LPDBC_A::_3_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_4_RTCOUT`"]
    #[inline(always)]
    pub fn is_4_rtcout(&self) -> bool {
        *self == LPDBC_A::_4_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_5_RTCOUT`"]
    #[inline(always)]
    pub fn is_5_rtcout(&self) -> bool {
        *self == LPDBC_A::_5_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_6_RTCOUT`"]
    #[inline(always)]
    pub fn is_6_rtcout(&self) -> bool {
        *self == LPDBC_A::_6_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_7_RTCOUT`"]
    #[inline(always)]
    pub fn is_7_rtcout(&self) -> bool {
        *self == LPDBC_A::_7_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_8_RTCOUT`"]
    #[inline(always)]
    pub fn is_8_rtcout(&self) -> bool {
        *self == LPDBC_A::_8_RTCOUT
    }
}
#[doc = "Field `LPDBC` writer - Low-power Debouncer Period"]
pub type LPDBC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, WUMR_SPEC, u8, LPDBC_A, 3, O>;
impl<'a, const O: u8> LPDBC_W<'a, O> {
    #[doc = "Disable the low-power debouncers."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LPDBC_A::DISABLE)
    }
    #[doc = "WKUP0/1 in active state for at least 2 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _2_rtcout(self) -> &'a mut W {
        self.variant(LPDBC_A::_2_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 3 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _3_rtcout(self) -> &'a mut W {
        self.variant(LPDBC_A::_3_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 4 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _4_rtcout(self) -> &'a mut W {
        self.variant(LPDBC_A::_4_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 5 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _5_rtcout(self) -> &'a mut W {
        self.variant(LPDBC_A::_5_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 6 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _6_rtcout(self) -> &'a mut W {
        self.variant(LPDBC_A::_6_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 7 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _7_rtcout(self) -> &'a mut W {
        self.variant(LPDBC_A::_7_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 8 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _8_rtcout(self) -> &'a mut W {
        self.variant(LPDBC_A::_8_RTCOUT)
    }
}
impl R {
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SMEN_R {
        SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Real-time Timer Wake-up Enable"]
    #[inline(always)]
    pub fn rtten(&self) -> RTTEN_R {
        RTTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Real-time Clock Wake-up Enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Low-power Debouncer Enable WKUP0"]
    #[inline(always)]
    pub fn lpdbcen0(&self) -> LPDBCEN0_R {
        LPDBCEN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low-power Debouncer Enable WKUP1"]
    #[inline(always)]
    pub fn lpdbcen1(&self) -> LPDBCEN1_R {
        LPDBCEN1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Low-power Debouncer Clear"]
    #[inline(always)]
    pub fn lpdbcclr(&self) -> LPDBCCLR_R {
        LPDBCCLR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline(always)]
    pub fn wkupdbc(&self) -> WKUPDBC_R {
        WKUPDBC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Low-power Debouncer Period"]
    #[inline(always)]
    pub fn lpdbc(&self) -> LPDBC_R {
        LPDBC_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline(always)]
    pub fn smen(&mut self) -> SMEN_W<1> {
        SMEN_W::new(self)
    }
    #[doc = "Bit 2 - Real-time Timer Wake-up Enable"]
    #[inline(always)]
    pub fn rtten(&mut self) -> RTTEN_W<2> {
        RTTEN_W::new(self)
    }
    #[doc = "Bit 3 - Real-time Clock Wake-up Enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W<3> {
        RTCEN_W::new(self)
    }
    #[doc = "Bit 5 - Low-power Debouncer Enable WKUP0"]
    #[inline(always)]
    pub fn lpdbcen0(&mut self) -> LPDBCEN0_W<5> {
        LPDBCEN0_W::new(self)
    }
    #[doc = "Bit 6 - Low-power Debouncer Enable WKUP1"]
    #[inline(always)]
    pub fn lpdbcen1(&mut self) -> LPDBCEN1_W<6> {
        LPDBCEN1_W::new(self)
    }
    #[doc = "Bit 7 - Low-power Debouncer Clear"]
    #[inline(always)]
    pub fn lpdbcclr(&mut self) -> LPDBCCLR_W<7> {
        LPDBCCLR_W::new(self)
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline(always)]
    pub fn wkupdbc(&mut self) -> WKUPDBC_W<12> {
        WKUPDBC_W::new(self)
    }
    #[doc = "Bits 16:18 - Low-power Debouncer Period"]
    #[inline(always)]
    pub fn lpdbc(&mut self) -> LPDBC_W<16> {
        LPDBC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supply Controller Wake-up Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wumr](index.html) module"]
pub struct WUMR_SPEC;
impl crate::RegisterSpec for WUMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wumr::R](R) reader structure"]
impl crate::Readable for WUMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wumr::W](W) writer structure"]
impl crate::Writable for WUMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WUMR to value 0"]
impl crate::Resettable for WUMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
