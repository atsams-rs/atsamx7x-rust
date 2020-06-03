#[doc = "Reader of register SSC_TSHR"]
pub type R = crate::R<u32, super::SSC_TSHR>;
#[doc = "Writer for register SSC_TSHR"]
pub type W = crate::W<u32, super::SSC_TSHR>;
#[doc = "Register SSC_TSHR `reset()`'s with value 0"]
impl crate::ResetValue for super::SSC_TSHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSDAT`"]
pub type TSDAT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSDAT`"]
pub struct TSDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TSDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmit Synchronization Data"]
    #[inline(always)]
    pub fn tsdat(&self) -> TSDAT_R {
        TSDAT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Synchronization Data"]
    #[inline(always)]
    pub fn tsdat(&mut self) -> TSDAT_W {
        TSDAT_W { w: self }
    }
}
