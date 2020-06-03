#[doc = "Reader of register AFEC_SEQ2R"]
pub type R = crate::R<u32, super::AFEC_SEQ2R>;
#[doc = "Writer for register AFEC_SEQ2R"]
pub type W = crate::W<u32, super::AFEC_SEQ2R>;
#[doc = "Register AFEC_SEQ2R `reset()`'s with value 0"]
impl crate::ResetValue for super::AFEC_SEQ2R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USCH8`"]
pub type USCH8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USCH8`"]
pub struct USCH8_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `USCH9`"]
pub type USCH9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USCH9`"]
pub struct USCH9_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `USCH10`"]
pub type USCH10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USCH10`"]
pub struct USCH10_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `USCH11`"]
pub type USCH11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USCH11`"]
pub struct USCH11_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - User Sequence Number 8"]
    #[inline(always)]
    pub fn usch8(&self) -> USCH8_R {
        USCH8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - User Sequence Number 9"]
    #[inline(always)]
    pub fn usch9(&self) -> USCH9_R {
        USCH9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - User Sequence Number 10"]
    #[inline(always)]
    pub fn usch10(&self) -> USCH10_R {
        USCH10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - User Sequence Number 11"]
    #[inline(always)]
    pub fn usch11(&self) -> USCH11_R {
        USCH11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - User Sequence Number 8"]
    #[inline(always)]
    pub fn usch8(&mut self) -> USCH8_W {
        USCH8_W { w: self }
    }
    #[doc = "Bits 4:7 - User Sequence Number 9"]
    #[inline(always)]
    pub fn usch9(&mut self) -> USCH9_W {
        USCH9_W { w: self }
    }
    #[doc = "Bits 8:11 - User Sequence Number 10"]
    #[inline(always)]
    pub fn usch10(&mut self) -> USCH10_W {
        USCH10_W { w: self }
    }
    #[doc = "Bits 12:15 - User Sequence Number 11"]
    #[inline(always)]
    pub fn usch11(&mut self) -> USCH11_W {
        USCH11_W { w: self }
    }
}
