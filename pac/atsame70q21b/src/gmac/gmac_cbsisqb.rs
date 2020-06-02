#[doc = "Reader of register GMAC_CBSISQB"]
pub type R = crate::R<u32, super::GMAC_CBSISQB>;
#[doc = "Writer for register GMAC_CBSISQB"]
pub type W = crate::W<u32, super::GMAC_CBSISQB>;
#[doc = "Register GMAC_CBSISQB `reset()`'s with value 0"]
impl crate::ResetValue for super::GMAC_CBSISQB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IS`"]
pub type IS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IS`"]
pub struct IS_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - IdleSlope"]
    #[inline(always)]
    pub fn is(&self) -> IS_R {
        IS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - IdleSlope"]
    #[inline(always)]
    pub fn is(&mut self) -> IS_W {
        IS_W { w: self }
    }
}
