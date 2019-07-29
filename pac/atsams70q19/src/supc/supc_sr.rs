#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SUPC_SR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
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
impl crate::ToBits<bool> for WKUPSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPSR::NO => false,
            WKUPSR::PRESENT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPS_R = crate::FR<bool, WKUPSR>;
impl WKUPS_R {
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == WKUPSR::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for SMWSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SMWSR::NO => false,
            SMWSR::PRESENT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SMWS_R = crate::FR<bool, SMWSR>;
impl SMWS_R {
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SMWSR::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for BODRSTSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BODRSTSR::NO => false,
            BODRSTSR::PRESENT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BODRSTS_R = crate::FR<bool, BODRSTSR>;
impl BODRSTS_R {
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == BODRSTSR::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for SMRSTSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SMRSTSR::NO => false,
            SMRSTSR::PRESENT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SMRSTS_R = crate::FR<bool, SMRSTSR>;
impl SMRSTS_R {
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SMRSTSR::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for SMSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SMSR::NO => false,
            SMSR::PRESENT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SMS_R = crate::FR<bool, SMSR>;
impl SMS_R {
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == SMSR::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for SMOSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SMOSR::HIGH => false,
            SMOSR::LOW => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SMOS_R = crate::FR<bool, SMOSR>;
impl SMOS_R {
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SMOSR::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for OSCSELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            OSCSELR::RC => false,
            OSCSELR::CRYST => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OSCSEL_R = crate::FR<bool, OSCSELR>;
impl OSCSEL_R {
    #[doc = "Checks if the value of the field is `RC`"]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        *self == OSCSELR::RC
    }
    #[doc = "Checks if the value of the field is `CRYST`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for LPDBCS0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LPDBCS0R::NO => false,
            LPDBCS0R::PRESENT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LPDBCS0_R = crate::FR<bool, LPDBCS0R>;
impl LPDBCS0_R {
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == LPDBCS0R::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for LPDBCS1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LPDBCS1R::NO => false,
            LPDBCS1R::PRESENT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LPDBCS1_R = crate::FR<bool, LPDBCS1R>;
impl LPDBCS1_R {
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == LPDBCS1R::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for WKUPIS0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPIS0R::DIS => false,
            WKUPIS0R::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPIS0_R = crate::FR<bool, WKUPIS0R>;
impl WKUPIS0_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS0R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for WKUPIS1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPIS1R::DIS => false,
            WKUPIS1R::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPIS1_R = crate::FR<bool, WKUPIS1R>;
impl WKUPIS1_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS1R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for WKUPIS2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPIS2R::DIS => false,
            WKUPIS2R::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPIS2_R = crate::FR<bool, WKUPIS2R>;
impl WKUPIS2_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS2R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for WKUPIS3R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPIS3R::DIS => false,
            WKUPIS3R::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPIS3_R = crate::FR<bool, WKUPIS3R>;
impl WKUPIS3_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS3R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for WKUPIS4R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPIS4R::DIS => false,
            WKUPIS4R::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPIS4_R = crate::FR<bool, WKUPIS4R>;
impl WKUPIS4_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS4R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for WKUPIS5R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPIS5R::DIS => false,
            WKUPIS5R::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPIS5_R = crate::FR<bool, WKUPIS5R>;
impl WKUPIS5_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS5R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for WKUPIS6R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPIS6R::DIS => false,
            WKUPIS6R::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPIS6_R = crate::FR<bool, WKUPIS6R>;
impl WKUPIS6_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS6R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for WKUPIS7R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPIS7R::DIS => false,
            WKUPIS7R::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPIS7_R = crate::FR<bool, WKUPIS7R>;
impl WKUPIS7_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS7R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for WKUPIS8R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPIS8R::DIS => false,
            WKUPIS8R::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPIS8_R = crate::FR<bool, WKUPIS8R>;
impl WKUPIS8_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS8R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for WKUPIS9R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPIS9R::DIS => false,
            WKUPIS9R::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPIS9_R = crate::FR<bool, WKUPIS9R>;
impl WKUPIS9_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS9R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for WKUPIS10R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPIS10R::DIS => false,
            WKUPIS10R::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPIS10_R = crate::FR<bool, WKUPIS10R>;
impl WKUPIS10_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS10R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for WKUPIS11R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPIS11R::DIS => false,
            WKUPIS11R::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPIS11_R = crate::FR<bool, WKUPIS11R>;
impl WKUPIS11_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS11R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for WKUPIS12R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPIS12R::DIS => false,
            WKUPIS12R::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPIS12_R = crate::FR<bool, WKUPIS12R>;
impl WKUPIS12_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS12R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
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
impl crate::ToBits<bool> for WKUPIS13R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPIS13R::DIS => false,
            WKUPIS13R::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPIS13_R = crate::FR<bool, WKUPIS13R>;
impl WKUPIS13_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == WKUPIS13R::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == WKUPIS13R::EN
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - WKUP Wake-up Status (cleared on read)"]
    #[inline(always)]
    pub fn wkups(&self) -> WKUPS_R {
        WKUPS_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Supply Monitor Detection Wake-up Status (cleared on read)"]
    #[inline(always)]
    pub fn smws(&self) -> SMWS_R {
        SMWS_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Brownout Detector Reset Status (cleared on read)"]
    #[inline(always)]
    pub fn bodrsts(&self) -> BODRSTS_R {
        BODRSTS_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Supply Monitor Reset Status (cleared on read)"]
    #[inline(always)]
    pub fn smrsts(&self) -> SMRSTS_R {
        SMRSTS_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Supply Monitor Status (cleared on read)"]
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Supply Monitor Output Status"]
    #[inline(always)]
    pub fn smos(&self) -> SMOS_R {
        SMOS_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 32-kHz Oscillator Selection Status"]
    #[inline(always)]
    pub fn oscsel(&self) -> OSCSEL_R {
        OSCSEL_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Low-power Debouncer Wake-up Status on WKUP0 (cleared on read)"]
    #[inline(always)]
    pub fn lpdbcs0(&self) -> LPDBCS0_R {
        LPDBCS0_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Low-power Debouncer Wake-up Status on WKUP1 (cleared on read)"]
    #[inline(always)]
    pub fn lpdbcs1(&self) -> LPDBCS1_R {
        LPDBCS1_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis0(&self) -> WKUPIS0_R {
        WKUPIS0_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis1(&self) -> WKUPIS1_R {
        WKUPIS1_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis2(&self) -> WKUPIS2_R {
        WKUPIS2_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis3(&self) -> WKUPIS3_R {
        WKUPIS3_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis4(&self) -> WKUPIS4_R {
        WKUPIS4_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis5(&self) -> WKUPIS5_R {
        WKUPIS5_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis6(&self) -> WKUPIS6_R {
        WKUPIS6_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis7(&self) -> WKUPIS7_R {
        WKUPIS7_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis8(&self) -> WKUPIS8_R {
        WKUPIS8_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis9(&self) -> WKUPIS9_R {
        WKUPIS9_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis10(&self) -> WKUPIS10_R {
        WKUPIS10_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis11(&self) -> WKUPIS11_R {
        WKUPIS11_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis12(&self) -> WKUPIS12_R {
        WKUPIS12_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - WKUPx Input Status (cleared on read)"]
    #[inline(always)]
    pub fn wkupis13(&self) -> WKUPIS13_R {
        WKUPIS13_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
}
