#[doc = "Writer for register PMC_FOCR"]
pub type W = crate::W<u32, super::PMC_FOCR>;
#[doc = "Register PMC_FOCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PMC_FOCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `FOCLR`"]
pub struct FOCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FOCLR_W<'a> {
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
    #[doc = "Bit 0 - Fault Output Clear"]
    #[inline(always)]
    pub fn foclr(&mut self) -> FOCLR_W {
        FOCLR_W { w: self }
    }
}
