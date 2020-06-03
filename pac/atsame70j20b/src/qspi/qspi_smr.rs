#[doc = "Reader of register QSPI_SMR"]
pub type R = crate::R<u32, super::QSPI_SMR>;
#[doc = "Writer for register QSPI_SMR"]
pub type W = crate::W<u32, super::QSPI_SMR>;
#[doc = "Register QSPI_SMR `reset()`'s with value 0"]
impl crate::ResetValue for super::QSPI_SMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Scrambling/Unscrambling Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCREN_A {
    #[doc = "0: The scrambling/unscrambling is disabled."]
    DISABLED = 0,
    #[doc = "1: The scrambling/unscrambling is enabled."]
    ENABLED = 1,
}
impl From<SCREN_A> for bool {
    #[inline(always)]
    fn from(variant: SCREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SCREN`"]
pub type SCREN_R = crate::R<bool, SCREN_A>;
impl SCREN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCREN_A {
        match self.bits {
            false => SCREN_A::DISABLED,
            true => SCREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCREN_A::ENABLED
    }
}
#[doc = "Write proxy for field `SCREN`"]
pub struct SCREN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCREN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The scrambling/unscrambling is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCREN_A::DISABLED)
    }
    #[doc = "The scrambling/unscrambling is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCREN_A::ENABLED)
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
#[doc = "Reader of field `RVDIS`"]
pub type RVDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RVDIS`"]
pub struct RVDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RVDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Scrambling/Unscrambling Enable"]
    #[inline(always)]
    pub fn scren(&self) -> SCREN_R {
        SCREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Scrambling/Unscrambling Random Value Disable"]
    #[inline(always)]
    pub fn rvdis(&self) -> RVDIS_R {
        RVDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scrambling/Unscrambling Enable"]
    #[inline(always)]
    pub fn scren(&mut self) -> SCREN_W {
        SCREN_W { w: self }
    }
    #[doc = "Bit 1 - Scrambling/Unscrambling Random Value Disable"]
    #[inline(always)]
    pub fn rvdis(&mut self) -> RVDIS_W {
        RVDIS_W { w: self }
    }
}
