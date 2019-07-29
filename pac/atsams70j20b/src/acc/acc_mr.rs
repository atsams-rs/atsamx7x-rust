#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACC_MR {
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
#[doc = "Possible values of the field `SELMINUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELMINUSR {
    #[doc = "Select TS"]
    TS,
    #[doc = "Select VREFP"]
    VREFP,
    #[doc = "Select DAC0"]
    DAC0,
    #[doc = "Select DAC1"]
    DAC1,
    #[doc = "Select AFE0_AD0"]
    AFE0_AD0,
    #[doc = "Select AFE0_AD1"]
    AFE0_AD1,
    #[doc = "Select AFE0_AD2"]
    AFE0_AD2,
    #[doc = "Select AFE0_AD3"]
    AFE0_AD3,
}
impl crate::ToBits<u8> for SELMINUSR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SELMINUSR::TS => 0,
            SELMINUSR::VREFP => 1,
            SELMINUSR::DAC0 => 2,
            SELMINUSR::DAC1 => 3,
            SELMINUSR::AFE0_AD0 => 4,
            SELMINUSR::AFE0_AD1 => 5,
            SELMINUSR::AFE0_AD2 => 6,
            SELMINUSR::AFE0_AD3 => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SELMINUS_R = crate::FR<u8, SELMINUSR>;
impl SELMINUS_R {
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == SELMINUSR::TS
    }
    #[doc = "Checks if the value of the field is `VREFP`"]
    #[inline(always)]
    pub fn is_vrefp(&self) -> bool {
        *self == SELMINUSR::VREFP
    }
    #[doc = "Checks if the value of the field is `DAC0`"]
    #[inline(always)]
    pub fn is_dac0(&self) -> bool {
        *self == SELMINUSR::DAC0
    }
    #[doc = "Checks if the value of the field is `DAC1`"]
    #[inline(always)]
    pub fn is_dac1(&self) -> bool {
        *self == SELMINUSR::DAC1
    }
    #[doc = "Checks if the value of the field is `AFE0_AD0`"]
    #[inline(always)]
    pub fn is_afe0_ad0(&self) -> bool {
        *self == SELMINUSR::AFE0_AD0
    }
    #[doc = "Checks if the value of the field is `AFE0_AD1`"]
    #[inline(always)]
    pub fn is_afe0_ad1(&self) -> bool {
        *self == SELMINUSR::AFE0_AD1
    }
    #[doc = "Checks if the value of the field is `AFE0_AD2`"]
    #[inline(always)]
    pub fn is_afe0_ad2(&self) -> bool {
        *self == SELMINUSR::AFE0_AD2
    }
    #[doc = "Checks if the value of the field is `AFE0_AD3`"]
    #[inline(always)]
    pub fn is_afe0_ad3(&self) -> bool {
        *self == SELMINUSR::AFE0_AD3
    }
}
#[doc = "Values that can be written to the field `SELMINUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELMINUSW {
    #[doc = "Select TS"]
    TS,
    #[doc = "Select VREFP"]
    VREFP,
    #[doc = "Select DAC0"]
    DAC0,
    #[doc = "Select DAC1"]
    DAC1,
    #[doc = "Select AFE0_AD0"]
    AFE0_AD0,
    #[doc = "Select AFE0_AD1"]
    AFE0_AD1,
    #[doc = "Select AFE0_AD2"]
    AFE0_AD2,
    #[doc = "Select AFE0_AD3"]
    AFE0_AD3,
}
impl SELMINUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELMINUSW::TS => 0,
            SELMINUSW::VREFP => 1,
            SELMINUSW::DAC0 => 2,
            SELMINUSW::DAC1 => 3,
            SELMINUSW::AFE0_AD0 => 4,
            SELMINUSW::AFE0_AD1 => 5,
            SELMINUSW::AFE0_AD2 => 6,
            SELMINUSW::AFE0_AD3 => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SELMINUSW<'a> {
    w: &'a mut W,
}
impl<'a> _SELMINUSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELMINUSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Select TS"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(SELMINUSW::TS)
    }
    #[doc = "Select VREFP"]
    #[inline(always)]
    pub fn vrefp(self) -> &'a mut W {
        self.variant(SELMINUSW::VREFP)
    }
    #[doc = "Select DAC0"]
    #[inline(always)]
    pub fn dac0(self) -> &'a mut W {
        self.variant(SELMINUSW::DAC0)
    }
    #[doc = "Select DAC1"]
    #[inline(always)]
    pub fn dac1(self) -> &'a mut W {
        self.variant(SELMINUSW::DAC1)
    }
    #[doc = "Select AFE0_AD0"]
    #[inline(always)]
    pub fn afe0_ad0(self) -> &'a mut W {
        self.variant(SELMINUSW::AFE0_AD0)
    }
    #[doc = "Select AFE0_AD1"]
    #[inline(always)]
    pub fn afe0_ad1(self) -> &'a mut W {
        self.variant(SELMINUSW::AFE0_AD1)
    }
    #[doc = "Select AFE0_AD2"]
    #[inline(always)]
    pub fn afe0_ad2(self) -> &'a mut W {
        self.variant(SELMINUSW::AFE0_AD2)
    }
    #[doc = "Select AFE0_AD3"]
    #[inline(always)]
    pub fn afe0_ad3(self) -> &'a mut W {
        self.variant(SELMINUSW::AFE0_AD3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Possible values of the field `SELPLUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELPLUSR {
    #[doc = "Select AFE0_AD0"]
    AFE0_AD0,
    #[doc = "Select AFE0_AD1"]
    AFE0_AD1,
    #[doc = "Select AFE0_AD2"]
    AFE0_AD2,
    #[doc = "Select AFE0_AD3"]
    AFE0_AD3,
    #[doc = "Select AFE0_AD4"]
    AFE0_AD4,
    #[doc = "Select AFE0_AD5"]
    AFE0_AD5,
    #[doc = "Select AFE1_AD0"]
    AFE1_AD0,
    #[doc = "Select AFE1_AD1"]
    AFE1_AD1,
}
impl crate::ToBits<u8> for SELPLUSR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SELPLUSR::AFE0_AD0 => 0,
            SELPLUSR::AFE0_AD1 => 1,
            SELPLUSR::AFE0_AD2 => 2,
            SELPLUSR::AFE0_AD3 => 3,
            SELPLUSR::AFE0_AD4 => 4,
            SELPLUSR::AFE0_AD5 => 5,
            SELPLUSR::AFE1_AD0 => 6,
            SELPLUSR::AFE1_AD1 => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SELPLUS_R = crate::FR<u8, SELPLUSR>;
impl SELPLUS_R {
    #[doc = "Checks if the value of the field is `AFE0_AD0`"]
    #[inline(always)]
    pub fn is_afe0_ad0(&self) -> bool {
        *self == SELPLUSR::AFE0_AD0
    }
    #[doc = "Checks if the value of the field is `AFE0_AD1`"]
    #[inline(always)]
    pub fn is_afe0_ad1(&self) -> bool {
        *self == SELPLUSR::AFE0_AD1
    }
    #[doc = "Checks if the value of the field is `AFE0_AD2`"]
    #[inline(always)]
    pub fn is_afe0_ad2(&self) -> bool {
        *self == SELPLUSR::AFE0_AD2
    }
    #[doc = "Checks if the value of the field is `AFE0_AD3`"]
    #[inline(always)]
    pub fn is_afe0_ad3(&self) -> bool {
        *self == SELPLUSR::AFE0_AD3
    }
    #[doc = "Checks if the value of the field is `AFE0_AD4`"]
    #[inline(always)]
    pub fn is_afe0_ad4(&self) -> bool {
        *self == SELPLUSR::AFE0_AD4
    }
    #[doc = "Checks if the value of the field is `AFE0_AD5`"]
    #[inline(always)]
    pub fn is_afe0_ad5(&self) -> bool {
        *self == SELPLUSR::AFE0_AD5
    }
    #[doc = "Checks if the value of the field is `AFE1_AD0`"]
    #[inline(always)]
    pub fn is_afe1_ad0(&self) -> bool {
        *self == SELPLUSR::AFE1_AD0
    }
    #[doc = "Checks if the value of the field is `AFE1_AD1`"]
    #[inline(always)]
    pub fn is_afe1_ad1(&self) -> bool {
        *self == SELPLUSR::AFE1_AD1
    }
}
#[doc = "Values that can be written to the field `SELPLUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELPLUSW {
    #[doc = "Select AFE0_AD0"]
    AFE0_AD0,
    #[doc = "Select AFE0_AD1"]
    AFE0_AD1,
    #[doc = "Select AFE0_AD2"]
    AFE0_AD2,
    #[doc = "Select AFE0_AD3"]
    AFE0_AD3,
    #[doc = "Select AFE0_AD4"]
    AFE0_AD4,
    #[doc = "Select AFE0_AD5"]
    AFE0_AD5,
    #[doc = "Select AFE1_AD0"]
    AFE1_AD0,
    #[doc = "Select AFE1_AD1"]
    AFE1_AD1,
}
impl SELPLUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SELPLUSW::AFE0_AD0 => 0,
            SELPLUSW::AFE0_AD1 => 1,
            SELPLUSW::AFE0_AD2 => 2,
            SELPLUSW::AFE0_AD3 => 3,
            SELPLUSW::AFE0_AD4 => 4,
            SELPLUSW::AFE0_AD5 => 5,
            SELPLUSW::AFE1_AD0 => 6,
            SELPLUSW::AFE1_AD1 => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SELPLUSW<'a> {
    w: &'a mut W,
}
impl<'a> _SELPLUSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELPLUSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Select AFE0_AD0"]
    #[inline(always)]
    pub fn afe0_ad0(self) -> &'a mut W {
        self.variant(SELPLUSW::AFE0_AD0)
    }
    #[doc = "Select AFE0_AD1"]
    #[inline(always)]
    pub fn afe0_ad1(self) -> &'a mut W {
        self.variant(SELPLUSW::AFE0_AD1)
    }
    #[doc = "Select AFE0_AD2"]
    #[inline(always)]
    pub fn afe0_ad2(self) -> &'a mut W {
        self.variant(SELPLUSW::AFE0_AD2)
    }
    #[doc = "Select AFE0_AD3"]
    #[inline(always)]
    pub fn afe0_ad3(self) -> &'a mut W {
        self.variant(SELPLUSW::AFE0_AD3)
    }
    #[doc = "Select AFE0_AD4"]
    #[inline(always)]
    pub fn afe0_ad4(self) -> &'a mut W {
        self.variant(SELPLUSW::AFE0_AD4)
    }
    #[doc = "Select AFE0_AD5"]
    #[inline(always)]
    pub fn afe0_ad5(self) -> &'a mut W {
        self.variant(SELPLUSW::AFE0_AD5)
    }
    #[doc = "Select AFE1_AD0"]
    #[inline(always)]
    pub fn afe1_ad0(self) -> &'a mut W {
        self.variant(SELPLUSW::AFE1_AD0)
    }
    #[doc = "Select AFE1_AD1"]
    #[inline(always)]
    pub fn afe1_ad1(self) -> &'a mut W {
        self.variant(SELPLUSW::AFE1_AD1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `ACEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACENR {
    #[doc = "Analog comparator disabled."]
    DIS,
    #[doc = "Analog comparator enabled."]
    EN,
}
impl crate::ToBits<bool> for ACENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ACENR::DIS => false,
            ACENR::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ACEN_R = crate::FR<bool, ACENR>;
impl ACEN_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ACENR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ACENR::EN
    }
}
#[doc = "Values that can be written to the field `ACEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACENW {
    #[doc = "Analog comparator disabled."]
    DIS,
    #[doc = "Analog comparator enabled."]
    EN,
}
impl ACENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ACENW::DIS => false,
            ACENW::EN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ACENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Analog comparator disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ACENW::DIS)
    }
    #[doc = "Analog comparator enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ACENW::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `EDGETYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGETYPR {
    #[doc = "Only rising edge of comparator output"]
    RISING,
    #[doc = "Falling edge of comparator output"]
    FALLING,
    #[doc = "Any edge of comparator output"]
    ANY,
}
impl crate::ToBits<u8> for EDGETYPR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            EDGETYPR::RISING => 0,
            EDGETYPR::FALLING => 1,
            EDGETYPR::ANY => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EDGETYP_R = crate::FR<u8, EDGETYPR>;
impl EDGETYP_R {
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == EDGETYPR::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == EDGETYPR::FALLING
    }
    #[doc = "Checks if the value of the field is `ANY`"]
    #[inline(always)]
    pub fn is_any(&self) -> bool {
        *self == EDGETYPR::ANY
    }
}
#[doc = "Values that can be written to the field `EDGETYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGETYPW {
    #[doc = "Only rising edge of comparator output"]
    RISING,
    #[doc = "Falling edge of comparator output"]
    FALLING,
    #[doc = "Any edge of comparator output"]
    ANY,
}
impl EDGETYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EDGETYPW::RISING => 0,
            EDGETYPW::FALLING => 1,
            EDGETYPW::ANY => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EDGETYPW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGETYPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGETYPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Only rising edge of comparator output"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EDGETYPW::RISING)
    }
    #[doc = "Falling edge of comparator output"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EDGETYPW::FALLING)
    }
    #[doc = "Any edge of comparator output"]
    #[inline(always)]
    pub fn any(self) -> &'a mut W {
        self.variant(EDGETYPW::ANY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Possible values of the field `INV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVR {
    #[doc = "Analog comparator output is directly processed."]
    DIS,
    #[doc = "Analog comparator output is inverted prior to being processed."]
    EN,
}
impl crate::ToBits<bool> for INVR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INVR::DIS => false,
            INVR::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INV_R = crate::FR<bool, INVR>;
impl INV_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == INVR::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == INVR::EN
    }
}
#[doc = "Values that can be written to the field `INV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVW {
    #[doc = "Analog comparator output is directly processed."]
    DIS,
    #[doc = "Analog comparator output is inverted prior to being processed."]
    EN,
}
impl INVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            INVW::DIS => false,
            INVW::EN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _INVW<'a> {
    w: &'a mut W,
}
impl<'a> _INVW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Analog comparator output is directly processed."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(INVW::DIS)
    }
    #[doc = "Analog comparator output is inverted prior to being processed."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(INVW::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `SELFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELFSR {
    #[doc = "The CE flag is used to drive the FAULT output."]
    CE,
    #[doc = "The output of the analog comparator flag is used to drive the FAULT output."]
    OUTPUT,
}
impl crate::ToBits<bool> for SELFSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SELFSR::CE => false,
            SELFSR::OUTPUT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SELFS_R = crate::FR<bool, SELFSR>;
impl SELFS_R {
    #[doc = "Checks if the value of the field is `CE`"]
    #[inline(always)]
    pub fn is_ce(&self) -> bool {
        *self == SELFSR::CE
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == SELFSR::OUTPUT
    }
}
#[doc = "Values that can be written to the field `SELFS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELFSW {
    #[doc = "The CE flag is used to drive the FAULT output."]
    CE,
    #[doc = "The output of the analog comparator flag is used to drive the FAULT output."]
    OUTPUT,
}
impl SELFSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SELFSW::CE => false,
            SELFSW::OUTPUT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SELFSW<'a> {
    w: &'a mut W,
}
impl<'a> _SELFSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELFSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The CE flag is used to drive the FAULT output."]
    #[inline(always)]
    pub fn ce(self) -> &'a mut W {
        self.variant(SELFSW::CE)
    }
    #[doc = "The output of the analog comparator flag is used to drive the FAULT output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(SELFSW::OUTPUT)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Possible values of the field `FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FER {
    #[doc = "The FAULT output is tied to 0."]
    DIS,
    #[doc = "The FAULT output is driven by the signal defined by SELFS."]
    EN,
}
impl crate::ToBits<bool> for FER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            FER::DIS => false,
            FER::EN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FE_R = crate::FR<bool, FER>;
impl FE_R {
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FER::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FER::EN
    }
}
#[doc = "Values that can be written to the field `FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEW {
    #[doc = "The FAULT output is tied to 0."]
    DIS,
    #[doc = "The FAULT output is driven by the signal defined by SELFS."]
    EN,
}
impl FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            FEW::DIS => false,
            FEW::EN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FEW<'a> {
    w: &'a mut W,
}
impl<'a> _FEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The FAULT output is tied to 0."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FEW::DIS)
    }
    #[doc = "The FAULT output is driven by the signal defined by SELFS."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FEW::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Selection for Minus Comparator Input"]
    #[inline(always)]
    pub fn selminus(&self) -> SELMINUS_R {
        SELMINUS_R::new((self.bits() & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Selection For Plus Comparator Input"]
    #[inline(always)]
    pub fn selplus(&self) -> SELPLUS_R {
        SELPLUS_R::new(((self.bits() >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn acen(&self) -> ACEN_R {
        ACEN_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Edge Type"]
    #[inline(always)]
    pub fn edgetyp(&self) -> EDGETYP_R {
        EDGETYP_R::new(((self.bits() >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Invert Comparator Output"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Selection Of Fault Source"]
    #[inline(always)]
    pub fn selfs(&self) -> SELFS_R {
        SELFS_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fault Enable"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Selection for Minus Comparator Input"]
    #[inline(always)]
    pub fn selminus(&mut self) -> _SELMINUSW {
        _SELMINUSW { w: self }
    }
    #[doc = "Bits 4:6 - Selection For Plus Comparator Input"]
    #[inline(always)]
    pub fn selplus(&mut self) -> _SELPLUSW {
        _SELPLUSW { w: self }
    }
    #[doc = "Bit 8 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn acen(&mut self) -> _ACENW {
        _ACENW { w: self }
    }
    #[doc = "Bits 9:10 - Edge Type"]
    #[inline(always)]
    pub fn edgetyp(&mut self) -> _EDGETYPW {
        _EDGETYPW { w: self }
    }
    #[doc = "Bit 12 - Invert Comparator Output"]
    #[inline(always)]
    pub fn inv(&mut self) -> _INVW {
        _INVW { w: self }
    }
    #[doc = "Bit 13 - Selection Of Fault Source"]
    #[inline(always)]
    pub fn selfs(&mut self) -> _SELFSW {
        _SELFSW { w: self }
    }
    #[doc = "Bit 14 - Fault Enable"]
    #[inline(always)]
    pub fn fe(&mut self) -> _FEW {
        _FEW { w: self }
    }
}
