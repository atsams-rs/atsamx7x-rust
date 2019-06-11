#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RTC_CR {
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
#[doc = r" Value of the field"]
pub struct UPDTIMR {
    bits: bool,
}
impl UPDTIMR {
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
pub struct UPDCALR {
    bits: bool,
}
impl UPDCALR {
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
#[doc = "Possible values of the field `TIMEVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEVSELR {
    #[doc = "Minute change"]
    MINUTE,
    #[doc = "Hour change"]
    HOUR,
    #[doc = "Every day at midnight"]
    MIDNIGHT,
    #[doc = "Every day at noon"]
    NOON,
}
impl TIMEVSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMEVSELR::MINUTE => 0,
            TIMEVSELR::HOUR => 1,
            TIMEVSELR::MIDNIGHT => 2,
            TIMEVSELR::NOON => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMEVSELR {
        match value {
            0 => TIMEVSELR::MINUTE,
            1 => TIMEVSELR::HOUR,
            2 => TIMEVSELR::MIDNIGHT,
            3 => TIMEVSELR::NOON,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MINUTE`"]
    #[inline]
    pub fn is_minute(&self) -> bool {
        *self == TIMEVSELR::MINUTE
    }
    #[doc = "Checks if the value of the field is `HOUR`"]
    #[inline]
    pub fn is_hour(&self) -> bool {
        *self == TIMEVSELR::HOUR
    }
    #[doc = "Checks if the value of the field is `MIDNIGHT`"]
    #[inline]
    pub fn is_midnight(&self) -> bool {
        *self == TIMEVSELR::MIDNIGHT
    }
    #[doc = "Checks if the value of the field is `NOON`"]
    #[inline]
    pub fn is_noon(&self) -> bool {
        *self == TIMEVSELR::NOON
    }
}
#[doc = "Possible values of the field `CALEVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALEVSELR {
    #[doc = "Week change (every Monday at time 00:00:00)"]
    WEEK,
    #[doc = "Month change (every 01 of each month at time 00:00:00)"]
    MONTH,
    #[doc = "Year change (every January 1 at time 00:00:00)"]
    YEAR,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CALEVSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CALEVSELR::WEEK => 0,
            CALEVSELR::MONTH => 1,
            CALEVSELR::YEAR => 2,
            CALEVSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CALEVSELR {
        match value {
            0 => CALEVSELR::WEEK,
            1 => CALEVSELR::MONTH,
            2 => CALEVSELR::YEAR,
            i => CALEVSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `WEEK`"]
    #[inline]
    pub fn is_week(&self) -> bool {
        *self == CALEVSELR::WEEK
    }
    #[doc = "Checks if the value of the field is `MONTH`"]
    #[inline]
    pub fn is_month(&self) -> bool {
        *self == CALEVSELR::MONTH
    }
    #[doc = "Checks if the value of the field is `YEAR`"]
    #[inline]
    pub fn is_year(&self) -> bool {
        *self == CALEVSELR::YEAR
    }
}
#[doc = r" Proxy"]
pub struct _UPDTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDTIMW<'a> {
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UPDCALW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDCALW<'a> {
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMEVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEVSELW {
    #[doc = "Minute change"]
    MINUTE,
    #[doc = "Hour change"]
    HOUR,
    #[doc = "Every day at midnight"]
    MIDNIGHT,
    #[doc = "Every day at noon"]
    NOON,
}
impl TIMEVSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMEVSELW::MINUTE => 0,
            TIMEVSELW::HOUR => 1,
            TIMEVSELW::MIDNIGHT => 2,
            TIMEVSELW::NOON => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMEVSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEVSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMEVSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Minute change"]
    #[inline]
    pub fn minute(self) -> &'a mut W {
        self.variant(TIMEVSELW::MINUTE)
    }
    #[doc = "Hour change"]
    #[inline]
    pub fn hour(self) -> &'a mut W {
        self.variant(TIMEVSELW::HOUR)
    }
    #[doc = "Every day at midnight"]
    #[inline]
    pub fn midnight(self) -> &'a mut W {
        self.variant(TIMEVSELW::MIDNIGHT)
    }
    #[doc = "Every day at noon"]
    #[inline]
    pub fn noon(self) -> &'a mut W {
        self.variant(TIMEVSELW::NOON)
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
#[doc = "Values that can be written to the field `CALEVSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALEVSELW {
    #[doc = "Week change (every Monday at time 00:00:00)"]
    WEEK,
    #[doc = "Month change (every 01 of each month at time 00:00:00)"]
    MONTH,
    #[doc = "Year change (every January 1 at time 00:00:00)"]
    YEAR,
}
impl CALEVSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CALEVSELW::WEEK => 0,
            CALEVSELW::MONTH => 1,
            CALEVSELW::YEAR => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CALEVSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CALEVSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CALEVSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Week change (every Monday at time 00:00:00)"]
    #[inline]
    pub fn week(self) -> &'a mut W {
        self.variant(CALEVSELW::WEEK)
    }
    #[doc = "Month change (every 01 of each month at time 00:00:00)"]
    #[inline]
    pub fn month(self) -> &'a mut W {
        self.variant(CALEVSELW::MONTH)
    }
    #[doc = "Year change (every January 1 at time 00:00:00)"]
    #[inline]
    pub fn year(self) -> &'a mut W {
        self.variant(CALEVSELW::YEAR)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Update Request Time Register"]
    #[inline]
    pub fn updtim(&self) -> UPDTIMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UPDTIMR { bits }
    }
    #[doc = "Bit 1 - Update Request Calendar Register"]
    #[inline]
    pub fn updcal(&self) -> UPDCALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UPDCALR { bits }
    }
    #[doc = "Bits 8:9 - Time Event Selection"]
    #[inline]
    pub fn timevsel(&self) -> TIMEVSELR {
        TIMEVSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Calendar Event Selection"]
    #[inline]
    pub fn calevsel(&self) -> CALEVSELR {
        CALEVSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Update Request Time Register"]
    #[inline]
    pub fn updtim(&mut self) -> _UPDTIMW {
        _UPDTIMW { w: self }
    }
    #[doc = "Bit 1 - Update Request Calendar Register"]
    #[inline]
    pub fn updcal(&mut self) -> _UPDCALW {
        _UPDCALW { w: self }
    }
    #[doc = "Bits 8:9 - Time Event Selection"]
    #[inline]
    pub fn timevsel(&mut self) -> _TIMEVSELW {
        _TIMEVSELW { w: self }
    }
    #[doc = "Bits 16:17 - Calendar Event Selection"]
    #[inline]
    pub fn calevsel(&mut self) -> _CALEVSELW {
        _CALEVSELW { w: self }
    }
}
