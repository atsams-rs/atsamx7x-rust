#[doc = "Reader of register GMAC_SCL"]
pub type R = crate::R<u32, super::GMAC_SCL>;
#[doc = "Writer for register GMAC_SCL"]
pub type W = crate::W<u32, super::GMAC_SCL>;
#[doc = "Register GMAC_SCL `reset()`'s with value 0"]
impl crate::ResetValue for super::GMAC_SCL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEC`"]
pub type SEC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SEC`"]
pub struct SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 1588 Timer Second Comparison Value"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1588 Timer Second Comparison Value"]
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W {
        SEC_W { w: self }
    }
}
