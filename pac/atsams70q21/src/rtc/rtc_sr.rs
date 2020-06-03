#[doc = "Reader of register RTC_SR"]
pub type R = crate::R<u32, super::RTC_SR>;
#[doc = "Acknowledge for Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKUPD_A {
    #[doc = "0: Time and calendar registers cannot be updated."]
    FREERUN = 0,
    #[doc = "1: Time and calendar registers can be updated."]
    UPDATE = 1,
}
impl From<ACKUPD_A> for bool {
    #[inline(always)]
    fn from(variant: ACKUPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACKUPD`"]
pub type ACKUPD_R = crate::R<bool, ACKUPD_A>;
impl ACKUPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACKUPD_A {
        match self.bits {
            false => ACKUPD_A::FREERUN,
            true => ACKUPD_A::UPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `FREERUN`"]
    #[inline(always)]
    pub fn is_freerun(&self) -> bool {
        *self == ACKUPD_A::FREERUN
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == ACKUPD_A::UPDATE
    }
}
#[doc = "Alarm Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARM_A {
    #[doc = "0: No alarm matching condition occurred."]
    NO_ALARMEVENT = 0,
    #[doc = "1: An alarm matching condition has occurred."]
    ALARMEVENT = 1,
}
impl From<ALARM_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALARM`"]
pub type ALARM_R = crate::R<bool, ALARM_A>;
impl ALARM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARM_A {
        match self.bits {
            false => ALARM_A::NO_ALARMEVENT,
            true => ALARM_A::ALARMEVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ALARMEVENT`"]
    #[inline(always)]
    pub fn is_no_alarmevent(&self) -> bool {
        *self == ALARM_A::NO_ALARMEVENT
    }
    #[doc = "Checks if the value of the field is `ALARMEVENT`"]
    #[inline(always)]
    pub fn is_alarmevent(&self) -> bool {
        *self == ALARM_A::ALARMEVENT
    }
}
#[doc = "Second Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEC_A {
    #[doc = "0: No second event has occurred since the last clear."]
    NO_SECEVENT = 0,
    #[doc = "1: At least one second event has occurred since the last clear."]
    SECEVENT = 1,
}
impl From<SEC_A> for bool {
    #[inline(always)]
    fn from(variant: SEC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEC`"]
pub type SEC_R = crate::R<bool, SEC_A>;
impl SEC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEC_A {
        match self.bits {
            false => SEC_A::NO_SECEVENT,
            true => SEC_A::SECEVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SECEVENT`"]
    #[inline(always)]
    pub fn is_no_secevent(&self) -> bool {
        *self == SEC_A::NO_SECEVENT
    }
    #[doc = "Checks if the value of the field is `SECEVENT`"]
    #[inline(always)]
    pub fn is_secevent(&self) -> bool {
        *self == SEC_A::SECEVENT
    }
}
#[doc = "Time Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEV_A {
    #[doc = "0: No time event has occurred since the last clear."]
    NO_TIMEVENT = 0,
    #[doc = "1: At least one time event has occurred since the last clear."]
    TIMEVENT = 1,
}
impl From<TIMEV_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMEV`"]
pub type TIMEV_R = crate::R<bool, TIMEV_A>;
impl TIMEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEV_A {
        match self.bits {
            false => TIMEV_A::NO_TIMEVENT,
            true => TIMEV_A::TIMEVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIMEVENT`"]
    #[inline(always)]
    pub fn is_no_timevent(&self) -> bool {
        *self == TIMEV_A::NO_TIMEVENT
    }
    #[doc = "Checks if the value of the field is `TIMEVENT`"]
    #[inline(always)]
    pub fn is_timevent(&self) -> bool {
        *self == TIMEV_A::TIMEVENT
    }
}
#[doc = "Calendar Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALEV_A {
    #[doc = "0: No calendar event has occurred since the last clear."]
    NO_CALEVENT = 0,
    #[doc = "1: At least one calendar event has occurred since the last clear."]
    CALEVENT = 1,
}
impl From<CALEV_A> for bool {
    #[inline(always)]
    fn from(variant: CALEV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CALEV`"]
pub type CALEV_R = crate::R<bool, CALEV_A>;
impl CALEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALEV_A {
        match self.bits {
            false => CALEV_A::NO_CALEVENT,
            true => CALEV_A::CALEVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CALEVENT`"]
    #[inline(always)]
    pub fn is_no_calevent(&self) -> bool {
        *self == CALEV_A::NO_CALEVENT
    }
    #[doc = "Checks if the value of the field is `CALEVENT`"]
    #[inline(always)]
    pub fn is_calevent(&self) -> bool {
        *self == CALEV_A::CALEVENT
    }
}
#[doc = "Time and/or Date Free Running Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDERR_A {
    #[doc = "0: The internal free running counters are carrying valid values since the last read of the Status Register (RTC_SR)."]
    CORRECT = 0,
    #[doc = "1: The internal free running counters have been corrupted (invalid date or time, non-BCD values) since the last read and/or they are still invalid."]
    ERR_TIMEDATE = 1,
}
impl From<TDERR_A> for bool {
    #[inline(always)]
    fn from(variant: TDERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDERR`"]
pub type TDERR_R = crate::R<bool, TDERR_A>;
impl TDERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDERR_A {
        match self.bits {
            false => TDERR_A::CORRECT,
            true => TDERR_A::ERR_TIMEDATE,
        }
    }
    #[doc = "Checks if the value of the field is `CORRECT`"]
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        *self == TDERR_A::CORRECT
    }
    #[doc = "Checks if the value of the field is `ERR_TIMEDATE`"]
    #[inline(always)]
    pub fn is_err_timedate(&self) -> bool {
        *self == TDERR_A::ERR_TIMEDATE
    }
}
impl R {
    #[doc = "Bit 0 - Acknowledge for Update"]
    #[inline(always)]
    pub fn ackupd(&self) -> ACKUPD_R {
        ACKUPD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    pub fn alarm(&self) -> ALARM_R {
        ALARM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Second Event"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Time Event"]
    #[inline(always)]
    pub fn timev(&self) -> TIMEV_R {
        TIMEV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Calendar Event"]
    #[inline(always)]
    pub fn calev(&self) -> CALEV_R {
        CALEV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Time and/or Date Free Running Error"]
    #[inline(always)]
    pub fn tderr(&self) -> TDERR_R {
        TDERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
