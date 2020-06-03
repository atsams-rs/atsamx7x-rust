#[doc = "Reader of register SDRAMC_OCMS"]
pub type R = crate::R<u32, super::SDRAMC_OCMS>;
#[doc = "Writer for register SDRAMC_OCMS"]
pub type W = crate::W<u32, super::SDRAMC_OCMS>;
#[doc = "Register SDRAMC_OCMS `reset()`'s with value 0"]
impl crate::ResetValue for super::SDRAMC_OCMS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDR_SE`"]
pub type SDR_SE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDR_SE`"]
pub struct SDR_SE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDR_SE_W<'a> {
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
    #[doc = "Bit 0 - SDRAM Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn sdr_se(&self) -> SDR_SE_R {
        SDR_SE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDRAM Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn sdr_se(&mut self) -> SDR_SE_W {
        SDR_SE_W { w: self }
    }
}
