#[doc = "Reader of register CCFG_CAN0"]
pub type R = crate::R<u32, super::CCFG_CAN0>;
#[doc = "Writer for register CCFG_CAN0"]
pub type W = crate::W<u32, super::CCFG_CAN0>;
#[doc = "Register CCFG_CAN0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CCFG_CAN0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAN0DMABA`"]
pub type CAN0DMABA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CAN0DMABA`"]
pub struct CAN0DMABA_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN0DMABA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - CAN0 DMA Base Address"]
    #[inline(always)]
    pub fn can0dmaba(&self) -> CAN0DMABA_R {
        CAN0DMABA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - CAN0 DMA Base Address"]
    #[inline(always)]
    pub fn can0dmaba(&mut self) -> CAN0DMABA_W {
        CAN0DMABA_W { w: self }
    }
}
