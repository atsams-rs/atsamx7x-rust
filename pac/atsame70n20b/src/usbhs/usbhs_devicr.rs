#[doc = "Writer for register USBHS_DEVICR"]
pub type W = crate::W<u32, super::USBHS_DEVICR>;
#[doc = "Register USBHS_DEVICR `reset()`'s with value 0"]
impl crate::ResetValue for super::USBHS_DEVICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SUSPC`"]
pub struct SUSPC_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPC_W<'a> {
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
#[doc = "Write proxy for field `MSOFC`"]
pub struct MSOFC_W<'a> {
    w: &'a mut W,
}
impl<'a> MSOFC_W<'a> {
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
#[doc = "Write proxy for field `SOFC`"]
pub struct SOFC_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFC_W<'a> {
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
#[doc = "Write proxy for field `EORSTC`"]
pub struct EORSTC_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSTC_W<'a> {
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
#[doc = "Write proxy for field `WAKEUPC`"]
pub struct WAKEUPC_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPC_W<'a> {
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
#[doc = "Write proxy for field `EORSMC`"]
pub struct EORSMC_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSMC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `UPRSMC`"]
pub struct UPRSMC_W<'a> {
    w: &'a mut W,
}
impl<'a> UPRSMC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Suspend Interrupt Clear"]
    #[inline(always)]
    pub fn suspc(&mut self) -> SUSPC_W {
        SUSPC_W { w: self }
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Clear"]
    #[inline(always)]
    pub fn msofc(&mut self) -> MSOFC_W {
        MSOFC_W { w: self }
    }
    #[doc = "Bit 2 - Start of Frame Interrupt Clear"]
    #[inline(always)]
    pub fn sofc(&mut self) -> SOFC_W {
        SOFC_W { w: self }
    }
    #[doc = "Bit 3 - End of Reset Interrupt Clear"]
    #[inline(always)]
    pub fn eorstc(&mut self) -> EORSTC_W {
        EORSTC_W { w: self }
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Clear"]
    #[inline(always)]
    pub fn wakeupc(&mut self) -> WAKEUPC_W {
        WAKEUPC_W { w: self }
    }
    #[doc = "Bit 5 - End of Resume Interrupt Clear"]
    #[inline(always)]
    pub fn eorsmc(&mut self) -> EORSMC_W {
        EORSMC_W { w: self }
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Clear"]
    #[inline(always)]
    pub fn uprsmc(&mut self) -> UPRSMC_W {
        UPRSMC_W { w: self }
    }
}
