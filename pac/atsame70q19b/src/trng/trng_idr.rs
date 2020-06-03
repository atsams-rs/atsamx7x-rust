#[doc = "Writer for register TRNG_IDR"]
pub type W = crate::W<u32, super::TRNG_IDR>;
#[doc = "Register TRNG_IDR `reset()`'s with value 0"]
impl crate::ResetValue for super::TRNG_IDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DATRDY`"]
pub struct DATRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> DATRDY_W<'a> {
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
    #[doc = "Bit 0 - Data Ready Interrupt Disable"]
    #[inline(always)]
    pub fn datrdy(&mut self) -> DATRDY_W {
        DATRDY_W { w: self }
    }
}
