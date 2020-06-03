#[doc = "Reader of register TWIHS_SWMR"]
pub type R = crate::R<u32, super::TWIHS_SWMR>;
#[doc = "Writer for register TWIHS_SWMR"]
pub type W = crate::W<u32, super::TWIHS_SWMR>;
#[doc = "Register TWIHS_SWMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TWIHS_SWMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SADR1`"]
pub type SADR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SADR1`"]
pub struct SADR1_W<'a> {
    w: &'a mut W,
}
impl<'a> SADR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `SADR2`"]
pub type SADR2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SADR2`"]
pub struct SADR2_W<'a> {
    w: &'a mut W,
}
impl<'a> SADR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SADR3`"]
pub type SADR3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SADR3`"]
pub struct SADR3_W<'a> {
    w: &'a mut W,
}
impl<'a> SADR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DATAM`"]
pub type DATAM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATAM`"]
pub struct DATAM_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Slave Address 1"]
    #[inline(always)]
    pub fn sadr1(&self) -> SADR1_R {
        SADR1_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Slave Address 2"]
    #[inline(always)]
    pub fn sadr2(&self) -> SADR2_R {
        SADR2_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Slave Address 3"]
    #[inline(always)]
    pub fn sadr3(&self) -> SADR3_R {
        SADR3_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:31 - Data Match"]
    #[inline(always)]
    pub fn datam(&self) -> DATAM_R {
        DATAM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Slave Address 1"]
    #[inline(always)]
    pub fn sadr1(&mut self) -> SADR1_W {
        SADR1_W { w: self }
    }
    #[doc = "Bits 8:14 - Slave Address 2"]
    #[inline(always)]
    pub fn sadr2(&mut self) -> SADR2_W {
        SADR2_W { w: self }
    }
    #[doc = "Bits 16:22 - Slave Address 3"]
    #[inline(always)]
    pub fn sadr3(&mut self) -> SADR3_W {
        SADR3_W { w: self }
    }
    #[doc = "Bits 24:31 - Data Match"]
    #[inline(always)]
    pub fn datam(&mut self) -> DATAM_W {
        DATAM_W { w: self }
    }
}
