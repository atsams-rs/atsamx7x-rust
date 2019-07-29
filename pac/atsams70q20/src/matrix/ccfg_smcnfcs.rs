#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCFG_SMCNFCS {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type SMC_NFCS0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SMC_NFCS0W<'a> {
    w: &'a mut W,
}
impl<'a> _SMC_NFCS0W<'a> {
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
#[doc = r"Reader of the field"]
pub type SMC_NFCS1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SMC_NFCS1W<'a> {
    w: &'a mut W,
}
impl<'a> _SMC_NFCS1W<'a> {
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
#[doc = r"Reader of the field"]
pub type SMC_NFCS2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SMC_NFCS2W<'a> {
    w: &'a mut W,
}
impl<'a> _SMC_NFCS2W<'a> {
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
#[doc = r"Reader of the field"]
pub type SMC_NFCS3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SMC_NFCS3W<'a> {
    w: &'a mut W,
}
impl<'a> _SMC_NFCS3W<'a> {
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
#[doc = r"Reader of the field"]
pub type SDRAMEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SDRAMENW<'a> {
    w: &'a mut W,
}
impl<'a> _SDRAMENW<'a> {
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
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SMC NAND Flash Chip Select 0 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs0(&self) -> SMC_NFCS0_R {
        SMC_NFCS0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - SMC NAND Flash Chip Select 1 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs1(&self) -> SMC_NFCS1_R {
        SMC_NFCS1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SMC NAND Flash Chip Select 2 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs2(&self) -> SMC_NFCS2_R {
        SMC_NFCS2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SMC NAND Flash Chip Select 3 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs3(&self) -> SMC_NFCS3_R {
        SMC_NFCS3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SDRAM Enable"]
    #[inline(always)]
    pub fn sdramen(&self) -> SDRAMEN_R {
        SDRAMEN_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SMC NAND Flash Chip Select 0 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs0(&mut self) -> _SMC_NFCS0W {
        _SMC_NFCS0W { w: self }
    }
    #[doc = "Bit 1 - SMC NAND Flash Chip Select 1 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs1(&mut self) -> _SMC_NFCS1W {
        _SMC_NFCS1W { w: self }
    }
    #[doc = "Bit 2 - SMC NAND Flash Chip Select 2 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs2(&mut self) -> _SMC_NFCS2W {
        _SMC_NFCS2W { w: self }
    }
    #[doc = "Bit 3 - SMC NAND Flash Chip Select 3 Assignment"]
    #[inline(always)]
    pub fn smc_nfcs3(&mut self) -> _SMC_NFCS3W {
        _SMC_NFCS3W { w: self }
    }
    #[doc = "Bit 4 - SDRAM Enable"]
    #[inline(always)]
    pub fn sdramen(&mut self) -> _SDRAMENW {
        _SDRAMENW { w: self }
    }
}
