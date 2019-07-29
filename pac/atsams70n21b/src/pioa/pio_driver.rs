#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIO_DRIVER {
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
#[doc = "Possible values of the field `LINE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE0R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE0R::LOW_DRIVE => false,
            LINE0R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE0_R = crate::FR<bool, LINE0R>;
impl LINE0_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE0R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE0R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE0W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE0W::LOW_DRIVE => false,
            LINE0W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE0W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE0W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE0W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE1R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE1R::LOW_DRIVE => false,
            LINE1R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE1_R = crate::FR<bool, LINE1R>;
impl LINE1_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE1R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE1R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE1W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE1W::LOW_DRIVE => false,
            LINE1W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE1W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE1W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE1W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE2R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE2R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE2R::LOW_DRIVE => false,
            LINE2R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE2_R = crate::FR<bool, LINE2R>;
impl LINE2_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE2R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE2R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE2W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE2W::LOW_DRIVE => false,
            LINE2W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE2W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE2W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE2W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE3R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE3R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE3R::LOW_DRIVE => false,
            LINE3R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE3_R = crate::FR<bool, LINE3R>;
impl LINE3_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE3R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE3R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE3W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE3W::LOW_DRIVE => false,
            LINE3W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE3W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE3W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE3W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE4R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE4R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE4R::LOW_DRIVE => false,
            LINE4R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE4_R = crate::FR<bool, LINE4R>;
impl LINE4_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE4R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE4R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE4W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE4W::LOW_DRIVE => false,
            LINE4W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE4W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE4W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE4W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE4W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE5R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE5R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE5R::LOW_DRIVE => false,
            LINE5R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE5_R = crate::FR<bool, LINE5R>;
impl LINE5_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE5R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE5R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE5W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE5W::LOW_DRIVE => false,
            LINE5W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE5W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE5W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE5W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE5W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE6R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE6R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE6R::LOW_DRIVE => false,
            LINE6R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE6_R = crate::FR<bool, LINE6R>;
impl LINE6_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE6R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE6R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE6W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE6W::LOW_DRIVE => false,
            LINE6W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE6W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE6W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE6W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE6W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE7R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE7R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE7R::LOW_DRIVE => false,
            LINE7R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE7_R = crate::FR<bool, LINE7R>;
impl LINE7_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE7R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE7R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE7W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE7W::LOW_DRIVE => false,
            LINE7W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE7W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE7W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE7W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE7W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE8R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE8R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE8R::LOW_DRIVE => false,
            LINE8R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE8_R = crate::FR<bool, LINE8R>;
impl LINE8_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE8R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE8R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE8W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE8W::LOW_DRIVE => false,
            LINE8W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE8W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE8W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE8W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE8W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE9R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE9R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE9R::LOW_DRIVE => false,
            LINE9R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE9_R = crate::FR<bool, LINE9R>;
impl LINE9_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE9R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE9R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE9W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE9W::LOW_DRIVE => false,
            LINE9W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE9W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE9W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE9W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE9W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE10R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE10R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE10R::LOW_DRIVE => false,
            LINE10R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE10_R = crate::FR<bool, LINE10R>;
impl LINE10_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE10R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE10R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE10W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE10W::LOW_DRIVE => false,
            LINE10W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE10W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE10W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE10W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE10W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE11R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE11R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE11R::LOW_DRIVE => false,
            LINE11R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE11_R = crate::FR<bool, LINE11R>;
impl LINE11_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE11R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE11R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE11W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE11W::LOW_DRIVE => false,
            LINE11W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE11W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE11W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE11W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE11W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE12R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE12R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE12R::LOW_DRIVE => false,
            LINE12R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE12_R = crate::FR<bool, LINE12R>;
impl LINE12_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE12R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE12R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE12W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE12W::LOW_DRIVE => false,
            LINE12W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE12W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE12W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE12W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE12W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE13R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE13R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE13R::LOW_DRIVE => false,
            LINE13R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE13_R = crate::FR<bool, LINE13R>;
impl LINE13_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE13R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE13R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE13W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE13W::LOW_DRIVE => false,
            LINE13W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE13W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE13W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE13W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE13W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE14R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE14R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE14R::LOW_DRIVE => false,
            LINE14R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE14_R = crate::FR<bool, LINE14R>;
impl LINE14_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE14R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE14R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE14W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE14W::LOW_DRIVE => false,
            LINE14W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE14W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE14W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE14W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE14W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE15R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE15R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE15R::LOW_DRIVE => false,
            LINE15R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE15_R = crate::FR<bool, LINE15R>;
impl LINE15_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE15R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE15R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE15W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE15W::LOW_DRIVE => false,
            LINE15W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE15W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE15W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE15W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE15W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE16R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE16R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE16R::LOW_DRIVE => false,
            LINE16R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE16_R = crate::FR<bool, LINE16R>;
impl LINE16_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE16R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE16R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE16W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE16W::LOW_DRIVE => false,
            LINE16W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE16W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE16W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE16W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE16W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE17R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE17R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE17R::LOW_DRIVE => false,
            LINE17R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE17_R = crate::FR<bool, LINE17R>;
impl LINE17_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE17R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE17R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE17W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE17W::LOW_DRIVE => false,
            LINE17W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE17W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE17W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE17W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE17W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE18R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE18R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE18R::LOW_DRIVE => false,
            LINE18R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE18_R = crate::FR<bool, LINE18R>;
impl LINE18_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE18R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE18R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE18W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE18W::LOW_DRIVE => false,
            LINE18W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE18W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE18W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE18W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE18W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE19R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE19R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE19R::LOW_DRIVE => false,
            LINE19R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE19_R = crate::FR<bool, LINE19R>;
impl LINE19_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE19R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE19R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE19W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE19W::LOW_DRIVE => false,
            LINE19W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE19W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE19W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE19W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE19W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE20R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE20R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE20R::LOW_DRIVE => false,
            LINE20R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE20_R = crate::FR<bool, LINE20R>;
impl LINE20_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE20R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE20R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE20W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE20W::LOW_DRIVE => false,
            LINE20W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE20W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE20W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE20W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE20W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE21R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE21R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE21R::LOW_DRIVE => false,
            LINE21R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE21_R = crate::FR<bool, LINE21R>;
impl LINE21_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE21R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE21R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE21W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE21W::LOW_DRIVE => false,
            LINE21W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE21W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE21W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE21W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE21W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE22R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE22R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE22R::LOW_DRIVE => false,
            LINE22R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE22_R = crate::FR<bool, LINE22R>;
impl LINE22_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE22R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE22R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE22W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE22W::LOW_DRIVE => false,
            LINE22W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE22W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE22W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE22W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE22W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE23R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE23R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE23R::LOW_DRIVE => false,
            LINE23R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE23_R = crate::FR<bool, LINE23R>;
impl LINE23_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE23R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE23R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE23W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE23W::LOW_DRIVE => false,
            LINE23W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE23W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE23W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE23W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE23W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE24R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE24R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE24R::LOW_DRIVE => false,
            LINE24R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE24_R = crate::FR<bool, LINE24R>;
impl LINE24_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE24R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE24R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE24W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE24W::LOW_DRIVE => false,
            LINE24W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE24W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE24W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE24W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE24W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE25R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE25R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE25R::LOW_DRIVE => false,
            LINE25R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE25_R = crate::FR<bool, LINE25R>;
impl LINE25_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE25R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE25R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE25W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE25W::LOW_DRIVE => false,
            LINE25W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE25W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE25W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE25W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE25W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE26R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE26R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE26R::LOW_DRIVE => false,
            LINE26R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE26_R = crate::FR<bool, LINE26R>;
impl LINE26_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE26R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE26R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE26W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE26W::LOW_DRIVE => false,
            LINE26W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE26W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE26W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE26W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE26W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE27R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE27R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE27R::LOW_DRIVE => false,
            LINE27R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE27_R = crate::FR<bool, LINE27R>;
impl LINE27_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE27R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE27R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE27W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE27W::LOW_DRIVE => false,
            LINE27W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE27W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE27W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE27W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE27W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE28R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE28R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE28R::LOW_DRIVE => false,
            LINE28R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE28_R = crate::FR<bool, LINE28R>;
impl LINE28_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE28R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE28R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE28W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE28W::LOW_DRIVE => false,
            LINE28W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE28W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE28W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE28W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE28W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE29R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE29R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE29R::LOW_DRIVE => false,
            LINE29R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE29_R = crate::FR<bool, LINE29R>;
impl LINE29_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE29R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE29R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE29W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE29W::LOW_DRIVE => false,
            LINE29W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE29W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE29W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE29W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE29W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE30R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE30R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE30R::LOW_DRIVE => false,
            LINE30R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE30_R = crate::FR<bool, LINE30R>;
impl LINE30_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE30R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE30R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE30W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE30W::LOW_DRIVE => false,
            LINE30W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE30W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE30W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE30W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE30W::HIGH_DRIVE)
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
#[doc = "Possible values of the field `LINE31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE31R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl crate::ToBits<bool> for LINE31R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LINE31R::LOW_DRIVE => false,
            LINE31R::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LINE31_R = crate::FR<bool, LINE31R>;
impl LINE31_R {
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline(always)]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE31R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline(always)]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE31R::HIGH_DRIVE
    }
}
#[doc = "Values that can be written to the field `LINE31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE31W {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE31W::LOW_DRIVE => false,
            LINE31W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LINE31W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE31W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LINE31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline(always)]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE31W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline(always)]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE31W::HIGH_DRIVE)
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
    #[doc = "Bit 0 - Drive of PIO Line 0"]
    #[inline(always)]
    pub fn line0(&self) -> LINE0_R {
        LINE0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Drive of PIO Line 1"]
    #[inline(always)]
    pub fn line1(&self) -> LINE1_R {
        LINE1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Drive of PIO Line 2"]
    #[inline(always)]
    pub fn line2(&self) -> LINE2_R {
        LINE2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Drive of PIO Line 3"]
    #[inline(always)]
    pub fn line3(&self) -> LINE3_R {
        LINE3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Drive of PIO Line 4"]
    #[inline(always)]
    pub fn line4(&self) -> LINE4_R {
        LINE4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Drive of PIO Line 5"]
    #[inline(always)]
    pub fn line5(&self) -> LINE5_R {
        LINE5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Drive of PIO Line 6"]
    #[inline(always)]
    pub fn line6(&self) -> LINE6_R {
        LINE6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Drive of PIO Line 7"]
    #[inline(always)]
    pub fn line7(&self) -> LINE7_R {
        LINE7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Drive of PIO Line 8"]
    #[inline(always)]
    pub fn line8(&self) -> LINE8_R {
        LINE8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Drive of PIO Line 9"]
    #[inline(always)]
    pub fn line9(&self) -> LINE9_R {
        LINE9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Drive of PIO Line 10"]
    #[inline(always)]
    pub fn line10(&self) -> LINE10_R {
        LINE10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Drive of PIO Line 11"]
    #[inline(always)]
    pub fn line11(&self) -> LINE11_R {
        LINE11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Drive of PIO Line 12"]
    #[inline(always)]
    pub fn line12(&self) -> LINE12_R {
        LINE12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Drive of PIO Line 13"]
    #[inline(always)]
    pub fn line13(&self) -> LINE13_R {
        LINE13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Drive of PIO Line 14"]
    #[inline(always)]
    pub fn line14(&self) -> LINE14_R {
        LINE14_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Drive of PIO Line 15"]
    #[inline(always)]
    pub fn line15(&self) -> LINE15_R {
        LINE15_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Drive of PIO Line 16"]
    #[inline(always)]
    pub fn line16(&self) -> LINE16_R {
        LINE16_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Drive of PIO Line 17"]
    #[inline(always)]
    pub fn line17(&self) -> LINE17_R {
        LINE17_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Drive of PIO Line 18"]
    #[inline(always)]
    pub fn line18(&self) -> LINE18_R {
        LINE18_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Drive of PIO Line 19"]
    #[inline(always)]
    pub fn line19(&self) -> LINE19_R {
        LINE19_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Drive of PIO Line 20"]
    #[inline(always)]
    pub fn line20(&self) -> LINE20_R {
        LINE20_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Drive of PIO Line 21"]
    #[inline(always)]
    pub fn line21(&self) -> LINE21_R {
        LINE21_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Drive of PIO Line 22"]
    #[inline(always)]
    pub fn line22(&self) -> LINE22_R {
        LINE22_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Drive of PIO Line 23"]
    #[inline(always)]
    pub fn line23(&self) -> LINE23_R {
        LINE23_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Drive of PIO Line 24"]
    #[inline(always)]
    pub fn line24(&self) -> LINE24_R {
        LINE24_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Drive of PIO Line 25"]
    #[inline(always)]
    pub fn line25(&self) -> LINE25_R {
        LINE25_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Drive of PIO Line 26"]
    #[inline(always)]
    pub fn line26(&self) -> LINE26_R {
        LINE26_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Drive of PIO Line 27"]
    #[inline(always)]
    pub fn line27(&self) -> LINE27_R {
        LINE27_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Drive of PIO Line 28"]
    #[inline(always)]
    pub fn line28(&self) -> LINE28_R {
        LINE28_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Drive of PIO Line 29"]
    #[inline(always)]
    pub fn line29(&self) -> LINE29_R {
        LINE29_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Drive of PIO Line 30"]
    #[inline(always)]
    pub fn line30(&self) -> LINE30_R {
        LINE30_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Drive of PIO Line 31"]
    #[inline(always)]
    pub fn line31(&self) -> LINE31_R {
        LINE31_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Drive of PIO Line 0"]
    #[inline(always)]
    pub fn line0(&mut self) -> _LINE0W {
        _LINE0W { w: self }
    }
    #[doc = "Bit 1 - Drive of PIO Line 1"]
    #[inline(always)]
    pub fn line1(&mut self) -> _LINE1W {
        _LINE1W { w: self }
    }
    #[doc = "Bit 2 - Drive of PIO Line 2"]
    #[inline(always)]
    pub fn line2(&mut self) -> _LINE2W {
        _LINE2W { w: self }
    }
    #[doc = "Bit 3 - Drive of PIO Line 3"]
    #[inline(always)]
    pub fn line3(&mut self) -> _LINE3W {
        _LINE3W { w: self }
    }
    #[doc = "Bit 4 - Drive of PIO Line 4"]
    #[inline(always)]
    pub fn line4(&mut self) -> _LINE4W {
        _LINE4W { w: self }
    }
    #[doc = "Bit 5 - Drive of PIO Line 5"]
    #[inline(always)]
    pub fn line5(&mut self) -> _LINE5W {
        _LINE5W { w: self }
    }
    #[doc = "Bit 6 - Drive of PIO Line 6"]
    #[inline(always)]
    pub fn line6(&mut self) -> _LINE6W {
        _LINE6W { w: self }
    }
    #[doc = "Bit 7 - Drive of PIO Line 7"]
    #[inline(always)]
    pub fn line7(&mut self) -> _LINE7W {
        _LINE7W { w: self }
    }
    #[doc = "Bit 8 - Drive of PIO Line 8"]
    #[inline(always)]
    pub fn line8(&mut self) -> _LINE8W {
        _LINE8W { w: self }
    }
    #[doc = "Bit 9 - Drive of PIO Line 9"]
    #[inline(always)]
    pub fn line9(&mut self) -> _LINE9W {
        _LINE9W { w: self }
    }
    #[doc = "Bit 10 - Drive of PIO Line 10"]
    #[inline(always)]
    pub fn line10(&mut self) -> _LINE10W {
        _LINE10W { w: self }
    }
    #[doc = "Bit 11 - Drive of PIO Line 11"]
    #[inline(always)]
    pub fn line11(&mut self) -> _LINE11W {
        _LINE11W { w: self }
    }
    #[doc = "Bit 12 - Drive of PIO Line 12"]
    #[inline(always)]
    pub fn line12(&mut self) -> _LINE12W {
        _LINE12W { w: self }
    }
    #[doc = "Bit 13 - Drive of PIO Line 13"]
    #[inline(always)]
    pub fn line13(&mut self) -> _LINE13W {
        _LINE13W { w: self }
    }
    #[doc = "Bit 14 - Drive of PIO Line 14"]
    #[inline(always)]
    pub fn line14(&mut self) -> _LINE14W {
        _LINE14W { w: self }
    }
    #[doc = "Bit 15 - Drive of PIO Line 15"]
    #[inline(always)]
    pub fn line15(&mut self) -> _LINE15W {
        _LINE15W { w: self }
    }
    #[doc = "Bit 16 - Drive of PIO Line 16"]
    #[inline(always)]
    pub fn line16(&mut self) -> _LINE16W {
        _LINE16W { w: self }
    }
    #[doc = "Bit 17 - Drive of PIO Line 17"]
    #[inline(always)]
    pub fn line17(&mut self) -> _LINE17W {
        _LINE17W { w: self }
    }
    #[doc = "Bit 18 - Drive of PIO Line 18"]
    #[inline(always)]
    pub fn line18(&mut self) -> _LINE18W {
        _LINE18W { w: self }
    }
    #[doc = "Bit 19 - Drive of PIO Line 19"]
    #[inline(always)]
    pub fn line19(&mut self) -> _LINE19W {
        _LINE19W { w: self }
    }
    #[doc = "Bit 20 - Drive of PIO Line 20"]
    #[inline(always)]
    pub fn line20(&mut self) -> _LINE20W {
        _LINE20W { w: self }
    }
    #[doc = "Bit 21 - Drive of PIO Line 21"]
    #[inline(always)]
    pub fn line21(&mut self) -> _LINE21W {
        _LINE21W { w: self }
    }
    #[doc = "Bit 22 - Drive of PIO Line 22"]
    #[inline(always)]
    pub fn line22(&mut self) -> _LINE22W {
        _LINE22W { w: self }
    }
    #[doc = "Bit 23 - Drive of PIO Line 23"]
    #[inline(always)]
    pub fn line23(&mut self) -> _LINE23W {
        _LINE23W { w: self }
    }
    #[doc = "Bit 24 - Drive of PIO Line 24"]
    #[inline(always)]
    pub fn line24(&mut self) -> _LINE24W {
        _LINE24W { w: self }
    }
    #[doc = "Bit 25 - Drive of PIO Line 25"]
    #[inline(always)]
    pub fn line25(&mut self) -> _LINE25W {
        _LINE25W { w: self }
    }
    #[doc = "Bit 26 - Drive of PIO Line 26"]
    #[inline(always)]
    pub fn line26(&mut self) -> _LINE26W {
        _LINE26W { w: self }
    }
    #[doc = "Bit 27 - Drive of PIO Line 27"]
    #[inline(always)]
    pub fn line27(&mut self) -> _LINE27W {
        _LINE27W { w: self }
    }
    #[doc = "Bit 28 - Drive of PIO Line 28"]
    #[inline(always)]
    pub fn line28(&mut self) -> _LINE28W {
        _LINE28W { w: self }
    }
    #[doc = "Bit 29 - Drive of PIO Line 29"]
    #[inline(always)]
    pub fn line29(&mut self) -> _LINE29W {
        _LINE29W { w: self }
    }
    #[doc = "Bit 30 - Drive of PIO Line 30"]
    #[inline(always)]
    pub fn line30(&mut self) -> _LINE30W {
        _LINE30W { w: self }
    }
    #[doc = "Bit 31 - Drive of PIO Line 31"]
    #[inline(always)]
    pub fn line31(&mut self) -> _LINE31W {
        _LINE31W { w: self }
    }
}
