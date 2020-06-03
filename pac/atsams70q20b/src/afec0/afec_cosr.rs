#[doc = "Reader of register AFEC_COSR"]
pub type R = crate::R<u32, super::AFEC_COSR>;
#[doc = "Writer for register AFEC_COSR"]
pub type W = crate::W<u32, super::AFEC_COSR>;
#[doc = "Register AFEC_COSR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFEC_COSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSEL`"]
pub type CSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSEL`"]
pub struct CSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Sample & Hold unit Correction Select"]
    #[inline(always)]
    pub fn csel(&self) -> CSEL_R {
        CSEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sample & Hold unit Correction Select"]
    #[inline(always)]
    pub fn csel(&mut self) -> CSEL_W {
        CSEL_W { w: self }
    }
}
