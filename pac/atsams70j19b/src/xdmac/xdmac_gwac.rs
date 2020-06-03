#[doc = "Reader of register XDMAC_GWAC"]
pub type R = crate::R<u32, super::XDMAC_GWAC>;
#[doc = "Writer for register XDMAC_GWAC"]
pub type W = crate::W<u32, super::XDMAC_GWAC>;
#[doc = "Register XDMAC_GWAC `reset()`'s with value 0"]
impl crate::ResetValue for super::XDMAC_GWAC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PW0`"]
pub type PW0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PW0`"]
pub struct PW0_W<'a> {
    w: &'a mut W,
}
impl<'a> PW0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PW1`"]
pub type PW1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PW1`"]
pub struct PW1_W<'a> {
    w: &'a mut W,
}
impl<'a> PW1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `PW2`"]
pub type PW2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PW2`"]
pub struct PW2_W<'a> {
    w: &'a mut W,
}
impl<'a> PW2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `PW3`"]
pub type PW3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PW3`"]
pub struct PW3_W<'a> {
    w: &'a mut W,
}
impl<'a> PW3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Pool Weight 0"]
    #[inline(always)]
    pub fn pw0(&self) -> PW0_R {
        PW0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Pool Weight 1"]
    #[inline(always)]
    pub fn pw1(&self) -> PW1_R {
        PW1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pool Weight 2"]
    #[inline(always)]
    pub fn pw2(&self) -> PW2_R {
        PW2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pool Weight 3"]
    #[inline(always)]
    pub fn pw3(&self) -> PW3_R {
        PW3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pool Weight 0"]
    #[inline(always)]
    pub fn pw0(&mut self) -> PW0_W {
        PW0_W { w: self }
    }
    #[doc = "Bits 4:7 - Pool Weight 1"]
    #[inline(always)]
    pub fn pw1(&mut self) -> PW1_W {
        PW1_W { w: self }
    }
    #[doc = "Bits 8:11 - Pool Weight 2"]
    #[inline(always)]
    pub fn pw2(&mut self) -> PW2_W {
        PW2_W { w: self }
    }
    #[doc = "Bits 12:15 - Pool Weight 3"]
    #[inline(always)]
    pub fn pw3(&mut self) -> PW3_W {
        PW3_W { w: self }
    }
}
