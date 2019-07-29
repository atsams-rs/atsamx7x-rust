#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWM_ETRG1 {
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
#[doc = r"Reader of the field"]
pub type MAXCNT_R = crate::FR<u32, u32>;
#[doc = r"Proxy"]
pub struct _MAXCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _MAXCNTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
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
impl crate::ToBits<u8> for TRGMODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TRGMODER::OFF => 0,
            TRGMODER::MODE1 => 1,
            TRGMODER::MODE2 => 2,
            TRGMODER::MODE3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TRGMODE_R = crate::FR<u8, TRGMODER>;
impl TRGMODE_R {
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == TRGMODER::OFF
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == TRGMODER::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == TRGMODER::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == TRGMODER::MODE3
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRGMODEW::OFF => 0,
            TRGMODEW::MODE1 => 1,
            TRGMODEW::MODE2 => 2,
            TRGMODEW::MODE3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TRGMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGMODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "External trigger is not enabled."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(TRGMODEW::OFF)
    }
    #[doc = "External PWM Reset Mode"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(TRGMODEW::MODE1)
    }
    #[doc = "External PWM Start Mode"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(TRGMODEW::MODE2)
    }
    #[doc = "Cycle-by-cycle Duty Mode"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(TRGMODEW::MODE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
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
impl crate::ToBits<bool> for TRGEDGER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TRGEDGER::FALLING_ZERO => false,
            TRGEDGER::RISING_ONE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TRGEDGE_R = crate::FR<bool, TRGEDGER>;
impl TRGEDGE_R {
    #[doc = "Checks if the value of the field is `FALLING_ZERO`"]
    #[inline(always)]
    pub fn is_falling_zero(&self) -> bool {
        *self == TRGEDGER::FALLING_ZERO
    }
    #[doc = "Checks if the value of the field is `RISING_ONE`"]
    #[inline(always)]
    pub fn is_rising_one(&self) -> bool {
        *self == TRGEDGER::RISING_ONE
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
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TRGEDGEW::FALLING_ZERO => false,
            TRGEDGEW::RISING_ONE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TRGEDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGEDGEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGEDGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TRGMODE = 1: TRGINx event detection on falling edge.TRGMODE = 2, 3: TRGINx active level is 0"]
    #[inline(always)]
    pub fn falling_zero(self) -> &'a mut W {
        self.variant(TRGEDGEW::FALLING_ZERO)
    }
    #[doc = "TRGMODE = 1: TRGINx event detection on rising edge.TRGMODE = 2, 3: TRGINx active level is 1"]
    #[inline(always)]
    pub fn rising_one(self) -> &'a mut W {
        self.variant(TRGEDGEW::RISING_ONE)
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
#[doc = r"Reader of the field"]
pub type TRGFILT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TRGFILTW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGFILTW<'a> {
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
#[doc = r"Reader of the field"]
pub type TRGSRC_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TRGSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGSRCW<'a> {
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
#[doc = r"Reader of the field"]
pub type RFEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RFENW<'a> {
    w: &'a mut W,
}
impl<'a> _RFENW<'a> {
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
    #[doc = "Bits 0:23 - Maximum Counter value"]
    #[inline(always)]
    pub fn maxcnt(&self) -> MAXCNT_R {
        MAXCNT_R::new((self.bits() & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:25 - External Trigger Mode"]
    #[inline(always)]
    pub fn trgmode(&self) -> TRGMODE_R {
        TRGMODE_R::new(((self.bits() >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Edge Selection"]
    #[inline(always)]
    pub fn trgedge(&self) -> TRGEDGE_R {
        TRGEDGE_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Filtered input"]
    #[inline(always)]
    pub fn trgfilt(&self) -> TRGFILT_R {
        TRGFILT_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Trigger Source"]
    #[inline(always)]
    pub fn trgsrc(&self) -> TRGSRC_R {
        TRGSRC_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Recoverable Fault Enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RFEN_R {
        RFEN_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:23 - Maximum Counter value"]
    #[inline(always)]
    pub fn maxcnt(&mut self) -> _MAXCNTW {
        _MAXCNTW { w: self }
    }
    #[doc = "Bits 24:25 - External Trigger Mode"]
    #[inline(always)]
    pub fn trgmode(&mut self) -> _TRGMODEW {
        _TRGMODEW { w: self }
    }
    #[doc = "Bit 28 - Edge Selection"]
    #[inline(always)]
    pub fn trgedge(&mut self) -> _TRGEDGEW {
        _TRGEDGEW { w: self }
    }
    #[doc = "Bit 29 - Filtered input"]
    #[inline(always)]
    pub fn trgfilt(&mut self) -> _TRGFILTW {
        _TRGFILTW { w: self }
    }
    #[doc = "Bit 30 - Trigger Source"]
    #[inline(always)]
    pub fn trgsrc(&mut self) -> _TRGSRCW {
        _TRGSRCW { w: self }
    }
    #[doc = "Bit 31 - Recoverable Fault Enable"]
    #[inline(always)]
    pub fn rfen(&mut self) -> _RFENW {
        _RFENW { w: self }
    }
}
