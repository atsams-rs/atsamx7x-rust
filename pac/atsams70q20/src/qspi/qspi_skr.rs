#[doc = "Writer for register QSPI_SKR"]
pub type W = crate::W<u32, super::QSPI_SKR>;
#[doc = "Register QSPI_SKR `reset()`'s with value 0"]
impl crate::ResetValue for super::QSPI_SKR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `USRK`"]
pub struct USRK_W<'a> {
    w: &'a mut W,
}
impl<'a> USRK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Scrambling User Key"]
    #[inline(always)]
    pub fn usrk(&mut self) -> USRK_W {
        USRK_W { w: self }
    }
}
