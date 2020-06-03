#[doc = "Reader of register AFEC_CSELR"]
pub type R = crate::R<u32, super::AFEC_CSELR>;
#[doc = "Writer for register AFEC_CSELR"]
pub type W = crate::W<u32, super::AFEC_CSELR>;
#[doc = "Register AFEC_CSELR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFEC_CSELR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSEL`"]
pub type CSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSEL`"]
pub struct CSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel Selection"]
    #[inline(always)]
    pub fn csel(&self) -> CSEL_R {
        CSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel Selection"]
    #[inline(always)]
    pub fn csel(&mut self) -> CSEL_W {
        CSEL_W { w: self }
    }
}
