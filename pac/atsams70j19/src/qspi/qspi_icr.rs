#[doc = "Reader of register QSPI_ICR"]
pub type R = crate::R<u32, super::QSPI_ICR>;
#[doc = "Writer for register QSPI_ICR"]
pub type W = crate::W<u32, super::QSPI_ICR>;
#[doc = "Register QSPI_ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::QSPI_ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INST`"]
pub type INST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INST`"]
pub struct INST_W<'a> {
    w: &'a mut W,
}
impl<'a> INST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `OPT`"]
pub type OPT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPT`"]
pub struct OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Instruction Code"]
    #[inline(always)]
    pub fn inst(&self) -> INST_R {
        INST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Option Code"]
    #[inline(always)]
    pub fn opt(&self) -> OPT_R {
        OPT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Instruction Code"]
    #[inline(always)]
    pub fn inst(&mut self) -> INST_W {
        INST_W { w: self }
    }
    #[doc = "Bits 16:23 - Option Code"]
    #[inline(always)]
    pub fn opt(&mut self) -> OPT_W {
        OPT_W { w: self }
    }
}
