#[doc = "Reader of register DACC_ACR"]
pub type R = crate::R<u32, super::DACC_ACR>;
#[doc = "Writer for register DACC_ACR"]
pub type W = crate::W<u32, super::DACC_ACR>;
#[doc = "Register DACC_ACR `reset()`'s with value 0"]
impl crate::ResetValue for super::DACC_ACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IBCTLCH0`"]
pub type IBCTLCH0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IBCTLCH0`"]
pub struct IBCTLCH0_W<'a> {
    w: &'a mut W,
}
impl<'a> IBCTLCH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `IBCTLCH1`"]
pub type IBCTLCH1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IBCTLCH1`"]
pub struct IBCTLCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> IBCTLCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Analog Output Current Control"]
    #[inline(always)]
    pub fn ibctlch0(&self) -> IBCTLCH0_R {
        IBCTLCH0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Analog Output Current Control"]
    #[inline(always)]
    pub fn ibctlch1(&self) -> IBCTLCH1_R {
        IBCTLCH1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog Output Current Control"]
    #[inline(always)]
    pub fn ibctlch0(&mut self) -> IBCTLCH0_W {
        IBCTLCH0_W { w: self }
    }
    #[doc = "Bits 2:3 - Analog Output Current Control"]
    #[inline(always)]
    pub fn ibctlch1(&mut self) -> IBCTLCH1_W {
        IBCTLCH1_W { w: self }
    }
}
