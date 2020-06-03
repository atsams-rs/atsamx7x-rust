#[doc = "Reader of register ACC_ACR"]
pub type R = crate::R<u32, super::ACC_ACR>;
#[doc = "Writer for register ACC_ACR"]
pub type W = crate::W<u32, super::ACC_ACR>;
#[doc = "Register ACC_ACR `reset()`'s with value 0"]
impl crate::ResetValue for super::ACC_ACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Current Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISEL_A {
    #[doc = "0: Low-power option."]
    LOPW = 0,
    #[doc = "1: High-speed option."]
    HISP = 1,
}
impl From<ISEL_A> for bool {
    #[inline(always)]
    fn from(variant: ISEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ISEL`"]
pub type ISEL_R = crate::R<bool, ISEL_A>;
impl ISEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISEL_A {
        match self.bits {
            false => ISEL_A::LOPW,
            true => ISEL_A::HISP,
        }
    }
    #[doc = "Checks if the value of the field is `LOPW`"]
    #[inline(always)]
    pub fn is_lopw(&self) -> bool {
        *self == ISEL_A::LOPW
    }
    #[doc = "Checks if the value of the field is `HISP`"]
    #[inline(always)]
    pub fn is_hisp(&self) -> bool {
        *self == ISEL_A::HISP
    }
}
#[doc = "Write proxy for field `ISEL`"]
pub struct ISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low-power option."]
    #[inline(always)]
    pub fn lopw(self) -> &'a mut W {
        self.variant(ISEL_A::LOPW)
    }
    #[doc = "High-speed option."]
    #[inline(always)]
    pub fn hisp(self) -> &'a mut W {
        self.variant(ISEL_A::HISP)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `HYST`"]
pub type HYST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HYST`"]
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Current Selection"]
    #[inline(always)]
    pub fn isel(&self) -> ISEL_R {
        ISEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Hysteresis Selection"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Current Selection"]
    #[inline(always)]
    pub fn isel(&mut self) -> ISEL_W {
        ISEL_W { w: self }
    }
    #[doc = "Bits 1:2 - Hysteresis Selection"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
}
