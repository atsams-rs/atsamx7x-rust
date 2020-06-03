#[doc = "Writer for register AES_KEYWR[%s]"]
pub type W = crate::W<u32, super::AES_KEYWR>;
#[doc = "Register AES_KEYWR[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::AES_KEYWR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `KEYW`"]
pub struct KEYW_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Word"]
    #[inline(always)]
    pub fn keyw(&mut self) -> KEYW_W {
        KEYW_W { w: self }
    }
}
