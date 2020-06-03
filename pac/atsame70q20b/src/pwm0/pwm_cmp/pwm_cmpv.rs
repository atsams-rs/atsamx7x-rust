#[doc = "Reader of register PWM_CMPV"]
pub type R = crate::R<u32, super::PWM_CMPV>;
#[doc = "Writer for register PWM_CMPV"]
pub type W = crate::W<u32, super::PWM_CMPV>;
#[doc = "Register PWM_CMPV `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_CMPV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CV`"]
pub type CV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CV`"]
pub struct CV_W<'a> {
    w: &'a mut W,
}
impl<'a> CV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Comparison x Value Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CVM_A {
    #[doc = "0: Compare when counter is incrementing"]
    COMPARE_AT_INCREMENT = 0,
    #[doc = "1: Compare when counter is decrementing"]
    COMPARE_AT_DECREMENT = 1,
}
impl From<CVM_A> for bool {
    #[inline(always)]
    fn from(variant: CVM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CVM`"]
pub type CVM_R = crate::R<bool, CVM_A>;
impl CVM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CVM_A {
        match self.bits {
            false => CVM_A::COMPARE_AT_INCREMENT,
            true => CVM_A::COMPARE_AT_DECREMENT,
        }
    }
    #[doc = "Checks if the value of the field is `COMPARE_AT_INCREMENT`"]
    #[inline(always)]
    pub fn is_compare_at_increment(&self) -> bool {
        *self == CVM_A::COMPARE_AT_INCREMENT
    }
    #[doc = "Checks if the value of the field is `COMPARE_AT_DECREMENT`"]
    #[inline(always)]
    pub fn is_compare_at_decrement(&self) -> bool {
        *self == CVM_A::COMPARE_AT_DECREMENT
    }
}
#[doc = "Write proxy for field `CVM`"]
pub struct CVM_W<'a> {
    w: &'a mut W,
}
impl<'a> CVM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CVM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Compare when counter is incrementing"]
    #[inline(always)]
    pub fn compare_at_increment(self) -> &'a mut W {
        self.variant(CVM_A::COMPARE_AT_INCREMENT)
    }
    #[doc = "Compare when counter is decrementing"]
    #[inline(always)]
    pub fn compare_at_decrement(self) -> &'a mut W {
        self.variant(CVM_A::COMPARE_AT_DECREMENT)
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
impl R {
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    pub fn cvm(&self) -> CVM_R {
        CVM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    pub fn cv(&mut self) -> CV_W {
        CV_W { w: self }
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    pub fn cvm(&mut self) -> CVM_W {
        CVM_W { w: self }
    }
}
