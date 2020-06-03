#[doc = "Reader of register PMC_PMMR"]
pub type R = crate::R<u32, super::PMC_PMMR>;
#[doc = "Writer for register PMC_PMMR"]
pub type W = crate::W<u32, super::PMC_PMMR>;
#[doc = "Register PMC_PMMR `reset()`'s with value 0"]
impl crate::ResetValue for super::PMC_PMMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PLLA_MMAX`"]
pub type PLLA_MMAX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PLLA_MMAX`"]
pub struct PLLA_MMAX_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLA_MMAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - PLLA Maximum Allowed Multiplier Value"]
    #[inline(always)]
    pub fn plla_mmax(&self) -> PLLA_MMAX_R {
        PLLA_MMAX_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - PLLA Maximum Allowed Multiplier Value"]
    #[inline(always)]
    pub fn plla_mmax(&mut self) -> PLLA_MMAX_W {
        PLLA_MMAX_W { w: self }
    }
}
