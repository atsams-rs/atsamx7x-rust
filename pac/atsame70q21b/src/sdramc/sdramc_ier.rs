#[doc = "Writer for register SDRAMC_IER"]
pub type W = crate::W<u32, super::SDRAMC_IER>;
#[doc = "Register SDRAMC_IER `reset()`'s with value 0"]
impl crate::ResetValue for super::SDRAMC_IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RES`"]
pub struct RES_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Refresh Error Interrupt Enable"]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W {
        RES_W { w: self }
    }
}
