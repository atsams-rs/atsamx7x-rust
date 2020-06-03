#[doc = "Reader of register XDMAC_GCFG"]
pub type R = crate::R<u32, super::XDMAC_GCFG>;
#[doc = "Writer for register XDMAC_GCFG"]
pub type W = crate::W<u32, super::XDMAC_GCFG>;
#[doc = "Register XDMAC_GCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::XDMAC_GCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CGDISREG`"]
pub type CGDISREG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CGDISREG`"]
pub struct CGDISREG_W<'a> {
    w: &'a mut W,
}
impl<'a> CGDISREG_W<'a> {
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
#[doc = "Reader of field `CGDISPIPE`"]
pub type CGDISPIPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CGDISPIPE`"]
pub struct CGDISPIPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CGDISPIPE_W<'a> {
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
#[doc = "Reader of field `CGDISFIFO`"]
pub type CGDISFIFO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CGDISFIFO`"]
pub struct CGDISFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> CGDISFIFO_W<'a> {
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
#[doc = "Reader of field `CGDISIF`"]
pub type CGDISIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CGDISIF`"]
pub struct CGDISIF_W<'a> {
    w: &'a mut W,
}
impl<'a> CGDISIF_W<'a> {
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
#[doc = "Reader of field `BXKBEN`"]
pub type BXKBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BXKBEN`"]
pub struct BXKBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BXKBEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Configuration Registers Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisreg(&self) -> CGDISREG_R {
        CGDISREG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pipeline Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdispipe(&self) -> CGDISPIPE_R {
        CGDISPIPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FIFO Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisfifo(&self) -> CGDISFIFO_R {
        CGDISFIFO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bus Interface Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisif(&self) -> CGDISIF_R {
        CGDISIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Boundary X Kilobyte Enable"]
    #[inline(always)]
    pub fn bxkben(&self) -> BXKBEN_R {
        BXKBEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configuration Registers Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisreg(&mut self) -> CGDISREG_W {
        CGDISREG_W { w: self }
    }
    #[doc = "Bit 1 - Pipeline Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdispipe(&mut self) -> CGDISPIPE_W {
        CGDISPIPE_W { w: self }
    }
    #[doc = "Bit 2 - FIFO Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisfifo(&mut self) -> CGDISFIFO_W {
        CGDISFIFO_W { w: self }
    }
    #[doc = "Bit 3 - Bus Interface Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisif(&mut self) -> CGDISIF_W {
        CGDISIF_W { w: self }
    }
    #[doc = "Bit 8 - Boundary X Kilobyte Enable"]
    #[inline(always)]
    pub fn bxkben(&mut self) -> BXKBEN_W {
        BXKBEN_W { w: self }
    }
}
