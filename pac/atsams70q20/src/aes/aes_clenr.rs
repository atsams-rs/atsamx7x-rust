#[doc = "Reader of register AES_CLENR"]
pub type R = crate::R<u32, super::AES_CLENR>;
#[doc = "Writer for register AES_CLENR"]
pub type W = crate::W<u32, super::AES_CLENR>;
#[doc = "Register AES_CLENR `reset()`'s with value 0"]
impl crate::ResetValue for super::AES_CLENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLEN`"]
pub type CLEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CLEN`"]
pub struct CLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Plaintext/Ciphertext Length"]
    #[inline(always)]
    pub fn clen(&self) -> CLEN_R {
        CLEN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Plaintext/Ciphertext Length"]
    #[inline(always)]
    pub fn clen(&mut self) -> CLEN_W {
        CLEN_W { w: self }
    }
}
