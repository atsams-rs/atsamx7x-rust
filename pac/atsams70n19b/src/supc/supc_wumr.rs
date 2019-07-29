#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SUPC_WUMR {
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
#[doc = "Possible values of the field `SMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMENR {
    #[doc = "The supply monitor detection has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "The supply monitor detection forces the wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for SMENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SMENR::NOT_ENABLE => false,
            SMENR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SMEN_R = crate::FR<bool, SMENR>;
impl SMEN_R {
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == SMENR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SMENR::ENABLE
    }
}
#[doc = "Values that can be written to the field `SMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMENW {
    #[doc = "The supply monitor detection has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "The supply monitor detection forces the wake-up of the core power supply."]
    ENABLE,
}
impl SMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SMENW::NOT_ENABLE => false,
            SMENW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SMENW<'a> {
    w: &'a mut W,
}
impl<'a> _SMENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The supply monitor detection has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMENW::NOT_ENABLE)
    }
    #[doc = "The supply monitor detection forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMENW::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `RTTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTTENR {
    #[doc = "The RTT alarm signal has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "The RTT alarm signal forces the wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for RTTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RTTENR::NOT_ENABLE => false,
            RTTENR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RTTEN_R = crate::FR<bool, RTTENR>;
impl RTTEN_R {
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == RTTENR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTTENR::ENABLE
    }
}
#[doc = "Values that can be written to the field `RTTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTTENW {
    #[doc = "The RTT alarm signal has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "The RTT alarm signal forces the wake-up of the core power supply."]
    ENABLE,
}
impl RTTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RTTENW::NOT_ENABLE => false,
            RTTENW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RTTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The RTT alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(RTTENW::NOT_ENABLE)
    }
    #[doc = "The RTT alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTTENW::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `RTCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCENR {
    #[doc = "The RTC alarm signal has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "The RTC alarm signal forces the wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for RTCENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RTCENR::NOT_ENABLE => false,
            RTCENR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RTCEN_R = crate::FR<bool, RTCENR>;
impl RTCEN_R {
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == RTCENR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RTCENR::ENABLE
    }
}
#[doc = "Values that can be written to the field `RTCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCENW {
    #[doc = "The RTC alarm signal has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "The RTC alarm signal forces the wake-up of the core power supply."]
    ENABLE,
}
impl RTCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCENW::NOT_ENABLE => false,
            RTCENW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RTCENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The RTC alarm signal has no wake-up effect."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(RTCENW::NOT_ENABLE)
    }
    #[doc = "The RTC alarm signal forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTCENW::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Possible values of the field `LPDBCEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCEN0R {
    #[doc = "The WKUP0 input pin is not connected to the low-power debouncer."]
    NOT_ENABLE,
    #[doc = "The WKUP0 input pin is connected to the low-power debouncer and forces a system wake-up."]
    ENABLE,
}
impl crate::ToBits<bool> for LPDBCEN0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LPDBCEN0R::NOT_ENABLE => false,
            LPDBCEN0R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LPDBCEN0_R = crate::FR<bool, LPDBCEN0R>;
impl LPDBCEN0_R {
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == LPDBCEN0R::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LPDBCEN0R::ENABLE
    }
}
#[doc = "Values that can be written to the field `LPDBCEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCEN0W {
    #[doc = "The WKUP0 input pin is not connected to the low-power debouncer."]
    NOT_ENABLE,
    #[doc = "The WKUP0 input pin is connected to the low-power debouncer and forces a system wake-up."]
    ENABLE,
}
impl LPDBCEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LPDBCEN0W::NOT_ENABLE => false,
            LPDBCEN0W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LPDBCEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _LPDBCEN0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPDBCEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The WKUP0 input pin is not connected to the low-power debouncer."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(LPDBCEN0W::NOT_ENABLE)
    }
    #[doc = "The WKUP0 input pin is connected to the low-power debouncer and forces a system wake-up."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPDBCEN0W::ENABLE)
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
#[doc = "Possible values of the field `LPDBCEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCEN1R {
    #[doc = "The WKUP1 input pin is not connected to the low-power debouncer."]
    NOT_ENABLE,
    #[doc = "The WKUP1 input pin is connected to the low-power debouncer and forces a system wake-up."]
    ENABLE,
}
impl crate::ToBits<bool> for LPDBCEN1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LPDBCEN1R::NOT_ENABLE => false,
            LPDBCEN1R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LPDBCEN1_R = crate::FR<bool, LPDBCEN1R>;
impl LPDBCEN1_R {
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == LPDBCEN1R::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LPDBCEN1R::ENABLE
    }
}
#[doc = "Values that can be written to the field `LPDBCEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCEN1W {
    #[doc = "The WKUP1 input pin is not connected to the low-power debouncer."]
    NOT_ENABLE,
    #[doc = "The WKUP1 input pin is connected to the low-power debouncer and forces a system wake-up."]
    ENABLE,
}
impl LPDBCEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LPDBCEN1W::NOT_ENABLE => false,
            LPDBCEN1W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LPDBCEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _LPDBCEN1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPDBCEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The WKUP1 input pin is not connected to the low-power debouncer."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(LPDBCEN1W::NOT_ENABLE)
    }
    #[doc = "The WKUP1 input pin is connected to the low-power debouncer and forces a system wake-up."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPDBCEN1W::ENABLE)
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
#[doc = "Possible values of the field `LPDBCCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCCLRR {
    #[doc = "A low-power debounce event does not create an immediate clear on the first half of GPBR registers."]
    NOT_ENABLE,
    #[doc = "A low-power debounce event on WKUP0 or WKUP1 generates an immediate clear on the first half of GPBR registers."]
    ENABLE,
}
impl crate::ToBits<bool> for LPDBCCLRR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LPDBCCLRR::NOT_ENABLE => false,
            LPDBCCLRR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LPDBCCLR_R = crate::FR<bool, LPDBCCLRR>;
impl LPDBCCLR_R {
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == LPDBCCLRR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == LPDBCCLRR::ENABLE
    }
}
#[doc = "Values that can be written to the field `LPDBCCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCCLRW {
    #[doc = "A low-power debounce event does not create an immediate clear on the first half of GPBR registers."]
    NOT_ENABLE,
    #[doc = "A low-power debounce event on WKUP0 or WKUP1 generates an immediate clear on the first half of GPBR registers."]
    ENABLE,
}
impl LPDBCCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LPDBCCLRW::NOT_ENABLE => false,
            LPDBCCLRW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LPDBCCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _LPDBCCLRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPDBCCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A low-power debounce event does not create an immediate clear on the first half of GPBR registers."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(LPDBCCLRW::NOT_ENABLE)
    }
    #[doc = "A low-power debounce event on WKUP0 or WKUP1 generates an immediate clear on the first half of GPBR registers."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPDBCCLRW::ENABLE)
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
#[doc = "Possible values of the field `WKUPDBC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPDBCR {
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    IMMEDIATE,
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    _3_SLCK,
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    _32_SLCK,
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    _512_SLCK,
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    _4096_SLCK,
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    _32768_SLCK,
}
impl crate::ToBits<u8> for WKUPDBCR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            WKUPDBCR::IMMEDIATE => 0,
            WKUPDBCR::_3_SLCK => 1,
            WKUPDBCR::_32_SLCK => 2,
            WKUPDBCR::_512_SLCK => 3,
            WKUPDBCR::_4096_SLCK => 4,
            WKUPDBCR::_32768_SLCK => 5,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPDBC_R = crate::FR<u8, WKUPDBCR>;
impl WKUPDBC_R {
    #[doc = "Checks if the value of the field is `IMMEDIATE`"]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == WKUPDBCR::IMMEDIATE
    }
    #[doc = "Checks if the value of the field is `_3_SLCK`"]
    #[inline(always)]
    pub fn is_3_slck(&self) -> bool {
        *self == WKUPDBCR::_3_SLCK
    }
    #[doc = "Checks if the value of the field is `_32_SLCK`"]
    #[inline(always)]
    pub fn is_32_slck(&self) -> bool {
        *self == WKUPDBCR::_32_SLCK
    }
    #[doc = "Checks if the value of the field is `_512_SLCK`"]
    #[inline(always)]
    pub fn is_512_slck(&self) -> bool {
        *self == WKUPDBCR::_512_SLCK
    }
    #[doc = "Checks if the value of the field is `_4096_SLCK`"]
    #[inline(always)]
    pub fn is_4096_slck(&self) -> bool {
        *self == WKUPDBCR::_4096_SLCK
    }
    #[doc = "Checks if the value of the field is `_32768_SLCK`"]
    #[inline(always)]
    pub fn is_32768_slck(&self) -> bool {
        *self == WKUPDBCR::_32768_SLCK
    }
}
#[doc = "Values that can be written to the field `WKUPDBC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPDBCW {
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    IMMEDIATE,
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    _3_SLCK,
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    _32_SLCK,
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    _512_SLCK,
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    _4096_SLCK,
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    _32768_SLCK,
}
impl WKUPDBCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            WKUPDBCW::IMMEDIATE => 0,
            WKUPDBCW::_3_SLCK => 1,
            WKUPDBCW::_32_SLCK => 2,
            WKUPDBCW::_512_SLCK => 3,
            WKUPDBCW::_4096_SLCK => 4,
            WKUPDBCW::_32768_SLCK => 5,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPDBCW<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPDBCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPDBCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut W {
        self.variant(WKUPDBCW::IMMEDIATE)
    }
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    #[inline(always)]
    pub fn _3_slck(self) -> &'a mut W {
        self.variant(WKUPDBCW::_3_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    #[inline(always)]
    pub fn _32_slck(self) -> &'a mut W {
        self.variant(WKUPDBCW::_32_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    #[inline(always)]
    pub fn _512_slck(self) -> &'a mut W {
        self.variant(WKUPDBCW::_512_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    #[inline(always)]
    pub fn _4096_slck(self) -> &'a mut W {
        self.variant(WKUPDBCW::_4096_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    #[inline(always)]
    pub fn _32768_slck(self) -> &'a mut W {
        self.variant(WKUPDBCW::_32768_SLCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `LPDBC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCR {
    #[doc = "Disable the low-power debouncers."]
    DISABLE,
    #[doc = "WKUP0/1 in active state for at least 2 RTCOUTx clock periods"]
    _2_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 3 RTCOUTx clock periods"]
    _3_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 4 RTCOUTx clock periods"]
    _4_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 5 RTCOUTx clock periods"]
    _5_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 6 RTCOUTx clock periods"]
    _6_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 7 RTCOUTx clock periods"]
    _7_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 8 RTCOUTx clock periods"]
    _8_RTCOUT,
}
impl crate::ToBits<u8> for LPDBCR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            LPDBCR::DISABLE => 0,
            LPDBCR::_2_RTCOUT => 1,
            LPDBCR::_3_RTCOUT => 2,
            LPDBCR::_4_RTCOUT => 3,
            LPDBCR::_5_RTCOUT => 4,
            LPDBCR::_6_RTCOUT => 5,
            LPDBCR::_7_RTCOUT => 6,
            LPDBCR::_8_RTCOUT => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LPDBC_R = crate::FR<u8, LPDBCR>;
impl LPDBC_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == LPDBCR::DISABLE
    }
    #[doc = "Checks if the value of the field is `_2_RTCOUT`"]
    #[inline(always)]
    pub fn is_2_rtcout(&self) -> bool {
        *self == LPDBCR::_2_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_3_RTCOUT`"]
    #[inline(always)]
    pub fn is_3_rtcout(&self) -> bool {
        *self == LPDBCR::_3_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_4_RTCOUT`"]
    #[inline(always)]
    pub fn is_4_rtcout(&self) -> bool {
        *self == LPDBCR::_4_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_5_RTCOUT`"]
    #[inline(always)]
    pub fn is_5_rtcout(&self) -> bool {
        *self == LPDBCR::_5_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_6_RTCOUT`"]
    #[inline(always)]
    pub fn is_6_rtcout(&self) -> bool {
        *self == LPDBCR::_6_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_7_RTCOUT`"]
    #[inline(always)]
    pub fn is_7_rtcout(&self) -> bool {
        *self == LPDBCR::_7_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_8_RTCOUT`"]
    #[inline(always)]
    pub fn is_8_rtcout(&self) -> bool {
        *self == LPDBCR::_8_RTCOUT
    }
}
#[doc = "Values that can be written to the field `LPDBC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCW {
    #[doc = "Disable the low-power debouncers."]
    DISABLE,
    #[doc = "WKUP0/1 in active state for at least 2 RTCOUTx clock periods"]
    _2_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 3 RTCOUTx clock periods"]
    _3_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 4 RTCOUTx clock periods"]
    _4_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 5 RTCOUTx clock periods"]
    _5_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 6 RTCOUTx clock periods"]
    _6_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 7 RTCOUTx clock periods"]
    _7_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 8 RTCOUTx clock periods"]
    _8_RTCOUT,
}
impl LPDBCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPDBCW::DISABLE => 0,
            LPDBCW::_2_RTCOUT => 1,
            LPDBCW::_3_RTCOUT => 2,
            LPDBCW::_4_RTCOUT => 3,
            LPDBCW::_5_RTCOUT => 4,
            LPDBCW::_6_RTCOUT => 5,
            LPDBCW::_7_RTCOUT => 6,
            LPDBCW::_8_RTCOUT => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LPDBCW<'a> {
    w: &'a mut W,
}
impl<'a> _LPDBCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPDBCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disable the low-power debouncers."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(LPDBCW::DISABLE)
    }
    #[doc = "WKUP0/1 in active state for at least 2 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _2_rtcout(self) -> &'a mut W {
        self.variant(LPDBCW::_2_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 3 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _3_rtcout(self) -> &'a mut W {
        self.variant(LPDBCW::_3_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 4 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _4_rtcout(self) -> &'a mut W {
        self.variant(LPDBCW::_4_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 5 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _5_rtcout(self) -> &'a mut W {
        self.variant(LPDBCW::_5_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 6 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _6_rtcout(self) -> &'a mut W {
        self.variant(LPDBCW::_6_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 7 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _7_rtcout(self) -> &'a mut W {
        self.variant(LPDBCW::_7_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 8 RTCOUTx clock periods"]
    #[inline(always)]
    pub fn _8_rtcout(self) -> &'a mut W {
        self.variant(LPDBCW::_8_RTCOUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SMEN_R {
        SMEN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Real-time Timer Wake-up Enable"]
    #[inline(always)]
    pub fn rtten(&self) -> RTTEN_R {
        RTTEN_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Real-time Clock Wake-up Enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Low-power Debouncer Enable WKUP0"]
    #[inline(always)]
    pub fn lpdbcen0(&self) -> LPDBCEN0_R {
        LPDBCEN0_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Low-power Debouncer Enable WKUP1"]
    #[inline(always)]
    pub fn lpdbcen1(&self) -> LPDBCEN1_R {
        LPDBCEN1_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Low-power Debouncer Clear"]
    #[inline(always)]
    pub fn lpdbcclr(&self) -> LPDBCCLR_R {
        LPDBCCLR_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline(always)]
    pub fn wkupdbc(&self) -> WKUPDBC_R {
        WKUPDBC_R::new(((self.bits() >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Low-power Debouncer Period"]
    #[inline(always)]
    pub fn lpdbc(&self) -> LPDBC_R {
        LPDBC_R::new(((self.bits() >> 16) & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline(always)]
    pub fn smen(&mut self) -> _SMENW {
        _SMENW { w: self }
    }
    #[doc = "Bit 2 - Real-time Timer Wake-up Enable"]
    #[inline(always)]
    pub fn rtten(&mut self) -> _RTTENW {
        _RTTENW { w: self }
    }
    #[doc = "Bit 3 - Real-time Clock Wake-up Enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> _RTCENW {
        _RTCENW { w: self }
    }
    #[doc = "Bit 5 - Low-power Debouncer Enable WKUP0"]
    #[inline(always)]
    pub fn lpdbcen0(&mut self) -> _LPDBCEN0W {
        _LPDBCEN0W { w: self }
    }
    #[doc = "Bit 6 - Low-power Debouncer Enable WKUP1"]
    #[inline(always)]
    pub fn lpdbcen1(&mut self) -> _LPDBCEN1W {
        _LPDBCEN1W { w: self }
    }
    #[doc = "Bit 7 - Low-power Debouncer Clear"]
    #[inline(always)]
    pub fn lpdbcclr(&mut self) -> _LPDBCCLRW {
        _LPDBCCLRW { w: self }
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline(always)]
    pub fn wkupdbc(&mut self) -> _WKUPDBCW {
        _WKUPDBCW { w: self }
    }
    #[doc = "Bits 16:18 - Low-power Debouncer Period"]
    #[inline(always)]
    pub fn lpdbc(&mut self) -> _LPDBCW {
        _LPDBCW { w: self }
    }
}
