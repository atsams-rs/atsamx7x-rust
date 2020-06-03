#[doc = "Reader of register US_LONPR"]
pub type R = crate::R<u32, super::US_LONPR>;
#[doc = "Writer for register US_LONPR"]
pub type W = crate::W<u32, super::US_LONPR>;
#[doc = "Register US_LONPR `reset()`'s with value 0"]
impl crate::ResetValue for super::US_LONPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LONPL`"]
pub type LONPL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LONPL`"]
pub struct LONPL_W<'a> {
    w: &'a mut W,
}
impl<'a> LONPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - LON Preamble Length"]
    #[inline(always)]
    pub fn lonpl(&self) -> LONPL_R {
        LONPL_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - LON Preamble Length"]
    #[inline(always)]
    pub fn lonpl(&mut self) -> LONPL_W {
        LONPL_W { w: self }
    }
}
