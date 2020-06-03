#[doc = "Reader of register US_FIDI_USART_MODE"]
pub type R = crate::R<u32, super::US_FIDI_USART_MODE>;
#[doc = "Writer for register US_FIDI_USART_MODE"]
pub type W = crate::W<u32, super::US_FIDI_USART_MODE>;
#[doc = "Register US_FIDI_USART_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::US_FIDI_USART_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FI_DI_RATIO`"]
pub type FI_DI_RATIO_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FI_DI_RATIO`"]
pub struct FI_DI_RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> FI_DI_RATIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - FI Over DI Ratio Value"]
    #[inline(always)]
    pub fn fi_di_ratio(&self) -> FI_DI_RATIO_R {
        FI_DI_RATIO_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - FI Over DI Ratio Value"]
    #[inline(always)]
    pub fn fi_di_ratio(&mut self) -> FI_DI_RATIO_W {
        FI_DI_RATIO_W { w: self }
    }
}
