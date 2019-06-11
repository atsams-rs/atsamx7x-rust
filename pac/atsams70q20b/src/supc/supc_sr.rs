#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SUPC_SR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `WKUPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPSR {
    #[doc = "No wake-up due to the assertion of the WKUP pins has occurred since the last read of SUPC_SR."]
    NO,
    #[doc = "At least one wake-up due to the assertion of the WKUP pins has occurred since the last read of SUPC_SR."]
    PRESENT,
}
impl WKUPSR {
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
            WKUPSR::NO => false,
            WKUPSR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPSR {
        match value {
            false => WKUPSR::NO,
            true => WKUPSR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == WKUPSR::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == WKUPSR::PRESENT
    }
}
#[doc = "Possible values of the field `SMWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMWSR {
    #[doc = "No wake-up due to a supply monitor detection has occurred since the last read of SUPC_SR."]
    NO,
    #[doc = "At least one wake-up due to a supply monitor detection has occurred since the last read of SUPC_SR."]
    PRESENT,
}
impl SMWSR {
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
            SMWSR::NO => false,
            SMWSR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMWSR {
        match value {
            false => SMWSR::NO,
            true => SMWSR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == SMWSR::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == SMWSR::PRESENT
    }
}
#[doc = "Possible values of the field `BODRSTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTSR {
    #[doc = "No core brownout rising edge event has been detected since the last read of the SUPC_SR."]
    NO,
    #[doc = "At least one brownout output rising edge event has been detected since the last read of the SUPC_SR."]
    PRESENT,
}
impl BODRSTSR {
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
            BODRSTSR::NO => false,
            BODRSTSR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODRSTSR {
        match value {
            false => BODRSTSR::NO,
            true => BODRSTSR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == BODRSTSR::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == BODRSTSR::PRESENT
    }
}
#[doc = "Possible values of the field `SMRSTS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMRSTSR {
    #[doc = "No supply monitor detection has generated a core reset since the last read of the SUPC_SR."]
    NO,
    #[doc = "At least one supply monitor detection has generated a core reset since the last read of the SUPC_SR."]
    PRESENT,
}
impl SMRSTSR {
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
            SMRSTSR::NO => false,
            SMRSTSR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMRSTSR {
        match value {
            false => SMRSTSR::NO,
            true => SMRSTSR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == SMRSTSR::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == SMRSTSR::PRESENT
    }
}
#[doc = "Possible values of the field `SMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMSR {
    #[doc = "No supply monitor detection since the last read of SUPC_SR."]
    NO,
    #[doc = "At least one supply monitor detection since the last read of SUPC_SR."]
    PRESENT,
}
impl SMSR {
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
            SMSR::NO => false,
            SMSR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMSR {
        match value {
            false => SMSR::NO,
            true => SMSR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == SMSR::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == SMSR::PRESENT
    }
}
#[doc = "Possible values of the field `SMOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMOSR {
    #[doc = "The supply monitor detected VDDIO higher than its threshold at its last measurement."]
    HIGH,
    #[doc = "The supply monitor detected VDDIO lower than its threshold at its last measurement."]
    LOW,
}
impl SMOSR {
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
            SMOSR::HIGH => false,
            SMOSR::LOW => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMOSR {
        match value {
            false => SMOSR::HIGH,
            true => SMOSR::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SMOSR::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SMOSR::LOW
    }
}
#[doc = "Possible values of the field `OSCSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSELR {
    #[doc = "The slow clock, SLCK, is generated by the embedded 32 kHz RC oscillator."]
    RC,
    #[doc = "The slow clock, SLCK, is generated by the 32 kHz crystal oscillator."]
    CRYST,
}
impl OSCSELR {
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
            OSCSELR::RC => false,
            OSCSELR::CRYST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OSCSELR {
        match value {
            false => OSCSELR::RC,
            true => OSCSELR::CRYST,
        }
    }
    #[doc = "Checks if the value of the field is `RC`"]
    #[inline]
    pub fn is_rc(&self) -> bool {
        *self == OSCSELR::RC
    }
    #[doc = "Checks if the value of the field is `CRYST`"]
    #[inline]
    pub fn is_cryst(&self) -> bool {
        *self == OSCSELR::CRYST
    }
}
#[doc = "Possible values of the field `LPDBCS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCS0R {
    #[doc = "No wake-up due to the assertion of the WKUP0 pin has occurred since the last read of SUPC_SR."]
    NO,
    #[doc = "At least one wake-up due to the assertion of the WKUP0 pin has occurred since the last read of SUPC_SR."]
    PRESENT,
}
impl LPDBCS0R {
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
            LPDBCS0R::NO => false,
            LPDBCS0R::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPDBCS0R {
        match value {
            false => LPDBCS0R::NO,
            true => LPDBCS0R::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == LPDBCS0R::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == LPDBCS0R::PRESENT
    }
}
#[doc = "Possible values of the field `LPDBCS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCS1R {
    #[doc = "No wake-up due to the assertion of the WKUP1 pin has occurred since the last read of SUPC_SR."]
    NO,
    #[doc = "At least one wake-up due to the assertion of the WKUP1 pin has occurred since the last read of SUPC_SR."]
    PRESENT,
}
impl LPDBCS1R {
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
            LPDBCS1R::NO => false,
            LPDBCS1R::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPDBCS1R {
        match value {
            false => LPDBCS1R::NO,
            true => LPDBCS1R::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == LPDBCS1R::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == LPDBCS1R::PRESENT
    }
}
#[doc = "Possible values of the field `WKUPIS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS0R {
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN,
}
impl WKUPIS0R {
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
            WKUPIS0R::DIS => false,
            WKUPIS0R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS0R {
        match value {
            false => WKUPIS0R::DIS,
            true => WKUPIS0R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS0R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS1R {
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN,
}
impl WKUPIS1R {
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
            WKUPIS1R::DIS => false,
            WKUPIS1R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS1R {
        match value {
            false => WKUPIS1R::DIS,
            true => WKUPIS1R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS1R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS2R {
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN,
}
impl WKUPIS2R {
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
            WKUPIS2R::DIS => false,
            WKUPIS2R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS2R {
        match value {
            false => WKUPIS2R::DIS,
            true => WKUPIS2R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS2R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS2R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS3R {
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN,
}
impl WKUPIS3R {
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
            WKUPIS3R::DIS => false,
            WKUPIS3R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS3R {
        match value {
            false => WKUPIS3R::DIS,
            true => WKUPIS3R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS3R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS3R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS4R {
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN,
}
impl WKUPIS4R {
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
            WKUPIS4R::DIS => false,
            WKUPIS4R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS4R {
        match value {
            false => WKUPIS4R::DIS,
            true => WKUPIS4R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS4R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS4R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS5R {
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN,
}
impl WKUPIS5R {
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
            WKUPIS5R::DIS => false,
            WKUPIS5R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS5R {
        match value {
            false => WKUPIS5R::DIS,
            true => WKUPIS5R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS5R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS5R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS6R {
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN,
}
impl WKUPIS6R {
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
            WKUPIS6R::DIS => false,
            WKUPIS6R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS6R {
        match value {
            false => WKUPIS6R::DIS,
            true => WKUPIS6R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS6R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS6R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS7R {
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN,
}
impl WKUPIS7R {
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
            WKUPIS7R::DIS => false,
            WKUPIS7R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS7R {
        match value {
            false => WKUPIS7R::DIS,
            true => WKUPIS7R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS7R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS7R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS8R {
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN,
}
impl WKUPIS8R {
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
            WKUPIS8R::DIS => false,
            WKUPIS8R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS8R {
        match value {
            false => WKUPIS8R::DIS,
            true => WKUPIS8R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS8R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS8R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS9R {
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN,
}
impl WKUPIS9R {
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
            WKUPIS9R::DIS => false,
            WKUPIS9R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS9R {
        match value {
            false => WKUPIS9R::DIS,
            true => WKUPIS9R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS9R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS9R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS10R {
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN,
}
impl WKUPIS10R {
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
            WKUPIS10R::DIS => false,
            WKUPIS10R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS10R {
        match value {
            false => WKUPIS10R::DIS,
            true => WKUPIS10R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS10R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS10R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS11R {
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN,
}
impl WKUPIS11R {
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
            WKUPIS11R::DIS => false,
            WKUPIS11R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS11R {
        match value {
            false => WKUPIS11R::DIS,
            true => WKUPIS11R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS11R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS11R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS12R {
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN,
}
impl WKUPIS12R {
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
            WKUPIS12R::DIS => false,
            WKUPIS12R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS12R {
        match value {
            false => WKUPIS12R::DIS,
            true => WKUPIS12R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS12R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS12R::EN
    }
}
#[doc = "Possible values of the field `WKUPIS13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS13R {
    #[doc = "The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DIS,
    #[doc = "The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    EN,
}
impl WKUPIS13R {
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
            WKUPIS13R::DIS => false,
            WKUPIS13R::EN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPIS13R {
        match value {
            false => WKUPIS13R::DIS,
            true => WKUPIS13R::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS13R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS13R::EN
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - WKUP Wake-up Status (cleared on read)"]
    #[inline]
    pub fn wkups(&self) -> WKUPSR {
        WKUPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Supply Monitor Detection Wake-up Status (cleared on read)"]
    #[inline]
    pub fn smws(&self) -> SMWSR {
        SMWSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Brownout Detector Reset Status (cleared on read)"]
    #[inline]
    pub fn bodrsts(&self) -> BODRSTSR {
        BODRSTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Supply Monitor Reset Status (cleared on read)"]
    #[inline]
    pub fn smrsts(&self) -> SMRSTSR {
        SMRSTSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Supply Monitor Status (cleared on read)"]
    #[inline]
    pub fn sms(&self) -> SMSR {
        SMSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Supply Monitor Output Status"]
    #[inline]
    pub fn smos(&self) -> SMOSR {
        SMOSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - 32-kHz Oscillator Selection Status"]
    #[inline]
    pub fn oscsel(&self) -> OSCSELR {
        OSCSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Low-power Debouncer Wake-up Status on WKUP0 (cleared on read)"]
    #[inline]
    pub fn lpdbcs0(&self) -> LPDBCS0R {
        LPDBCS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Low-power Debouncer Wake-up Status on WKUP1 (cleared on read)"]
    #[inline]
    pub fn lpdbcs1(&self) -> LPDBCS1R {
        LPDBCS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - WKUPx Input Status (cleared on read)"]
    #[inline]
    pub fn wkupis0(&self) -> WKUPIS0R {
        WKUPIS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - WKUPx Input Status (cleared on read)"]
    #[inline]
    pub fn wkupis1(&self) -> WKUPIS1R {
        WKUPIS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - WKUPx Input Status (cleared on read)"]
    #[inline]
    pub fn wkupis2(&self) -> WKUPIS2R {
        WKUPIS2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - WKUPx Input Status (cleared on read)"]
    #[inline]
    pub fn wkupis3(&self) -> WKUPIS3R {
        WKUPIS3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - WKUPx Input Status (cleared on read)"]
    #[inline]
    pub fn wkupis4(&self) -> WKUPIS4R {
        WKUPIS4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - WKUPx Input Status (cleared on read)"]
    #[inline]
    pub fn wkupis5(&self) -> WKUPIS5R {
        WKUPIS5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - WKUPx Input Status (cleared on read)"]
    #[inline]
    pub fn wkupis6(&self) -> WKUPIS6R {
        WKUPIS6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - WKUPx Input Status (cleared on read)"]
    #[inline]
    pub fn wkupis7(&self) -> WKUPIS7R {
        WKUPIS7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - WKUPx Input Status (cleared on read)"]
    #[inline]
    pub fn wkupis8(&self) -> WKUPIS8R {
        WKUPIS8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - WKUPx Input Status (cleared on read)"]
    #[inline]
    pub fn wkupis9(&self) -> WKUPIS9R {
        WKUPIS9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - WKUPx Input Status (cleared on read)"]
    #[inline]
    pub fn wkupis10(&self) -> WKUPIS10R {
        WKUPIS10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - WKUPx Input Status (cleared on read)"]
    #[inline]
    pub fn wkupis11(&self) -> WKUPIS11R {
        WKUPIS11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - WKUPx Input Status (cleared on read)"]
    #[inline]
    pub fn wkupis12(&self) -> WKUPIS12R {
        WKUPIS12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - WKUPx Input Status (cleared on read)"]
    #[inline]
    pub fn wkupis13(&self) -> WKUPIS13R {
        WKUPIS13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
