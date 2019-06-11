#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWM_ETRG1 {
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
#[doc = r" Value of the field"]
pub struct MAXCNTR {
    bits: u32,
}
impl MAXCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `TRGMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGMODER {
    #[doc = "External trigger is not enabled."]
    OFF,
    #[doc = "External PWM Reset Mode"]
    MODE1,
    #[doc = "External PWM Start Mode"]
    MODE2,
    #[doc = "Cycle-by-cycle Duty Mode"]
    MODE3,
}
impl TRGMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRGMODER::OFF => 0,
            TRGMODER::MODE1 => 1,
            TRGMODER::MODE2 => 2,
            TRGMODER::MODE3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRGMODER {
        match value {
            0 => TRGMODER::OFF,
            1 => TRGMODER::MODE1,
            2 => TRGMODER::MODE2,
            3 => TRGMODER::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == TRGMODER::OFF
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline]
    pub fn is_mode1(&self) -> bool {
        *self == TRGMODER::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline]
    pub fn is_mode2(&self) -> bool {
        *self == TRGMODER::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline]
    pub fn is_mode3(&self) -> bool {
        *self == TRGMODER::MODE3
    }
}
#[doc = "Possible values of the field `TRGEDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEDGER {
    #[doc = "TRGMODE = 1: TRGINx event detection on falling edge.TRGMODE = 2, 3: TRGINx active level is 0"]
    FALLING_ZERO,
    #[doc = "TRGMODE = 1: TRGINx event detection on rising edge.TRGMODE = 2, 3: TRGINx active level is 1"]
    RISING_ONE,
}
impl TRGEDGER {
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
            TRGEDGER::FALLING_ZERO => false,
            TRGEDGER::RISING_ONE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRGEDGER {
        match value {
            false => TRGEDGER::FALLING_ZERO,
            true => TRGEDGER::RISING_ONE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_ZERO`"]
    #[inline]
    pub fn is_falling_zero(&self) -> bool {
        *self == TRGEDGER::FALLING_ZERO
    }
    #[doc = "Checks if the value of the field is `RISING_ONE`"]
    #[inline]
    pub fn is_rising_one(&self) -> bool {
        *self == TRGEDGER::RISING_ONE
    }
}
#[doc = r" Value of the field"]
pub struct TRGFILTR {
    bits: bool,
}
impl TRGFILTR {
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
pub struct TRGSRCR {
    bits: bool,
}
impl TRGSRCR {
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
pub struct RFENR {
    bits: bool,
}
impl RFENR {
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
#[doc = r" Proxy"]
pub struct _MAXCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _MAXCNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRGMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGMODEW {
    #[doc = "External trigger is not enabled."]
    OFF,
    #[doc = "External PWM Reset Mode"]
    MODE1,
    #[doc = "External PWM Start Mode"]
    MODE2,
    #[doc = "Cycle-by-cycle Duty Mode"]
    MODE3,
}
impl TRGMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRGMODEW::OFF => 0,
            TRGMODEW::MODE1 => 1,
            TRGMODEW::MODE2 => 2,
            TRGMODEW::MODE3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRGMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRGMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External trigger is not enabled."]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(TRGMODEW::OFF)
    }
    #[doc = "External PWM Reset Mode"]
    #[inline]
    pub fn mode1(self) -> &'a mut W {
        self.variant(TRGMODEW::MODE1)
    }
    #[doc = "External PWM Start Mode"]
    #[inline]
    pub fn mode2(self) -> &'a mut W {
        self.variant(TRGMODEW::MODE2)
    }
    #[doc = "Cycle-by-cycle Duty Mode"]
    #[inline]
    pub fn mode3(self) -> &'a mut W {
        self.variant(TRGMODEW::MODE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRGEDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEDGEW {
    #[doc = "TRGMODE = 1: TRGINx event detection on falling edge.TRGMODE = 2, 3: TRGINx active level is 0"]
    FALLING_ZERO,
    #[doc = "TRGMODE = 1: TRGINx event detection on rising edge.TRGMODE = 2, 3: TRGINx active level is 1"]
    RISING_ONE,
}
impl TRGEDGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRGEDGEW::FALLING_ZERO => false,
            TRGEDGEW::RISING_ONE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRGEDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGEDGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRGEDGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TRGMODE = 1: TRGINx event detection on falling edge.TRGMODE = 2, 3: TRGINx active level is 0"]
    #[inline]
    pub fn falling_zero(self) -> &'a mut W {
        self.variant(TRGEDGEW::FALLING_ZERO)
    }
    #[doc = "TRGMODE = 1: TRGINx event detection on rising edge.TRGMODE = 2, 3: TRGINx active level is 1"]
    #[inline]
    pub fn rising_one(self) -> &'a mut W {
        self.variant(TRGEDGEW::RISING_ONE)
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
#[doc = r" Proxy"]
pub struct _TRGFILTW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGFILTW<'a> {
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
#[doc = r" Proxy"]
pub struct _TRGSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGSRCW<'a> {
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
#[doc = r" Proxy"]
pub struct _RFENW<'a> {
    w: &'a mut W,
}
impl<'a> _RFENW<'a> {
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
    #[doc = "Bits 0:23 - Maximum Counter value"]
    #[inline]
    pub fn maxcnt(&self) -> MAXCNTR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        MAXCNTR { bits }
    }
    #[doc = "Bits 24:25 - External Trigger Mode"]
    #[inline]
    pub fn trgmode(&self) -> TRGMODER {
        TRGMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Edge Selection"]
    #[inline]
    pub fn trgedge(&self) -> TRGEDGER {
        TRGEDGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Filtered input"]
    #[inline]
    pub fn trgfilt(&self) -> TRGFILTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TRGFILTR { bits }
    }
    #[doc = "Bit 30 - Trigger Source"]
    #[inline]
    pub fn trgsrc(&self) -> TRGSRCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TRGSRCR { bits }
    }
    #[doc = "Bit 31 - Recoverable Fault Enable"]
    #[inline]
    pub fn rfen(&self) -> RFENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RFENR { bits }
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
    #[doc = "Bits 0:23 - Maximum Counter value"]
    #[inline]
    pub fn maxcnt(&mut self) -> _MAXCNTW {
        _MAXCNTW { w: self }
    }
    #[doc = "Bits 24:25 - External Trigger Mode"]
    #[inline]
    pub fn trgmode(&mut self) -> _TRGMODEW {
        _TRGMODEW { w: self }
    }
    #[doc = "Bit 28 - Edge Selection"]
    #[inline]
    pub fn trgedge(&mut self) -> _TRGEDGEW {
        _TRGEDGEW { w: self }
    }
    #[doc = "Bit 29 - Filtered input"]
    #[inline]
    pub fn trgfilt(&mut self) -> _TRGFILTW {
        _TRGFILTW { w: self }
    }
    #[doc = "Bit 30 - Trigger Source"]
    #[inline]
    pub fn trgsrc(&mut self) -> _TRGSRCW {
        _TRGSRCW { w: self }
    }
    #[doc = "Bit 31 - Recoverable Fault Enable"]
    #[inline]
    pub fn rfen(&mut self) -> _RFENW {
        _RFENW { w: self }
    }
}
