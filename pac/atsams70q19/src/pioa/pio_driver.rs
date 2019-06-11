#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIO_DRIVER {
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
#[doc = "Possible values of the field `LINE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINE0R {
    #[doc = "Lowest drive"]
    LOW_DRIVE,
    #[doc = "Highest drive"]
    HIGH_DRIVE,
}
impl LINE0R {
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
            LINE0R::LOW_DRIVE => false,
            LINE0R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE0R {
        match value {
            false => LINE0R::LOW_DRIVE,
            true => LINE0R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE0R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE0R::HIGH_DRIVE
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
impl LINE1R {
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
            LINE1R::LOW_DRIVE => false,
            LINE1R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE1R {
        match value {
            false => LINE1R::LOW_DRIVE,
            true => LINE1R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE1R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE1R::HIGH_DRIVE
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
impl LINE2R {
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
            LINE2R::LOW_DRIVE => false,
            LINE2R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE2R {
        match value {
            false => LINE2R::LOW_DRIVE,
            true => LINE2R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE2R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE2R::HIGH_DRIVE
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
impl LINE3R {
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
            LINE3R::LOW_DRIVE => false,
            LINE3R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE3R {
        match value {
            false => LINE3R::LOW_DRIVE,
            true => LINE3R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE3R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE3R::HIGH_DRIVE
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
impl LINE4R {
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
            LINE4R::LOW_DRIVE => false,
            LINE4R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE4R {
        match value {
            false => LINE4R::LOW_DRIVE,
            true => LINE4R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE4R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE4R::HIGH_DRIVE
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
impl LINE5R {
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
            LINE5R::LOW_DRIVE => false,
            LINE5R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE5R {
        match value {
            false => LINE5R::LOW_DRIVE,
            true => LINE5R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE5R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE5R::HIGH_DRIVE
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
impl LINE6R {
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
            LINE6R::LOW_DRIVE => false,
            LINE6R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE6R {
        match value {
            false => LINE6R::LOW_DRIVE,
            true => LINE6R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE6R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE6R::HIGH_DRIVE
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
impl LINE7R {
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
            LINE7R::LOW_DRIVE => false,
            LINE7R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE7R {
        match value {
            false => LINE7R::LOW_DRIVE,
            true => LINE7R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE7R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE7R::HIGH_DRIVE
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
impl LINE8R {
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
            LINE8R::LOW_DRIVE => false,
            LINE8R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE8R {
        match value {
            false => LINE8R::LOW_DRIVE,
            true => LINE8R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE8R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE8R::HIGH_DRIVE
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
impl LINE9R {
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
            LINE9R::LOW_DRIVE => false,
            LINE9R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE9R {
        match value {
            false => LINE9R::LOW_DRIVE,
            true => LINE9R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE9R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE9R::HIGH_DRIVE
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
impl LINE10R {
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
            LINE10R::LOW_DRIVE => false,
            LINE10R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE10R {
        match value {
            false => LINE10R::LOW_DRIVE,
            true => LINE10R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE10R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE10R::HIGH_DRIVE
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
impl LINE11R {
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
            LINE11R::LOW_DRIVE => false,
            LINE11R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE11R {
        match value {
            false => LINE11R::LOW_DRIVE,
            true => LINE11R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE11R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE11R::HIGH_DRIVE
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
impl LINE12R {
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
            LINE12R::LOW_DRIVE => false,
            LINE12R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE12R {
        match value {
            false => LINE12R::LOW_DRIVE,
            true => LINE12R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE12R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE12R::HIGH_DRIVE
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
impl LINE13R {
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
            LINE13R::LOW_DRIVE => false,
            LINE13R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE13R {
        match value {
            false => LINE13R::LOW_DRIVE,
            true => LINE13R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE13R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE13R::HIGH_DRIVE
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
impl LINE14R {
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
            LINE14R::LOW_DRIVE => false,
            LINE14R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE14R {
        match value {
            false => LINE14R::LOW_DRIVE,
            true => LINE14R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE14R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE14R::HIGH_DRIVE
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
impl LINE15R {
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
            LINE15R::LOW_DRIVE => false,
            LINE15R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE15R {
        match value {
            false => LINE15R::LOW_DRIVE,
            true => LINE15R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE15R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE15R::HIGH_DRIVE
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
impl LINE16R {
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
            LINE16R::LOW_DRIVE => false,
            LINE16R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE16R {
        match value {
            false => LINE16R::LOW_DRIVE,
            true => LINE16R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE16R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE16R::HIGH_DRIVE
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
impl LINE17R {
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
            LINE17R::LOW_DRIVE => false,
            LINE17R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE17R {
        match value {
            false => LINE17R::LOW_DRIVE,
            true => LINE17R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE17R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE17R::HIGH_DRIVE
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
impl LINE18R {
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
            LINE18R::LOW_DRIVE => false,
            LINE18R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE18R {
        match value {
            false => LINE18R::LOW_DRIVE,
            true => LINE18R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE18R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE18R::HIGH_DRIVE
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
impl LINE19R {
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
            LINE19R::LOW_DRIVE => false,
            LINE19R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE19R {
        match value {
            false => LINE19R::LOW_DRIVE,
            true => LINE19R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE19R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE19R::HIGH_DRIVE
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
impl LINE20R {
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
            LINE20R::LOW_DRIVE => false,
            LINE20R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE20R {
        match value {
            false => LINE20R::LOW_DRIVE,
            true => LINE20R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE20R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE20R::HIGH_DRIVE
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
impl LINE21R {
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
            LINE21R::LOW_DRIVE => false,
            LINE21R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE21R {
        match value {
            false => LINE21R::LOW_DRIVE,
            true => LINE21R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE21R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE21R::HIGH_DRIVE
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
impl LINE22R {
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
            LINE22R::LOW_DRIVE => false,
            LINE22R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE22R {
        match value {
            false => LINE22R::LOW_DRIVE,
            true => LINE22R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE22R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE22R::HIGH_DRIVE
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
impl LINE23R {
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
            LINE23R::LOW_DRIVE => false,
            LINE23R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE23R {
        match value {
            false => LINE23R::LOW_DRIVE,
            true => LINE23R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE23R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE23R::HIGH_DRIVE
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
impl LINE24R {
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
            LINE24R::LOW_DRIVE => false,
            LINE24R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE24R {
        match value {
            false => LINE24R::LOW_DRIVE,
            true => LINE24R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE24R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE24R::HIGH_DRIVE
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
impl LINE25R {
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
            LINE25R::LOW_DRIVE => false,
            LINE25R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE25R {
        match value {
            false => LINE25R::LOW_DRIVE,
            true => LINE25R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE25R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE25R::HIGH_DRIVE
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
impl LINE26R {
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
            LINE26R::LOW_DRIVE => false,
            LINE26R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE26R {
        match value {
            false => LINE26R::LOW_DRIVE,
            true => LINE26R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE26R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE26R::HIGH_DRIVE
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
impl LINE27R {
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
            LINE27R::LOW_DRIVE => false,
            LINE27R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE27R {
        match value {
            false => LINE27R::LOW_DRIVE,
            true => LINE27R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE27R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE27R::HIGH_DRIVE
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
impl LINE28R {
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
            LINE28R::LOW_DRIVE => false,
            LINE28R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE28R {
        match value {
            false => LINE28R::LOW_DRIVE,
            true => LINE28R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE28R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE28R::HIGH_DRIVE
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
impl LINE29R {
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
            LINE29R::LOW_DRIVE => false,
            LINE29R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE29R {
        match value {
            false => LINE29R::LOW_DRIVE,
            true => LINE29R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE29R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE29R::HIGH_DRIVE
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
impl LINE30R {
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
            LINE30R::LOW_DRIVE => false,
            LINE30R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE30R {
        match value {
            false => LINE30R::LOW_DRIVE,
            true => LINE30R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE30R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE30R::HIGH_DRIVE
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
impl LINE31R {
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
            LINE31R::LOW_DRIVE => false,
            LINE31R::HIGH_DRIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LINE31R {
        match value {
            false => LINE31R::LOW_DRIVE,
            true => LINE31R::HIGH_DRIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_DRIVE`"]
    #[inline]
    pub fn is_low_drive(&self) -> bool {
        *self == LINE31R::LOW_DRIVE
    }
    #[doc = "Checks if the value of the field is `HIGH_DRIVE`"]
    #[inline]
    pub fn is_high_drive(&self) -> bool {
        *self == LINE31R::HIGH_DRIVE
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE0W::LOW_DRIVE => false,
            LINE0W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE0W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE0W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE0W::HIGH_DRIVE)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE1W::LOW_DRIVE => false,
            LINE1W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE1W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE1W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE1W::HIGH_DRIVE)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE2W::LOW_DRIVE => false,
            LINE2W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE2W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE2W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE2W::HIGH_DRIVE)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE3W::LOW_DRIVE => false,
            LINE3W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE3W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE3W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE3W::HIGH_DRIVE)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE4W::LOW_DRIVE => false,
            LINE4W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE4W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE4W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE4W::HIGH_DRIVE)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE5W::LOW_DRIVE => false,
            LINE5W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE5W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE5W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE5W::HIGH_DRIVE)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE6W::LOW_DRIVE => false,
            LINE6W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE6W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE6W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE6W::HIGH_DRIVE)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE7W::LOW_DRIVE => false,
            LINE7W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE7W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE7W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE7W::HIGH_DRIVE)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE8W::LOW_DRIVE => false,
            LINE8W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE8W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE8W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE8W::HIGH_DRIVE)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE9W::LOW_DRIVE => false,
            LINE9W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE9W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE9W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE9W::HIGH_DRIVE)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE10W::LOW_DRIVE => false,
            LINE10W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE10W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE10W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE10W::HIGH_DRIVE)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE11W::LOW_DRIVE => false,
            LINE11W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE11W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE11W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE11W::HIGH_DRIVE)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE12W::LOW_DRIVE => false,
            LINE12W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE12W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE12W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE12W::HIGH_DRIVE)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE13W::LOW_DRIVE => false,
            LINE13W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE13W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE13W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE13W::HIGH_DRIVE)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE14W::LOW_DRIVE => false,
            LINE14W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE14W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE14W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE14W::HIGH_DRIVE)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE15W::LOW_DRIVE => false,
            LINE15W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE15W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE15W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE15W::HIGH_DRIVE)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE16W::LOW_DRIVE => false,
            LINE16W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE16W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE16W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE16W::HIGH_DRIVE)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE17W::LOW_DRIVE => false,
            LINE17W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE17W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE17W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE17W::HIGH_DRIVE)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE18W::LOW_DRIVE => false,
            LINE18W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE18W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE18W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE18W::HIGH_DRIVE)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE19W::LOW_DRIVE => false,
            LINE19W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE19W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE19W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE19W::HIGH_DRIVE)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE20W::LOW_DRIVE => false,
            LINE20W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE20W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE20W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE20W::HIGH_DRIVE)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE21W::LOW_DRIVE => false,
            LINE21W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE21W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE21W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE21W::HIGH_DRIVE)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE22W::LOW_DRIVE => false,
            LINE22W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE22W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE22W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE22W::HIGH_DRIVE)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE23W::LOW_DRIVE => false,
            LINE23W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE23W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE23W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE23W::HIGH_DRIVE)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE24W::LOW_DRIVE => false,
            LINE24W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE24W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE24W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE24W::HIGH_DRIVE)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE25W::LOW_DRIVE => false,
            LINE25W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE25W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE25W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE25W::HIGH_DRIVE)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE26W::LOW_DRIVE => false,
            LINE26W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE26W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE26W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE26W::HIGH_DRIVE)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE27W::LOW_DRIVE => false,
            LINE27W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE27W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE27W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE27W::HIGH_DRIVE)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE28W::LOW_DRIVE => false,
            LINE28W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE28W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE28W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE28W::HIGH_DRIVE)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE29W::LOW_DRIVE => false,
            LINE29W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE29W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE29W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE29W::HIGH_DRIVE)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE30W::LOW_DRIVE => false,
            LINE30W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE30W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE30W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE30W::HIGH_DRIVE)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LINE31W::LOW_DRIVE => false,
            LINE31W::HIGH_DRIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LINE31W<'a> {
    w: &'a mut W,
}
impl<'a> _LINE31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LINE31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Lowest drive"]
    #[inline]
    pub fn low_drive(self) -> &'a mut W {
        self.variant(LINE31W::LOW_DRIVE)
    }
    #[doc = "Highest drive"]
    #[inline]
    pub fn high_drive(self) -> &'a mut W {
        self.variant(LINE31W::HIGH_DRIVE)
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
    #[doc = "Bit 0 - Drive of PIO Line 0"]
    #[inline]
    pub fn line0(&self) -> LINE0R {
        LINE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Drive of PIO Line 1"]
    #[inline]
    pub fn line1(&self) -> LINE1R {
        LINE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Drive of PIO Line 2"]
    #[inline]
    pub fn line2(&self) -> LINE2R {
        LINE2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Drive of PIO Line 3"]
    #[inline]
    pub fn line3(&self) -> LINE3R {
        LINE3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Drive of PIO Line 4"]
    #[inline]
    pub fn line4(&self) -> LINE4R {
        LINE4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Drive of PIO Line 5"]
    #[inline]
    pub fn line5(&self) -> LINE5R {
        LINE5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Drive of PIO Line 6"]
    #[inline]
    pub fn line6(&self) -> LINE6R {
        LINE6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Drive of PIO Line 7"]
    #[inline]
    pub fn line7(&self) -> LINE7R {
        LINE7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Drive of PIO Line 8"]
    #[inline]
    pub fn line8(&self) -> LINE8R {
        LINE8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Drive of PIO Line 9"]
    #[inline]
    pub fn line9(&self) -> LINE9R {
        LINE9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Drive of PIO Line 10"]
    #[inline]
    pub fn line10(&self) -> LINE10R {
        LINE10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Drive of PIO Line 11"]
    #[inline]
    pub fn line11(&self) -> LINE11R {
        LINE11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Drive of PIO Line 12"]
    #[inline]
    pub fn line12(&self) -> LINE12R {
        LINE12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Drive of PIO Line 13"]
    #[inline]
    pub fn line13(&self) -> LINE13R {
        LINE13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Drive of PIO Line 14"]
    #[inline]
    pub fn line14(&self) -> LINE14R {
        LINE14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Drive of PIO Line 15"]
    #[inline]
    pub fn line15(&self) -> LINE15R {
        LINE15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Drive of PIO Line 16"]
    #[inline]
    pub fn line16(&self) -> LINE16R {
        LINE16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Drive of PIO Line 17"]
    #[inline]
    pub fn line17(&self) -> LINE17R {
        LINE17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Drive of PIO Line 18"]
    #[inline]
    pub fn line18(&self) -> LINE18R {
        LINE18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Drive of PIO Line 19"]
    #[inline]
    pub fn line19(&self) -> LINE19R {
        LINE19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Drive of PIO Line 20"]
    #[inline]
    pub fn line20(&self) -> LINE20R {
        LINE20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Drive of PIO Line 21"]
    #[inline]
    pub fn line21(&self) -> LINE21R {
        LINE21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Drive of PIO Line 22"]
    #[inline]
    pub fn line22(&self) -> LINE22R {
        LINE22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Drive of PIO Line 23"]
    #[inline]
    pub fn line23(&self) -> LINE23R {
        LINE23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Drive of PIO Line 24"]
    #[inline]
    pub fn line24(&self) -> LINE24R {
        LINE24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Drive of PIO Line 25"]
    #[inline]
    pub fn line25(&self) -> LINE25R {
        LINE25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Drive of PIO Line 26"]
    #[inline]
    pub fn line26(&self) -> LINE26R {
        LINE26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Drive of PIO Line 27"]
    #[inline]
    pub fn line27(&self) -> LINE27R {
        LINE27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Drive of PIO Line 28"]
    #[inline]
    pub fn line28(&self) -> LINE28R {
        LINE28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Drive of PIO Line 29"]
    #[inline]
    pub fn line29(&self) -> LINE29R {
        LINE29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Drive of PIO Line 30"]
    #[inline]
    pub fn line30(&self) -> LINE30R {
        LINE30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Drive of PIO Line 31"]
    #[inline]
    pub fn line31(&self) -> LINE31R {
        LINE31R::_from({
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
    #[doc = "Bit 0 - Drive of PIO Line 0"]
    #[inline]
    pub fn line0(&mut self) -> _LINE0W {
        _LINE0W { w: self }
    }
    #[doc = "Bit 1 - Drive of PIO Line 1"]
    #[inline]
    pub fn line1(&mut self) -> _LINE1W {
        _LINE1W { w: self }
    }
    #[doc = "Bit 2 - Drive of PIO Line 2"]
    #[inline]
    pub fn line2(&mut self) -> _LINE2W {
        _LINE2W { w: self }
    }
    #[doc = "Bit 3 - Drive of PIO Line 3"]
    #[inline]
    pub fn line3(&mut self) -> _LINE3W {
        _LINE3W { w: self }
    }
    #[doc = "Bit 4 - Drive of PIO Line 4"]
    #[inline]
    pub fn line4(&mut self) -> _LINE4W {
        _LINE4W { w: self }
    }
    #[doc = "Bit 5 - Drive of PIO Line 5"]
    #[inline]
    pub fn line5(&mut self) -> _LINE5W {
        _LINE5W { w: self }
    }
    #[doc = "Bit 6 - Drive of PIO Line 6"]
    #[inline]
    pub fn line6(&mut self) -> _LINE6W {
        _LINE6W { w: self }
    }
    #[doc = "Bit 7 - Drive of PIO Line 7"]
    #[inline]
    pub fn line7(&mut self) -> _LINE7W {
        _LINE7W { w: self }
    }
    #[doc = "Bit 8 - Drive of PIO Line 8"]
    #[inline]
    pub fn line8(&mut self) -> _LINE8W {
        _LINE8W { w: self }
    }
    #[doc = "Bit 9 - Drive of PIO Line 9"]
    #[inline]
    pub fn line9(&mut self) -> _LINE9W {
        _LINE9W { w: self }
    }
    #[doc = "Bit 10 - Drive of PIO Line 10"]
    #[inline]
    pub fn line10(&mut self) -> _LINE10W {
        _LINE10W { w: self }
    }
    #[doc = "Bit 11 - Drive of PIO Line 11"]
    #[inline]
    pub fn line11(&mut self) -> _LINE11W {
        _LINE11W { w: self }
    }
    #[doc = "Bit 12 - Drive of PIO Line 12"]
    #[inline]
    pub fn line12(&mut self) -> _LINE12W {
        _LINE12W { w: self }
    }
    #[doc = "Bit 13 - Drive of PIO Line 13"]
    #[inline]
    pub fn line13(&mut self) -> _LINE13W {
        _LINE13W { w: self }
    }
    #[doc = "Bit 14 - Drive of PIO Line 14"]
    #[inline]
    pub fn line14(&mut self) -> _LINE14W {
        _LINE14W { w: self }
    }
    #[doc = "Bit 15 - Drive of PIO Line 15"]
    #[inline]
    pub fn line15(&mut self) -> _LINE15W {
        _LINE15W { w: self }
    }
    #[doc = "Bit 16 - Drive of PIO Line 16"]
    #[inline]
    pub fn line16(&mut self) -> _LINE16W {
        _LINE16W { w: self }
    }
    #[doc = "Bit 17 - Drive of PIO Line 17"]
    #[inline]
    pub fn line17(&mut self) -> _LINE17W {
        _LINE17W { w: self }
    }
    #[doc = "Bit 18 - Drive of PIO Line 18"]
    #[inline]
    pub fn line18(&mut self) -> _LINE18W {
        _LINE18W { w: self }
    }
    #[doc = "Bit 19 - Drive of PIO Line 19"]
    #[inline]
    pub fn line19(&mut self) -> _LINE19W {
        _LINE19W { w: self }
    }
    #[doc = "Bit 20 - Drive of PIO Line 20"]
    #[inline]
    pub fn line20(&mut self) -> _LINE20W {
        _LINE20W { w: self }
    }
    #[doc = "Bit 21 - Drive of PIO Line 21"]
    #[inline]
    pub fn line21(&mut self) -> _LINE21W {
        _LINE21W { w: self }
    }
    #[doc = "Bit 22 - Drive of PIO Line 22"]
    #[inline]
    pub fn line22(&mut self) -> _LINE22W {
        _LINE22W { w: self }
    }
    #[doc = "Bit 23 - Drive of PIO Line 23"]
    #[inline]
    pub fn line23(&mut self) -> _LINE23W {
        _LINE23W { w: self }
    }
    #[doc = "Bit 24 - Drive of PIO Line 24"]
    #[inline]
    pub fn line24(&mut self) -> _LINE24W {
        _LINE24W { w: self }
    }
    #[doc = "Bit 25 - Drive of PIO Line 25"]
    #[inline]
    pub fn line25(&mut self) -> _LINE25W {
        _LINE25W { w: self }
    }
    #[doc = "Bit 26 - Drive of PIO Line 26"]
    #[inline]
    pub fn line26(&mut self) -> _LINE26W {
        _LINE26W { w: self }
    }
    #[doc = "Bit 27 - Drive of PIO Line 27"]
    #[inline]
    pub fn line27(&mut self) -> _LINE27W {
        _LINE27W { w: self }
    }
    #[doc = "Bit 28 - Drive of PIO Line 28"]
    #[inline]
    pub fn line28(&mut self) -> _LINE28W {
        _LINE28W { w: self }
    }
    #[doc = "Bit 29 - Drive of PIO Line 29"]
    #[inline]
    pub fn line29(&mut self) -> _LINE29W {
        _LINE29W { w: self }
    }
    #[doc = "Bit 30 - Drive of PIO Line 30"]
    #[inline]
    pub fn line30(&mut self) -> _LINE30W {
        _LINE30W { w: self }
    }
    #[doc = "Bit 31 - Drive of PIO Line 31"]
    #[inline]
    pub fn line31(&mut self) -> _LINE31W {
        _LINE31W { w: self }
    }
}
