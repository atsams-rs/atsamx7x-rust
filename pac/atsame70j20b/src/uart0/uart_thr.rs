#[doc = "Writer for register UART_THR"]
pub type W = crate::W<u32, super::UART_THR>;
#[doc = "Register UART_THR `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_THR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TXCHR`"]
pub struct TXCHR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Character to be Transmitted"]
    #[inline(always)]
    pub fn txchr(&mut self) -> TXCHR_W {
        TXCHR_W { w: self }
    }
}
