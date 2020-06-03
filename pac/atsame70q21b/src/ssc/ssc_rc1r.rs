#[doc = "Reader of register SSC_RC1R"]
pub type R = crate::R<u32, super::SSC_RC1R>;
#[doc = "Writer for register SSC_RC1R"]
pub type W = crate::W<u32, super::SSC_RC1R>;
#[doc = "Register SSC_RC1R `reset()`'s with value 0"]
impl crate::ResetValue for super::SSC_RC1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CP1`"]
pub type CP1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CP1`"]
pub struct CP1_W<'a> {
    w: &'a mut W,
}
impl<'a> CP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Receive Compare Data 1"]
    #[inline(always)]
    pub fn cp1(&self) -> CP1_R {
        CP1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Compare Data 1"]
    #[inline(always)]
    pub fn cp1(&mut self) -> CP1_W {
        CP1_W { w: self }
    }
}
