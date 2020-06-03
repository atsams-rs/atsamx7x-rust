#[doc = "Reader of register SSC_RC0R"]
pub type R = crate::R<u32, super::SSC_RC0R>;
#[doc = "Writer for register SSC_RC0R"]
pub type W = crate::W<u32, super::SSC_RC0R>;
#[doc = "Register SSC_RC0R `reset()`'s with value 0"]
impl crate::ResetValue for super::SSC_RC0R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CP0`"]
pub type CP0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CP0`"]
pub struct CP0_W<'a> {
    w: &'a mut W,
}
impl<'a> CP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Receive Compare Data 0"]
    #[inline(always)]
    pub fn cp0(&self) -> CP0_R {
        CP0_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Receive Compare Data 0"]
    #[inline(always)]
    pub fn cp0(&mut self) -> CP0_W {
        CP0_W { w: self }
    }
}
