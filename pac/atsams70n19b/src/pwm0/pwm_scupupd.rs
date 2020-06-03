#[doc = "Writer for register PWM_SCUPUPD"]
pub type W = crate::W<u32, super::PWM_SCUPUPD>;
#[doc = "Register PWM_SCUPUPD `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_SCUPUPD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `UPRUPD`"]
pub struct UPRUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> UPRUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:3 - Update Period Update"]
    #[inline(always)]
    pub fn uprupd(&mut self) -> UPRUPD_W {
        UPRUPD_W { w: self }
    }
}
