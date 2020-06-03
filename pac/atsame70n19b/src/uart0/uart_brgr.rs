#[doc = "Reader of register UART_BRGR"]
pub type R = crate::R<u32, super::UART_BRGR>;
#[doc = "Writer for register UART_BRGR"]
pub type W = crate::W<u32, super::UART_BRGR>;
#[doc = "Register UART_BRGR `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_BRGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CD`"]
pub type CD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CD`"]
pub struct CD_W<'a> {
    w: &'a mut W,
}
impl<'a> CD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    pub fn cd(&mut self) -> CD_W {
        CD_W { w: self }
    }
}
