#[doc = "Reader of register HSMCI_CFG"]
pub type R = crate::R<u32, super::HSMCI_CFG>;
#[doc = "Writer for register HSMCI_CFG"]
pub type W = crate::W<u32, super::HSMCI_CFG>;
#[doc = "Register HSMCI_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::HSMCI_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFOMODE`"]
pub type FIFOMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIFOMODE`"]
pub struct FIFOMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOMODE_W<'a> {
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
#[doc = "Reader of field `FERRCTRL`"]
pub type FERRCTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FERRCTRL`"]
pub struct FERRCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FERRCTRL_W<'a> {
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
#[doc = "Reader of field `HSMODE`"]
pub type HSMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSMODE`"]
pub struct HSMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSMODE_W<'a> {
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
#[doc = "Reader of field `LSYNC`"]
pub type LSYNC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSYNC`"]
pub struct LSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> LSYNC_W<'a> {
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
impl R {
    #[doc = "Bit 0 - HSMCI Internal FIFO control mode"]
    #[inline(always)]
    pub fn fifomode(&self) -> FIFOMODE_R {
        FIFOMODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flow Error flag reset control mode"]
    #[inline(always)]
    pub fn ferrctrl(&self) -> FERRCTRL_R {
        FERRCTRL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - High Speed Mode"]
    #[inline(always)]
    pub fn hsmode(&self) -> HSMODE_R {
        HSMODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Synchronize on the last block"]
    #[inline(always)]
    pub fn lsync(&self) -> LSYNC_R {
        LSYNC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HSMCI Internal FIFO control mode"]
    #[inline(always)]
    pub fn fifomode(&mut self) -> FIFOMODE_W {
        FIFOMODE_W { w: self }
    }
    #[doc = "Bit 4 - Flow Error flag reset control mode"]
    #[inline(always)]
    pub fn ferrctrl(&mut self) -> FERRCTRL_W {
        FERRCTRL_W { w: self }
    }
    #[doc = "Bit 8 - High Speed Mode"]
    #[inline(always)]
    pub fn hsmode(&mut self) -> HSMODE_W {
        HSMODE_W { w: self }
    }
    #[doc = "Bit 12 - Synchronize on the last block"]
    #[inline(always)]
    pub fn lsync(&mut self) -> LSYNC_W {
        LSYNC_W { w: self }
    }
}
