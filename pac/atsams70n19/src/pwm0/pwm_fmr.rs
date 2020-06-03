#[doc = "Reader of register PWM_FMR"]
pub type R = crate::R<u32, super::PWM_FMR>;
#[doc = "Writer for register PWM_FMR"]
pub type W = crate::W<u32, super::PWM_FMR>;
#[doc = "Register PWM_FMR `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_FMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FPOL`"]
pub type FPOL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FPOL`"]
pub struct FPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> FPOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `FMOD`"]
pub type FMOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FMOD`"]
pub struct FMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> FMOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `FFIL`"]
pub type FFIL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FFIL`"]
pub struct FFIL_W<'a> {
    w: &'a mut W,
}
impl<'a> FFIL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Fault Polarity"]
    #[inline(always)]
    pub fn fpol(&self) -> FPOL_R {
        FPOL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Activation Mode"]
    #[inline(always)]
    pub fn fmod(&self) -> FMOD_R {
        FMOD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Fault Filtering"]
    #[inline(always)]
    pub fn ffil(&self) -> FFIL_R {
        FFIL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fault Polarity"]
    #[inline(always)]
    pub fn fpol(&mut self) -> FPOL_W {
        FPOL_W { w: self }
    }
    #[doc = "Bits 8:15 - Fault Activation Mode"]
    #[inline(always)]
    pub fn fmod(&mut self) -> FMOD_W {
        FMOD_W { w: self }
    }
    #[doc = "Bits 16:23 - Fault Filtering"]
    #[inline(always)]
    pub fn ffil(&mut self) -> FFIL_W {
        FFIL_W { w: self }
    }
}
