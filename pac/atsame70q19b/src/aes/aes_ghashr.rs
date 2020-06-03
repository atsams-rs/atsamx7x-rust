#[doc = "Reader of register AES_GHASHR[%s]"]
pub type R = crate::R<u32, super::AES_GHASHR>;
#[doc = "Writer for register AES_GHASHR[%s]"]
pub type W = crate::W<u32, super::AES_GHASHR>;
#[doc = "Register AES_GHASHR[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::AES_GHASHR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GHASH`"]
pub type GHASH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GHASH`"]
pub struct GHASH_W<'a> {
    w: &'a mut W,
}
impl<'a> GHASH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Intermediate GCM Hash Word x"]
    #[inline(always)]
    pub fn ghash(&self) -> GHASH_R {
        GHASH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Intermediate GCM Hash Word x"]
    #[inline(always)]
    pub fn ghash(&mut self) -> GHASH_W {
        GHASH_W { w: self }
    }
}
