#[doc = "Reader of register PIO_DRIVER"]
pub type R = crate::R<u32, super::PIO_DRIVER>;
#[doc = "Writer for register PIO_DRIVER"]
pub type W = crate::W<u32, super::PIO_DRIVER>;
#[doc = "Register PIO_DRIVER `reset()`'s with value 0"]
impl crate::ResetValue for super::PIO_DRIVER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Drive of PIO Line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE0_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE0_A> for bool {
    #[inline(always)]
    fn from(variant: LINE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE0`"]
pub type LINE0_R = crate::R<bool, LINE0_A>;
impl LINE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE0_A {
        match self.bits {
            false => LINE0_A::LOW_DRIVE,
            true => LINE0_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE0_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE0_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE0`"]
pub struct LINE0_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE0_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE0_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE1_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE1_A> for bool {
    #[inline(always)]
    fn from(variant: LINE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE1`"]
pub type LINE1_R = crate::R<bool, LINE1_A>;
impl LINE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE1_A {
        match self.bits {
            false => LINE1_A::LOW_DRIVE,
            true => LINE1_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE1_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE1_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE1`"]
pub struct LINE1_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE1_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE1_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE2_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE2_A> for bool {
    #[inline(always)]
    fn from(variant: LINE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE2`"]
pub type LINE2_R = crate::R<bool, LINE2_A>;
impl LINE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE2_A {
        match self.bits {
            false => LINE2_A::LOW_DRIVE,
            true => LINE2_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE2_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE2_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE2`"]
pub struct LINE2_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE2_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE2_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE3_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE3_A> for bool {
    #[inline(always)]
    fn from(variant: LINE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE3`"]
pub type LINE3_R = crate::R<bool, LINE3_A>;
impl LINE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE3_A {
        match self.bits {
            false => LINE3_A::LOW_DRIVE,
            true => LINE3_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE3_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE3_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE3`"]
pub struct LINE3_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE3_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE3_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE4_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE4_A> for bool {
    #[inline(always)]
    fn from(variant: LINE4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE4`"]
pub type LINE4_R = crate::R<bool, LINE4_A>;
impl LINE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE4_A {
        match self.bits {
            false => LINE4_A::LOW_DRIVE,
            true => LINE4_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE4_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE4_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE4`"]
pub struct LINE4_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE4_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE4_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE5_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE5_A> for bool {
    #[inline(always)]
    fn from(variant: LINE5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE5`"]
pub type LINE5_R = crate::R<bool, LINE5_A>;
impl LINE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE5_A {
        match self.bits {
            false => LINE5_A::LOW_DRIVE,
            true => LINE5_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE5_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE5_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE5`"]
pub struct LINE5_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE5_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE5_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE6_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE6_A> for bool {
    #[inline(always)]
    fn from(variant: LINE6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE6`"]
pub type LINE6_R = crate::R<bool, LINE6_A>;
impl LINE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE6_A {
        match self.bits {
            false => LINE6_A::LOW_DRIVE,
            true => LINE6_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE6_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE6_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE6`"]
pub struct LINE6_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE6_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE6_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE7_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE7_A> for bool {
    #[inline(always)]
    fn from(variant: LINE7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE7`"]
pub type LINE7_R = crate::R<bool, LINE7_A>;
impl LINE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE7_A {
        match self.bits {
            false => LINE7_A::LOW_DRIVE,
            true => LINE7_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE7_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE7_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE7`"]
pub struct LINE7_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE7_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE7_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE8_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE8_A> for bool {
    #[inline(always)]
    fn from(variant: LINE8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE8`"]
pub type LINE8_R = crate::R<bool, LINE8_A>;
impl LINE8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE8_A {
        match self.bits {
            false => LINE8_A::LOW_DRIVE,
            true => LINE8_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE8_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE8_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE8`"]
pub struct LINE8_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE8_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE8_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE9_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE9_A> for bool {
    #[inline(always)]
    fn from(variant: LINE9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE9`"]
pub type LINE9_R = crate::R<bool, LINE9_A>;
impl LINE9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE9_A {
        match self.bits {
            false => LINE9_A::LOW_DRIVE,
            true => LINE9_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE9_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE9_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE9`"]
pub struct LINE9_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE9_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE9_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE10_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE10_A> for bool {
    #[inline(always)]
    fn from(variant: LINE10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE10`"]
pub type LINE10_R = crate::R<bool, LINE10_A>;
impl LINE10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE10_A {
        match self.bits {
            false => LINE10_A::LOW_DRIVE,
            true => LINE10_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE10_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE10_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE10`"]
pub struct LINE10_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE10_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE10_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE11_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE11_A> for bool {
    #[inline(always)]
    fn from(variant: LINE11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE11`"]
pub type LINE11_R = crate::R<bool, LINE11_A>;
impl LINE11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE11_A {
        match self.bits {
            false => LINE11_A::LOW_DRIVE,
            true => LINE11_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE11_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE11_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE11`"]
pub struct LINE11_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE11_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE11_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE12_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE12_A> for bool {
    #[inline(always)]
    fn from(variant: LINE12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE12`"]
pub type LINE12_R = crate::R<bool, LINE12_A>;
impl LINE12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE12_A {
        match self.bits {
            false => LINE12_A::LOW_DRIVE,
            true => LINE12_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE12_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE12_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE12`"]
pub struct LINE12_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE12_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE12_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE13_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE13_A> for bool {
    #[inline(always)]
    fn from(variant: LINE13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE13`"]
pub type LINE13_R = crate::R<bool, LINE13_A>;
impl LINE13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE13_A {
        match self.bits {
            false => LINE13_A::LOW_DRIVE,
            true => LINE13_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE13_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE13_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE13`"]
pub struct LINE13_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE13_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE13_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE14_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE14_A> for bool {
    #[inline(always)]
    fn from(variant: LINE14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE14`"]
pub type LINE14_R = crate::R<bool, LINE14_A>;
impl LINE14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE14_A {
        match self.bits {
            false => LINE14_A::LOW_DRIVE,
            true => LINE14_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE14_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE14_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE14`"]
pub struct LINE14_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE14_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE14_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE15_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE15_A> for bool {
    #[inline(always)]
    fn from(variant: LINE15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE15`"]
pub type LINE15_R = crate::R<bool, LINE15_A>;
impl LINE15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE15_A {
        match self.bits {
            false => LINE15_A::LOW_DRIVE,
            true => LINE15_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE15_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE15_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE15`"]
pub struct LINE15_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE15_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE15_A::HIGH_DRIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Drive of PIO Line 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE16_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE16_A> for bool {
    #[inline(always)]
    fn from(variant: LINE16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE16`"]
pub type LINE16_R = crate::R<bool, LINE16_A>;
impl LINE16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE16_A {
        match self.bits {
            false => LINE16_A::LOW_DRIVE,
            true => LINE16_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE16_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE16_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE16`"]
pub struct LINE16_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE16_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE16_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE17_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE17_A> for bool {
    #[inline(always)]
    fn from(variant: LINE17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE17`"]
pub type LINE17_R = crate::R<bool, LINE17_A>;
impl LINE17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE17_A {
        match self.bits {
            false => LINE17_A::LOW_DRIVE,
            true => LINE17_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE17_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE17_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE17`"]
pub struct LINE17_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE17_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE17_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE18_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE18_A> for bool {
    #[inline(always)]
    fn from(variant: LINE18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE18`"]
pub type LINE18_R = crate::R<bool, LINE18_A>;
impl LINE18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE18_A {
        match self.bits {
            false => LINE18_A::LOW_DRIVE,
            true => LINE18_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE18_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE18_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE18`"]
pub struct LINE18_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE18_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE18_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE19_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE19_A> for bool {
    #[inline(always)]
    fn from(variant: LINE19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE19`"]
pub type LINE19_R = crate::R<bool, LINE19_A>;
impl LINE19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE19_A {
        match self.bits {
            false => LINE19_A::LOW_DRIVE,
            true => LINE19_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE19_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE19_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE19`"]
pub struct LINE19_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE19_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE19_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE20_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE20_A> for bool {
    #[inline(always)]
    fn from(variant: LINE20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE20`"]
pub type LINE20_R = crate::R<bool, LINE20_A>;
impl LINE20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE20_A {
        match self.bits {
            false => LINE20_A::LOW_DRIVE,
            true => LINE20_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE20_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE20_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE20`"]
pub struct LINE20_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE20_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE20_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE21_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE21_A> for bool {
    #[inline(always)]
    fn from(variant: LINE21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE21`"]
pub type LINE21_R = crate::R<bool, LINE21_A>;
impl LINE21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE21_A {
        match self.bits {
            false => LINE21_A::LOW_DRIVE,
            true => LINE21_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE21_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE21_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE21`"]
pub struct LINE21_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE21_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE21_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE22_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE22_A> for bool {
    #[inline(always)]
    fn from(variant: LINE22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE22`"]
pub type LINE22_R = crate::R<bool, LINE22_A>;
impl LINE22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE22_A {
        match self.bits {
            false => LINE22_A::LOW_DRIVE,
            true => LINE22_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE22_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE22_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE22`"]
pub struct LINE22_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE22_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE22_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE23_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE23_A> for bool {
    #[inline(always)]
    fn from(variant: LINE23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE23`"]
pub type LINE23_R = crate::R<bool, LINE23_A>;
impl LINE23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE23_A {
        match self.bits {
            false => LINE23_A::LOW_DRIVE,
            true => LINE23_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE23_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE23_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE23`"]
pub struct LINE23_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE23_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE23_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE24_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE24_A> for bool {
    #[inline(always)]
    fn from(variant: LINE24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE24`"]
pub type LINE24_R = crate::R<bool, LINE24_A>;
impl LINE24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE24_A {
        match self.bits {
            false => LINE24_A::LOW_DRIVE,
            true => LINE24_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE24_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE24_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE24`"]
pub struct LINE24_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE24_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE24_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE25_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE25_A> for bool {
    #[inline(always)]
    fn from(variant: LINE25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE25`"]
pub type LINE25_R = crate::R<bool, LINE25_A>;
impl LINE25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE25_A {
        match self.bits {
            false => LINE25_A::LOW_DRIVE,
            true => LINE25_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE25_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE25_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE25`"]
pub struct LINE25_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE25_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE25_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE26_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE26_A> for bool {
    #[inline(always)]
    fn from(variant: LINE26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE26`"]
pub type LINE26_R = crate::R<bool, LINE26_A>;
impl LINE26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE26_A {
        match self.bits {
            false => LINE26_A::LOW_DRIVE,
            true => LINE26_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE26_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE26_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE26`"]
pub struct LINE26_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE26_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE26_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE27_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE27_A> for bool {
    #[inline(always)]
    fn from(variant: LINE27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE27`"]
pub type LINE27_R = crate::R<bool, LINE27_A>;
impl LINE27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE27_A {
        match self.bits {
            false => LINE27_A::LOW_DRIVE,
            true => LINE27_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE27_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE27_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE27`"]
pub struct LINE27_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE27_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE27_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE28_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE28_A> for bool {
    #[inline(always)]
    fn from(variant: LINE28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE28`"]
pub type LINE28_R = crate::R<bool, LINE28_A>;
impl LINE28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE28_A {
        match self.bits {
            false => LINE28_A::LOW_DRIVE,
            true => LINE28_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE28_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE28_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE28`"]
pub struct LINE28_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE28_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE28_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE29_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE29_A> for bool {
    #[inline(always)]
    fn from(variant: LINE29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE29`"]
pub type LINE29_R = crate::R<bool, LINE29_A>;
impl LINE29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE29_A {
        match self.bits {
            false => LINE29_A::LOW_DRIVE,
            true => LINE29_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE29_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE29_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE29`"]
pub struct LINE29_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE29_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE29_A::HIGH_DRIVE)
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
#[doc = "Drive of PIO Line 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE30_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE30_A> for bool {
    #[inline(always)]
    fn from(variant: LINE30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE30`"]
pub type LINE30_R = crate::R<bool, LINE30_A>;
impl LINE30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE30_A {
        match self.bits {
            false => LINE30_A::LOW_DRIVE,
            true => LINE30_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE30_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE30_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE30`"]
pub struct LINE30_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE30_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE30_A::HIGH_DRIVE)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Drive of PIO Line 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE31_A {
    #[doc = "0: Lowest drive"]
    LOW_DRIVE = 0,
    #[doc = "1: Highest drive"]
    HIGH_DRIVE = 1,
}
impl From<LINE31_A> for bool {
    #[inline(always)]
    fn from(variant: LINE31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LINE31`"]
pub type LINE31_R = crate::R<bool, LINE31_A>;
impl LINE31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINE31_A {
        match self.bits {
            false => LINE31_A::LOW_DRIVE,
            true => LINE31_A::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE31_A::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE31_A::HIGH_DRIVE
    }
}
#[doc = "Write proxy for field `LINE31`"]
pub struct LINE31_W<'a> {
    w: &'a mut W,
}
impl<'a> LINE31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE31_A::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE31_A::HIGH_DRIVE)
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
    #[doc = "Bit 0 - Drive of PIO Line 0"]
    #[inline(always)]
    pub fn line0(&self) -> LINE0_R {
        LINE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Drive of PIO Line 1"]
    #[inline(always)]
    pub fn line1(&self) -> LINE1_R {
        LINE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Drive of PIO Line 2"]
    #[inline(always)]
    pub fn line2(&self) -> LINE2_R {
        LINE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Drive of PIO Line 3"]
    #[inline(always)]
    pub fn line3(&self) -> LINE3_R {
        LINE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Drive of PIO Line 4"]
    #[inline(always)]
    pub fn line4(&self) -> LINE4_R {
        LINE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Drive of PIO Line 5"]
    #[inline(always)]
    pub fn line5(&self) -> LINE5_R {
        LINE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Drive of PIO Line 6"]
    #[inline(always)]
    pub fn line6(&self) -> LINE6_R {
        LINE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Drive of PIO Line 7"]
    #[inline(always)]
    pub fn line7(&self) -> LINE7_R {
        LINE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Drive of PIO Line 8"]
    #[inline(always)]
    pub fn line8(&self) -> LINE8_R {
        LINE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Drive of PIO Line 9"]
    #[inline(always)]
    pub fn line9(&self) -> LINE9_R {
        LINE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Drive of PIO Line 10"]
    #[inline(always)]
    pub fn line10(&self) -> LINE10_R {
        LINE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Drive of PIO Line 11"]
    #[inline(always)]
    pub fn line11(&self) -> LINE11_R {
        LINE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Drive of PIO Line 12"]
    #[inline(always)]
    pub fn line12(&self) -> LINE12_R {
        LINE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Drive of PIO Line 13"]
    #[inline(always)]
    pub fn line13(&self) -> LINE13_R {
        LINE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Drive of PIO Line 14"]
    #[inline(always)]
    pub fn line14(&self) -> LINE14_R {
        LINE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Drive of PIO Line 15"]
    #[inline(always)]
    pub fn line15(&self) -> LINE15_R {
        LINE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Drive of PIO Line 16"]
    #[inline(always)]
    pub fn line16(&self) -> LINE16_R {
        LINE16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Drive of PIO Line 17"]
    #[inline(always)]
    pub fn line17(&self) -> LINE17_R {
        LINE17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Drive of PIO Line 18"]
    #[inline(always)]
    pub fn line18(&self) -> LINE18_R {
        LINE18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Drive of PIO Line 19"]
    #[inline(always)]
    pub fn line19(&self) -> LINE19_R {
        LINE19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Drive of PIO Line 20"]
    #[inline(always)]
    pub fn line20(&self) -> LINE20_R {
        LINE20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Drive of PIO Line 21"]
    #[inline(always)]
    pub fn line21(&self) -> LINE21_R {
        LINE21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Drive of PIO Line 22"]
    #[inline(always)]
    pub fn line22(&self) -> LINE22_R {
        LINE22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Drive of PIO Line 23"]
    #[inline(always)]
    pub fn line23(&self) -> LINE23_R {
        LINE23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Drive of PIO Line 24"]
    #[inline(always)]
    pub fn line24(&self) -> LINE24_R {
        LINE24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Drive of PIO Line 25"]
    #[inline(always)]
    pub fn line25(&self) -> LINE25_R {
        LINE25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Drive of PIO Line 26"]
    #[inline(always)]
    pub fn line26(&self) -> LINE26_R {
        LINE26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Drive of PIO Line 27"]
    #[inline(always)]
    pub fn line27(&self) -> LINE27_R {
        LINE27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Drive of PIO Line 28"]
    #[inline(always)]
    pub fn line28(&self) -> LINE28_R {
        LINE28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Drive of PIO Line 29"]
    #[inline(always)]
    pub fn line29(&self) -> LINE29_R {
        LINE29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Drive of PIO Line 30"]
    #[inline(always)]
    pub fn line30(&self) -> LINE30_R {
        LINE30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Drive of PIO Line 31"]
    #[inline(always)]
    pub fn line31(&self) -> LINE31_R {
        LINE31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drive of PIO Line 0"]
    #[inline(always)]
    pub fn line0(&mut self) -> LINE0_W {
        LINE0_W { w: self }
    }
    #[doc = "Bit 1 - Drive of PIO Line 1"]
    #[inline(always)]
    pub fn line1(&mut self) -> LINE1_W {
        LINE1_W { w: self }
    }
    #[doc = "Bit 2 - Drive of PIO Line 2"]
    #[inline(always)]
    pub fn line2(&mut self) -> LINE2_W {
        LINE2_W { w: self }
    }
    #[doc = "Bit 3 - Drive of PIO Line 3"]
    #[inline(always)]
    pub fn line3(&mut self) -> LINE3_W {
        LINE3_W { w: self }
    }
    #[doc = "Bit 4 - Drive of PIO Line 4"]
    #[inline(always)]
    pub fn line4(&mut self) -> LINE4_W {
        LINE4_W { w: self }
    }
    #[doc = "Bit 5 - Drive of PIO Line 5"]
    #[inline(always)]
    pub fn line5(&mut self) -> LINE5_W {
        LINE5_W { w: self }
    }
    #[doc = "Bit 6 - Drive of PIO Line 6"]
    #[inline(always)]
    pub fn line6(&mut self) -> LINE6_W {
        LINE6_W { w: self }
    }
    #[doc = "Bit 7 - Drive of PIO Line 7"]
    #[inline(always)]
    pub fn line7(&mut self) -> LINE7_W {
        LINE7_W { w: self }
    }
    #[doc = "Bit 8 - Drive of PIO Line 8"]
    #[inline(always)]
    pub fn line8(&mut self) -> LINE8_W {
        LINE8_W { w: self }
    }
    #[doc = "Bit 9 - Drive of PIO Line 9"]
    #[inline(always)]
    pub fn line9(&mut self) -> LINE9_W {
        LINE9_W { w: self }
    }
    #[doc = "Bit 10 - Drive of PIO Line 10"]
    #[inline(always)]
    pub fn line10(&mut self) -> LINE10_W {
        LINE10_W { w: self }
    }
    #[doc = "Bit 11 - Drive of PIO Line 11"]
    #[inline(always)]
    pub fn line11(&mut self) -> LINE11_W {
        LINE11_W { w: self }
    }
    #[doc = "Bit 12 - Drive of PIO Line 12"]
    #[inline(always)]
    pub fn line12(&mut self) -> LINE12_W {
        LINE12_W { w: self }
    }
    #[doc = "Bit 13 - Drive of PIO Line 13"]
    #[inline(always)]
    pub fn line13(&mut self) -> LINE13_W {
        LINE13_W { w: self }
    }
    #[doc = "Bit 14 - Drive of PIO Line 14"]
    #[inline(always)]
    pub fn line14(&mut self) -> LINE14_W {
        LINE14_W { w: self }
    }
    #[doc = "Bit 15 - Drive of PIO Line 15"]
    #[inline(always)]
    pub fn line15(&mut self) -> LINE15_W {
        LINE15_W { w: self }
    }
    #[doc = "Bit 16 - Drive of PIO Line 16"]
    #[inline(always)]
    pub fn line16(&mut self) -> LINE16_W {
        LINE16_W { w: self }
    }
    #[doc = "Bit 17 - Drive of PIO Line 17"]
    #[inline(always)]
    pub fn line17(&mut self) -> LINE17_W {
        LINE17_W { w: self }
    }
    #[doc = "Bit 18 - Drive of PIO Line 18"]
    #[inline(always)]
    pub fn line18(&mut self) -> LINE18_W {
        LINE18_W { w: self }
    }
    #[doc = "Bit 19 - Drive of PIO Line 19"]
    #[inline(always)]
    pub fn line19(&mut self) -> LINE19_W {
        LINE19_W { w: self }
    }
    #[doc = "Bit 20 - Drive of PIO Line 20"]
    #[inline(always)]
    pub fn line20(&mut self) -> LINE20_W {
        LINE20_W { w: self }
    }
    #[doc = "Bit 21 - Drive of PIO Line 21"]
    #[inline(always)]
    pub fn line21(&mut self) -> LINE21_W {
        LINE21_W { w: self }
    }
    #[doc = "Bit 22 - Drive of PIO Line 22"]
    #[inline(always)]
    pub fn line22(&mut self) -> LINE22_W {
        LINE22_W { w: self }
    }
    #[doc = "Bit 23 - Drive of PIO Line 23"]
    #[inline(always)]
    pub fn line23(&mut self) -> LINE23_W {
        LINE23_W { w: self }
    }
    #[doc = "Bit 24 - Drive of PIO Line 24"]
    #[inline(always)]
    pub fn line24(&mut self) -> LINE24_W {
        LINE24_W { w: self }
    }
    #[doc = "Bit 25 - Drive of PIO Line 25"]
    #[inline(always)]
    pub fn line25(&mut self) -> LINE25_W {
        LINE25_W { w: self }
    }
    #[doc = "Bit 26 - Drive of PIO Line 26"]
    #[inline(always)]
    pub fn line26(&mut self) -> LINE26_W {
        LINE26_W { w: self }
    }
    #[doc = "Bit 27 - Drive of PIO Line 27"]
    #[inline(always)]
    pub fn line27(&mut self) -> LINE27_W {
        LINE27_W { w: self }
    }
    #[doc = "Bit 28 - Drive of PIO Line 28"]
    #[inline(always)]
    pub fn line28(&mut self) -> LINE28_W {
        LINE28_W { w: self }
    }
    #[doc = "Bit 29 - Drive of PIO Line 29"]
    #[inline(always)]
    pub fn line29(&mut self) -> LINE29_W {
        LINE29_W { w: self }
    }
    #[doc = "Bit 30 - Drive of PIO Line 30"]
    #[inline(always)]
    pub fn line30(&mut self) -> LINE30_W {
        LINE30_W { w: self }
    }
    #[doc = "Bit 31 - Drive of PIO Line 31"]
    #[inline(always)]
    pub fn line31(&mut self) -> LINE31_W {
        LINE31_W { w: self }
    }
}
