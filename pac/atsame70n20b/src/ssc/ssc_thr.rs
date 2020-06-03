#[doc = "Writer for register SSC_THR"]
pub type W = crate::W<u32, super::SSC_THR>;
#[doc = "Register SSC_THR `reset()`'s with value 0"]
impl crate::ResetValue for super::SSC_THR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TDAT`"]
pub struct TDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Data"]
    #[inline(always)]
    pub fn tdat(&mut self) -> TDAT_W {
        TDAT_W { w: self }
    }
}
