#[doc = "Writer for register PWM_DMAR"]
pub type W = crate::W<u32, super::PWM_DMAR>;
#[doc = "Register PWM_DMAR `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_DMAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DMADUTY`"]
pub struct DMADUTY_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADUTY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:23 - Duty-Cycle Holding Register for DMA Access"]
    #[inline(always)]
    pub fn dmaduty(&mut self) -> DMADUTY_W {
        DMADUTY_W { w: self }
    }
}
