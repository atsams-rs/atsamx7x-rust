#[doc = "Reader of register US_IDTRX"]
pub type R = crate::R<u32, super::US_IDTRX>;
#[doc = "Writer for register US_IDTRX"]
pub type W = crate::W<u32, super::US_IDTRX>;
#[doc = "Register US_IDTRX `reset()`'s with value 0"]
impl crate::ResetValue for super::US_IDTRX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDTRX`"]
pub type IDTRX_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IDTRX`"]
pub struct IDTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> IDTRX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Reception (comm_type = 1 mode only)"]
    #[inline(always)]
    pub fn idtrx(&self) -> IDTRX_R {
        IDTRX_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Reception (comm_type = 1 mode only)"]
    #[inline(always)]
    pub fn idtrx(&mut self) -> IDTRX_W {
        IDTRX_W { w: self }
    }
}
