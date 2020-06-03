#[doc = "Writer for register PWM_CMPMUPD"]
pub type W = crate::W<u32, super::PWM_CMPMUPD>;
#[doc = "Register PWM_CMPMUPD `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_CMPMUPD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CENUPD`"]
pub struct CENUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CENUPD_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `CTRUPD`"]
pub struct CTRUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `CPRUPD`"]
pub struct CPRUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CPRUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `CUPRUPD`"]
pub struct CUPRUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CUPRUPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Comparison x Enable Update"]
    #[inline(always)]
    pub fn cenupd(&mut self) -> CENUPD_W {
        CENUPD_W { w: self }
    }
    #[doc = "Bits 4:7 - Comparison x Trigger Update"]
    #[inline(always)]
    pub fn ctrupd(&mut self) -> CTRUPD_W {
        CTRUPD_W { w: self }
    }
    #[doc = "Bits 8:11 - Comparison x Period Update"]
    #[inline(always)]
    pub fn cprupd(&mut self) -> CPRUPD_W {
        CPRUPD_W { w: self }
    }
    #[doc = "Bits 16:19 - Comparison x Update Period Update"]
    #[inline(always)]
    pub fn cuprupd(&mut self) -> CUPRUPD_W {
        CUPRUPD_W { w: self }
    }
}
