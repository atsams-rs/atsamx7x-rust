#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RTC_SR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
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
impl ACKUPDR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ACKUPDR::FREERUN => false,
            ACKUPDR::UPDATE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACKUPDR {
        match value {
            false => ACKUPDR::FREERUN,
            true => ACKUPDR::UPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `FREERUN`"]
    #[inline]
    pub fn is_freerun(&self) -> bool {
        *self == ACKUPDR::FREERUN
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline]
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
impl ALARMR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            ALARMR::NO_ALARMEVENT => false,
            ALARMR::ALARMEVENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALARMR {
        match value {
            false => ALARMR::NO_ALARMEVENT,
            true => ALARMR::ALARMEVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ALARMEVENT`"]
    #[inline]
    pub fn is_no_alarmevent(&self) -> bool {
        *self == ALARMR::NO_ALARMEVENT
    }
    #[doc = "Checks if the value of the field is `ALARMEVENT`"]
    #[inline]
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
impl SECR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SECR::NO_SECEVENT => false,
            SECR::SECEVENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SECR {
        match value {
            false => SECR::NO_SECEVENT,
            true => SECR::SECEVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SECEVENT`"]
    #[inline]
    pub fn is_no_secevent(&self) -> bool {
        *self == SECR::NO_SECEVENT
    }
    #[doc = "Checks if the value of the field is `SECEVENT`"]
    #[inline]
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
impl TIMEVR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TIMEVR::NO_TIMEVENT => false,
            TIMEVR::TIMEVENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIMEVR {
        match value {
            false => TIMEVR::NO_TIMEVENT,
            true => TIMEVR::TIMEVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIMEVENT`"]
    #[inline]
    pub fn is_no_timevent(&self) -> bool {
        *self == TIMEVR::NO_TIMEVENT
    }
    #[doc = "Checks if the value of the field is `TIMEVENT`"]
    #[inline]
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
impl CALEVR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            CALEVR::NO_CALEVENT => false,
            CALEVR::CALEVENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CALEVR {
        match value {
            false => CALEVR::NO_CALEVENT,
            true => CALEVR::CALEVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CALEVENT`"]
    #[inline]
    pub fn is_no_calevent(&self) -> bool {
        *self == CALEVR::NO_CALEVENT
    }
    #[doc = "Checks if the value of the field is `CALEVENT`"]
    #[inline]
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
impl TDERRR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TDERRR::CORRECT => false,
            TDERRR::ERR_TIMEDATE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDERRR {
        match value {
            false => TDERRR::CORRECT,
            true => TDERRR::ERR_TIMEDATE,
        }
    }
    #[doc = "Checks if the value of the field is `CORRECT`"]
    #[inline]
    pub fn is_correct(&self) -> bool {
        *self == TDERRR::CORRECT
    }
    #[doc = "Checks if the value of the field is `ERR_TIMEDATE`"]
    #[inline]
    pub fn is_err_timedate(&self) -> bool {
        *self == TDERRR::ERR_TIMEDATE
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Acknowledge for Update"]
    #[inline]
    pub fn ackupd(&self) -> ACKUPDR {
        ACKUPDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Alarm Flag"]
    #[inline]
    pub fn alarm(&self) -> ALARMR {
        ALARMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Second Event"]
    #[inline]
    pub fn sec(&self) -> SECR {
        SECR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Time Event"]
    #[inline]
    pub fn timev(&self) -> TIMEVR {
        TIMEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Calendar Event"]
    #[inline]
    pub fn calev(&self) -> CALEVR {
        CALEVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Time and/or Date Free Running Error"]
    #[inline]
    pub fn tderr(&self) -> TDERRR {
        TDERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
