#[doc = "Reader of register CCFG_SMCNFCS"]
pub type R = crate::R<u32, super::CCFG_SMCNFCS>;
#[doc = "Writer for register CCFG_SMCNFCS"]
pub type W = crate::W<u32, super::CCFG_SMCNFCS>;
#[doc = "Register CCFG_SMCNFCS `reset()`'s with value 0"]
impl crate::ResetValue for super::CCFG_SMCNFCS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SMC_NFCS0`"]
pub type SMC_NFCS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMC_NFCS0`"]
pub struct SMC_NFCS0_W<'a> {
    w: &'a mut W,
}
impl<'a> SMC_NFCS0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SMC_NFCS1`"]
pub type SMC_NFCS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMC_NFCS1`"]
pub struct SMC_NFCS1_W<'a> {
    w: &'a mut W,
}
impl<'a> SMC_NFCS1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SMC_NFCS2`"]
pub type SMC_NFCS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMC_NFCS2`"]
pub struct SMC_NFCS2_W<'a> {
    w: &'a mut W,
}
impl<'a> SMC_NFCS2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SMC_NFCS3`"]
pub type SMC_NFCS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMC_NFCS3`"]
pub struct SMC_NFCS3_W<'a> {
    w: &'a mut W,
}
impl<'a> SMC_NFCS3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SDRAMEN`"]
pub type SDRAMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDRAMEN`"]
pub struct SDRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDRAMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SMC NAND Flash Chip Select 0 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs0(&self) -> SMC_NFCS0_R {
        SMC_NFCS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SMC NAND Flash Chip Select 1 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs1(&self) -> SMC_NFCS1_R {
        SMC_NFCS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SMC NAND Flash Chip Select 2 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs2(&self) -> SMC_NFCS2_R {
        SMC_NFCS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SMC NAND Flash Chip Select 3 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs3(&self) -> SMC_NFCS3_R {
        SMC_NFCS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SDRAM Enable"]
    #[inline(always)]
    pub fn sdramen(&self) -> SDRAMEN_R {
        SDRAMEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SMC NAND Flash Chip Select 0 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs0(&mut self) -> SMC_NFCS0_W {
        SMC_NFCS0_W { w: self }
    }
    #[doc = "Bit 1 - SMC NAND Flash Chip Select 1 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs1(&mut self) -> SMC_NFCS1_W {
        SMC_NFCS1_W { w: self }
    }
    #[doc = "Bit 2 - SMC NAND Flash Chip Select 2 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs2(&mut self) -> SMC_NFCS2_W {
        SMC_NFCS2_W { w: self }
    }
    #[doc = "Bit 3 - SMC NAND Flash Chip Select 3 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs3(&mut self) -> SMC_NFCS3_W {
        SMC_NFCS3_W { w: self }
    }
    #[doc = "Bit 4 - SDRAM Enable"]
    #[inline(always)]
    pub fn sdramen(&mut self) -> SDRAMEN_W {
        SDRAMEN_W { w: self }
    }
}
