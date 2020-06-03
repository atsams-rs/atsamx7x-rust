#[doc = "Writer for register PWM_CMPVUPD"]
pub type W = crate::W<u32, super::PWM_CMPVUPD>;
#[doc = "Register PWM_CMPVUPD `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_CMPVUPD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CVUPD`"]
pub struct CVUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CVUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Write proxy for field `CVMUPD`"]
pub struct CVMUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CVMUPD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:23 - Comparison x Value Update"]
    #[inline(always)]
    pub fn cvupd(&mut self) -> CVUPD_W {
        CVUPD_W { w: self }
    }
    #[doc = "Bit 24 - Comparison x Value Mode Update"]
    #[inline(always)]
    pub fn cvmupd(&mut self) -> CVMUPD_W {
        CVMUPD_W { w: self }
    }
}
