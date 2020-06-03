#[doc = "Reader of register SMC_CYCLE"]
pub type R = crate::R<u32, super::SMC_CYCLE>;
#[doc = "Writer for register SMC_CYCLE"]
pub type W = crate::W<u32, super::SMC_CYCLE>;
#[doc = "Register SMC_CYCLE `reset()`'s with value 0"]
impl crate::ResetValue for super::SMC_CYCLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NWE_CYCLE`"]
pub type NWE_CYCLE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NWE_CYCLE`"]
pub struct NWE_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> NWE_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "Reader of field `NRD_CYCLE`"]
pub type NRD_CYCLE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NRD_CYCLE`"]
pub struct NRD_CYCLE_W<'a> {
    w: &'a mut W,
}
impl<'a> NRD_CYCLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 16)) | (((value as u32) & 0x01ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Total Write Cycle Length"]
    #[inline(always)]
    pub fn nwe_cycle(&self) -> NWE_CYCLE_R {
        NWE_CYCLE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - Total Read Cycle Length"]
    #[inline(always)]
    pub fn nrd_cycle(&self) -> NRD_CYCLE_R {
        NRD_CYCLE_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Total Write Cycle Length"]
    #[inline(always)]
    pub fn nwe_cycle(&mut self) -> NWE_CYCLE_W {
        NWE_CYCLE_W { w: self }
    }
    #[doc = "Bits 16:24 - Total Read Cycle Length"]
    #[inline(always)]
    pub fn nrd_cycle(&mut self) -> NRD_CYCLE_W {
        NRD_CYCLE_W { w: self }
    }
}
