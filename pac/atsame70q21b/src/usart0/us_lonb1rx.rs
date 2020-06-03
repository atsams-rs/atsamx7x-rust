#[doc = "Reader of register US_LONB1RX"]
pub type R = crate::R<u32, super::US_LONB1RX>;
#[doc = "Writer for register US_LONB1RX"]
pub type W = crate::W<u32, super::US_LONB1RX>;
#[doc = "Register US_LONB1RX `reset()`'s with value 0"]
impl crate::ResetValue for super::US_LONB1RX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BETA1RX`"]
pub type BETA1RX_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BETA1RX`"]
pub struct BETA1RX_W<'a> {
    w: &'a mut W,
}
impl<'a> BETA1RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - LON Beta1 Length after Reception"]
    #[inline(always)]
    pub fn beta1rx(&self) -> BETA1RX_R {
        BETA1RX_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON Beta1 Length after Reception"]
    #[inline(always)]
    pub fn beta1rx(&mut self) -> BETA1RX_W {
        BETA1RX_W { w: self }
    }
}
