#[doc = "Reader of register US_IF"]
pub type R = crate::R<u32, super::US_IF>;
#[doc = "Writer for register US_IF"]
pub type W = crate::W<u32, super::US_IF>;
#[doc = "Register US_IF `reset()`'s with value 0"]
impl crate::ResetValue for super::US_IF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IRDA_FILTER`"]
pub type IRDA_FILTER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRDA_FILTER`"]
pub struct IRDA_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> IRDA_FILTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - IrDA Filter"]
    #[inline(always)]
    pub fn irda_filter(&self) -> IRDA_FILTER_R {
        IRDA_FILTER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IrDA Filter"]
    #[inline(always)]
    pub fn irda_filter(&mut self) -> IRDA_FILTER_W {
        IRDA_FILTER_W { w: self }
    }
}
