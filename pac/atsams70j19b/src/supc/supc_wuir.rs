#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SUPC_WUIR {
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
#[doc = "Possible values of the field `WKUPEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN0R {
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    DISABLE,
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    ENABLE,
}
impl WKUPEN0R {
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
            WKUPEN0R::DISABLE => false,
            WKUPEN0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN0R {
        match value {
            false => WKUPEN0R::DISABLE,
            true => WKUPEN0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN0R::ENABLE
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
impl WKUPEN1R {
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
            WKUPEN1R::DISABLE => false,
            WKUPEN1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN1R {
        match value {
            false => WKUPEN1R::DISABLE,
            true => WKUPEN1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN1R::ENABLE
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
impl WKUPEN2R {
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
            WKUPEN2R::DISABLE => false,
            WKUPEN2R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN2R {
        match value {
            false => WKUPEN2R::DISABLE,
            true => WKUPEN2R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN2R::ENABLE
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
impl WKUPEN3R {
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
            WKUPEN3R::DISABLE => false,
            WKUPEN3R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN3R {
        match value {
            false => WKUPEN3R::DISABLE,
            true => WKUPEN3R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN3R::ENABLE
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
impl WKUPEN4R {
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
            WKUPEN4R::DISABLE => false,
            WKUPEN4R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN4R {
        match value {
            false => WKUPEN4R::DISABLE,
            true => WKUPEN4R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN4R::ENABLE
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
impl WKUPEN5R {
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
            WKUPEN5R::DISABLE => false,
            WKUPEN5R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN5R {
        match value {
            false => WKUPEN5R::DISABLE,
            true => WKUPEN5R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN5R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN5R::ENABLE
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
impl WKUPEN6R {
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
            WKUPEN6R::DISABLE => false,
            WKUPEN6R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN6R {
        match value {
            false => WKUPEN6R::DISABLE,
            true => WKUPEN6R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN6R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN6R::ENABLE
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
impl WKUPEN7R {
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
            WKUPEN7R::DISABLE => false,
            WKUPEN7R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN7R {
        match value {
            false => WKUPEN7R::DISABLE,
            true => WKUPEN7R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN7R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN7R::ENABLE
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
impl WKUPEN8R {
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
            WKUPEN8R::DISABLE => false,
            WKUPEN8R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN8R {
        match value {
            false => WKUPEN8R::DISABLE,
            true => WKUPEN8R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN8R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN8R::ENABLE
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
impl WKUPEN9R {
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
            WKUPEN9R::DISABLE => false,
            WKUPEN9R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN9R {
        match value {
            false => WKUPEN9R::DISABLE,
            true => WKUPEN9R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN9R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN9R::ENABLE
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
impl WKUPEN10R {
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
            WKUPEN10R::DISABLE => false,
            WKUPEN10R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN10R {
        match value {
            false => WKUPEN10R::DISABLE,
            true => WKUPEN10R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN10R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN10R::ENABLE
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
impl WKUPEN11R {
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
            WKUPEN11R::DISABLE => false,
            WKUPEN11R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN11R {
        match value {
            false => WKUPEN11R::DISABLE,
            true => WKUPEN11R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN11R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN11R::ENABLE
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
impl WKUPEN12R {
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
            WKUPEN12R::DISABLE => false,
            WKUPEN12R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN12R {
        match value {
            false => WKUPEN12R::DISABLE,
            true => WKUPEN12R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN12R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN12R::ENABLE
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
impl WKUPEN13R {
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
            WKUPEN13R::DISABLE => false,
            WKUPEN13R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPEN13R {
        match value {
            false => WKUPEN13R::DISABLE,
            true => WKUPEN13R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN13R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN13R::ENABLE
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
impl WKUPT0R {
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
            WKUPT0R::LOW => false,
            WKUPT0R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT0R {
        match value {
            false => WKUPT0R::LOW,
            true => WKUPT0R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == WKUPT0R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == WKUPT0R::HIGH
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
impl WKUPT1R {
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
            WKUPT1R::LOW => false,
            WKUPT1R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT1R {
        match value {
            false => WKUPT1R::LOW,
            true => WKUPT1R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == WKUPT1R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == WKUPT1R::HIGH
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
impl WKUPT2R {
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
            WKUPT2R::LOW => false,
            WKUPT2R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT2R {
        match value {
            false => WKUPT2R::LOW,
            true => WKUPT2R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == WKUPT2R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == WKUPT2R::HIGH
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
impl WKUPT3R {
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
            WKUPT3R::LOW => false,
            WKUPT3R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT3R {
        match value {
            false => WKUPT3R::LOW,
            true => WKUPT3R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == WKUPT3R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == WKUPT3R::HIGH
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
impl WKUPT4R {
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
            WKUPT4R::LOW => false,
            WKUPT4R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT4R {
        match value {
            false => WKUPT4R::LOW,
            true => WKUPT4R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == WKUPT4R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == WKUPT4R::HIGH
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
impl WKUPT5R {
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
            WKUPT5R::LOW => false,
            WKUPT5R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT5R {
        match value {
            false => WKUPT5R::LOW,
            true => WKUPT5R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == WKUPT5R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == WKUPT5R::HIGH
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
impl WKUPT6R {
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
            WKUPT6R::LOW => false,
            WKUPT6R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT6R {
        match value {
            false => WKUPT6R::LOW,
            true => WKUPT6R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == WKUPT6R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == WKUPT6R::HIGH
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
impl WKUPT7R {
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
            WKUPT7R::LOW => false,
            WKUPT7R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT7R {
        match value {
            false => WKUPT7R::LOW,
            true => WKUPT7R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == WKUPT7R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == WKUPT7R::HIGH
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
impl WKUPT8R {
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
            WKUPT8R::LOW => false,
            WKUPT8R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT8R {
        match value {
            false => WKUPT8R::LOW,
            true => WKUPT8R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == WKUPT8R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == WKUPT8R::HIGH
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
impl WKUPT9R {
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
            WKUPT9R::LOW => false,
            WKUPT9R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT9R {
        match value {
            false => WKUPT9R::LOW,
            true => WKUPT9R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == WKUPT9R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == WKUPT9R::HIGH
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
impl WKUPT10R {
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
            WKUPT10R::LOW => false,
            WKUPT10R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT10R {
        match value {
            false => WKUPT10R::LOW,
            true => WKUPT10R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == WKUPT10R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == WKUPT10R::HIGH
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
impl WKUPT11R {
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
            WKUPT11R::LOW => false,
            WKUPT11R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT11R {
        match value {
            false => WKUPT11R::LOW,
            true => WKUPT11R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == WKUPT11R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == WKUPT11R::HIGH
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
impl WKUPT12R {
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
            WKUPT12R::LOW => false,
            WKUPT12R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT12R {
        match value {
            false => WKUPT12R::LOW,
            true => WKUPT12R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == WKUPT12R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == WKUPT12R::HIGH
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
impl WKUPT13R {
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
            WKUPT13R::LOW => false,
            WKUPT13R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WKUPT13R {
        match value {
            false => WKUPT13R::LOW,
            true => WKUPT13R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == WKUPT13R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == WKUPT13R::HIGH
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN0W::DISABLE => false,
            WKUPEN0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN0W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN0W::ENABLE)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN1W::DISABLE => false,
            WKUPEN1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN1W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN1W::ENABLE)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN2W::DISABLE => false,
            WKUPEN2W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN2W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN2W::ENABLE)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN3W::DISABLE => false,
            WKUPEN3W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN3W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN3W::ENABLE)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN4W::DISABLE => false,
            WKUPEN4W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN4W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN4W::ENABLE)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN5W::DISABLE => false,
            WKUPEN5W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN5W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN5W::ENABLE)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN6W::DISABLE => false,
            WKUPEN6W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN6W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN6W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN6W::ENABLE)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN7W::DISABLE => false,
            WKUPEN7W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN7W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN7W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN7W::ENABLE)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN8W::DISABLE => false,
            WKUPEN8W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN8W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN8W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN8W::ENABLE)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN9W::DISABLE => false,
            WKUPEN9W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN9W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN9W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN9W::ENABLE)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN10W::DISABLE => false,
            WKUPEN10W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN10W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN10W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN10W::ENABLE)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN11W::DISABLE => false,
            WKUPEN11W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN11W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN11W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN11W::ENABLE)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN12W::DISABLE => false,
            WKUPEN12W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN12W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN12W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN12W::ENABLE)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPEN13W::DISABLE => false,
            WKUPEN13W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPEN13W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPEN13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPEN13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding wake-up input has no wake-up effect."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN13W::DISABLE)
    }
    #[doc = "The corresponding wake-up input is enabled for a wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN13W::ENABLE)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT0W::LOW => false,
            WKUPT0W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT0W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT0W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT0W::HIGH)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT1W::LOW => false,
            WKUPT1W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT1W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT1W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT1W::HIGH)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT2W::LOW => false,
            WKUPT2W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT2W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT2W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT2W::HIGH)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT3W::LOW => false,
            WKUPT3W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT3W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT3W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT3W::HIGH)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT4W::LOW => false,
            WKUPT4W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT4W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT4W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT4W::HIGH)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT5W::LOW => false,
            WKUPT5W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT5W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT5W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT5W::HIGH)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT6W::LOW => false,
            WKUPT6W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT6W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT6W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT6W::HIGH)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT7W::LOW => false,
            WKUPT7W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT7W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT7W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT7W::HIGH)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT8W::LOW => false,
            WKUPT8W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT8W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT8W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT8W::HIGH)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT9W::LOW => false,
            WKUPT9W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT9W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT9W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT9W::HIGH)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT10W::LOW => false,
            WKUPT10W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT10W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT10W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT10W::HIGH)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT11W::LOW => false,
            WKUPT11W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT11W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT11W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT11W::HIGH)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT12W::LOW => false,
            WKUPT12W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT12W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT12W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT12W::HIGH)
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WKUPT13W::LOW => false,
            WKUPT13W::HIGH => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPT13W<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPT13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPT13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT13W::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wake-up input forces the wake-up of the core power supply."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT13W::HIGH)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Wake-up Input Enable 0 to 0"]
    #[inline]
    pub fn wkupen0(&self) -> WKUPEN0R {
        WKUPEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Wake-up Input Enable 0 to 1"]
    #[inline]
    pub fn wkupen1(&self) -> WKUPEN1R {
        WKUPEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Wake-up Input Enable 0 to 2"]
    #[inline]
    pub fn wkupen2(&self) -> WKUPEN2R {
        WKUPEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Wake-up Input Enable 0 to 3"]
    #[inline]
    pub fn wkupen3(&self) -> WKUPEN3R {
        WKUPEN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Wake-up Input Enable 0 to 4"]
    #[inline]
    pub fn wkupen4(&self) -> WKUPEN4R {
        WKUPEN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Wake-up Input Enable 0 to 5"]
    #[inline]
    pub fn wkupen5(&self) -> WKUPEN5R {
        WKUPEN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Wake-up Input Enable 0 to 6"]
    #[inline]
    pub fn wkupen6(&self) -> WKUPEN6R {
        WKUPEN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Wake-up Input Enable 0 to 7"]
    #[inline]
    pub fn wkupen7(&self) -> WKUPEN7R {
        WKUPEN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Wake-up Input Enable 0 to 8"]
    #[inline]
    pub fn wkupen8(&self) -> WKUPEN8R {
        WKUPEN8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Wake-up Input Enable 0 to 9"]
    #[inline]
    pub fn wkupen9(&self) -> WKUPEN9R {
        WKUPEN9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Wake-up Input Enable 0 to 10"]
    #[inline]
    pub fn wkupen10(&self) -> WKUPEN10R {
        WKUPEN10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Wake-up Input Enable 0 to 11"]
    #[inline]
    pub fn wkupen11(&self) -> WKUPEN11R {
        WKUPEN11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Wake-up Input Enable 0 to 12"]
    #[inline]
    pub fn wkupen12(&self) -> WKUPEN12R {
        WKUPEN12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Wake-up Input Enable 0 to 13"]
    #[inline]
    pub fn wkupen13(&self) -> WKUPEN13R {
        WKUPEN13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Wake-up Input Type 0 to 0"]
    #[inline]
    pub fn wkupt0(&self) -> WKUPT0R {
        WKUPT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Wake-up Input Type 0 to 1"]
    #[inline]
    pub fn wkupt1(&self) -> WKUPT1R {
        WKUPT1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Wake-up Input Type 0 to 2"]
    #[inline]
    pub fn wkupt2(&self) -> WKUPT2R {
        WKUPT2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Wake-up Input Type 0 to 3"]
    #[inline]
    pub fn wkupt3(&self) -> WKUPT3R {
        WKUPT3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Wake-up Input Type 0 to 4"]
    #[inline]
    pub fn wkupt4(&self) -> WKUPT4R {
        WKUPT4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Wake-up Input Type 0 to 5"]
    #[inline]
    pub fn wkupt5(&self) -> WKUPT5R {
        WKUPT5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Wake-up Input Type 0 to 6"]
    #[inline]
    pub fn wkupt6(&self) -> WKUPT6R {
        WKUPT6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Wake-up Input Type 0 to 7"]
    #[inline]
    pub fn wkupt7(&self) -> WKUPT7R {
        WKUPT7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Wake-up Input Type 0 to 8"]
    #[inline]
    pub fn wkupt8(&self) -> WKUPT8R {
        WKUPT8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Wake-up Input Type 0 to 9"]
    #[inline]
    pub fn wkupt9(&self) -> WKUPT9R {
        WKUPT9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Wake-up Input Type 0 to 10"]
    #[inline]
    pub fn wkupt10(&self) -> WKUPT10R {
        WKUPT10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Wake-up Input Type 0 to 11"]
    #[inline]
    pub fn wkupt11(&self) -> WKUPT11R {
        WKUPT11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Wake-up Input Type 0 to 12"]
    #[inline]
    pub fn wkupt12(&self) -> WKUPT12R {
        WKUPT12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Wake-up Input Type 0 to 13"]
    #[inline]
    pub fn wkupt13(&self) -> WKUPT13R {
        WKUPT13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
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
    #[doc = "Bit 0 - Wake-up Input Enable 0 to 0"]
    #[inline]
    pub fn wkupen0(&mut self) -> _WKUPEN0W {
        _WKUPEN0W { w: self }
    }
    #[doc = "Bit 1 - Wake-up Input Enable 0 to 1"]
    #[inline]
    pub fn wkupen1(&mut self) -> _WKUPEN1W {
        _WKUPEN1W { w: self }
    }
    #[doc = "Bit 2 - Wake-up Input Enable 0 to 2"]
    #[inline]
    pub fn wkupen2(&mut self) -> _WKUPEN2W {
        _WKUPEN2W { w: self }
    }
    #[doc = "Bit 3 - Wake-up Input Enable 0 to 3"]
    #[inline]
    pub fn wkupen3(&mut self) -> _WKUPEN3W {
        _WKUPEN3W { w: self }
    }
    #[doc = "Bit 4 - Wake-up Input Enable 0 to 4"]
    #[inline]
    pub fn wkupen4(&mut self) -> _WKUPEN4W {
        _WKUPEN4W { w: self }
    }
    #[doc = "Bit 5 - Wake-up Input Enable 0 to 5"]
    #[inline]
    pub fn wkupen5(&mut self) -> _WKUPEN5W {
        _WKUPEN5W { w: self }
    }
    #[doc = "Bit 6 - Wake-up Input Enable 0 to 6"]
    #[inline]
    pub fn wkupen6(&mut self) -> _WKUPEN6W {
        _WKUPEN6W { w: self }
    }
    #[doc = "Bit 7 - Wake-up Input Enable 0 to 7"]
    #[inline]
    pub fn wkupen7(&mut self) -> _WKUPEN7W {
        _WKUPEN7W { w: self }
    }
    #[doc = "Bit 8 - Wake-up Input Enable 0 to 8"]
    #[inline]
    pub fn wkupen8(&mut self) -> _WKUPEN8W {
        _WKUPEN8W { w: self }
    }
    #[doc = "Bit 9 - Wake-up Input Enable 0 to 9"]
    #[inline]
    pub fn wkupen9(&mut self) -> _WKUPEN9W {
        _WKUPEN9W { w: self }
    }
    #[doc = "Bit 10 - Wake-up Input Enable 0 to 10"]
    #[inline]
    pub fn wkupen10(&mut self) -> _WKUPEN10W {
        _WKUPEN10W { w: self }
    }
    #[doc = "Bit 11 - Wake-up Input Enable 0 to 11"]
    #[inline]
    pub fn wkupen11(&mut self) -> _WKUPEN11W {
        _WKUPEN11W { w: self }
    }
    #[doc = "Bit 12 - Wake-up Input Enable 0 to 12"]
    #[inline]
    pub fn wkupen12(&mut self) -> _WKUPEN12W {
        _WKUPEN12W { w: self }
    }
    #[doc = "Bit 13 - Wake-up Input Enable 0 to 13"]
    #[inline]
    pub fn wkupen13(&mut self) -> _WKUPEN13W {
        _WKUPEN13W { w: self }
    }
    #[doc = "Bit 16 - Wake-up Input Type 0 to 0"]
    #[inline]
    pub fn wkupt0(&mut self) -> _WKUPT0W {
        _WKUPT0W { w: self }
    }
    #[doc = "Bit 17 - Wake-up Input Type 0 to 1"]
    #[inline]
    pub fn wkupt1(&mut self) -> _WKUPT1W {
        _WKUPT1W { w: self }
    }
    #[doc = "Bit 18 - Wake-up Input Type 0 to 2"]
    #[inline]
    pub fn wkupt2(&mut self) -> _WKUPT2W {
        _WKUPT2W { w: self }
    }
    #[doc = "Bit 19 - Wake-up Input Type 0 to 3"]
    #[inline]
    pub fn wkupt3(&mut self) -> _WKUPT3W {
        _WKUPT3W { w: self }
    }
    #[doc = "Bit 20 - Wake-up Input Type 0 to 4"]
    #[inline]
    pub fn wkupt4(&mut self) -> _WKUPT4W {
        _WKUPT4W { w: self }
    }
    #[doc = "Bit 21 - Wake-up Input Type 0 to 5"]
    #[inline]
    pub fn wkupt5(&mut self) -> _WKUPT5W {
        _WKUPT5W { w: self }
    }
    #[doc = "Bit 22 - Wake-up Input Type 0 to 6"]
    #[inline]
    pub fn wkupt6(&mut self) -> _WKUPT6W {
        _WKUPT6W { w: self }
    }
    #[doc = "Bit 23 - Wake-up Input Type 0 to 7"]
    #[inline]
    pub fn wkupt7(&mut self) -> _WKUPT7W {
        _WKUPT7W { w: self }
    }
    #[doc = "Bit 24 - Wake-up Input Type 0 to 8"]
    #[inline]
    pub fn wkupt8(&mut self) -> _WKUPT8W {
        _WKUPT8W { w: self }
    }
    #[doc = "Bit 25 - Wake-up Input Type 0 to 9"]
    #[inline]
    pub fn wkupt9(&mut self) -> _WKUPT9W {
        _WKUPT9W { w: self }
    }
    #[doc = "Bit 26 - Wake-up Input Type 0 to 10"]
    #[inline]
    pub fn wkupt10(&mut self) -> _WKUPT10W {
        _WKUPT10W { w: self }
    }
    #[doc = "Bit 27 - Wake-up Input Type 0 to 11"]
    #[inline]
    pub fn wkupt11(&mut self) -> _WKUPT11W {
        _WKUPT11W { w: self }
    }
    #[doc = "Bit 28 - Wake-up Input Type 0 to 12"]
    #[inline]
    pub fn wkupt12(&mut self) -> _WKUPT12W {
        _WKUPT12W { w: self }
    }
    #[doc = "Bit 29 - Wake-up Input Type 0 to 13"]
    #[inline]
    pub fn wkupt13(&mut self) -> _WKUPT13W {
        _WKUPT13W { w: self }
    }
}
