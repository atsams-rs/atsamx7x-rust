#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFEC_MR {
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
#[doc = "Possible values of the field `TRGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGENR {
    #[doc = "Hardware triggers are disabled. Starting a conversion is only possible by software."]
    DIS,
    #[doc = "Hardware trigger selected by TRGSEL field is enabled."]
    EN,
}
impl TRGENR {
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
            TRGENR::DIS => false,
            TRGENR::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRGENR {
        match value {
            false => TRGENR::DIS,
            true => TRGENR::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == TRGENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == TRGENR::EN
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
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRGSELR::AFEC_TRIG0 => 0,
            TRGSELR::AFEC_TRIG1 => 1,
            TRGSELR::AFEC_TRIG2 => 2,
            TRGSELR::AFEC_TRIG3 => 3,
            TRGSELR::AFEC_TRIG4 => 4,
            TRGSELR::AFEC_TRIG5 => 5,
            TRGSELR::AFEC_TRIG6 => 6,
            TRGSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRGSELR {
        match value {
            0 => TRGSELR::AFEC_TRIG0,
            1 => TRGSELR::AFEC_TRIG1,
            2 => TRGSELR::AFEC_TRIG2,
            3 => TRGSELR::AFEC_TRIG3,
            4 => TRGSELR::AFEC_TRIG4,
            5 => TRGSELR::AFEC_TRIG5,
            6 => TRGSELR::AFEC_TRIG6,
            i => TRGSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AFEC_TRIG0`"]
    #[inline]
    pub fn is_afec_trig0(&self) -> bool {
        *self == TRGSELR::AFEC_TRIG0
    }
    #[doc = "Checks if the value of the field is `AFEC_TRIG1`"]
    #[inline]
    pub fn is_afec_trig1(&self) -> bool {
        *self == TRGSELR::AFEC_TRIG1
    }
    #[doc = "Checks if the value of the field is `AFEC_TRIG2`"]
    #[inline]
    pub fn is_afec_trig2(&self) -> bool {
        *self == TRGSELR::AFEC_TRIG2
    }
    #[doc = "Checks if the value of the field is `AFEC_TRIG3`"]
    #[inline]
    pub fn is_afec_trig3(&self) -> bool {
        *self == TRGSELR::AFEC_TRIG3
    }
    #[doc = "Checks if the value of the field is `AFEC_TRIG4`"]
    #[inline]
    pub fn is_afec_trig4(&self) -> bool {
        *self == TRGSELR::AFEC_TRIG4
    }
    #[doc = "Checks if the value of the field is `AFEC_TRIG5`"]
    #[inline]
    pub fn is_afec_trig5(&self) -> bool {
        *self == TRGSELR::AFEC_TRIG5
    }
    #[doc = "Checks if the value of the field is `AFEC_TRIG6`"]
    #[inline]
    pub fn is_afec_trig6(&self) -> bool {
        *self == TRGSELR::AFEC_TRIG6
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
impl SLEEPR {
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
            SLEEPR::NORMAL => false,
            SLEEPR::SLEEP => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLEEPR {
        match value {
            false => SLEEPR::NORMAL,
            true => SLEEPR::SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == SLEEPR::NORMAL
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline]
    pub fn is_sleep(&self) -> bool {
        *self == SLEEPR::SLEEP
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
impl FWUPR {
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
            FWUPR::OFF => false,
            FWUPR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FWUPR {
        match value {
            false => FWUPR::OFF,
            true => FWUPR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == FWUPR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == FWUPR::ON
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
impl FREERUNR {
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
            FREERUNR::OFF => false,
            FREERUNR::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FREERUNR {
        match value {
            false => FREERUNR::OFF,
            true => FREERUNR::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == FREERUNR::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == FREERUNR::ON
    }
}
#[doc = r" Value of the field"]
pub struct PRESCALR {
    bits: u8,
}
impl PRESCALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
impl STARTUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
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
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STARTUPR {
        match value {
            0 => STARTUPR::SUT0,
            1 => STARTUPR::SUT8,
            2 => STARTUPR::SUT16,
            3 => STARTUPR::SUT24,
            4 => STARTUPR::SUT64,
            5 => STARTUPR::SUT80,
            6 => STARTUPR::SUT96,
            7 => STARTUPR::SUT112,
            8 => STARTUPR::SUT512,
            9 => STARTUPR::SUT576,
            10 => STARTUPR::SUT640,
            11 => STARTUPR::SUT704,
            12 => STARTUPR::SUT768,
            13 => STARTUPR::SUT832,
            14 => STARTUPR::SUT896,
            15 => STARTUPR::SUT960,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SUT0`"]
    #[inline]
    pub fn is_sut0(&self) -> bool {
        *self == STARTUPR::SUT0
    }
    #[doc = "Checks if the value of the field is `SUT8`"]
    #[inline]
    pub fn is_sut8(&self) -> bool {
        *self == STARTUPR::SUT8
    }
    #[doc = "Checks if the value of the field is `SUT16`"]
    #[inline]
    pub fn is_sut16(&self) -> bool {
        *self == STARTUPR::SUT16
    }
    #[doc = "Checks if the value of the field is `SUT24`"]
    #[inline]
    pub fn is_sut24(&self) -> bool {
        *self == STARTUPR::SUT24
    }
    #[doc = "Checks if the value of the field is `SUT64`"]
    #[inline]
    pub fn is_sut64(&self) -> bool {
        *self == STARTUPR::SUT64
    }
    #[doc = "Checks if the value of the field is `SUT80`"]
    #[inline]
    pub fn is_sut80(&self) -> bool {
        *self == STARTUPR::SUT80
    }
    #[doc = "Checks if the value of the field is `SUT96`"]
    #[inline]
    pub fn is_sut96(&self) -> bool {
        *self == STARTUPR::SUT96
    }
    #[doc = "Checks if the value of the field is `SUT112`"]
    #[inline]
    pub fn is_sut112(&self) -> bool {
        *self == STARTUPR::SUT112
    }
    #[doc = "Checks if the value of the field is `SUT512`"]
    #[inline]
    pub fn is_sut512(&self) -> bool {
        *self == STARTUPR::SUT512
    }
    #[doc = "Checks if the value of the field is `SUT576`"]
    #[inline]
    pub fn is_sut576(&self) -> bool {
        *self == STARTUPR::SUT576
    }
    #[doc = "Checks if the value of the field is `SUT640`"]
    #[inline]
    pub fn is_sut640(&self) -> bool {
        *self == STARTUPR::SUT640
    }
    #[doc = "Checks if the value of the field is `SUT704`"]
    #[inline]
    pub fn is_sut704(&self) -> bool {
        *self == STARTUPR::SUT704
    }
    #[doc = "Checks if the value of the field is `SUT768`"]
    #[inline]
    pub fn is_sut768(&self) -> bool {
        *self == STARTUPR::SUT768
    }
    #[doc = "Checks if the value of the field is `SUT832`"]
    #[inline]
    pub fn is_sut832(&self) -> bool {
        *self == STARTUPR::SUT832
    }
    #[doc = "Checks if the value of the field is `SUT896`"]
    #[inline]
    pub fn is_sut896(&self) -> bool {
        *self == STARTUPR::SUT896
    }
    #[doc = "Checks if the value of the field is `SUT960`"]
    #[inline]
    pub fn is_sut960(&self) -> bool {
        *self == STARTUPR::SUT960
    }
}
#[doc = r" Value of the field"]
pub struct ONER {
    bits: bool,
}
impl ONER {
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
pub struct TRACKTIMR {
    bits: u8,
}
impl TRACKTIMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRANSFERR {
    bits: u8,
}
impl TRANSFERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
impl USEQR {
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
            USEQR::NUM_ORDER => false,
            USEQR::REG_ORDER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USEQR {
        match value {
            false => USEQR::NUM_ORDER,
            true => USEQR::REG_ORDER,
        }
    }
    #[doc = "Checks if the value of the field is `NUM_ORDER`"]
    #[inline]
    pub fn is_num_order(&self) -> bool {
        *self == USEQR::NUM_ORDER
    }
    #[doc = "Checks if the value of the field is `REG_ORDER`"]
    #[inline]
    pub fn is_reg_order(&self) -> bool {
        *self == USEQR::REG_ORDER
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRGENW::DIS => false,
            TRGENW::EN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRGENW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware triggers are disabled. Starting a conversion is only possible by software."]
    #[inline]
    pub fn dis(self) -> &'a mut W {
        self.variant(TRGENW::DIS)
    }
    #[doc = "Hardware trigger selected by TRGSEL field is enabled."]
    #[inline]
    pub fn en(self) -> &'a mut W {
        self.variant(TRGENW::EN)
    }
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
    #[inline]
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
#[doc = r" Proxy"]
pub struct _TRGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRGSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "AFE0_ADTRG for AFEC0 / AFE1_ADTRG for AFEC1"]
    #[inline]
    pub fn afec_trig0(self) -> &'a mut W {
        self.variant(TRGSELW::AFEC_TRIG0)
    }
    #[doc = "TIOA Output of the Timer Counter Channel 0 for AFEC0/TIOA Output of the Timer Counter Channel 3 for AFEC1"]
    #[inline]
    pub fn afec_trig1(self) -> &'a mut W {
        self.variant(TRGSELW::AFEC_TRIG1)
    }
    #[doc = "TIOA Output of the Timer Counter Channel 1 for AFEC0/TIOA Output of the Timer Counter Channel 4 for AFEC1"]
    #[inline]
    pub fn afec_trig2(self) -> &'a mut W {
        self.variant(TRGSELW::AFEC_TRIG2)
    }
    #[doc = "TIOA Output of the Timer Counter Channel 2 for AFEC0/TIOA Output of the Timer Counter Channel 5 for AFEC1"]
    #[inline]
    pub fn afec_trig3(self) -> &'a mut W {
        self.variant(TRGSELW::AFEC_TRIG3)
    }
    #[doc = "PWM0 event line 0 for AFEC0 / PWM1 event line 0 for AFEC1"]
    #[inline]
    pub fn afec_trig4(self) -> &'a mut W {
        self.variant(TRGSELW::AFEC_TRIG4)
    }
    #[doc = "PWM0 event line 1 for AFEC0 / PWM1 event line 1 for AFEC1"]
    #[inline]
    pub fn afec_trig5(self) -> &'a mut W {
        self.variant(TRGSELW::AFEC_TRIG5)
    }
    #[doc = "Analog Comparator"]
    #[inline]
    pub fn afec_trig6(self) -> &'a mut W {
        self.variant(TRGSELW::AFEC_TRIG6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLEEPW::NORMAL => false,
            SLEEPW::SLEEP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLEEPW<'a> {
    w: &'a mut W,
}
impl<'a> _SLEEPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLEEPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode: The AFE and reference voltage circuitry are kept ON between conversions."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(SLEEPW::NORMAL)
    }
    #[doc = "Sleep mode: The AFE and reference voltage circuitry are OFF between conversions."]
    #[inline]
    pub fn sleep(self) -> &'a mut W {
        self.variant(SLEEPW::SLEEP)
    }
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FWUPW::OFF => false,
            FWUPW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FWUPW<'a> {
    w: &'a mut W,
}
impl<'a> _FWUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FWUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal Sleep mode: The sleep mode is defined by the SLEEP bit."]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(FWUPW::OFF)
    }
    #[doc = "Fast wake-up Sleep mode: The voltage reference is ON between conversions and AFE is OFF."]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(FWUPW::ON)
    }
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FREERUNW::OFF => false,
            FREERUNW::ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FREERUNW<'a> {
    w: &'a mut W,
}
impl<'a> _FREERUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FREERUNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(FREERUNW::OFF)
    }
    #[doc = "Free Run mode: Never wait for any trigger."]
    #[inline]
    pub fn on(self) -> &'a mut W {
        self.variant(FREERUNW::ON)
    }
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRESCALW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
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
#[doc = r" Proxy"]
pub struct _STARTUPW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTUPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "0 periods of AFE clock"]
    #[inline]
    pub fn sut0(self) -> &'a mut W {
        self.variant(STARTUPW::SUT0)
    }
    #[doc = "8 periods of AFE clock"]
    #[inline]
    pub fn sut8(self) -> &'a mut W {
        self.variant(STARTUPW::SUT8)
    }
    #[doc = "16 periods of AFE clock"]
    #[inline]
    pub fn sut16(self) -> &'a mut W {
        self.variant(STARTUPW::SUT16)
    }
    #[doc = "24 periods of AFE clock"]
    #[inline]
    pub fn sut24(self) -> &'a mut W {
        self.variant(STARTUPW::SUT24)
    }
    #[doc = "64 periods of AFE clock"]
    #[inline]
    pub fn sut64(self) -> &'a mut W {
        self.variant(STARTUPW::SUT64)
    }
    #[doc = "80 periods of AFE clock"]
    #[inline]
    pub fn sut80(self) -> &'a mut W {
        self.variant(STARTUPW::SUT80)
    }
    #[doc = "96 periods of AFE clock"]
    #[inline]
    pub fn sut96(self) -> &'a mut W {
        self.variant(STARTUPW::SUT96)
    }
    #[doc = "112 periods of AFE clock"]
    #[inline]
    pub fn sut112(self) -> &'a mut W {
        self.variant(STARTUPW::SUT112)
    }
    #[doc = "512 periods of AFE clock"]
    #[inline]
    pub fn sut512(self) -> &'a mut W {
        self.variant(STARTUPW::SUT512)
    }
    #[doc = "576 periods of AFE clock"]
    #[inline]
    pub fn sut576(self) -> &'a mut W {
        self.variant(STARTUPW::SUT576)
    }
    #[doc = "640 periods of AFE clock"]
    #[inline]
    pub fn sut640(self) -> &'a mut W {
        self.variant(STARTUPW::SUT640)
    }
    #[doc = "704 periods of AFE clock"]
    #[inline]
    pub fn sut704(self) -> &'a mut W {
        self.variant(STARTUPW::SUT704)
    }
    #[doc = "768 periods of AFE clock"]
    #[inline]
    pub fn sut768(self) -> &'a mut W {
        self.variant(STARTUPW::SUT768)
    }
    #[doc = "832 periods of AFE clock"]
    #[inline]
    pub fn sut832(self) -> &'a mut W {
        self.variant(STARTUPW::SUT832)
    }
    #[doc = "896 periods of AFE clock"]
    #[inline]
    pub fn sut896(self) -> &'a mut W {
        self.variant(STARTUPW::SUT896)
    }
    #[doc = "960 periods of AFE clock"]
    #[inline]
    pub fn sut960(self) -> &'a mut W {
        self.variant(STARTUPW::SUT960)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ONEW<'a> {
    w: &'a mut W,
}
impl<'a> _ONEW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRACKTIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TRACKTIMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRANSFERW<'a> {
    w: &'a mut W,
}
impl<'a> _TRANSFERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USEQW::NUM_ORDER => false,
            USEQW::REG_ORDER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USEQW<'a> {
    w: &'a mut W,
}
impl<'a> _USEQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USEQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal mode: The controller converts channels in a simple numeric order."]
    #[inline]
    pub fn num_order(self) -> &'a mut W {
        self.variant(USEQW::NUM_ORDER)
    }
    #[doc = "User Sequence mode: The sequence respects what is defined in AFEC_SEQ1R and AFEC_SEQ1R."]
    #[inline]
    pub fn reg_order(self) -> &'a mut W {
        self.variant(USEQW::REG_ORDER)
    }
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline]
    pub fn trgen(&self) -> TRGENR {
        TRGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline]
    pub fn trgsel(&self) -> TRGSELR {
        TRGSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline]
    pub fn sleep(&self) -> SLEEPR {
        SLEEPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Fast Wake-up"]
    #[inline]
    pub fn fwup(&self) -> FWUPR {
        FWUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Free Run Mode"]
    #[inline]
    pub fn freerun(&self) -> FREERUNR {
        FREERUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline]
    pub fn prescal(&self) -> PRESCALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRESCALR { bits }
    }
    #[doc = "Bits 16:19 - Start-up Time"]
    #[inline]
    pub fn startup(&self) -> STARTUPR {
        STARTUPR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - One"]
    #[inline]
    pub fn one(&self) -> ONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ONER { bits }
    }
    #[doc = "Bits 24:27 - Tracking Time"]
    #[inline]
    pub fn tracktim(&self) -> TRACKTIMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRACKTIMR { bits }
    }
    #[doc = "Bits 28:29 - Transfer Period"]
    #[inline]
    pub fn transfer(&self) -> TRANSFERR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRANSFERR { bits }
    }
    #[doc = "Bit 31 - User Sequence Enable"]
    #[inline]
    pub fn useq(&self) -> USEQR {
        USEQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline]
    pub fn trgen(&mut self) -> _TRGENW {
        _TRGENW { w: self }
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline]
    pub fn trgsel(&mut self) -> _TRGSELW {
        _TRGSELW { w: self }
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline]
    pub fn sleep(&mut self) -> _SLEEPW {
        _SLEEPW { w: self }
    }
    #[doc = "Bit 6 - Fast Wake-up"]
    #[inline]
    pub fn fwup(&mut self) -> _FWUPW {
        _FWUPW { w: self }
    }
    #[doc = "Bit 7 - Free Run Mode"]
    #[inline]
    pub fn freerun(&mut self) -> _FREERUNW {
        _FREERUNW { w: self }
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline]
    pub fn prescal(&mut self) -> _PRESCALW {
        _PRESCALW { w: self }
    }
    #[doc = "Bits 16:19 - Start-up Time"]
    #[inline]
    pub fn startup(&mut self) -> _STARTUPW {
        _STARTUPW { w: self }
    }
    #[doc = "Bit 23 - One"]
    #[inline]
    pub fn one(&mut self) -> _ONEW {
        _ONEW { w: self }
    }
    #[doc = "Bits 24:27 - Tracking Time"]
    #[inline]
    pub fn tracktim(&mut self) -> _TRACKTIMW {
        _TRACKTIMW { w: self }
    }
    #[doc = "Bits 28:29 - Transfer Period"]
    #[inline]
    pub fn transfer(&mut self) -> _TRANSFERW {
        _TRANSFERW { w: self }
    }
    #[doc = "Bit 31 - User Sequence Enable"]
    #[inline]
    pub fn useq(&mut self) -> _USEQW {
        _USEQW { w: self }
    }
}
