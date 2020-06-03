#[doc = "Reader of register SDRAMC_MDR"]
pub type R = crate::R<u32, super::SDRAMC_MDR>;
#[doc = "Writer for register SDRAMC_MDR"]
pub type W = crate::W<u32, super::SDRAMC_MDR>;
#[doc = "Register SDRAMC_MDR `reset()`'s with value 0"]
impl crate::ResetValue for super::SDRAMC_MDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Memory Device Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MD_A {
    #[doc = "0: SDRAM"]
    SDRAM = 0,
    #[doc = "1: Low-power SDRAM"]
    LPSDRAM = 1,
}
impl From<MD_A> for u8 {
    #[inline(always)]
    fn from(variant: MD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MD`"]
pub type MD_R = crate::R<u8, MD_A>;
impl MD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MD_A::SDRAM),
            1 => Val(MD_A::LPSDRAM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SDRAM`"]
    #[inline(always)]
    pub fn is_sdram(&self) -> bool {
        *self == MD_A::SDRAM
    }
    #[doc = "Checks if the value of the field is `LPSDRAM`"]
    #[inline(always)]
    pub fn is_lpsdram(&self) -> bool {
        *self == MD_A::LPSDRAM
    }
}
#[doc = "Write proxy for field `MD`"]
pub struct MD_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SDRAM"]
    #[inline(always)]
    pub fn sdram(self) -> &'a mut W {
        self.variant(MD_A::SDRAM)
    }
    #[doc = "Low-power SDRAM"]
    #[inline(always)]
    pub fn lpsdram(self) -> &'a mut W {
        self.variant(MD_A::LPSDRAM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory Device Type"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory Device Type"]
    #[inline(always)]
    pub fn md(&mut self) -> MD_W {
        MD_W { w: self }
    }
}
