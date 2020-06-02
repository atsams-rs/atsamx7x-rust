#[doc = "Writer for register PWM_FCR"]
pub type W = crate::W<u32, super::PWM_FCR>;
#[doc = "Register PWM_FCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `FCLR`"]
pub struct FCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Fault Clear"]
    #[inline(always)]
    pub fn fclr(&mut self) -> FCLR_W {
        FCLR_W { w: self }
    }
}
