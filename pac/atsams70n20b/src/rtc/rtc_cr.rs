#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RTC_CR {
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
pub type UPDTIM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _UPDTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDTIMW<'a> {
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
pub type UPDCAL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _UPDCALW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDCALW<'a> {
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
impl crate::ToBits<u8> for TIMEVSELR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TIMEVSELR::MINUTE => 0,
            TIMEVSELR::HOUR => 1,
            TIMEVSELR::MIDNIGHT => 2,
            TIMEVSELR::NOON => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TIMEVSEL_R = crate::FR<u8, TIMEVSELR>;
impl TIMEVSEL_R {
    #[doc = "Checks if the value of the field is `MINUTE`"]
    #[inline(always)]
    pub fn is_minute(&self) -> bool {
        *self == TIMEVSELR::MINUTE
    }
    #[doc = "Checks if the value of the field is `HOUR`"]
    #[inline(always)]
    pub fn is_hour(&self) -> bool {
        *self == TIMEVSELR::HOUR
    }
    #[doc = "Checks if the value of the field is `MIDNIGHT`"]
    #[inline(always)]
    pub fn is_midnight(&self) -> bool {
        *self == TIMEVSELR::MIDNIGHT
    }
    #[doc = "Checks if the value of the field is `NOON`"]
    #[inline(always)]
    pub fn is_noon(&self) -> bool {
        *self == TIMEVSELR::NOON
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMEVSELW::MINUTE => 0,
            TIMEVSELW::HOUR => 1,
            TIMEVSELW::MIDNIGHT => 2,
            TIMEVSELW::NOON => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TIMEVSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEVSELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEVSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Minute change"]
    #[inline(always)]
    pub fn minute(self) -> &'a mut W {
        self.variant(TIMEVSELW::MINUTE)
    }
    #[doc = "Hour change"]
    #[inline(always)]
    pub fn hour(self) -> &'a mut W {
        self.variant(TIMEVSELW::HOUR)
    }
    #[doc = "Every day at midnight"]
    #[inline(always)]
    pub fn midnight(self) -> &'a mut W {
        self.variant(TIMEVSELW::MIDNIGHT)
    }
    #[doc = "Every day at noon"]
    #[inline(always)]
    pub fn noon(self) -> &'a mut W {
        self.variant(TIMEVSELW::NOON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
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
}
impl crate::ToBits<u8> for CALEVSELR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CALEVSELR::WEEK => 0,
            CALEVSELR::MONTH => 1,
            CALEVSELR::YEAR => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CALEVSEL_R = crate::FR<u8, CALEVSELR>;
impl CALEVSEL_R {
    #[doc = "Checks if the value of the field is `WEEK`"]
    #[inline(always)]
    pub fn is_week(&self) -> bool {
        *self == CALEVSELR::WEEK
    }
    #[doc = "Checks if the value of the field is `MONTH`"]
    #[inline(always)]
    pub fn is_month(&self) -> bool {
        *self == CALEVSELR::MONTH
    }
    #[doc = "Checks if the value of the field is `YEAR`"]
    #[inline(always)]
    pub fn is_year(&self) -> bool {
        *self == CALEVSELR::YEAR
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CALEVSELW::WEEK => 0,
            CALEVSELW::MONTH => 1,
            CALEVSELW::YEAR => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CALEVSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CALEVSELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALEVSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Week change (every Monday at time 00:00:00)"]
    #[inline(always)]
    pub fn week(self) -> &'a mut W {
        self.variant(CALEVSELW::WEEK)
    }
    #[doc = "Month change (every 01 of each month at time 00:00:00)"]
    #[inline(always)]
    pub fn month(self) -> &'a mut W {
        self.variant(CALEVSELW::MONTH)
    }
    #[doc = "Year change (every January 1 at time 00:00:00)"]
    #[inline(always)]
    pub fn year(self) -> &'a mut W {
        self.variant(CALEVSELW::YEAR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Update Request Time Register"]
    #[inline(always)]
    pub fn updtim(&self) -> UPDTIM_R {
        UPDTIM_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Update Request Calendar Register"]
    #[inline(always)]
    pub fn updcal(&self) -> UPDCAL_R {
        UPDCAL_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Time Event Selection"]
    #[inline(always)]
    pub fn timevsel(&self) -> TIMEVSEL_R {
        TIMEVSEL_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Calendar Event Selection"]
    #[inline(always)]
    pub fn calevsel(&self) -> CALEVSEL_R {
        CALEVSEL_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Update Request Time Register"]
    #[inline(always)]
    pub fn updtim(&mut self) -> _UPDTIMW {
        _UPDTIMW { w: self }
    }
    #[doc = "Bit 1 - Update Request Calendar Register"]
    #[inline(always)]
    pub fn updcal(&mut self) -> _UPDCALW {
        _UPDCALW { w: self }
    }
    #[doc = "Bits 8:9 - Time Event Selection"]
    #[inline(always)]
    pub fn timevsel(&mut self) -> _TIMEVSELW {
        _TIMEVSELW { w: self }
    }
    #[doc = "Bits 16:17 - Calendar Event Selection"]
    #[inline(always)]
    pub fn calevsel(&mut self) -> _CALEVSELW {
        _CALEVSELW { w: self }
    }
}
