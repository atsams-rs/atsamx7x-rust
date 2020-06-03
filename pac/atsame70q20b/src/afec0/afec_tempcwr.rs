#[doc = "Reader of register AFEC_TEMPCWR"]
pub type R = crate::R<u32, super::AFEC_TEMPCWR>;
#[doc = "Writer for register AFEC_TEMPCWR"]
pub type W = crate::W<u32, super::AFEC_TEMPCWR>;
#[doc = "Register AFEC_TEMPCWR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFEC_TEMPCWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TLOWTHRES`"]
pub type TLOWTHRES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TLOWTHRES`"]
pub struct TLOWTHRES_W<'a> {
    w: &'a mut W,
}
impl<'a> TLOWTHRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `THIGHTHRES`"]
pub type THIGHTHRES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `THIGHTHRES`"]
pub struct THIGHTHRES_W<'a> {
    w: &'a mut W,
}
impl<'a> THIGHTHRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Temperature Low Threshold"]
    #[inline(always)]
    pub fn tlowthres(&self) -> TLOWTHRES_R {
        TLOWTHRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Temperature High Threshold"]
    #[inline(always)]
    pub fn thighthres(&self) -> THIGHTHRES_R {
        THIGHTHRES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Temperature Low Threshold"]
    #[inline(always)]
    pub fn tlowthres(&mut self) -> TLOWTHRES_W {
        TLOWTHRES_W { w: self }
    }
    #[doc = "Bits 16:31 - Temperature High Threshold"]
    #[inline(always)]
    pub fn thighthres(&mut self) -> THIGHTHRES_W {
        THIGHTHRES_W { w: self }
    }
}
