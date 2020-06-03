#[doc = "Reader of register PWM_SSPR"]
pub type R = crate::R<u32, super::PWM_SSPR>;
#[doc = "Writer for register PWM_SSPR"]
pub type W = crate::W<u32, super::PWM_SSPR>;
#[doc = "Register PWM_SSPR `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_SSPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPRD`"]
pub type SPRD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPRD`"]
pub struct SPRD_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Reader of field `SPRDM`"]
pub type SPRDM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPRDM`"]
pub struct SPRDM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRDM_W<'a> {
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
    #[doc = "Bits 0:23 - Spread Spectrum Limit Value"]
    #[inline(always)]
    pub fn sprd(&self) -> SPRD_R {
        SPRD_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - Spread Spectrum Counter Mode"]
    #[inline(always)]
    pub fn sprdm(&self) -> SPRDM_R {
        SPRDM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Spread Spectrum Limit Value"]
    #[inline(always)]
    pub fn sprd(&mut self) -> SPRD_W {
        SPRD_W { w: self }
    }
    #[doc = "Bit 24 - Spread Spectrum Counter Mode"]
    #[inline(always)]
    pub fn sprdm(&mut self) -> SPRDM_W {
        SPRDM_W { w: self }
    }
}
