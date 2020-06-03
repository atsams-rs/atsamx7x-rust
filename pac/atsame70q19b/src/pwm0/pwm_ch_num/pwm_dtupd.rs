#[doc = "Writer for register PWM_DTUPD"]
pub type W = crate::W<u32, super::PWM_DTUPD>;
#[doc = "Register PWM_DTUPD `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_DTUPD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DTHUPD`"]
pub struct DTHUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DTHUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Write proxy for field `DTLUPD`"]
pub struct DTLUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DTLUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Dead-Time Value Update for PWMHx Output"]
    #[inline(always)]
    pub fn dthupd(&mut self) -> DTHUPD_W {
        DTHUPD_W { w: self }
    }
    #[doc = "Bits 16:31 - Dead-Time Value Update for PWMLx Output"]
    #[inline(always)]
    pub fn dtlupd(&mut self) -> DTLUPD_W {
        DTLUPD_W { w: self }
    }
}
