#[doc = "Reader of register GMAC_ST2RPQ[%s]"]
pub type R = crate::R<u32, super::GMAC_ST2RPQ>;
#[doc = "Writer for register GMAC_ST2RPQ[%s]"]
pub type W = crate::W<u32, super::GMAC_ST2RPQ>;
#[doc = "Register GMAC_ST2RPQ[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::GMAC_ST2RPQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QNB`"]
pub type QNB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `QNB`"]
pub struct QNB_W<'a> {
    w: &'a mut W,
}
impl<'a> QNB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `VLANP`"]
pub type VLANP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VLANP`"]
pub struct VLANP_W<'a> {
    w: &'a mut W,
}
impl<'a> VLANP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `VLANE`"]
pub type VLANE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VLANE`"]
pub struct VLANE_W<'a> {
    w: &'a mut W,
}
impl<'a> VLANE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `I2ETH`"]
pub type I2ETH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2ETH`"]
pub struct I2ETH_W<'a> {
    w: &'a mut W,
}
impl<'a> I2ETH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Reader of field `ETHE`"]
pub type ETHE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETHE`"]
pub struct ETHE_W<'a> {
    w: &'a mut W,
}
impl<'a> ETHE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `COMPA`"]
pub type COMPA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPA`"]
pub struct COMPA_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 13)) | (((value as u32) & 0x1f) << 13);
        self.w
    }
}
#[doc = "Reader of field `COMPAE`"]
pub type COMPAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPAE`"]
pub struct COMPAE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPAE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `COMPB`"]
pub type COMPB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPB`"]
pub struct COMPB_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Reader of field `COMPBE`"]
pub type COMPBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPBE`"]
pub struct COMPBE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `COMPC`"]
pub type COMPC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMPC`"]
pub struct COMPC_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | (((value as u32) & 0x1f) << 25);
        self.w
    }
}
#[doc = "Reader of field `COMPCE`"]
pub type COMPCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPCE`"]
pub struct COMPCE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Queue Number (0-5)"]
    #[inline(always)]
    pub fn qnb(&self) -> QNB_R {
        QNB_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - VLAN Priority"]
    #[inline(always)]
    pub fn vlanp(&self) -> VLANP_R {
        VLANP_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - VLAN Enable"]
    #[inline(always)]
    pub fn vlane(&self) -> VLANE_R {
        VLANE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - Index of Screening Type 2 EtherType register x"]
    #[inline(always)]
    pub fn i2eth(&self) -> I2ETH_R {
        I2ETH_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 12 - EtherType Enable"]
    #[inline(always)]
    pub fn ethe(&self) -> ETHE_R {
        ETHE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:17 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compa(&self) -> COMPA_R {
        COMPA_R::new(((self.bits >> 13) & 0x1f) as u8)
    }
    #[doc = "Bit 18 - Compare A Enable"]
    #[inline(always)]
    pub fn compae(&self) -> COMPAE_R {
        COMPAE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:23 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compb(&self) -> COMPB_R {
        COMPB_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Compare B Enable"]
    #[inline(always)]
    pub fn compbe(&self) -> COMPBE_R {
        COMPBE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:29 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compc(&self) -> COMPC_R {
        COMPC_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - Compare C Enable"]
    #[inline(always)]
    pub fn compce(&self) -> COMPCE_R {
        COMPCE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Queue Number (0-5)"]
    #[inline(always)]
    pub fn qnb(&mut self) -> QNB_W {
        QNB_W { w: self }
    }
    #[doc = "Bits 4:6 - VLAN Priority"]
    #[inline(always)]
    pub fn vlanp(&mut self) -> VLANP_W {
        VLANP_W { w: self }
    }
    #[doc = "Bit 8 - VLAN Enable"]
    #[inline(always)]
    pub fn vlane(&mut self) -> VLANE_W {
        VLANE_W { w: self }
    }
    #[doc = "Bits 9:11 - Index of Screening Type 2 EtherType register x"]
    #[inline(always)]
    pub fn i2eth(&mut self) -> I2ETH_W {
        I2ETH_W { w: self }
    }
    #[doc = "Bit 12 - EtherType Enable"]
    #[inline(always)]
    pub fn ethe(&mut self) -> ETHE_W {
        ETHE_W { w: self }
    }
    #[doc = "Bits 13:17 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compa(&mut self) -> COMPA_W {
        COMPA_W { w: self }
    }
    #[doc = "Bit 18 - Compare A Enable"]
    #[inline(always)]
    pub fn compae(&mut self) -> COMPAE_W {
        COMPAE_W { w: self }
    }
    #[doc = "Bits 19:23 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compb(&mut self) -> COMPB_W {
        COMPB_W { w: self }
    }
    #[doc = "Bit 24 - Compare B Enable"]
    #[inline(always)]
    pub fn compbe(&mut self) -> COMPBE_W {
        COMPBE_W { w: self }
    }
    #[doc = "Bits 25:29 - Index of Screening Type 2 Compare Word 0/Word 1 register x"]
    #[inline(always)]
    pub fn compc(&mut self) -> COMPC_W {
        COMPC_W { w: self }
    }
    #[doc = "Bit 30 - Compare C Enable"]
    #[inline(always)]
    pub fn compce(&mut self) -> COMPCE_W {
        COMPCE_W { w: self }
    }
}
