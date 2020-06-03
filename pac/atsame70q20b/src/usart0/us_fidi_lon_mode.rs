#[doc = "Reader of register US_FIDI_LON_MODE"]
pub type R = crate::R<u32, super::US_FIDI_LON_MODE>;
#[doc = "Writer for register US_FIDI_LON_MODE"]
pub type W = crate::W<u32, super::US_FIDI_LON_MODE>;
#[doc = "Register US_FIDI_LON_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::US_FIDI_LON_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BETA2`"]
pub type BETA2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BETA2`"]
pub struct BETA2_W<'a> {
    w: &'a mut W,
}
impl<'a> BETA2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - LON BETA2 Length"]
    #[inline(always)]
    pub fn beta2(&self) -> BETA2_R {
        BETA2_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON BETA2 Length"]
    #[inline(always)]
    pub fn beta2(&mut self) -> BETA2_W {
        BETA2_W { w: self }
    }
}
