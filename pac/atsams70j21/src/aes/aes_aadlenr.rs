#[doc = "Reader of register AES_AADLENR"]
pub type R = crate::R<u32, super::AES_AADLENR>;
#[doc = "Writer for register AES_AADLENR"]
pub type W = crate::W<u32, super::AES_AADLENR>;
#[doc = "Register AES_AADLENR `reset()`'s with value 0"]
impl crate::ResetValue for super::AES_AADLENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AADLEN`"]
pub type AADLEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AADLEN`"]
pub struct AADLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AADLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Additional Authenticated Data Length"]
    #[inline(always)]
    pub fn aadlen(&self) -> AADLEN_R {
        AADLEN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Additional Authenticated Data Length"]
    #[inline(always)]
    pub fn aadlen(&mut self) -> AADLEN_W {
        AADLEN_W { w: self }
    }
}
