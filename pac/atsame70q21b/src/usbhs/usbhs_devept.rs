#[doc = "Reader of register USBHS_DEVEPT"]
pub type R = crate::R<u32, super::USBHS_DEVEPT>;
#[doc = "Writer for register USBHS_DEVEPT"]
pub type W = crate::W<u32, super::USBHS_DEVEPT>;
#[doc = "Register USBHS_DEVEPT `reset()`'s with value 0"]
impl crate::ResetValue for super::USBHS_DEVEPT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EPEN0`"]
pub type EPEN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN0`"]
pub struct EPEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN0_W<'a> {
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
#[doc = "Reader of field `EPEN1`"]
pub type EPEN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN1`"]
pub struct EPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN1_W<'a> {
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
#[doc = "Reader of field `EPEN2`"]
pub type EPEN2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN2`"]
pub struct EPEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN2_W<'a> {
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
#[doc = "Reader of field `EPEN3`"]
pub type EPEN3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN3`"]
pub struct EPEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN3_W<'a> {
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
#[doc = "Reader of field `EPEN4`"]
pub type EPEN4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN4`"]
pub struct EPEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN4_W<'a> {
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
#[doc = "Reader of field `EPEN5`"]
pub type EPEN5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN5`"]
pub struct EPEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN5_W<'a> {
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
#[doc = "Reader of field `EPEN6`"]
pub type EPEN6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN6`"]
pub struct EPEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN6_W<'a> {
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
#[doc = "Reader of field `EPEN7`"]
pub type EPEN7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN7`"]
pub struct EPEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `EPEN8`"]
pub type EPEN8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN8`"]
pub struct EPEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN8_W<'a> {
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
#[doc = "Reader of field `EPEN9`"]
pub type EPEN9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPEN9`"]
pub struct EPEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `EPRST0`"]
pub type EPRST0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPRST0`"]
pub struct EPRST0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `EPRST1`"]
pub type EPRST1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPRST1`"]
pub struct EPRST1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `EPRST2`"]
pub type EPRST2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPRST2`"]
pub struct EPRST2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST2_W<'a> {
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
#[doc = "Reader of field `EPRST3`"]
pub type EPRST3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPRST3`"]
pub struct EPRST3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `EPRST4`"]
pub type EPRST4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPRST4`"]
pub struct EPRST4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `EPRST5`"]
pub type EPRST5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPRST5`"]
pub struct EPRST5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `EPRST6`"]
pub type EPRST6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPRST6`"]
pub struct EPRST6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `EPRST7`"]
pub type EPRST7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPRST7`"]
pub struct EPRST7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST7_W<'a> {
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
#[doc = "Reader of field `EPRST8`"]
pub type EPRST8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPRST8`"]
pub struct EPRST8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST8_W<'a> {
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
#[doc = "Reader of field `EPRST9`"]
pub type EPRST9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPRST9`"]
pub struct EPRST9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRST9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Endpoint 0 Enable"]
    #[inline(always)]
    pub fn epen0(&self) -> EPEN0_R {
        EPEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 Enable"]
    #[inline(always)]
    pub fn epen1(&self) -> EPEN1_R {
        EPEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 Enable"]
    #[inline(always)]
    pub fn epen2(&self) -> EPEN2_R {
        EPEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 Enable"]
    #[inline(always)]
    pub fn epen3(&self) -> EPEN3_R {
        EPEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 Enable"]
    #[inline(always)]
    pub fn epen4(&self) -> EPEN4_R {
        EPEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 Enable"]
    #[inline(always)]
    pub fn epen5(&self) -> EPEN5_R {
        EPEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 Enable"]
    #[inline(always)]
    pub fn epen6(&self) -> EPEN6_R {
        EPEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7 Enable"]
    #[inline(always)]
    pub fn epen7(&self) -> EPEN7_R {
        EPEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint 8 Enable"]
    #[inline(always)]
    pub fn epen8(&self) -> EPEN8_R {
        EPEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint 9 Enable"]
    #[inline(always)]
    pub fn epen9(&self) -> EPEN9_R {
        EPEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint 0 Reset"]
    #[inline(always)]
    pub fn eprst0(&self) -> EPRST0_R {
        EPRST0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint 1 Reset"]
    #[inline(always)]
    pub fn eprst1(&self) -> EPRST1_R {
        EPRST1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Endpoint 2 Reset"]
    #[inline(always)]
    pub fn eprst2(&self) -> EPRST2_R {
        EPRST2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Endpoint 3 Reset"]
    #[inline(always)]
    pub fn eprst3(&self) -> EPRST3_R {
        EPRST3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Endpoint 4 Reset"]
    #[inline(always)]
    pub fn eprst4(&self) -> EPRST4_R {
        EPRST4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Endpoint 5 Reset"]
    #[inline(always)]
    pub fn eprst5(&self) -> EPRST5_R {
        EPRST5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Endpoint 6 Reset"]
    #[inline(always)]
    pub fn eprst6(&self) -> EPRST6_R {
        EPRST6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Endpoint 7 Reset"]
    #[inline(always)]
    pub fn eprst7(&self) -> EPRST7_R {
        EPRST7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Endpoint 8 Reset"]
    #[inline(always)]
    pub fn eprst8(&self) -> EPRST8_R {
        EPRST8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Endpoint 9 Reset"]
    #[inline(always)]
    pub fn eprst9(&self) -> EPRST9_R {
        EPRST9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint 0 Enable"]
    #[inline(always)]
    pub fn epen0(&mut self) -> EPEN0_W {
        EPEN0_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint 1 Enable"]
    #[inline(always)]
    pub fn epen1(&mut self) -> EPEN1_W {
        EPEN1_W { w: self }
    }
    #[doc = "Bit 2 - Endpoint 2 Enable"]
    #[inline(always)]
    pub fn epen2(&mut self) -> EPEN2_W {
        EPEN2_W { w: self }
    }
    #[doc = "Bit 3 - Endpoint 3 Enable"]
    #[inline(always)]
    pub fn epen3(&mut self) -> EPEN3_W {
        EPEN3_W { w: self }
    }
    #[doc = "Bit 4 - Endpoint 4 Enable"]
    #[inline(always)]
    pub fn epen4(&mut self) -> EPEN4_W {
        EPEN4_W { w: self }
    }
    #[doc = "Bit 5 - Endpoint 5 Enable"]
    #[inline(always)]
    pub fn epen5(&mut self) -> EPEN5_W {
        EPEN5_W { w: self }
    }
    #[doc = "Bit 6 - Endpoint 6 Enable"]
    #[inline(always)]
    pub fn epen6(&mut self) -> EPEN6_W {
        EPEN6_W { w: self }
    }
    #[doc = "Bit 7 - Endpoint 7 Enable"]
    #[inline(always)]
    pub fn epen7(&mut self) -> EPEN7_W {
        EPEN7_W { w: self }
    }
    #[doc = "Bit 8 - Endpoint 8 Enable"]
    #[inline(always)]
    pub fn epen8(&mut self) -> EPEN8_W {
        EPEN8_W { w: self }
    }
    #[doc = "Bit 9 - Endpoint 9 Enable"]
    #[inline(always)]
    pub fn epen9(&mut self) -> EPEN9_W {
        EPEN9_W { w: self }
    }
    #[doc = "Bit 16 - Endpoint 0 Reset"]
    #[inline(always)]
    pub fn eprst0(&mut self) -> EPRST0_W {
        EPRST0_W { w: self }
    }
    #[doc = "Bit 17 - Endpoint 1 Reset"]
    #[inline(always)]
    pub fn eprst1(&mut self) -> EPRST1_W {
        EPRST1_W { w: self }
    }
    #[doc = "Bit 18 - Endpoint 2 Reset"]
    #[inline(always)]
    pub fn eprst2(&mut self) -> EPRST2_W {
        EPRST2_W { w: self }
    }
    #[doc = "Bit 19 - Endpoint 3 Reset"]
    #[inline(always)]
    pub fn eprst3(&mut self) -> EPRST3_W {
        EPRST3_W { w: self }
    }
    #[doc = "Bit 20 - Endpoint 4 Reset"]
    #[inline(always)]
    pub fn eprst4(&mut self) -> EPRST4_W {
        EPRST4_W { w: self }
    }
    #[doc = "Bit 21 - Endpoint 5 Reset"]
    #[inline(always)]
    pub fn eprst5(&mut self) -> EPRST5_W {
        EPRST5_W { w: self }
    }
    #[doc = "Bit 22 - Endpoint 6 Reset"]
    #[inline(always)]
    pub fn eprst6(&mut self) -> EPRST6_W {
        EPRST6_W { w: self }
    }
    #[doc = "Bit 23 - Endpoint 7 Reset"]
    #[inline(always)]
    pub fn eprst7(&mut self) -> EPRST7_W {
        EPRST7_W { w: self }
    }
    #[doc = "Bit 24 - Endpoint 8 Reset"]
    #[inline(always)]
    pub fn eprst8(&mut self) -> EPRST8_W {
        EPRST8_W { w: self }
    }
    #[doc = "Bit 25 - Endpoint 9 Reset"]
    #[inline(always)]
    pub fn eprst9(&mut self) -> EPRST9_W {
        EPRST9_W { w: self }
    }
}
