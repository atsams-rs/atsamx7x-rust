#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TC_BMR {
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
#[doc = "Possible values of the field `TC0XC0S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0XC0SR {
    #[doc = "Signal connected to XC0: TCLK0"]
    TCLK0,
    #[doc = "Signal connected to XC0: TIOA1"]
    TIOA1,
    #[doc = "Signal connected to XC0: TIOA2"]
    TIOA2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TC0XC0SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TC0XC0SR::TCLK0 => 0,
            TC0XC0SR::TIOA1 => 2,
            TC0XC0SR::TIOA2 => 3,
            TC0XC0SR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TC0XC0SR {
        match value {
            0 => TC0XC0SR::TCLK0,
            2 => TC0XC0SR::TIOA1,
            3 => TC0XC0SR::TIOA2,
            i => TC0XC0SR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TCLK0`"]
    #[inline]
    pub fn is_tclk0(&self) -> bool {
        *self == TC0XC0SR::TCLK0
    }
    #[doc = "Checks if the value of the field is `TIOA1`"]
    #[inline]
    pub fn is_tioa1(&self) -> bool {
        *self == TC0XC0SR::TIOA1
    }
    #[doc = "Checks if the value of the field is `TIOA2`"]
    #[inline]
    pub fn is_tioa2(&self) -> bool {
        *self == TC0XC0SR::TIOA2
    }
}
#[doc = "Possible values of the field `TC1XC1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1XC1SR {
    #[doc = "Signal connected to XC1: TCLK1"]
    TCLK1,
    #[doc = "Signal connected to XC1: TIOA0"]
    TIOA0,
    #[doc = "Signal connected to XC1: TIOA2"]
    TIOA2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TC1XC1SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TC1XC1SR::TCLK1 => 0,
            TC1XC1SR::TIOA0 => 2,
            TC1XC1SR::TIOA2 => 3,
            TC1XC1SR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TC1XC1SR {
        match value {
            0 => TC1XC1SR::TCLK1,
            2 => TC1XC1SR::TIOA0,
            3 => TC1XC1SR::TIOA2,
            i => TC1XC1SR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TCLK1`"]
    #[inline]
    pub fn is_tclk1(&self) -> bool {
        *self == TC1XC1SR::TCLK1
    }
    #[doc = "Checks if the value of the field is `TIOA0`"]
    #[inline]
    pub fn is_tioa0(&self) -> bool {
        *self == TC1XC1SR::TIOA0
    }
    #[doc = "Checks if the value of the field is `TIOA2`"]
    #[inline]
    pub fn is_tioa2(&self) -> bool {
        *self == TC1XC1SR::TIOA2
    }
}
#[doc = "Possible values of the field `TC2XC2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2XC2SR {
    #[doc = "Signal connected to XC2: TCLK2"]
    TCLK2,
    #[doc = "Signal connected to XC2: TIOA0"]
    TIOA0,
    #[doc = "Signal connected to XC2: TIOA1"]
    TIOA1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TC2XC2SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TC2XC2SR::TCLK2 => 0,
            TC2XC2SR::TIOA0 => 2,
            TC2XC2SR::TIOA1 => 3,
            TC2XC2SR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TC2XC2SR {
        match value {
            0 => TC2XC2SR::TCLK2,
            2 => TC2XC2SR::TIOA0,
            3 => TC2XC2SR::TIOA1,
            i => TC2XC2SR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `TCLK2`"]
    #[inline]
    pub fn is_tclk2(&self) -> bool {
        *self == TC2XC2SR::TCLK2
    }
    #[doc = "Checks if the value of the field is `TIOA0`"]
    #[inline]
    pub fn is_tioa0(&self) -> bool {
        *self == TC2XC2SR::TIOA0
    }
    #[doc = "Checks if the value of the field is `TIOA1`"]
    #[inline]
    pub fn is_tioa1(&self) -> bool {
        *self == TC2XC2SR::TIOA1
    }
}
#[doc = r" Value of the field"]
pub struct QDENR {
    bits: bool,
}
impl QDENR {
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
#[doc = r" Value of the field"]
pub struct POSENR {
    bits: bool,
}
impl POSENR {
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
#[doc = r" Value of the field"]
pub struct SPEEDENR {
    bits: bool,
}
impl SPEEDENR {
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
#[doc = r" Value of the field"]
pub struct QDTRANSR {
    bits: bool,
}
impl QDTRANSR {
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
#[doc = r" Value of the field"]
pub struct EDGPHAR {
    bits: bool,
}
impl EDGPHAR {
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
#[doc = r" Value of the field"]
pub struct INVAR {
    bits: bool,
}
impl INVAR {
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
#[doc = r" Value of the field"]
pub struct INVBR {
    bits: bool,
}
impl INVBR {
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
#[doc = r" Value of the field"]
pub struct INVIDXR {
    bits: bool,
}
impl INVIDXR {
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
#[doc = r" Value of the field"]
pub struct SWAPR {
    bits: bool,
}
impl SWAPR {
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
#[doc = r" Value of the field"]
pub struct IDXPHBR {
    bits: bool,
}
impl IDXPHBR {
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
#[doc = r" Value of the field"]
pub struct MAXFILTR {
    bits: u8,
}
impl MAXFILTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `TC0XC0S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0XC0SW {
    #[doc = "Signal connected to XC0: TCLK0"]
    TCLK0,
    #[doc = "Signal connected to XC0: TIOA1"]
    TIOA1,
    #[doc = "Signal connected to XC0: TIOA2"]
    TIOA2,
}
impl TC0XC0SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TC0XC0SW::TCLK0 => 0,
            TC0XC0SW::TIOA1 => 2,
            TC0XC0SW::TIOA2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TC0XC0SW<'a> {
    w: &'a mut W,
}
impl<'a> _TC0XC0SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TC0XC0SW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Signal connected to XC0: TCLK0"]
    #[inline]
    pub fn tclk0(self) -> &'a mut W {
        self.variant(TC0XC0SW::TCLK0)
    }
    #[doc = "Signal connected to XC0: TIOA1"]
    #[inline]
    pub fn tioa1(self) -> &'a mut W {
        self.variant(TC0XC0SW::TIOA1)
    }
    #[doc = "Signal connected to XC0: TIOA2"]
    #[inline]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC0XC0SW::TIOA2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TC1XC1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1XC1SW {
    #[doc = "Signal connected to XC1: TCLK1"]
    TCLK1,
    #[doc = "Signal connected to XC1: TIOA0"]
    TIOA0,
    #[doc = "Signal connected to XC1: TIOA2"]
    TIOA2,
}
impl TC1XC1SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TC1XC1SW::TCLK1 => 0,
            TC1XC1SW::TIOA0 => 2,
            TC1XC1SW::TIOA2 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TC1XC1SW<'a> {
    w: &'a mut W,
}
impl<'a> _TC1XC1SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TC1XC1SW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Signal connected to XC1: TCLK1"]
    #[inline]
    pub fn tclk1(self) -> &'a mut W {
        self.variant(TC1XC1SW::TCLK1)
    }
    #[doc = "Signal connected to XC1: TIOA0"]
    #[inline]
    pub fn tioa0(self) -> &'a mut W {
        self.variant(TC1XC1SW::TIOA0)
    }
    #[doc = "Signal connected to XC1: TIOA2"]
    #[inline]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC1XC1SW::TIOA2)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TC2XC2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2XC2SW {
    #[doc = "Signal connected to XC2: TCLK2"]
    TCLK2,
    #[doc = "Signal connected to XC2: TIOA0"]
    TIOA0,
    #[doc = "Signal connected to XC2: TIOA1"]
    TIOA1,
}
impl TC2XC2SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TC2XC2SW::TCLK2 => 0,
            TC2XC2SW::TIOA0 => 2,
            TC2XC2SW::TIOA1 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TC2XC2SW<'a> {
    w: &'a mut W,
}
impl<'a> _TC2XC2SW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TC2XC2SW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Signal connected to XC2: TCLK2"]
    #[inline]
    pub fn tclk2(self) -> &'a mut W {
        self.variant(TC2XC2SW::TCLK2)
    }
    #[doc = "Signal connected to XC2: TIOA0"]
    #[inline]
    pub fn tioa0(self) -> &'a mut W {
        self.variant(TC2XC2SW::TIOA0)
    }
    #[doc = "Signal connected to XC2: TIOA1"]
    #[inline]
    pub fn tioa1(self) -> &'a mut W {
        self.variant(TC2XC2SW::TIOA1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _QDENW<'a> {
    w: &'a mut W,
}
impl<'a> _QDENW<'a> {
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
#[doc = r" Proxy"]
pub struct _POSENW<'a> {
    w: &'a mut W,
}
impl<'a> _POSENW<'a> {
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
#[doc = r" Proxy"]
pub struct _SPEEDENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPEEDENW<'a> {
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
#[doc = r" Proxy"]
pub struct _QDTRANSW<'a> {
    w: &'a mut W,
}
impl<'a> _QDTRANSW<'a> {
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
#[doc = r" Proxy"]
pub struct _EDGPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGPHAW<'a> {
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
#[doc = r" Proxy"]
pub struct _INVAW<'a> {
    w: &'a mut W,
}
impl<'a> _INVAW<'a> {
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
#[doc = r" Proxy"]
pub struct _INVBW<'a> {
    w: &'a mut W,
}
impl<'a> _INVBW<'a> {
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
#[doc = r" Proxy"]
pub struct _INVIDXW<'a> {
    w: &'a mut W,
}
impl<'a> _INVIDXW<'a> {
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
#[doc = r" Proxy"]
pub struct _SWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _SWAPW<'a> {
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
#[doc = r" Proxy"]
pub struct _IDXPHBW<'a> {
    w: &'a mut W,
}
impl<'a> _IDXPHBW<'a> {
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
#[doc = r" Proxy"]
pub struct _MAXFILTW<'a> {
    w: &'a mut W,
}
impl<'a> _MAXFILTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline]
    pub fn tc0xc0s(&self) -> TC0XC0SR {
        TC0XC0SR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline]
    pub fn tc1xc1s(&self) -> TC1XC1SR {
        TC1XC1SR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline]
    pub fn tc2xc2s(&self) -> TC2XC2SR {
        TC2XC2SR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Quadrature Decoder Enabled"]
    #[inline]
    pub fn qden(&self) -> QDENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        QDENR { bits }
    }
    #[doc = "Bit 9 - Position Enabled"]
    #[inline]
    pub fn posen(&self) -> POSENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        POSENR { bits }
    }
    #[doc = "Bit 10 - Speed Enabled"]
    #[inline]
    pub fn speeden(&self) -> SPEEDENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPEEDENR { bits }
    }
    #[doc = "Bit 11 - Quadrature Decoding Transparent"]
    #[inline]
    pub fn qdtrans(&self) -> QDTRANSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        QDTRANSR { bits }
    }
    #[doc = "Bit 12 - Edge on PHA Count Mode"]
    #[inline]
    pub fn edgpha(&self) -> EDGPHAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EDGPHAR { bits }
    }
    #[doc = "Bit 13 - Inverted PHA"]
    #[inline]
    pub fn inva(&self) -> INVAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INVAR { bits }
    }
    #[doc = "Bit 14 - Inverted PHB"]
    #[inline]
    pub fn invb(&self) -> INVBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INVBR { bits }
    }
    #[doc = "Bit 15 - Inverted Index"]
    #[inline]
    pub fn invidx(&self) -> INVIDXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INVIDXR { bits }
    }
    #[doc = "Bit 16 - Swap PHA and PHB"]
    #[inline]
    pub fn swap(&self) -> SWAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SWAPR { bits }
    }
    #[doc = "Bit 17 - Index Pin is PHB Pin"]
    #[inline]
    pub fn idxphb(&self) -> IDXPHBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IDXPHBR { bits }
    }
    #[doc = "Bits 20:25 - Maximum Filter"]
    #[inline]
    pub fn maxfilt(&self) -> MAXFILTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAXFILTR { bits }
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
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline]
    pub fn tc0xc0s(&mut self) -> _TC0XC0SW {
        _TC0XC0SW { w: self }
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline]
    pub fn tc1xc1s(&mut self) -> _TC1XC1SW {
        _TC1XC1SW { w: self }
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline]
    pub fn tc2xc2s(&mut self) -> _TC2XC2SW {
        _TC2XC2SW { w: self }
    }
    #[doc = "Bit 8 - Quadrature Decoder Enabled"]
    #[inline]
    pub fn qden(&mut self) -> _QDENW {
        _QDENW { w: self }
    }
    #[doc = "Bit 9 - Position Enabled"]
    #[inline]
    pub fn posen(&mut self) -> _POSENW {
        _POSENW { w: self }
    }
    #[doc = "Bit 10 - Speed Enabled"]
    #[inline]
    pub fn speeden(&mut self) -> _SPEEDENW {
        _SPEEDENW { w: self }
    }
    #[doc = "Bit 11 - Quadrature Decoding Transparent"]
    #[inline]
    pub fn qdtrans(&mut self) -> _QDTRANSW {
        _QDTRANSW { w: self }
    }
    #[doc = "Bit 12 - Edge on PHA Count Mode"]
    #[inline]
    pub fn edgpha(&mut self) -> _EDGPHAW {
        _EDGPHAW { w: self }
    }
    #[doc = "Bit 13 - Inverted PHA"]
    #[inline]
    pub fn inva(&mut self) -> _INVAW {
        _INVAW { w: self }
    }
    #[doc = "Bit 14 - Inverted PHB"]
    #[inline]
    pub fn invb(&mut self) -> _INVBW {
        _INVBW { w: self }
    }
    #[doc = "Bit 15 - Inverted Index"]
    #[inline]
    pub fn invidx(&mut self) -> _INVIDXW {
        _INVIDXW { w: self }
    }
    #[doc = "Bit 16 - Swap PHA and PHB"]
    #[inline]
    pub fn swap(&mut self) -> _SWAPW {
        _SWAPW { w: self }
    }
    #[doc = "Bit 17 - Index Pin is PHB Pin"]
    #[inline]
    pub fn idxphb(&mut self) -> _IDXPHBW {
        _IDXPHBW { w: self }
    }
    #[doc = "Bits 20:25 - Maximum Filter"]
    #[inline]
    pub fn maxfilt(&mut self) -> _MAXFILTW {
        _MAXFILTW { w: self }
    }
}
