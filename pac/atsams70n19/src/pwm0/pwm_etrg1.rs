#[doc = "Reader of register PWM_ETRG1"]
pub type R = crate::R<u32, super::PWM_ETRG1>;
#[doc = "Writer for register PWM_ETRG1"]
pub type W = crate::W<u32, super::PWM_ETRG1>;
#[doc = "Register PWM_ETRG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_ETRG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAXCNT`"]
pub type MAXCNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MAXCNT`"]
pub struct MAXCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "External Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGMODE_A {
    #[doc = "0: External trigger is not enabled."]
    OFF = 0,
    #[doc = "1: External PWM Reset Mode"]
    MODE1 = 1,
    #[doc = "2: External PWM Start Mode"]
    MODE2 = 2,
    #[doc = "3: Cycle-by-cycle Duty Mode"]
    MODE3 = 3,
}
impl From<TRGMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRGMODE`"]
pub type TRGMODE_R = crate::R<u8, TRGMODE_A>;
impl TRGMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGMODE_A {
        match self.bits {
            0 => TRGMODE_A::OFF,
            1 => TRGMODE_A::MODE1,
            2 => TRGMODE_A::MODE2,
            3 => TRGMODE_A::MODE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == TRGMODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == TRGMODE_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == TRGMODE_A::MODE2
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == TRGMODE_A::MODE3
    }
}
#[doc = "Write proxy for field `TRGMODE`"]
pub struct TRGMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "External trigger is not enabled."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(TRGMODE_A::OFF)
    }
    #[doc = "External PWM Reset Mode"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(TRGMODE_A::MODE1)
    }
    #[doc = "External PWM Start Mode"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(TRGMODE_A::MODE2)
    }
    #[doc = "Cycle-by-cycle Duty Mode"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut W {
        self.variant(TRGMODE_A::MODE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEDGE_A {
    #[doc = "0: TRGMODE = 1: TRGINx event detection on falling edge.TRGMODE = 2, 3: TRGINx active level is 0"]
    FALLING_ZERO = 0,
    #[doc = "1: TRGMODE = 1: TRGINx event detection on rising edge.TRGMODE = 2, 3: TRGINx active level is 1"]
    RISING_ONE = 1,
}
impl From<TRGEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRGEDGE`"]
pub type TRGEDGE_R = crate::R<bool, TRGEDGE_A>;
impl TRGEDGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEDGE_A {
        match self.bits {
            false => TRGEDGE_A::FALLING_ZERO,
            true => TRGEDGE_A::RISING_ONE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_ZERO`"]
    #[inline(always)]
    pub fn is_falling_zero(&self) -> bool {
        *self == TRGEDGE_A::FALLING_ZERO
    }
    #[doc = "Checks if the value of the field is `RISING_ONE`"]
    #[inline(always)]
    pub fn is_rising_one(&self) -> bool {
        *self == TRGEDGE_A::RISING_ONE
    }
}
#[doc = "Write proxy for field `TRGEDGE`"]
pub struct TRGEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGEDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGEDGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TRGMODE = 1: TRGINx event detection on falling edge.TRGMODE = 2, 3: TRGINx active level is 0"]
    #[inline(always)]
    pub fn falling_zero(self) -> &'a mut W {
        self.variant(TRGEDGE_A::FALLING_ZERO)
    }
    #[doc = "TRGMODE = 1: TRGINx event detection on rising edge.TRGMODE = 2, 3: TRGINx active level is 1"]
    #[inline(always)]
    pub fn rising_one(self) -> &'a mut W {
        self.variant(TRGEDGE_A::RISING_ONE)
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
#[doc = "Reader of field `TRGFILT`"]
pub type TRGFILT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRGFILT`"]
pub struct TRGFILT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGFILT_W<'a> {
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
#[doc = "Reader of field `TRGSRC`"]
pub type TRGSRC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRGSRC`"]
pub struct TRGSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSRC_W<'a> {
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
#[doc = "Reader of field `RFEN`"]
pub type RFEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFEN`"]
pub struct RFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RFEN_W<'a> {
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
    #[doc = "Bits 0:23 - Maximum Counter value"]
    #[inline(always)]
    pub fn maxcnt(&self) -> MAXCNT_R {
        MAXCNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:25 - External Trigger Mode"]
    #[inline(always)]
    pub fn trgmode(&self) -> TRGMODE_R {
        TRGMODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Edge Selection"]
    #[inline(always)]
    pub fn trgedge(&self) -> TRGEDGE_R {
        TRGEDGE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Filtered input"]
    #[inline(always)]
    pub fn trgfilt(&self) -> TRGFILT_R {
        TRGFILT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Trigger Source"]
    #[inline(always)]
    pub fn trgsrc(&self) -> TRGSRC_R {
        TRGSRC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Recoverable Fault Enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RFEN_R {
        RFEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Maximum Counter value"]
    #[inline(always)]
    pub fn maxcnt(&mut self) -> MAXCNT_W {
        MAXCNT_W { w: self }
    }
    #[doc = "Bits 24:25 - External Trigger Mode"]
    #[inline(always)]
    pub fn trgmode(&mut self) -> TRGMODE_W {
        TRGMODE_W { w: self }
    }
    #[doc = "Bit 28 - Edge Selection"]
    #[inline(always)]
    pub fn trgedge(&mut self) -> TRGEDGE_W {
        TRGEDGE_W { w: self }
    }
    #[doc = "Bit 29 - Filtered input"]
    #[inline(always)]
    pub fn trgfilt(&mut self) -> TRGFILT_W {
        TRGFILT_W { w: self }
    }
    #[doc = "Bit 30 - Trigger Source"]
    #[inline(always)]
    pub fn trgsrc(&mut self) -> TRGSRC_W {
        TRGSRC_W { w: self }
    }
    #[doc = "Bit 31 - Recoverable Fault Enable"]
    #[inline(always)]
    pub fn rfen(&mut self) -> RFEN_W {
        RFEN_W { w: self }
    }
}
