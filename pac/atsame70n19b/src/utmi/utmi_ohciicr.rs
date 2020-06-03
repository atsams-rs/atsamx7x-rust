#[doc = "Reader of register UTMI_OHCIICR"]
pub type R = crate::R<u32, super::UTMI_OHCIICR>;
#[doc = "Writer for register UTMI_OHCIICR"]
pub type W = crate::W<u32, super::UTMI_OHCIICR>;
#[doc = "Register UTMI_OHCIICR `reset()`'s with value 0"]
impl crate::ResetValue for super::UTMI_OHCIICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RES0`"]
pub type RES0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RES0`"]
pub struct RES0_W<'a> {
    w: &'a mut W,
}
impl<'a> RES0_W<'a> {
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
#[doc = "Reader of field `ARIE`"]
pub type ARIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ARIE`"]
pub struct ARIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARIE_W<'a> {
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
#[doc = "Reader of field `APPSTART`"]
pub type APPSTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APPSTART`"]
pub struct APPSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> APPSTART_W<'a> {
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
#[doc = "Reader of field `UDPPUDIS`"]
pub type UDPPUDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UDPPUDIS`"]
pub struct UDPPUDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> UDPPUDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB PORTx Reset"]
    #[inline(always)]
    pub fn res0(&self) -> RES0_R {
        RES0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - OHCI Asynchronous Resume Interrupt Enable"]
    #[inline(always)]
    pub fn arie(&self) -> ARIE_R {
        ARIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn appstart(&self) -> APPSTART_R {
        APPSTART_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 23 - USB Device Pull-up Disable"]
    #[inline(always)]
    pub fn udppudis(&self) -> UDPPUDIS_R {
        UDPPUDIS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB PORTx Reset"]
    #[inline(always)]
    pub fn res0(&mut self) -> RES0_W {
        RES0_W { w: self }
    }
    #[doc = "Bit 4 - OHCI Asynchronous Resume Interrupt Enable"]
    #[inline(always)]
    pub fn arie(&mut self) -> ARIE_W {
        ARIE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn appstart(&mut self) -> APPSTART_W {
        APPSTART_W { w: self }
    }
    #[doc = "Bit 23 - USB Device Pull-up Disable"]
    #[inline(always)]
    pub fn udppudis(&mut self) -> UDPPUDIS_W {
        UDPPUDIS_W { w: self }
    }
}
