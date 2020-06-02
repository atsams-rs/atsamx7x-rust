#[doc = "Writer for register PWM_SSPUP"]
pub type W = crate::W<u32, super::PWM_SSPUP>;
#[doc = "Register PWM_SSPUP `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_SSPUP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SPRDUP`"]
pub struct SPRDUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPRDUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:23 - Spread Spectrum Limit Value Update"]
    #[inline(always)]
    pub fn sprdup(&mut self) -> SPRDUP_W {
        SPRDUP_W { w: self }
    }
}
