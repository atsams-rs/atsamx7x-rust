#[doc = "Reader of register ISI_PDECF"]
pub type R = crate::R<u32, super::ISI_PDECF>;
#[doc = "Writer for register ISI_PDECF"]
pub type W = crate::W<u32, super::ISI_PDECF>;
#[doc = "Register ISI_PDECF `reset()`'s with value 0"]
impl crate::ResetValue for super::ISI_PDECF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEC_FACTOR`"]
pub type DEC_FACTOR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEC_FACTOR`"]
pub struct DEC_FACTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEC_FACTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Decimation Factor"]
    #[inline(always)]
    pub fn dec_factor(&self) -> DEC_FACTOR_R {
        DEC_FACTOR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Decimation Factor"]
    #[inline(always)]
    pub fn dec_factor(&mut self) -> DEC_FACTOR_W {
        DEC_FACTOR_W { w: self }
    }
}
