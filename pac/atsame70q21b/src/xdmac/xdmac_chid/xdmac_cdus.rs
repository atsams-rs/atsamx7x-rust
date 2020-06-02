#[doc = "Reader of register XDMAC_CDUS"]
pub type R = crate::R<u32, super::XDMAC_CDUS>;
#[doc = "Writer for register XDMAC_CDUS"]
pub type W = crate::W<u32, super::XDMAC_CDUS>;
#[doc = "Register XDMAC_CDUS `reset()`'s with value 0"]
impl crate::ResetValue for super::XDMAC_CDUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DUBS`"]
pub type DUBS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DUBS`"]
pub struct DUBS_W<'a> {
    w: &'a mut W,
}
impl<'a> DUBS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Channel x Destination Microblock Stride"]
    #[inline(always)]
    pub fn dubs(&self) -> DUBS_R {
        DUBS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel x Destination Microblock Stride"]
    #[inline(always)]
    pub fn dubs(&mut self) -> DUBS_W {
        DUBS_W { w: self }
    }
}
