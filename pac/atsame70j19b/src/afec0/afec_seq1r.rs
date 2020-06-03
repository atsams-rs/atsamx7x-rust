#[doc = "Reader of register AFEC_SEQ1R"]
pub type R = crate::R<u32, super::AFEC_SEQ1R>;
#[doc = "Writer for register AFEC_SEQ1R"]
pub type W = crate::W<u32, super::AFEC_SEQ1R>;
#[doc = "Register AFEC_SEQ1R `reset()`'s with value 0"]
impl crate::ResetValue for super::AFEC_SEQ1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USCH0`"]
pub type USCH0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USCH0`"]
pub struct USCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `USCH1`"]
pub type USCH1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USCH1`"]
pub struct USCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `USCH2`"]
pub type USCH2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USCH2`"]
pub struct USCH2_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `USCH3`"]
pub type USCH3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USCH3`"]
pub struct USCH3_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `USCH4`"]
pub type USCH4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USCH4`"]
pub struct USCH4_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `USCH5`"]
pub type USCH5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USCH5`"]
pub struct USCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `USCH6`"]
pub type USCH6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USCH6`"]
pub struct USCH6_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `USCH7`"]
pub type USCH7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USCH7`"]
pub struct USCH7_W<'a> {
    w: &'a mut W,
}
impl<'a> USCH7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - User Sequence Number 0"]
    #[inline(always)]
    pub fn usch0(&self) -> USCH0_R {
        USCH0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - User Sequence Number 1"]
    #[inline(always)]
    pub fn usch1(&self) -> USCH1_R {
        USCH1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - User Sequence Number 2"]
    #[inline(always)]
    pub fn usch2(&self) -> USCH2_R {
        USCH2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - User Sequence Number 3"]
    #[inline(always)]
    pub fn usch3(&self) -> USCH3_R {
        USCH3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - User Sequence Number 4"]
    #[inline(always)]
    pub fn usch4(&self) -> USCH4_R {
        USCH4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - User Sequence Number 5"]
    #[inline(always)]
    pub fn usch5(&self) -> USCH5_R {
        USCH5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - User Sequence Number 6"]
    #[inline(always)]
    pub fn usch6(&self) -> USCH6_R {
        USCH6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - User Sequence Number 7"]
    #[inline(always)]
    pub fn usch7(&self) -> USCH7_R {
        USCH7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - User Sequence Number 0"]
    #[inline(always)]
    pub fn usch0(&mut self) -> USCH0_W {
        USCH0_W { w: self }
    }
    #[doc = "Bits 4:7 - User Sequence Number 1"]
    #[inline(always)]
    pub fn usch1(&mut self) -> USCH1_W {
        USCH1_W { w: self }
    }
    #[doc = "Bits 8:11 - User Sequence Number 2"]
    #[inline(always)]
    pub fn usch2(&mut self) -> USCH2_W {
        USCH2_W { w: self }
    }
    #[doc = "Bits 12:15 - User Sequence Number 3"]
    #[inline(always)]
    pub fn usch3(&mut self) -> USCH3_W {
        USCH3_W { w: self }
    }
    #[doc = "Bits 16:19 - User Sequence Number 4"]
    #[inline(always)]
    pub fn usch4(&mut self) -> USCH4_W {
        USCH4_W { w: self }
    }
    #[doc = "Bits 20:23 - User Sequence Number 5"]
    #[inline(always)]
    pub fn usch5(&mut self) -> USCH5_W {
        USCH5_W { w: self }
    }
    #[doc = "Bits 24:27 - User Sequence Number 6"]
    #[inline(always)]
    pub fn usch6(&mut self) -> USCH6_W {
        USCH6_W { w: self }
    }
    #[doc = "Bits 28:31 - User Sequence Number 7"]
    #[inline(always)]
    pub fn usch7(&mut self) -> USCH7_W {
        USCH7_W { w: self }
    }
}
