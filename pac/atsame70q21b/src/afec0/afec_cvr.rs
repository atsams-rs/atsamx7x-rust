#[doc = "Reader of register AFEC_CVR"]
pub type R = crate::R<u32, super::AFEC_CVR>;
#[doc = "Writer for register AFEC_CVR"]
pub type W = crate::W<u32, super::AFEC_CVR>;
#[doc = "Register AFEC_CVR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFEC_CVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OFFSETCORR`"]
pub type OFFSETCORR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OFFSETCORR`"]
pub struct OFFSETCORR_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSETCORR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `GAINCORR`"]
pub type GAINCORR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GAINCORR`"]
pub struct GAINCORR_W<'a> {
    w: &'a mut W,
}
impl<'a> GAINCORR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Offset Correction"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OFFSETCORR_R {
        OFFSETCORR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Gain Correction"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GAINCORR_R {
        GAINCORR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Offset Correction"]
    #[inline(always)]
    pub fn offsetcorr(&mut self) -> OFFSETCORR_W {
        OFFSETCORR_W { w: self }
    }
    #[doc = "Bits 16:31 - Gain Correction"]
    #[inline(always)]
    pub fn gaincorr(&mut self) -> GAINCORR_W {
        GAINCORR_W { w: self }
    }
}
