#[doc = "Writer for register AES_IDATAR[%s]"]
pub type W = crate::W<u32, super::AES_IDATAR>;
#[doc = "Register AES_IDATAR[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::AES_IDATAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `IDATA`"]
pub struct IDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> IDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Input Data Word"]
    #[inline(always)]
    pub fn idata(&mut self) -> IDATA_W {
        IDATA_W { w: self }
    }
}
