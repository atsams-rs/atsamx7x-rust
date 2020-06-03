#[doc = "Reader of register TC_EMR"]
pub type R = crate::R<u32, super::TC_EMR>;
#[doc = "Writer for register TC_EMR"]
pub type W = crate::W<u32, super::TC_EMR>;
#[doc = "Register TC_EMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TC_EMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Trigger Source for Input A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGSRCA_A {
    #[doc = "0: The trigger/capture input A is driven by external pin TIOAx"]
    EXTERNAL_TIOAX = 0,
    #[doc = "1: The trigger/capture input A is driven internally by PWMx"]
    PWMX = 1,
}
impl From<TRIGSRCA_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSRCA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRIGSRCA`"]
pub type TRIGSRCA_R = crate::R<u8, TRIGSRCA_A>;
impl TRIGSRCA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRIGSRCA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRIGSRCA_A::EXTERNAL_TIOAX),
            1 => Val(TRIGSRCA_A::PWMX),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_TIOAX`"]
    #[inline(always)]
    pub fn is_external_tioax(&self) -> bool {
        *self == TRIGSRCA_A::EXTERNAL_TIOAX
    }
    #[doc = "Checks if the value of the field is `PWMX`"]
    #[inline(always)]
    pub fn is_pwmx(&self) -> bool {
        *self == TRIGSRCA_A::PWMX
    }
}
#[doc = "Write proxy for field `TRIGSRCA`"]
pub struct TRIGSRCA_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGSRCA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGSRCA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The trigger/capture input A is driven by external pin TIOAx"]
    #[inline(always)]
    pub fn external_tioax(self) -> &'a mut W {
        self.variant(TRIGSRCA_A::EXTERNAL_TIOAX)
    }
    #[doc = "The trigger/capture input A is driven internally by PWMx"]
    #[inline(always)]
    pub fn pwmx(self) -> &'a mut W {
        self.variant(TRIGSRCA_A::PWMX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Trigger Source for Input B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGSRCB_A {
    #[doc = "0: The trigger/capture input B is driven by external pin TIOBx"]
    EXTERNAL_TIOBX = 0,
    #[doc = "1: For TC0 to TC10: The trigger/capture input B is driven internally by the comparator output (see Figure 7-16) of the PWMx.For TC11: The trigger/capture input B is driven internally by the GTSUCOMP signal of the Ethernet MAC (GMAC)."]
    PWMX = 1,
}
impl From<TRIGSRCB_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSRCB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRIGSRCB`"]
pub type TRIGSRCB_R = crate::R<u8, TRIGSRCB_A>;
impl TRIGSRCB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRIGSRCB_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRIGSRCB_A::EXTERNAL_TIOBX),
            1 => Val(TRIGSRCB_A::PWMX),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_TIOBX`"]
    #[inline(always)]
    pub fn is_external_tiobx(&self) -> bool {
        *self == TRIGSRCB_A::EXTERNAL_TIOBX
    }
    #[doc = "Checks if the value of the field is `PWMX`"]
    #[inline(always)]
    pub fn is_pwmx(&self) -> bool {
        *self == TRIGSRCB_A::PWMX
    }
}
#[doc = "Write proxy for field `TRIGSRCB`"]
pub struct TRIGSRCB_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGSRCB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGSRCB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The trigger/capture input B is driven by external pin TIOBx"]
    #[inline(always)]
    pub fn external_tiobx(self) -> &'a mut W {
        self.variant(TRIGSRCB_A::EXTERNAL_TIOBX)
    }
    #[doc = "For TC0 to TC10: The trigger/capture input B is driven internally by the comparator output (see Figure 7-16) of the PWMx.For TC11: The trigger/capture input B is driven internally by the GTSUCOMP signal of the Ethernet MAC (GMAC)."]
    #[inline(always)]
    pub fn pwmx(self) -> &'a mut W {
        self.variant(TRIGSRCB_A::PWMX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `NODIVCLK`"]
pub type NODIVCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NODIVCLK`"]
pub struct NODIVCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> NODIVCLK_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - Trigger Source for Input A"]
    #[inline(always)]
    pub fn trigsrca(&self) -> TRIGSRCA_R {
        TRIGSRCA_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Trigger Source for Input B"]
    #[inline(always)]
    pub fn trigsrcb(&self) -> TRIGSRCB_R {
        TRIGSRCB_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - No Divided Clock"]
    #[inline(always)]
    pub fn nodivclk(&self) -> NODIVCLK_R {
        NODIVCLK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trigger Source for Input A"]
    #[inline(always)]
    pub fn trigsrca(&mut self) -> TRIGSRCA_W {
        TRIGSRCA_W { w: self }
    }
    #[doc = "Bits 4:5 - Trigger Source for Input B"]
    #[inline(always)]
    pub fn trigsrcb(&mut self) -> TRIGSRCB_W {
        TRIGSRCB_W { w: self }
    }
    #[doc = "Bit 8 - No Divided Clock"]
    #[inline(always)]
    pub fn nodivclk(&mut self) -> NODIVCLK_W {
        NODIVCLK_W { w: self }
    }
}
