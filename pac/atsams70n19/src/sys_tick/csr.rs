#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSR {
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
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = "counter disabled"]
    VALUE_0,
    #[doc = "counter enabled"]
    VALUE_1,
}
impl ENABLER {
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
            ENABLER::VALUE_0 => false,
            ENABLER::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENABLER {
        match value {
            false => ENABLER::VALUE_0,
            true => ENABLER::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == ENABLER::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == ENABLER::VALUE_1
    }
}
#[doc = "Possible values of the field `TICKINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICKINTR {
    #[doc = "counting down to 0 does not assert the SysTick exception request"]
    VALUE_0,
    #[doc = "counting down to 0 asserts the SysTick exception request"]
    VALUE_1,
}
impl TICKINTR {
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
            TICKINTR::VALUE_0 => false,
            TICKINTR::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TICKINTR {
        match value {
            false => TICKINTR::VALUE_0,
            true => TICKINTR::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == TICKINTR::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == TICKINTR::VALUE_1
    }
}
#[doc = "Possible values of the field `CLKSOURCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSOURCER {
    #[doc = "external clock"]
    VALUE_0,
    #[doc = "processor clock"]
    VALUE_1,
}
impl CLKSOURCER {
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
            CLKSOURCER::VALUE_0 => false,
            CLKSOURCER::VALUE_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKSOURCER {
        match value {
            false => CLKSOURCER::VALUE_0,
            true => CLKSOURCER::VALUE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline]
    pub fn is_value_0(&self) -> bool {
        *self == CLKSOURCER::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline]
    pub fn is_value_1(&self) -> bool {
        *self == CLKSOURCER::VALUE_1
    }
}
#[doc = r" Value of the field"]
pub struct COUNTFLAGR {
    bits: bool,
}
impl COUNTFLAGR {
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
#[doc = "Values that can be written to the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLEW {
    #[doc = "counter disabled"]
    VALUE_0,
    #[doc = "counter enabled"]
    VALUE_1,
}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENABLEW::VALUE_0 => false,
            ENABLEW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "counter disabled"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(ENABLEW::VALUE_0)
    }
    #[doc = "counter enabled"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(ENABLEW::VALUE_1)
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
#[doc = "Values that can be written to the field `TICKINT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICKINTW {
    #[doc = "counting down to 0 does not assert the SysTick exception request"]
    VALUE_0,
    #[doc = "counting down to 0 asserts the SysTick exception request"]
    VALUE_1,
}
impl TICKINTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TICKINTW::VALUE_0 => false,
            TICKINTW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TICKINTW<'a> {
    w: &'a mut W,
}
impl<'a> _TICKINTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TICKINTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "counting down to 0 does not assert the SysTick exception request"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(TICKINTW::VALUE_0)
    }
    #[doc = "counting down to 0 asserts the SysTick exception request"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(TICKINTW::VALUE_1)
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
#[doc = "Values that can be written to the field `CLKSOURCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSOURCEW {
    #[doc = "external clock"]
    VALUE_0,
    #[doc = "processor clock"]
    VALUE_1,
}
impl CLKSOURCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKSOURCEW::VALUE_0 => false,
            CLKSOURCEW::VALUE_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSOURCEW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSOURCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSOURCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "external clock"]
    #[inline]
    pub fn value_0(self) -> &'a mut W {
        self.variant(CLKSOURCEW::VALUE_0)
    }
    #[doc = "processor clock"]
    #[inline]
    pub fn value_1(self) -> &'a mut W {
        self.variant(CLKSOURCEW::VALUE_1)
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
#[doc = r" Proxy"]
pub struct _COUNTFLAGW<'a> {
    w: &'a mut W,
}
impl<'a> _COUNTFLAGW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enables the counter"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        ENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enables SysTick exception request"]
    #[inline]
    pub fn tickint(&self) -> TICKINTR {
        TICKINTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Indicates the clock source"]
    #[inline]
    pub fn clksource(&self) -> CLKSOURCER {
        CLKSOURCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Returns 1 if timer counted to 0 since last time this was read"]
    #[inline]
    pub fn countflag(&self) -> COUNTFLAGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COUNTFLAGR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Enables the counter"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bit 1 - Enables SysTick exception request"]
    #[inline]
    pub fn tickint(&mut self) -> _TICKINTW {
        _TICKINTW { w: self }
    }
    #[doc = "Bit 2 - Indicates the clock source"]
    #[inline]
    pub fn clksource(&mut self) -> _CLKSOURCEW {
        _CLKSOURCEW { w: self }
    }
    #[doc = "Bit 16 - Returns 1 if timer counted to 0 since last time this was read"]
    #[inline]
    pub fn countflag(&mut self) -> _COUNTFLAGW {
        _COUNTFLAGW { w: self }
    }
}
