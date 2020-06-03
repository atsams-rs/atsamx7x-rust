#[doc = "Reader of register SDRAMC_CFR1"]
pub type R = crate::R<u32, super::SDRAMC_CFR1>;
#[doc = "Writer for register SDRAMC_CFR1"]
pub type W = crate::W<u32, super::SDRAMC_CFR1>;
#[doc = "Register SDRAMC_CFR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SDRAMC_CFR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TMRD`"]
pub type TMRD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TMRD`"]
pub struct TMRD_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Support Unaligned Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNAL_A {
    #[doc = "0: Unaligned access is not supported."]
    UNSUPPORTED = 0,
    #[doc = "1: Unaligned access is supported."]
    SUPPORTED = 1,
}
impl From<UNAL_A> for bool {
    #[inline(always)]
    fn from(variant: UNAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UNAL`"]
pub type UNAL_R = crate::R<bool, UNAL_A>;
impl UNAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNAL_A {
        match self.bits {
            false => UNAL_A::UNSUPPORTED,
            true => UNAL_A::SUPPORTED,
        }
    }
    #[doc = "Checks if the value of the field is `UNSUPPORTED`"]
    #[inline(always)]
    pub fn is_unsupported(&self) -> bool {
        *self == UNAL_A::UNSUPPORTED
    }
    #[doc = "Checks if the value of the field is `SUPPORTED`"]
    #[inline(always)]
    pub fn is_supported(&self) -> bool {
        *self == UNAL_A::SUPPORTED
    }
}
#[doc = "Write proxy for field `UNAL`"]
pub struct UNAL_W<'a> {
    w: &'a mut W,
}
impl<'a> UNAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Unaligned access is not supported."]
    #[inline(always)]
    pub fn unsupported(self) -> &'a mut W {
        self.variant(UNAL_A::UNSUPPORTED)
    }
    #[doc = "Unaligned access is supported."]
    #[inline(always)]
    pub fn supported(self) -> &'a mut W {
        self.variant(UNAL_A::SUPPORTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Load Mode Register Command to Active or Refresh Command"]
    #[inline(always)]
    pub fn tmrd(&self) -> TMRD_R {
        TMRD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Support Unaligned Access"]
    #[inline(always)]
    pub fn unal(&self) -> UNAL_R {
        UNAL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Load Mode Register Command to Active or Refresh Command"]
    #[inline(always)]
    pub fn tmrd(&mut self) -> TMRD_W {
        TMRD_W { w: self }
    }
    #[doc = "Bit 8 - Support Unaligned Access"]
    #[inline(always)]
    pub fn unal(&mut self) -> UNAL_W {
        UNAL_W { w: self }
    }
}
