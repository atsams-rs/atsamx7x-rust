#[doc = "Reader of register US_IDTTX"]
pub type R = crate::R<u32, super::US_IDTTX>;
#[doc = "Writer for register US_IDTTX"]
pub type W = crate::W<u32, super::US_IDTTX>;
#[doc = "Register US_IDTTX `reset()`'s with value 0"]
impl crate::ResetValue for super::US_IDTTX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDTTX`"]
pub type IDTTX_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IDTTX`"]
pub struct IDTTX_W<'a> {
    w: &'a mut W,
}
impl<'a> IDTTX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Transmission (comm_type = 1 mode only)"]
    #[inline(always)]
    pub fn idttx(&self) -> IDTTX_R {
        IDTTX_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Transmission (comm_type = 1 mode only)"]
    #[inline(always)]
    pub fn idttx(&mut self) -> IDTTX_W {
        IDTTX_W { w: self }
    }
}
