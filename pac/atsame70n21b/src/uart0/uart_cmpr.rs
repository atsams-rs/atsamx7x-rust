#[doc = "Reader of register UART_CMPR"]
pub type R = crate::R<u32, super::UART_CMPR>;
#[doc = "Writer for register UART_CMPR"]
pub type W = crate::W<u32, super::UART_CMPR>;
#[doc = "Register UART_CMPR `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_CMPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VAL1`"]
pub type VAL1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VAL1`"]
pub struct VAL1_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPMODE_A {
    #[doc = "0: Any character is received and comparison function drives CMP flag."]
    FLAG_ONLY = 0,
    #[doc = "1: Comparison condition must be met to start reception."]
    START_CONDITION = 1,
}
impl From<CMPMODE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CMPMODE`"]
pub type CMPMODE_R = crate::R<bool, CMPMODE_A>;
impl CMPMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMODE_A {
        match self.bits {
            false => CMPMODE_A::FLAG_ONLY,
            true => CMPMODE_A::START_CONDITION,
        }
    }
    #[doc = "Checks if the value of the field is `FLAG_ONLY`"]
    #[inline(always)]
    pub fn is_flag_only(&self) -> bool {
        *self == CMPMODE_A::FLAG_ONLY
    }
    #[doc = "Checks if the value of the field is `START_CONDITION`"]
    #[inline(always)]
    pub fn is_start_condition(&self) -> bool {
        *self == CMPMODE_A::START_CONDITION
    }
}
#[doc = "Write proxy for field `CMPMODE`"]
pub struct CMPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Any character is received and comparison function drives CMP flag."]
    #[inline(always)]
    pub fn flag_only(self) -> &'a mut W {
        self.variant(CMPMODE_A::FLAG_ONLY)
    }
    #[doc = "Comparison condition must be met to start reception."]
    #[inline(always)]
    pub fn start_condition(self) -> &'a mut W {
        self.variant(CMPMODE_A::START_CONDITION)
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
#[doc = "Reader of field `CMPPAR`"]
pub type CMPPAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPPAR`"]
pub struct CMPPAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPPAR_W<'a> {
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
#[doc = "Reader of field `VAL2`"]
pub type VAL2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VAL2`"]
pub struct VAL2_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - First Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val1(&self) -> VAL1_R {
        VAL1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CMPMODE_R {
        CMPMODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Compare Parity"]
    #[inline(always)]
    pub fn cmppar(&self) -> CMPPAR_R {
        CMPPAR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Second Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val2(&self) -> VAL2_R {
        VAL2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - First Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val1(&mut self) -> VAL1_W {
        VAL1_W { w: self }
    }
    #[doc = "Bit 12 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&mut self) -> CMPMODE_W {
        CMPMODE_W { w: self }
    }
    #[doc = "Bit 14 - Compare Parity"]
    #[inline(always)]
    pub fn cmppar(&mut self) -> CMPPAR_W {
        CMPPAR_W { w: self }
    }
    #[doc = "Bits 16:23 - Second Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val2(&mut self) -> VAL2_W {
        VAL2_W { w: self }
    }
}
