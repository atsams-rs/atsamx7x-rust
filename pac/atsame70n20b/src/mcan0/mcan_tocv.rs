#[doc = "Reader of register MCAN_TOCV"]
pub type R = crate::R<u32, super::MCAN_TOCV>;
#[doc = "Writer for register MCAN_TOCV"]
pub type W = crate::W<u32, super::MCAN_TOCV>;
#[doc = "Register MCAN_TOCV `reset()`'s with value 0"]
impl crate::ResetValue for super::MCAN_TOCV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TOC`"]
pub type TOC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOC`"]
pub struct TOC_W<'a> {
    w: &'a mut W,
}
impl<'a> TOC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timeout Counter (cleared on write)"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timeout Counter (cleared on write)"]
    #[inline(always)]
    pub fn toc(&mut self) -> TOC_W {
        TOC_W { w: self }
    }
}
