#[doc = "Reader of register XDMAC_CUBC"]
pub type R = crate::R<u32, super::XDMAC_CUBC>;
#[doc = "Writer for register XDMAC_CUBC"]
pub type W = crate::W<u32, super::XDMAC_CUBC>;
#[doc = "Register XDMAC_CUBC `reset()`'s with value 0"]
impl crate::ResetValue for super::XDMAC_CUBC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UBLEN`"]
pub type UBLEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UBLEN`"]
pub struct UBLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UBLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Channel x Microblock Length"]
    #[inline(always)]
    pub fn ublen(&self) -> UBLEN_R {
        UBLEN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel x Microblock Length"]
    #[inline(always)]
    pub fn ublen(&mut self) -> UBLEN_W {
        UBLEN_W { w: self }
    }
}
