#[doc = "Reader of register XDMAC_CSUS"]
pub type R = crate::R<u32, super::XDMAC_CSUS>;
#[doc = "Writer for register XDMAC_CSUS"]
pub type W = crate::W<u32, super::XDMAC_CSUS>;
#[doc = "Register XDMAC_CSUS `reset()`'s with value 0"]
impl crate::ResetValue for super::XDMAC_CSUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SUBS`"]
pub type SUBS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SUBS`"]
pub struct SUBS_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Channel x Source Microblock Stride"]
    #[inline(always)]
    pub fn subs(&self) -> SUBS_R {
        SUBS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel x Source Microblock Stride"]
    #[inline(always)]
    pub fn subs(&mut self) -> SUBS_W {
        SUBS_W { w: self }
    }
}
