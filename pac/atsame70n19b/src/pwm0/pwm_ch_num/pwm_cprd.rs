#[doc = "Reader of register PWM_CPRD"]
pub type R = crate::R<u32, super::PWM_CPRD>;
#[doc = "Writer for register PWM_CPRD"]
pub type W = crate::W<u32, super::PWM_CPRD>;
#[doc = "Register PWM_CPRD `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_CPRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CPRD`"]
pub type CPRD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CPRD`"]
pub struct CPRD_W<'a> {
    w: &'a mut W,
}
impl<'a> CPRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Channel Period"]
    #[inline(always)]
    pub fn cprd(&self) -> CPRD_R {
        CPRD_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Period"]
    #[inline(always)]
    pub fn cprd(&mut self) -> CPRD_W {
        CPRD_W { w: self }
    }
}
