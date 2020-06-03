#[doc = "Reader of register MCAN_TSCV"]
pub type R = crate::R<u32, super::MCAN_TSCV>;
#[doc = "Writer for register MCAN_TSCV"]
pub type W = crate::W<u32, super::MCAN_TSCV>;
#[doc = "Register MCAN_TSCV `reset()`'s with value 0"]
impl crate::ResetValue for super::MCAN_TSCV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSC`"]
pub type TSC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSC`"]
pub struct TSC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Timestamp Counter (cleared on write)"]
    #[inline(always)]
    pub fn tsc(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp Counter (cleared on write)"]
    #[inline(always)]
    pub fn tsc(&mut self) -> TSC_W {
        TSC_W { w: self }
    }
}
