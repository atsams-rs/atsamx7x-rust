#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFEC_MR {
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
#[doc = "Possible values of the field `TRGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGENR {
    #[doc = "Hardware triggers are disabled. Starting a conversion is only possible by software."]
    DIS,
    #[doc = "Hardware trigger selected by TRGSEL field is enabled."]
    EN,
}
impl crate::ToBits<bool> for TRGENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TRGENR::DIS => false,
            TRGENR::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TRGEN_R = crate::FR<bool, TRGENR>;
impl TRGEN_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TRGENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TRGENR::EN
    }
}
#[doc = "Values that can be written to the field `TRGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGENW {
    #[doc = "Hardware triggers are disabled. Starting a conversion is only possible by software."]
    DIS,
    #[doc = "Hardware trigger selected by TRGSEL field is enabled."]
    EN,
}
impl TRGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TRGENW::DIS => false,
            TRGENW::EN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TRGENW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware triggers are disabled. Starting a conversion is only possible by software."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TRGENW::DIS)
    }
    #[doc = "Hardware trigger selected by TRGSEL field is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TRGENW::EN)
    }
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
#[doc = "Possible values of the field `TRGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSELR {
    #[doc = "AFE0_ADTRG for AFEC0 / AFE1_ADTRG for AFEC1"]
    AFEC_TRIG0,
    #[doc = "TIOA Output of the Timer Counter Channel 0 for AFEC0/TIOA Output of the Timer Counter Channel 3 for AFEC1"]
    AFEC_TRIG1,
    #[doc = "TIOA Output of the Timer Counter Channel 1 for AFEC0/TIOA Output of the Timer Counter Channel 4 for AFEC1"]
    AFEC_TRIG2,
    #[doc = "TIOA Output of the Timer Counter Channel 2 for AFEC0/TIOA Output of the Timer Counter Channel 5 for AFEC1"]
    AFEC_TRIG3,
    #[doc = "PWM0 event line 0 for AFEC0 / PWM1 event line 0 for AFEC1"]
    AFEC_TRIG4,
    #[doc = "PWM0 event line 1 for AFEC0 / PWM1 event line 1 for AFEC1"]
    AFEC_TRIG5,
    #[doc = "Analog Comparator"]
    AFEC_TRIG6,
}
impl crate::ToBits<u8> for TRGSELR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TRGSELR::AFEC_TRIG0 => 0,
            TRGSELR::AFEC_TRIG1 => 1,
            TRGSELR::AFEC_TRIG2 => 2,
            TRGSELR::AFEC_TRIG3 => 3,
            TRGSELR::AFEC_TRIG4 => 4,
            TRGSELR::AFEC_TRIG5 => 5,
            TRGSELR::AFEC_TRIG6 => 6,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TRGSEL_R = crate::FR<u8, TRGSELR>;
impl TRGSEL_R {
    #[doc = "Checks if the value of the field is `AFEC_TRIG0`"]
    #[inline(always)]
    pub fn is_afec_trig0(&self) -> bool {
        *self == TRGSELR::AFEC_TRIG0
    }
    #[doc = "Checks if the value of the field is `AFEC_TRIG1`"]
    #[inline(always)]
    pub fn is_afec_trig1(&self) -> bool {
        *self == TRGSELR::AFEC_TRIG1
    }
    #[doc = "Checks if the value of the field is `AFEC_TRIG2`"]
    #[inline(always)]
    pub fn is_afec_trig2(&self) -> bool {
        *self == TRGSELR::AFEC_TRIG2
    }
    #[doc = "Checks if the value of the field is `AFEC_TRIG3`"]
    #[inline(always)]
    pub fn is_afec_trig3(&self) -> bool {
        *self == TRGSELR::AFEC_TRIG3
    }
    #[doc = "Checks if the value of the field is `AFEC_TRIG4`"]
    #[inline(always)]
    pub fn is_afec_trig4(&self) -> bool {
        *self == TRGSELR::AFEC_TRIG4
    }
    #[doc = "Checks if the value of the field is `AFEC_TRIG5`"]
    #[inline(always)]
    pub fn is_afec_trig5(&self) -> bool {
        *self == TRGSELR::AFEC_TRIG5
    }
    #[doc = "Checks if the value of the field is `AFEC_TRIG6`"]
    #[inline(always)]
    pub fn is_afec_trig6(&self) -> bool {
        *self == TRGSELR::AFEC_TRIG6
    }
}
#[doc = "Values that can be written to the field `TRGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSELW {
    #[doc = "AFE0_ADTRG for AFEC0 / AFE1_ADTRG for AFEC1"]
    AFEC_TRIG0,
    #[doc = "TIOA Output of the Timer Counter Channel 0 for AFEC0/TIOA Output of the Timer Counter Channel 3 for AFEC1"]
    AFEC_TRIG1,
    #[doc = "TIOA Output of the Timer Counter Channel 1 for AFEC0/TIOA Output of the Timer Counter Channel 4 for AFEC1"]
    AFEC_TRIG2,
    #[doc = "TIOA Output of the Timer Counter Channel 2 for AFEC0/TIOA Output of the Timer Counter Channel 5 for AFEC1"]
    AFEC_TRIG3,
    #[doc = "PWM0 event line 0 for AFEC0 / PWM1 event line 0 for AFEC1"]
    AFEC_TRIG4,
    #[doc = "PWM0 event line 1 for AFEC0 / PWM1 event line 1 for AFEC1"]
    AFEC_TRIG5,
    #[doc = "Analog Comparator"]
    AFEC_TRIG6,
}
impl TRGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRGSELW::AFEC_TRIG0 => 0,
            TRGSELW::AFEC_TRIG1 => 1,
            TRGSELW::AFEC_TRIG2 => 2,
            TRGSELW::AFEC_TRIG3 => 3,
            TRGSELW::AFEC_TRIG4 => 4,
            TRGSELW::AFEC_TRIG5 => 5,
            TRGSELW::AFEC_TRIG6 => 6,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TRGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGSELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "AFE0_ADTRG for AFEC0 / AFE1_ADTRG for AFEC1"]
    #[inline(always)]
    pub fn afec_trig0(self) -> &'a mut W {
        self.variant(TRGSELW::AFEC_TRIG0)
    }
    #[doc = "TIOA Output of the Timer Counter Channel 0 for AFEC0/TIOA Output of the Timer Counter Channel 3 for AFEC1"]
    #[inline(always)]
    pub fn afec_trig1(self) -> &'a mut W {
        self.variant(TRGSELW::AFEC_TRIG1)
    }
    #[doc = "TIOA Output of the Timer Counter Channel 1 for AFEC0/TIOA Output of the Timer Counter Channel 4 for AFEC1"]
    #[inline(always)]
    pub fn afec_trig2(self) -> &'a mut W {
        self.variant(TRGSELW::AFEC_TRIG2)
    }
    #[doc = "TIOA Output of the Timer Counter Channel 2 for AFEC0/TIOA Output of the Timer Counter Channel 5 for AFEC1"]
    #[inline(always)]
    pub fn afec_trig3(self) -> &'a mut W {
        self.variant(TRGSELW::AFEC_TRIG3)
    }
    #[doc = "PWM0 event line 0 for AFEC0 / PWM1 event line 0 for AFEC1"]
    #[inline(always)]
    pub fn afec_trig4(self) -> &'a mut W {
        self.variant(TRGSELW::AFEC_TRIG4)
    }
    #[doc = "PWM0 event line 1 for AFEC0 / PWM1 event line 1 for AFEC1"]
    #[inline(always)]
    pub fn afec_trig5(self) -> &'a mut W {
        self.variant(TRGSELW::AFEC_TRIG5)
    }
    #[doc = "Analog Comparator"]
    #[inline(always)]
    pub fn afec_trig6(self) -> &'a mut W {
        self.variant(TRGSELW::AFEC_TRIG6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `SLEEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPR {
    #[doc = "Normal mode: The AFE and reference voltage circuitry are kept ON between conversions."]
    NORMAL,
    #[doc = "Sleep mode: The AFE and reference voltage circuitry are OFF between conversions."]
    SLEEP,
}
impl crate::ToBits<bool> for SLEEPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SLEEPR::NORMAL => false,
            SLEEPR::SLEEP => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SLEEP_R = crate::FR<bool, SLEEPR>;
impl SLEEP_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SLEEPR::NORMAL
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == SLEEPR::SLEEP
    }
}
#[doc = "Values that can be written to the field `SLEEP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPW {
    #[doc = "Normal mode: The AFE and reference voltage circuitry are kept ON between conversions."]
    NORMAL,
    #[doc = "Sleep mode: The AFE and reference voltage circuitry are OFF between conversions."]
    SLEEP,
}
impl SLEEPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEEPW::NORMAL => false,
            SLEEPW::SLEEP => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SLEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode: The AFE and reference voltage circuitry are kept ON between conversions."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SLEEPW::NORMAL)
    }
    #[doc = "Sleep mode: The AFE and reference voltage circuitry are OFF between conversions."]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut W {
        self.variant(SLEEPW::SLEEP)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Possible values of the field `FWUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWUPR {
    #[doc = "Normal Sleep mode: The sleep mode is defined by the SLEEP bit."]
    OFF,
    #[doc = "Fast wake-up Sleep mode: The voltage reference is ON between conversions and AFE is OFF."]
    ON,
}
impl crate::ToBits<bool> for FWUPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            FWUPR::OFF => false,
            FWUPR::ON => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FWUP_R = crate::FR<bool, FWUPR>;
impl FWUP_R {
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FWUPR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == FWUPR::ON
    }
}
#[doc = "Values that can be written to the field `FWUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWUPW {
    #[doc = "Normal Sleep mode: The sleep mode is defined by the SLEEP bit."]
    OFF,
    #[doc = "Fast wake-up Sleep mode: The voltage reference is ON between conversions and AFE is OFF."]
    ON,
}
impl FWUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            FWUPW::OFF => false,
            FWUPW::ON => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FWUPW<'a> {
    w: &'a mut W,
}
impl<'a> _FWUPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FWUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal Sleep mode: The sleep mode is defined by the SLEEP bit."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(FWUPW::OFF)
    }
    #[doc = "Fast wake-up Sleep mode: The voltage reference is ON between conversions and AFE is OFF."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(FWUPW::ON)
    }
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
#[doc = "Possible values of the field `FREERUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREERUNR {
    #[doc = "Normal mode"]
    OFF,
    #[doc = "Free Run mode: Never wait for any trigger."]
    ON,
}
impl crate::ToBits<bool> for FREERUNR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            FREERUNR::OFF => false,
            FREERUNR::ON => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FREERUN_R = crate::FR<bool, FREERUNR>;
impl FREERUN_R {
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FREERUNR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == FREERUNR::ON
    }
}
#[doc = "Values that can be written to the field `FREERUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREERUNW {
    #[doc = "Normal mode"]
    OFF,
    #[doc = "Free Run mode: Never wait for any trigger."]
    ON,
}
impl FREERUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            FREERUNW::OFF => false,
            FREERUNW::ON => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FREERUNW<'a> {
    w: &'a mut W,
}
impl<'a> _FREERUNW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREERUNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(FREERUNW::OFF)
    }
    #[doc = "Free Run mode: Never wait for any trigger."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(FREERUNW::ON)
    }
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
#[doc = r"Reader of the field"]
pub type PRESCAL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRESCALW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `STARTUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTUPR {
    #[doc = "0 periods of AFE clock"]
    SUT0,
    #[doc = "8 periods of AFE clock"]
    SUT8,
    #[doc = "16 periods of AFE clock"]
    SUT16,
    #[doc = "24 periods of AFE clock"]
    SUT24,
    #[doc = "64 periods of AFE clock"]
    SUT64,
    #[doc = "80 periods of AFE clock"]
    SUT80,
    #[doc = "96 periods of AFE clock"]
    SUT96,
    #[doc = "112 periods of AFE clock"]
    SUT112,
    #[doc = "512 periods of AFE clock"]
    SUT512,
    #[doc = "576 periods of AFE clock"]
    SUT576,
    #[doc = "640 periods of AFE clock"]
    SUT640,
    #[doc = "704 periods of AFE clock"]
    SUT704,
    #[doc = "768 periods of AFE clock"]
    SUT768,
    #[doc = "832 periods of AFE clock"]
    SUT832,
    #[doc = "896 periods of AFE clock"]
    SUT896,
    #[doc = "960 periods of AFE clock"]
    SUT960,
}
impl crate::ToBits<u8> for STARTUPR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            STARTUPR::SUT0 => 0,
            STARTUPR::SUT8 => 1,
            STARTUPR::SUT16 => 2,
            STARTUPR::SUT24 => 3,
            STARTUPR::SUT64 => 4,
            STARTUPR::SUT80 => 5,
            STARTUPR::SUT96 => 6,
            STARTUPR::SUT112 => 7,
            STARTUPR::SUT512 => 8,
            STARTUPR::SUT576 => 9,
            STARTUPR::SUT640 => 10,
            STARTUPR::SUT704 => 11,
            STARTUPR::SUT768 => 12,
            STARTUPR::SUT832 => 13,
            STARTUPR::SUT896 => 14,
            STARTUPR::SUT960 => 15,
        }
    }
}
#[doc = r"Reader of the field"]
pub type STARTUP_R = crate::FR<u8, STARTUPR>;
impl STARTUP_R {
    #[doc = "Checks if the value of the field is `SUT0`"]
    #[inline(always)]
    pub fn is_sut0(&self) -> bool {
        *self == STARTUPR::SUT0
    }
    #[doc = "Checks if the value of the field is `SUT8`"]
    #[inline(always)]
    pub fn is_sut8(&self) -> bool {
        *self == STARTUPR::SUT8
    }
    #[doc = "Checks if the value of the field is `SUT16`"]
    #[inline(always)]
    pub fn is_sut16(&self) -> bool {
        *self == STARTUPR::SUT16
    }
    #[doc = "Checks if the value of the field is `SUT24`"]
    #[inline(always)]
    pub fn is_sut24(&self) -> bool {
        *self == STARTUPR::SUT24
    }
    #[doc = "Checks if the value of the field is `SUT64`"]
    #[inline(always)]
    pub fn is_sut64(&self) -> bool {
        *self == STARTUPR::SUT64
    }
    #[doc = "Checks if the value of the field is `SUT80`"]
    #[inline(always)]
    pub fn is_sut80(&self) -> bool {
        *self == STARTUPR::SUT80
    }
    #[doc = "Checks if the value of the field is `SUT96`"]
    #[inline(always)]
    pub fn is_sut96(&self) -> bool {
        *self == STARTUPR::SUT96
    }
    #[doc = "Checks if the value of the field is `SUT112`"]
    #[inline(always)]
    pub fn is_sut112(&self) -> bool {
        *self == STARTUPR::SUT112
    }
    #[doc = "Checks if the value of the field is `SUT512`"]
    #[inline(always)]
    pub fn is_sut512(&self) -> bool {
        *self == STARTUPR::SUT512
    }
    #[doc = "Checks if the value of the field is `SUT576`"]
    #[inline(always)]
    pub fn is_sut576(&self) -> bool {
        *self == STARTUPR::SUT576
    }
    #[doc = "Checks if the value of the field is `SUT640`"]
    #[inline(always)]
    pub fn is_sut640(&self) -> bool {
        *self == STARTUPR::SUT640
    }
    #[doc = "Checks if the value of the field is `SUT704`"]
    #[inline(always)]
    pub fn is_sut704(&self) -> bool {
        *self == STARTUPR::SUT704
    }
    #[doc = "Checks if the value of the field is `SUT768`"]
    #[inline(always)]
    pub fn is_sut768(&self) -> bool {
        *self == STARTUPR::SUT768
    }
    #[doc = "Checks if the value of the field is `SUT832`"]
    #[inline(always)]
    pub fn is_sut832(&self) -> bool {
        *self == STARTUPR::SUT832
    }
    #[doc = "Checks if the value of the field is `SUT896`"]
    #[inline(always)]
    pub fn is_sut896(&self) -> bool {
        *self == STARTUPR::SUT896
    }
    #[doc = "Checks if the value of the field is `SUT960`"]
    #[inline(always)]
    pub fn is_sut960(&self) -> bool {
        *self == STARTUPR::SUT960
    }
}
#[doc = "Values that can be written to the field `STARTUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTUPW {
    #[doc = "0 periods of AFE clock"]
    SUT0,
    #[doc = "8 periods of AFE clock"]
    SUT8,
    #[doc = "16 periods of AFE clock"]
    SUT16,
    #[doc = "24 periods of AFE clock"]
    SUT24,
    #[doc = "64 periods of AFE clock"]
    SUT64,
    #[doc = "80 periods of AFE clock"]
    SUT80,
    #[doc = "96 periods of AFE clock"]
    SUT96,
    #[doc = "112 periods of AFE clock"]
    SUT112,
    #[doc = "512 periods of AFE clock"]
    SUT512,
    #[doc = "576 periods of AFE clock"]
    SUT576,
    #[doc = "640 periods of AFE clock"]
    SUT640,
    #[doc = "704 periods of AFE clock"]
    SUT704,
    #[doc = "768 periods of AFE clock"]
    SUT768,
    #[doc = "832 periods of AFE clock"]
    SUT832,
    #[doc = "896 periods of AFE clock"]
    SUT896,
    #[doc = "960 periods of AFE clock"]
    SUT960,
}
impl STARTUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            STARTUPW::SUT0 => 0,
            STARTUPW::SUT8 => 1,
            STARTUPW::SUT16 => 2,
            STARTUPW::SUT24 => 3,
            STARTUPW::SUT64 => 4,
            STARTUPW::SUT80 => 5,
            STARTUPW::SUT96 => 6,
            STARTUPW::SUT112 => 7,
            STARTUPW::SUT512 => 8,
            STARTUPW::SUT576 => 9,
            STARTUPW::SUT640 => 10,
            STARTUPW::SUT704 => 11,
            STARTUPW::SUT768 => 12,
            STARTUPW::SUT832 => 13,
            STARTUPW::SUT896 => 14,
            STARTUPW::SUT960 => 15,
        }
    }
}
#[doc = r"Proxy"]
pub struct _STARTUPW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTUPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTUPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "0 periods of AFE clock"]
    #[inline(always)]
    pub fn sut0(self) -> &'a mut W {
        self.variant(STARTUPW::SUT0)
    }
    #[doc = "8 periods of AFE clock"]
    #[inline(always)]
    pub fn sut8(self) -> &'a mut W {
        self.variant(STARTUPW::SUT8)
    }
    #[doc = "16 periods of AFE clock"]
    #[inline(always)]
    pub fn sut16(self) -> &'a mut W {
        self.variant(STARTUPW::SUT16)
    }
    #[doc = "24 periods of AFE clock"]
    #[inline(always)]
    pub fn sut24(self) -> &'a mut W {
        self.variant(STARTUPW::SUT24)
    }
    #[doc = "64 periods of AFE clock"]
    #[inline(always)]
    pub fn sut64(self) -> &'a mut W {
        self.variant(STARTUPW::SUT64)
    }
    #[doc = "80 periods of AFE clock"]
    #[inline(always)]
    pub fn sut80(self) -> &'a mut W {
        self.variant(STARTUPW::SUT80)
    }
    #[doc = "96 periods of AFE clock"]
    #[inline(always)]
    pub fn sut96(self) -> &'a mut W {
        self.variant(STARTUPW::SUT96)
    }
    #[doc = "112 periods of AFE clock"]
    #[inline(always)]
    pub fn sut112(self) -> &'a mut W {
        self.variant(STARTUPW::SUT112)
    }
    #[doc = "512 periods of AFE clock"]
    #[inline(always)]
    pub fn sut512(self) -> &'a mut W {
        self.variant(STARTUPW::SUT512)
    }
    #[doc = "576 periods of AFE clock"]
    #[inline(always)]
    pub fn sut576(self) -> &'a mut W {
        self.variant(STARTUPW::SUT576)
    }
    #[doc = "640 periods of AFE clock"]
    #[inline(always)]
    pub fn sut640(self) -> &'a mut W {
        self.variant(STARTUPW::SUT640)
    }
    #[doc = "704 periods of AFE clock"]
    #[inline(always)]
    pub fn sut704(self) -> &'a mut W {
        self.variant(STARTUPW::SUT704)
    }
    #[doc = "768 periods of AFE clock"]
    #[inline(always)]
    pub fn sut768(self) -> &'a mut W {
        self.variant(STARTUPW::SUT768)
    }
    #[doc = "832 periods of AFE clock"]
    #[inline(always)]
    pub fn sut832(self) -> &'a mut W {
        self.variant(STARTUPW::SUT832)
    }
    #[doc = "896 periods of AFE clock"]
    #[inline(always)]
    pub fn sut896(self) -> &'a mut W {
        self.variant(STARTUPW::SUT896)
    }
    #[doc = "960 periods of AFE clock"]
    #[inline(always)]
    pub fn sut960(self) -> &'a mut W {
        self.variant(STARTUPW::SUT960)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ONE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ONEW<'a> {
    w: &'a mut W,
}
impl<'a> _ONEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TRACKTIM_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TRACKTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TRACKTIMW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TRANSFER_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TRANSFERW<'a> {
    w: &'a mut W,
}
impl<'a> _TRANSFERW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Possible values of the field `USEQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USEQR {
    #[doc = "Normal mode: The controller converts channels in a simple numeric order."]
    NUM_ORDER,
    #[doc = "User Sequence mode: The sequence respects what is defined in AFEC_SEQ1R and AFEC_SEQ1R."]
    REG_ORDER,
}
impl crate::ToBits<bool> for USEQR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USEQR::NUM_ORDER => false,
            USEQR::REG_ORDER => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USEQ_R = crate::FR<bool, USEQR>;
impl USEQ_R {
    #[doc = "Checks if the value of the field is `NUM_ORDER`"]
    #[inline(always)]
    pub fn is_num_order(&self) -> bool {
        *self == USEQR::NUM_ORDER
    }
    #[doc = "Checks if the value of the field is `REG_ORDER`"]
    #[inline(always)]
    pub fn is_reg_order(&self) -> bool {
        *self == USEQR::REG_ORDER
    }
}
#[doc = "Values that can be written to the field `USEQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USEQW {
    #[doc = "Normal mode: The controller converts channels in a simple numeric order."]
    NUM_ORDER,
    #[doc = "User Sequence mode: The sequence respects what is defined in AFEC_SEQ1R and AFEC_SEQ1R."]
    REG_ORDER,
}
impl USEQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USEQW::NUM_ORDER => false,
            USEQW::REG_ORDER => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USEQW<'a> {
    w: &'a mut W,
}
impl<'a> _USEQW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USEQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode: The controller converts channels in a simple numeric order."]
    #[inline(always)]
    pub fn num_order(self) -> &'a mut W {
        self.variant(USEQW::NUM_ORDER)
    }
    #[doc = "User Sequence mode: The sequence respects what is defined in AFEC_SEQ1R and AFEC_SEQ1R."]
    #[inline(always)]
    pub fn reg_order(self) -> &'a mut W {
        self.variant(USEQW::REG_ORDER)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn trgen(&self) -> TRGEN_R {
        TRGEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits() >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast Wake-up"]
    #[inline(always)]
    pub fn fwup(&self) -> FWUP_R {
        FWUP_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Free Run Mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FREERUN_R {
        FREERUN_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline(always)]
    pub fn prescal(&self) -> PRESCAL_R {
        PRESCAL_R::new(((self.bits() >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Start-up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits() >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - One"]
    #[inline(always)]
    pub fn one(&self) -> ONE_R {
        ONE_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Tracking Time"]
    #[inline(always)]
    pub fn tracktim(&self) -> TRACKTIM_R {
        TRACKTIM_R::new(((self.bits() >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Transfer Period"]
    #[inline(always)]
    pub fn transfer(&self) -> TRANSFER_R {
        TRANSFER_R::new(((self.bits() >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 31 - User Sequence Enable"]
    #[inline(always)]
    pub fn useq(&self) -> USEQ_R {
        USEQ_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn trgen(&mut self) -> _TRGENW {
        _TRGENW { w: self }
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> _TRGSELW {
        _TRGSELW { w: self }
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&mut self) -> _SLEEPW {
        _SLEEPW { w: self }
    }
    #[doc = "Bit 6 - Fast Wake-up"]
    #[inline(always)]
    pub fn fwup(&mut self) -> _FWUPW {
        _FWUPW { w: self }
    }
    #[doc = "Bit 7 - Free Run Mode"]
    #[inline(always)]
    pub fn freerun(&mut self) -> _FREERUNW {
        _FREERUNW { w: self }
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline(always)]
    pub fn prescal(&mut self) -> _PRESCALW {
        _PRESCALW { w: self }
    }
    #[doc = "Bits 16:19 - Start-up Time"]
    #[inline(always)]
    pub fn startup(&mut self) -> _STARTUPW {
        _STARTUPW { w: self }
    }
    #[doc = "Bit 23 - One"]
    #[inline(always)]
    pub fn one(&mut self) -> _ONEW {
        _ONEW { w: self }
    }
    #[doc = "Bits 24:27 - Tracking Time"]
    #[inline(always)]
    pub fn tracktim(&mut self) -> _TRACKTIMW {
        _TRACKTIMW { w: self }
    }
    #[doc = "Bits 28:29 - Transfer Period"]
    #[inline(always)]
    pub fn transfer(&mut self) -> _TRANSFERW {
        _TRANSFERW { w: self }
    }
    #[doc = "Bit 31 - User Sequence Enable"]
    #[inline(always)]
    pub fn useq(&mut self) -> _USEQW {
        _USEQW { w: self }
    }
}
