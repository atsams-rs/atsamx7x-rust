#[doc = "Reader of register XDMAC_CBC"]
pub type R = crate::R<u32, super::XDMAC_CBC>;
#[doc = "Writer for register XDMAC_CBC"]
pub type W = crate::W<u32, super::XDMAC_CBC>;
#[doc = "Register XDMAC_CBC `reset()`'s with value 0"]
impl crate::ResetValue for super::XDMAC_CBC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BLEN`"]
pub type BLEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BLEN`"]
pub struct BLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Channel x Block Length"]
    #[inline(always)]
    pub fn blen(&self) -> BLEN_R {
        BLEN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Channel x Block Length"]
    #[inline(always)]
    pub fn blen(&mut self) -> BLEN_W {
        BLEN_W { w: self }
    }
}
