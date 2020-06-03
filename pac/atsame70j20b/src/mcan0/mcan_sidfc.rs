#[doc = "Reader of register MCAN_SIDFC"]
pub type R = crate::R<u32, super::MCAN_SIDFC>;
#[doc = "Writer for register MCAN_SIDFC"]
pub type W = crate::W<u32, super::MCAN_SIDFC>;
#[doc = "Register MCAN_SIDFC `reset()`'s with value 0"]
impl crate::ResetValue for super::MCAN_SIDFC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLSSA`"]
pub type FLSSA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FLSSA`"]
pub struct FLSSA_W<'a> {
    w: &'a mut W,
}
impl<'a> FLSSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | (((value as u32) & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Reader of field `LSS`"]
pub type LSS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LSS`"]
pub struct LSS_W<'a> {
    w: &'a mut W,
}
impl<'a> LSS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - Filter List Standard Start Address"]
    #[inline(always)]
    pub fn flssa(&self) -> FLSSA_R {
        FLSSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23 - List Size Standard"]
    #[inline(always)]
    pub fn lss(&self) -> LSS_R {
        LSS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Filter List Standard Start Address"]
    #[inline(always)]
    pub fn flssa(&mut self) -> FLSSA_W {
        FLSSA_W { w: self }
    }
    #[doc = "Bits 16:23 - List Size Standard"]
    #[inline(always)]
    pub fn lss(&mut self) -> LSS_W {
        LSS_W { w: self }
    }
}
