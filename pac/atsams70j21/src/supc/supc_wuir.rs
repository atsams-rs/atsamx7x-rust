#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SUPC_WUIR {
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
#[doc = "Possible values of the field `WKUPEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN0R {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for WKUPEN0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPEN0R::DISABLE => false,
            WKUPEN0R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPEN0_R = crate::FR<bool, WKUPEN0R>;
impl WKUPEN0_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN0R::ENABLE
    }
}
#[doc = "Values that can be written to the field `WKUPEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN0W {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN0W::DISABLE => false,
            WKUPEN0W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN0W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN0W::ENABLE)
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
#[doc = "Possible values of the field `WKUPEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN1R {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for WKUPEN1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPEN1R::DISABLE => false,
            WKUPEN1R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPEN1_R = crate::FR<bool, WKUPEN1R>;
impl WKUPEN1_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN1R::ENABLE
    }
}
#[doc = "Values that can be written to the field `WKUPEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN1W {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN1W::DISABLE => false,
            WKUPEN1W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN1W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN1W::ENABLE)
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
#[doc = "Possible values of the field `WKUPEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN2R {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for WKUPEN2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPEN2R::DISABLE => false,
            WKUPEN2R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPEN2_R = crate::FR<bool, WKUPEN2R>;
impl WKUPEN2_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN2R::ENABLE
    }
}
#[doc = "Values that can be written to the field `WKUPEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN2W {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN2W::DISABLE => false,
            WKUPEN2W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN2W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN2W::ENABLE)
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
#[doc = "Possible values of the field `WKUPEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN3R {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for WKUPEN3R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPEN3R::DISABLE => false,
            WKUPEN3R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPEN3_R = crate::FR<bool, WKUPEN3R>;
impl WKUPEN3_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN3R::ENABLE
    }
}
#[doc = "Values that can be written to the field `WKUPEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN3W {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN3W::DISABLE => false,
            WKUPEN3W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN3W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN3W::ENABLE)
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
#[doc = "Possible values of the field `WKUPEN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN4R {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for WKUPEN4R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPEN4R::DISABLE => false,
            WKUPEN4R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPEN4_R = crate::FR<bool, WKUPEN4R>;
impl WKUPEN4_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN4R::ENABLE
    }
}
#[doc = "Values that can be written to the field `WKUPEN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN4W {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN4W::DISABLE => false,
            WKUPEN4W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN4W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN4W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN4W::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `WKUPEN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN5R {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for WKUPEN5R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPEN5R::DISABLE => false,
            WKUPEN5R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPEN5_R = crate::FR<bool, WKUPEN5R>;
impl WKUPEN5_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN5R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN5R::ENABLE
    }
}
#[doc = "Values that can be written to the field `WKUPEN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN5W {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN5W::DISABLE => false,
            WKUPEN5W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN5W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN5W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN5W::ENABLE)
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
#[doc = "Possible values of the field `WKUPEN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN6R {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for WKUPEN6R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPEN6R::DISABLE => false,
            WKUPEN6R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPEN6_R = crate::FR<bool, WKUPEN6R>;
impl WKUPEN6_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN6R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN6R::ENABLE
    }
}
#[doc = "Values that can be written to the field `WKUPEN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN6W {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN6W::DISABLE => false,
            WKUPEN6W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPEN6W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN6W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN6W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN6W::ENABLE)
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
#[doc = "Possible values of the field `WKUPEN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN7R {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for WKUPEN7R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPEN7R::DISABLE => false,
            WKUPEN7R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPEN7_R = crate::FR<bool, WKUPEN7R>;
impl WKUPEN7_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN7R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN7R::ENABLE
    }
}
#[doc = "Values that can be written to the field `WKUPEN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN7W {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN7W::DISABLE => false,
            WKUPEN7W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPEN7W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN7W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN7W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN7W::ENABLE)
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
#[doc = "Possible values of the field `WKUPEN8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN8R {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for WKUPEN8R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPEN8R::DISABLE => false,
            WKUPEN8R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPEN8_R = crate::FR<bool, WKUPEN8R>;
impl WKUPEN8_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN8R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN8R::ENABLE
    }
}
#[doc = "Values that can be written to the field `WKUPEN8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN8W {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN8W::DISABLE => false,
            WKUPEN8W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPEN8W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN8W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN8W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN8W::ENABLE)
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
#[doc = "Possible values of the field `WKUPEN9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN9R {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for WKUPEN9R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPEN9R::DISABLE => false,
            WKUPEN9R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPEN9_R = crate::FR<bool, WKUPEN9R>;
impl WKUPEN9_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN9R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN9R::ENABLE
    }
}
#[doc = "Values that can be written to the field `WKUPEN9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN9W {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN9W::DISABLE => false,
            WKUPEN9W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPEN9W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN9W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN9W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN9W::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Possible values of the field `WKUPEN10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN10R {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for WKUPEN10R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPEN10R::DISABLE => false,
            WKUPEN10R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPEN10_R = crate::FR<bool, WKUPEN10R>;
impl WKUPEN10_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN10R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN10R::ENABLE
    }
}
#[doc = "Values that can be written to the field `WKUPEN10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN10W {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN10W::DISABLE => false,
            WKUPEN10W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPEN10W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN10W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN10W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN10W::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `WKUPEN11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN11R {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for WKUPEN11R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPEN11R::DISABLE => false,
            WKUPEN11R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPEN11_R = crate::FR<bool, WKUPEN11R>;
impl WKUPEN11_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN11R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN11R::ENABLE
    }
}
#[doc = "Values that can be written to the field `WKUPEN11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN11W {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN11W::DISABLE => false,
            WKUPEN11W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPEN11W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN11W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN11W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN11W::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Possible values of the field `WKUPEN12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN12R {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for WKUPEN12R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPEN12R::DISABLE => false,
            WKUPEN12R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPEN12_R = crate::FR<bool, WKUPEN12R>;
impl WKUPEN12_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN12R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN12R::ENABLE
    }
}
#[doc = "Values that can be written to the field `WKUPEN12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN12W {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN12W::DISABLE => false,
            WKUPEN12W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPEN12W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN12W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN12W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN12W::ENABLE)
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
#[doc = "Possible values of the field `WKUPEN13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN13R {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl crate::ToBits<bool> for WKUPEN13R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPEN13R::DISABLE => false,
            WKUPEN13R::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPEN13_R = crate::FR<bool, WKUPEN13R>;
impl WKUPEN13_R {
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN13R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN13R::ENABLE
    }
}
#[doc = "Values that can be written to the field `WKUPEN13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN13W {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN13W::DISABLE => false,
            WKUPEN13W::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPEN13W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN13W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPEN13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN13W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN13W::ENABLE)
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
#[doc = "Possible values of the field `WKUPT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT0R {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl crate::ToBits<bool> for WKUPT0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPT0R::LOW => false,
            WKUPT0R::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPT0_R = crate::FR<bool, WKUPT0R>;
impl WKUPT0_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT0R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT0R::HIGH
    }
}
#[doc = "Values that can be written to the field `WKUPT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT0W {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl WKUPT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT0W::LOW => false,
            WKUPT0W::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPT0W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT0W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT0W::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `WKUPT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT1R {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl crate::ToBits<bool> for WKUPT1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPT1R::LOW => false,
            WKUPT1R::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPT1_R = crate::FR<bool, WKUPT1R>;
impl WKUPT1_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT1R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT1R::HIGH
    }
}
#[doc = "Values that can be written to the field `WKUPT1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT1W {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl WKUPT1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT1W::LOW => false,
            WKUPT1W::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPT1W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT1W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT1W::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Possible values of the field `WKUPT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT2R {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl crate::ToBits<bool> for WKUPT2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPT2R::LOW => false,
            WKUPT2R::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPT2_R = crate::FR<bool, WKUPT2R>;
impl WKUPT2_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT2R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT2R::HIGH
    }
}
#[doc = "Values that can be written to the field `WKUPT2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT2W {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl WKUPT2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT2W::LOW => false,
            WKUPT2W::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPT2W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT2W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT2W::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `WKUPT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT3R {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl crate::ToBits<bool> for WKUPT3R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPT3R::LOW => false,
            WKUPT3R::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPT3_R = crate::FR<bool, WKUPT3R>;
impl WKUPT3_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT3R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT3R::HIGH
    }
}
#[doc = "Values that can be written to the field `WKUPT3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT3W {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl WKUPT3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT3W::LOW => false,
            WKUPT3W::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPT3W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT3W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT3W::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Possible values of the field `WKUPT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT4R {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl crate::ToBits<bool> for WKUPT4R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPT4R::LOW => false,
            WKUPT4R::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPT4_R = crate::FR<bool, WKUPT4R>;
impl WKUPT4_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT4R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT4R::HIGH
    }
}
#[doc = "Values that can be written to the field `WKUPT4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT4W {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl WKUPT4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT4W::LOW => false,
            WKUPT4W::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPT4W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT4W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT4W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT4W::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `WKUPT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT5R {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl crate::ToBits<bool> for WKUPT5R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPT5R::LOW => false,
            WKUPT5R::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPT5_R = crate::FR<bool, WKUPT5R>;
impl WKUPT5_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT5R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT5R::HIGH
    }
}
#[doc = "Values that can be written to the field `WKUPT5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT5W {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl WKUPT5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT5W::LOW => false,
            WKUPT5W::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPT5W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT5W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT5W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT5W::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Possible values of the field `WKUPT6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT6R {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl crate::ToBits<bool> for WKUPT6R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPT6R::LOW => false,
            WKUPT6R::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPT6_R = crate::FR<bool, WKUPT6R>;
impl WKUPT6_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT6R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT6R::HIGH
    }
}
#[doc = "Values that can be written to the field `WKUPT6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT6W {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl WKUPT6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT6W::LOW => false,
            WKUPT6W::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPT6W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT6W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT6W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT6W::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `WKUPT7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT7R {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl crate::ToBits<bool> for WKUPT7R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPT7R::LOW => false,
            WKUPT7R::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPT7_R = crate::FR<bool, WKUPT7R>;
impl WKUPT7_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT7R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT7R::HIGH
    }
}
#[doc = "Values that can be written to the field `WKUPT7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT7W {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl WKUPT7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT7W::LOW => false,
            WKUPT7W::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPT7W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT7W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT7W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT7W::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Possible values of the field `WKUPT8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT8R {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl crate::ToBits<bool> for WKUPT8R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPT8R::LOW => false,
            WKUPT8R::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPT8_R = crate::FR<bool, WKUPT8R>;
impl WKUPT8_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT8R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT8R::HIGH
    }
}
#[doc = "Values that can be written to the field `WKUPT8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT8W {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl WKUPT8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT8W::LOW => false,
            WKUPT8W::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPT8W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT8W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT8W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT8W::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `WKUPT9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT9R {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl crate::ToBits<bool> for WKUPT9R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPT9R::LOW => false,
            WKUPT9R::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPT9_R = crate::FR<bool, WKUPT9R>;
impl WKUPT9_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT9R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT9R::HIGH
    }
}
#[doc = "Values that can be written to the field `WKUPT9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT9W {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl WKUPT9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT9W::LOW => false,
            WKUPT9W::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPT9W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT9W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT9W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT9W::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Possible values of the field `WKUPT10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT10R {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl crate::ToBits<bool> for WKUPT10R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPT10R::LOW => false,
            WKUPT10R::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPT10_R = crate::FR<bool, WKUPT10R>;
impl WKUPT10_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT10R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT10R::HIGH
    }
}
#[doc = "Values that can be written to the field `WKUPT10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT10W {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl WKUPT10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT10W::LOW => false,
            WKUPT10W::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPT10W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT10W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT10W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT10W::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Possible values of the field `WKUPT11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT11R {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl crate::ToBits<bool> for WKUPT11R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPT11R::LOW => false,
            WKUPT11R::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPT11_R = crate::FR<bool, WKUPT11R>;
impl WKUPT11_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT11R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT11R::HIGH
    }
}
#[doc = "Values that can be written to the field `WKUPT11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT11W {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl WKUPT11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT11W::LOW => false,
            WKUPT11W::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPT11W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT11W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT11W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT11W::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Possible values of the field `WKUPT12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT12R {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl crate::ToBits<bool> for WKUPT12R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPT12R::LOW => false,
            WKUPT12R::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPT12_R = crate::FR<bool, WKUPT12R>;
impl WKUPT12_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT12R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT12R::HIGH
    }
}
#[doc = "Values that can be written to the field `WKUPT12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT12W {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl WKUPT12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT12W::LOW => false,
            WKUPT12W::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPT12W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT12W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT12W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT12W::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Possible values of the field `WKUPT13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT13R {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl crate::ToBits<bool> for WKUPT13R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WKUPT13R::LOW => false,
            WKUPT13R::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WKUPT13_R = crate::FR<bool, WKUPT13R>;
impl WKUPT13_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT13R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT13R::HIGH
    }
}
#[doc = "Values that can be written to the field `WKUPT13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT13W {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    LOW,
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    HIGH,
}
impl WKUPT13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT13W::LOW => false,
            WKUPT13W::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WKUPT13W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT13W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKUPT13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT13W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT13W::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Wake-up Input Enable 0 to 0"]
    #[inline(always)]
    pub fn wkupen0(&self) -> WKUPEN0_R {
        WKUPEN0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wake-up Input Enable 0 to 1"]
    #[inline(always)]
    pub fn wkupen1(&self) -> WKUPEN1_R {
        WKUPEN1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wake-up Input Enable 0 to 2"]
    #[inline(always)]
    pub fn wkupen2(&self) -> WKUPEN2_R {
        WKUPEN2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wake-up Input Enable 0 to 3"]
    #[inline(always)]
    pub fn wkupen3(&self) -> WKUPEN3_R {
        WKUPEN3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wake-up Input Enable 0 to 4"]
    #[inline(always)]
    pub fn wkupen4(&self) -> WKUPEN4_R {
        WKUPEN4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wake-up Input Enable 0 to 5"]
    #[inline(always)]
    pub fn wkupen5(&self) -> WKUPEN5_R {
        WKUPEN5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wake-up Input Enable 0 to 6"]
    #[inline(always)]
    pub fn wkupen6(&self) -> WKUPEN6_R {
        WKUPEN6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wake-up Input Enable 0 to 7"]
    #[inline(always)]
    pub fn wkupen7(&self) -> WKUPEN7_R {
        WKUPEN7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Wake-up Input Enable 0 to 8"]
    #[inline(always)]
    pub fn wkupen8(&self) -> WKUPEN8_R {
        WKUPEN8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Wake-up Input Enable 0 to 9"]
    #[inline(always)]
    pub fn wkupen9(&self) -> WKUPEN9_R {
        WKUPEN9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Wake-up Input Enable 0 to 10"]
    #[inline(always)]
    pub fn wkupen10(&self) -> WKUPEN10_R {
        WKUPEN10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Wake-up Input Enable 0 to 11"]
    #[inline(always)]
    pub fn wkupen11(&self) -> WKUPEN11_R {
        WKUPEN11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Wake-up Input Enable 0 to 12"]
    #[inline(always)]
    pub fn wkupen12(&self) -> WKUPEN12_R {
        WKUPEN12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Wake-up Input Enable 0 to 13"]
    #[inline(always)]
    pub fn wkupen13(&self) -> WKUPEN13_R {
        WKUPEN13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Wake-up Input Type 0 to 0"]
    #[inline(always)]
    pub fn wkupt0(&self) -> WKUPT0_R {
        WKUPT0_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Wake-up Input Type 0 to 1"]
    #[inline(always)]
    pub fn wkupt1(&self) -> WKUPT1_R {
        WKUPT1_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Wake-up Input Type 0 to 2"]
    #[inline(always)]
    pub fn wkupt2(&self) -> WKUPT2_R {
        WKUPT2_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Wake-up Input Type 0 to 3"]
    #[inline(always)]
    pub fn wkupt3(&self) -> WKUPT3_R {
        WKUPT3_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Wake-up Input Type 0 to 4"]
    #[inline(always)]
    pub fn wkupt4(&self) -> WKUPT4_R {
        WKUPT4_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Wake-up Input Type 0 to 5"]
    #[inline(always)]
    pub fn wkupt5(&self) -> WKUPT5_R {
        WKUPT5_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Wake-up Input Type 0 to 6"]
    #[inline(always)]
    pub fn wkupt6(&self) -> WKUPT6_R {
        WKUPT6_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Wake-up Input Type 0 to 7"]
    #[inline(always)]
    pub fn wkupt7(&self) -> WKUPT7_R {
        WKUPT7_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Wake-up Input Type 0 to 8"]
    #[inline(always)]
    pub fn wkupt8(&self) -> WKUPT8_R {
        WKUPT8_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Wake-up Input Type 0 to 9"]
    #[inline(always)]
    pub fn wkupt9(&self) -> WKUPT9_R {
        WKUPT9_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Wake-up Input Type 0 to 10"]
    #[inline(always)]
    pub fn wkupt10(&self) -> WKUPT10_R {
        WKUPT10_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Wake-up Input Type 0 to 11"]
    #[inline(always)]
    pub fn wkupt11(&self) -> WKUPT11_R {
        WKUPT11_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Wake-up Input Type 0 to 12"]
    #[inline(always)]
    pub fn wkupt12(&self) -> WKUPT12_R {
        WKUPT12_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Wake-up Input Type 0 to 13"]
    #[inline(always)]
    pub fn wkupt13(&self) -> WKUPT13_R {
        WKUPT13_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Wake-up Input Enable 0 to 0"]
    #[inline(always)]
    pub fn wkupen0(&mut self) -> _WKUPEN0W {
        _WKUPEN0W { w: self }
    }
    #[doc = "Bit 1 - Wake-up Input Enable 0 to 1"]
    #[inline(always)]
    pub fn wkupen1(&mut self) -> _WKUPEN1W {
        _WKUPEN1W { w: self }
    }
    #[doc = "Bit 2 - Wake-up Input Enable 0 to 2"]
    #[inline(always)]
    pub fn wkupen2(&mut self) -> _WKUPEN2W {
        _WKUPEN2W { w: self }
    }
    #[doc = "Bit 3 - Wake-up Input Enable 0 to 3"]
    #[inline(always)]
    pub fn wkupen3(&mut self) -> _WKUPEN3W {
        _WKUPEN3W { w: self }
    }
    #[doc = "Bit 4 - Wake-up Input Enable 0 to 4"]
    #[inline(always)]
    pub fn wkupen4(&mut self) -> _WKUPEN4W {
        _WKUPEN4W { w: self }
    }
    #[doc = "Bit 5 - Wake-up Input Enable 0 to 5"]
    #[inline(always)]
    pub fn wkupen5(&mut self) -> _WKUPEN5W {
        _WKUPEN5W { w: self }
    }
    #[doc = "Bit 6 - Wake-up Input Enable 0 to 6"]
    #[inline(always)]
    pub fn wkupen6(&mut self) -> _WKUPEN6W {
        _WKUPEN6W { w: self }
    }
    #[doc = "Bit 7 - Wake-up Input Enable 0 to 7"]
    #[inline(always)]
    pub fn wkupen7(&mut self) -> _WKUPEN7W {
        _WKUPEN7W { w: self }
    }
    #[doc = "Bit 8 - Wake-up Input Enable 0 to 8"]
    #[inline(always)]
    pub fn wkupen8(&mut self) -> _WKUPEN8W {
        _WKUPEN8W { w: self }
    }
    #[doc = "Bit 9 - Wake-up Input Enable 0 to 9"]
    #[inline(always)]
    pub fn wkupen9(&mut self) -> _WKUPEN9W {
        _WKUPEN9W { w: self }
    }
    #[doc = "Bit 10 - Wake-up Input Enable 0 to 10"]
    #[inline(always)]
    pub fn wkupen10(&mut self) -> _WKUPEN10W {
        _WKUPEN10W { w: self }
    }
    #[doc = "Bit 11 - Wake-up Input Enable 0 to 11"]
    #[inline(always)]
    pub fn wkupen11(&mut self) -> _WKUPEN11W {
        _WKUPEN11W { w: self }
    }
    #[doc = "Bit 12 - Wake-up Input Enable 0 to 12"]
    #[inline(always)]
    pub fn wkupen12(&mut self) -> _WKUPEN12W {
        _WKUPEN12W { w: self }
    }
    #[doc = "Bit 13 - Wake-up Input Enable 0 to 13"]
    #[inline(always)]
    pub fn wkupen13(&mut self) -> _WKUPEN13W {
        _WKUPEN13W { w: self }
    }
    #[doc = "Bit 16 - Wake-up Input Type 0 to 0"]
    #[inline(always)]
    pub fn wkupt0(&mut self) -> _WKUPT0W {
        _WKUPT0W { w: self }
    }
    #[doc = "Bit 17 - Wake-up Input Type 0 to 1"]
    #[inline(always)]
    pub fn wkupt1(&mut self) -> _WKUPT1W {
        _WKUPT1W { w: self }
    }
    #[doc = "Bit 18 - Wake-up Input Type 0 to 2"]
    #[inline(always)]
    pub fn wkupt2(&mut self) -> _WKUPT2W {
        _WKUPT2W { w: self }
    }
    #[doc = "Bit 19 - Wake-up Input Type 0 to 3"]
    #[inline(always)]
    pub fn wkupt3(&mut self) -> _WKUPT3W {
        _WKUPT3W { w: self }
    }
    #[doc = "Bit 20 - Wake-up Input Type 0 to 4"]
    #[inline(always)]
    pub fn wkupt4(&mut self) -> _WKUPT4W {
        _WKUPT4W { w: self }
    }
    #[doc = "Bit 21 - Wake-up Input Type 0 to 5"]
    #[inline(always)]
    pub fn wkupt5(&mut self) -> _WKUPT5W {
        _WKUPT5W { w: self }
    }
    #[doc = "Bit 22 - Wake-up Input Type 0 to 6"]
    #[inline(always)]
    pub fn wkupt6(&mut self) -> _WKUPT6W {
        _WKUPT6W { w: self }
    }
    #[doc = "Bit 23 - Wake-up Input Type 0 to 7"]
    #[inline(always)]
    pub fn wkupt7(&mut self) -> _WKUPT7W {
        _WKUPT7W { w: self }
    }
    #[doc = "Bit 24 - Wake-up Input Type 0 to 8"]
    #[inline(always)]
    pub fn wkupt8(&mut self) -> _WKUPT8W {
        _WKUPT8W { w: self }
    }
    #[doc = "Bit 25 - Wake-up Input Type 0 to 9"]
    #[inline(always)]
    pub fn wkupt9(&mut self) -> _WKUPT9W {
        _WKUPT9W { w: self }
    }
    #[doc = "Bit 26 - Wake-up Input Type 0 to 10"]
    #[inline(always)]
    pub fn wkupt10(&mut self) -> _WKUPT10W {
        _WKUPT10W { w: self }
    }
    #[doc = "Bit 27 - Wake-up Input Type 0 to 11"]
    #[inline(always)]
    pub fn wkupt11(&mut self) -> _WKUPT11W {
        _WKUPT11W { w: self }
    }
    #[doc = "Bit 28 - Wake-up Input Type 0 to 12"]
    #[inline(always)]
    pub fn wkupt12(&mut self) -> _WKUPT12W {
        _WKUPT12W { w: self }
    }
    #[doc = "Bit 29 - Wake-up Input Type 0 to 13"]
    #[inline(always)]
    pub fn wkupt13(&mut self) -> _WKUPT13W {
        _WKUPT13W { w: self }
    }
}
