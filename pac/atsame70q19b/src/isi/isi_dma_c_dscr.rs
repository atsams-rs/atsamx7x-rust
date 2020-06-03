#[doc = "Reader of register ISI_DMA_C_DSCR"]
pub type R = crate::R<u32, super::ISI_DMA_C_DSCR>;
#[doc = "Writer for register ISI_DMA_C_DSCR"]
pub type W = crate::W<u32, super::ISI_DMA_C_DSCR>;
#[doc = "Register ISI_DMA_C_DSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::ISI_DMA_C_DSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `C_DSCR`"]
pub type C_DSCR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `C_DSCR`"]
pub struct C_DSCR_W<'a> {
    w: &'a mut W,
}
impl<'a> C_DSCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Codec Descriptor Base Address"]
    #[inline(always)]
    pub fn c_dscr(&self) -> C_DSCR_R {
        C_DSCR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Codec Descriptor Base Address"]
    #[inline(always)]
    pub fn c_dscr(&mut self) -> C_DSCR_W {
        C_DSCR_W { w: self }
    }
}
