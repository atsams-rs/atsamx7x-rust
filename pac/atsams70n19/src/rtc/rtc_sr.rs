#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RTC_SR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `ACKUPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKUPDR {
    #[doc = "Time and calendar registers cannot be updated."]
    FREERUN,
    #[doc = "Time and calendar registers can be updated."]
    UPDATE,
}
impl crate::ToBits<bool> for ACKUPDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ACKUPDR::FREERUN => false,
            ACKUPDR::UPDATE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ACKUPD_R = crate::FR<bool, ACKUPDR>;
impl ACKUPD_R {
    #[doc = "Checks if the value of the field is `FREERUN`"]
    #[inline(always)]
    pub fn is_freerun(&self) -> bool {
        *self == ACKUPDR::FREERUN
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == ACKUPDR::UPDATE
    }
}
#[doc = "Possible values of the field `ALARM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARMR {
    #[doc = "No alarm matching condition occurred."]
    NO_ALARMEVENT,
    #[doc = "An alarm matching condition has occurred."]
    ALARMEVENT,
}
impl crate::ToBits<bool> for ALARMR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ALARMR::NO_ALARMEVENT => false,
            ALARMR::ALARMEVENT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ALARM_R = crate::FR<bool, ALARMR>;
impl ALARM_R {
    #[doc = "Checks if the value of the field is `NO_ALARMEVENT`"]
    #[inline(always)]
    pub fn is_no_alarmevent(&self) -> bool {
        *self == ALARMR::NO_ALARMEVENT
    }
    #[doc = "Checks if the value of the field is `ALARMEVENT`"]
    #[inline(always)]
    pub fn is_alarmevent(&self) -> bool {
        *self == ALARMR::ALARMEVENT
    }
}
#[doc = "Possible values of the field `SEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECR {
    #[doc = "No second event has occurred since the last clear."]
    NO_SECEVENT,
    #[doc = "At least one second event has occurred since the last clear."]
    SECEVENT,
}
impl crate::ToBits<bool> for SECR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SECR::NO_SECEVENT => false,
            SECR::SECEVENT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SEC_R = crate::FR<bool, SECR>;
impl SEC_R {
    #[doc = "Checks if the value of the field is `NO_SECEVENT`"]
    #[inline(always)]
    pub fn is_no_secevent(&self) -> bool {
        *self == SECR::NO_SECEVENT
    }
    #[doc = "Checks if the value of the field is `SECEVENT`"]
    #[inline(always)]
    pub fn is_secevent(&self) -> bool {
        *self == SECR::SECEVENT
    }
}
#[doc = "Possible values of the field `TIMEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEVR {
    #[doc = "No time event has occurred since the last clear."]
    NO_TIMEVENT,
    #[doc = "At least one time event has occurred since the last clear."]
    TIMEVENT,
}
impl crate::ToBits<bool> for TIMEVR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TIMEVR::NO_TIMEVENT => false,
            TIMEVR::TIMEVENT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TIMEV_R = crate::FR<bool, TIMEVR>;
impl TIMEV_R {
    #[doc = "Checks if the value of the field is `NO_TIMEVENT`"]
    #[inline(always)]
    pub fn is_no_timevent(&self) -> bool {
        *self == TIMEVR::NO_TIMEVENT
    }
    #[doc = "Checks if the value of the field is `TIMEVENT`"]
    #[inline(always)]
    pub fn is_timevent(&self) -> bool {
        *self == TIMEVR::TIMEVENT
    }
}
#[doc = "Possible values of the field `CALEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALEVR {
    #[doc = "No calendar event has occurred since the last clear."]
    NO_CALEVENT,
    #[doc = "At least one calendar event has occurred since the last clear."]
    CALEVENT,
}
impl crate::ToBits<bool> for CALEVR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CALEVR::NO_CALEVENT => false,
            CALEVR::CALEVENT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CALEV_R = crate::FR<bool, CALEVR>;
impl CALEV_R {
    #[doc = "Checks if the value of the field is `NO_CALEVENT`"]
    #[inline(always)]
    pub fn is_no_calevent(&self) -> bool {
        *self == CALEVR::NO_CALEVENT
    }
    #[doc = "Checks if the value of the field is `CALEVENT`"]
    #[inline(always)]
    pub fn is_calevent(&self) -> bool {
        *self == CALEVR::CALEVENT
    }
}
#[doc = "Possible values of the field `TDERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDERRR {
    #[doc = "The internal free running counters are carrying valid values since the last read of the Status Register (RTC_SR)."]
    CORRECT,
    #[doc = "The internal free running counters have been corrupted (invalid date or time, non-BCD values) since the last read and/or they are still invalid."]
    ERR_TIMEDATE,
}
impl crate::ToBits<bool> for TDERRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TDERRR::CORRECT => false,
            TDERRR::ERR_TIMEDATE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TDERR_R = crate::FR<bool, TDERRR>;
impl TDERR_R {
    #[doc = "Checks if the value of the field is `CORRECT`"]
    #[inline(always)]
    pub fn is_correct(&self) -> bool {
        *self == TDERRR::CORRECT
    }
    #[doc = "Checks if the value of the field is `ERR_TIMEDATE`"]
    #[inline(always)]
    pub fn is_err_timedate(&self) -> bool {
        *self == TDERRR::ERR_TIMEDATE
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Acknowledge for Update"]
    #[inline(always)]
    pub fn ackupd(&self) -> ACKUPD_R {
        ACKUPD_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline(always)]
    pub fn alarm(&self) -> ALARM_R {
        ALARM_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Second Event"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Time Event"]
    #[inline(always)]
    pub fn timev(&self) -> TIMEV_R {
        TIMEV_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Calendar Event"]
    #[inline(always)]
    pub fn calev(&self) -> CALEV_R {
        CALEV_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Time and/or Date Free Running Error"]
    #[inline(always)]
    pub fn tderr(&self) -> TDERR_R {
        TDERR_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
}
