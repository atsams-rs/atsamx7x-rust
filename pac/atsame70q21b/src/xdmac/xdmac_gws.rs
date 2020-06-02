#[doc = "Reader of register XDMAC_GWS"]
pub type R = crate::R<u32, super::XDMAC_GWS>;
#[doc = "Writer for register XDMAC_GWS"]
pub type W = crate::W<u32, super::XDMAC_GWS>;
#[doc = "Register XDMAC_GWS `reset()`'s with value 0"]
impl crate::ResetValue for super::XDMAC_GWS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WS0`"]
pub type WS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS0`"]
pub struct WS0_W<'a> {
    w: &'a mut W,
}
impl<'a> WS0_W<'a> {
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
#[doc = "Reader of field `WS1`"]
pub type WS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS1`"]
pub struct WS1_W<'a> {
    w: &'a mut W,
}
impl<'a> WS1_W<'a> {
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
#[doc = "Reader of field `WS2`"]
pub type WS2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS2`"]
pub struct WS2_W<'a> {
    w: &'a mut W,
}
impl<'a> WS2_W<'a> {
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
#[doc = "Reader of field `WS3`"]
pub type WS3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS3`"]
pub struct WS3_W<'a> {
    w: &'a mut W,
}
impl<'a> WS3_W<'a> {
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
#[doc = "Reader of field `WS4`"]
pub type WS4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS4`"]
pub struct WS4_W<'a> {
    w: &'a mut W,
}
impl<'a> WS4_W<'a> {
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
#[doc = "Reader of field `WS5`"]
pub type WS5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS5`"]
pub struct WS5_W<'a> {
    w: &'a mut W,
}
impl<'a> WS5_W<'a> {
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
#[doc = "Reader of field `WS6`"]
pub type WS6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS6`"]
pub struct WS6_W<'a> {
    w: &'a mut W,
}
impl<'a> WS6_W<'a> {
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
#[doc = "Reader of field `WS7`"]
pub type WS7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS7`"]
pub struct WS7_W<'a> {
    w: &'a mut W,
}
impl<'a> WS7_W<'a> {
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
#[doc = "Reader of field `WS8`"]
pub type WS8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS8`"]
pub struct WS8_W<'a> {
    w: &'a mut W,
}
impl<'a> WS8_W<'a> {
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
#[doc = "Reader of field `WS9`"]
pub type WS9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS9`"]
pub struct WS9_W<'a> {
    w: &'a mut W,
}
impl<'a> WS9_W<'a> {
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
#[doc = "Reader of field `WS10`"]
pub type WS10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS10`"]
pub struct WS10_W<'a> {
    w: &'a mut W,
}
impl<'a> WS10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `WS11`"]
pub type WS11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS11`"]
pub struct WS11_W<'a> {
    w: &'a mut W,
}
impl<'a> WS11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `WS12`"]
pub type WS12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS12`"]
pub struct WS12_W<'a> {
    w: &'a mut W,
}
impl<'a> WS12_W<'a> {
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
#[doc = "Reader of field `WS13`"]
pub type WS13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS13`"]
pub struct WS13_W<'a> {
    w: &'a mut W,
}
impl<'a> WS13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `WS14`"]
pub type WS14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS14`"]
pub struct WS14_W<'a> {
    w: &'a mut W,
}
impl<'a> WS14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `WS15`"]
pub type WS15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS15`"]
pub struct WS15_W<'a> {
    w: &'a mut W,
}
impl<'a> WS15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `WS16`"]
pub type WS16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS16`"]
pub struct WS16_W<'a> {
    w: &'a mut W,
}
impl<'a> WS16_W<'a> {
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
#[doc = "Reader of field `WS17`"]
pub type WS17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS17`"]
pub struct WS17_W<'a> {
    w: &'a mut W,
}
impl<'a> WS17_W<'a> {
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
#[doc = "Reader of field `WS18`"]
pub type WS18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS18`"]
pub struct WS18_W<'a> {
    w: &'a mut W,
}
impl<'a> WS18_W<'a> {
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
#[doc = "Reader of field `WS19`"]
pub type WS19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS19`"]
pub struct WS19_W<'a> {
    w: &'a mut W,
}
impl<'a> WS19_W<'a> {
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
#[doc = "Reader of field `WS20`"]
pub type WS20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS20`"]
pub struct WS20_W<'a> {
    w: &'a mut W,
}
impl<'a> WS20_W<'a> {
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
#[doc = "Reader of field `WS21`"]
pub type WS21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS21`"]
pub struct WS21_W<'a> {
    w: &'a mut W,
}
impl<'a> WS21_W<'a> {
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
#[doc = "Reader of field `WS22`"]
pub type WS22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS22`"]
pub struct WS22_W<'a> {
    w: &'a mut W,
}
impl<'a> WS22_W<'a> {
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
#[doc = "Reader of field `WS23`"]
pub type WS23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS23`"]
pub struct WS23_W<'a> {
    w: &'a mut W,
}
impl<'a> WS23_W<'a> {
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
    #[doc = "Bit 0 - XDMAC Channel 0 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws0(&self) -> WS0_R {
        WS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws1(&self) -> WS1_R {
        WS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws2(&self) -> WS2_R {
        WS2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws3(&self) -> WS3_R {
        WS3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws4(&self) -> WS4_R {
        WS4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws5(&self) -> WS5_R {
        WS5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws6(&self) -> WS6_R {
        WS6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws7(&self) -> WS7_R {
        WS7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws8(&self) -> WS8_R {
        WS8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws9(&self) -> WS9_R {
        WS9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws10(&self) -> WS10_R {
        WS10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws11(&self) -> WS11_R {
        WS11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws12(&self) -> WS12_R {
        WS12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws13(&self) -> WS13_R {
        WS13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws14(&self) -> WS14_R {
        WS14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws15(&self) -> WS15_R {
        WS15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws16(&self) -> WS16_R {
        WS16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws17(&self) -> WS17_R {
        WS17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws18(&self) -> WS18_R {
        WS18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws19(&self) -> WS19_R {
        WS19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws20(&self) -> WS20_R {
        WS20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws21(&self) -> WS21_R {
        WS21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws22(&self) -> WS22_R {
        WS22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws23(&self) -> WS23_R {
        WS23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws0(&mut self) -> WS0_W {
        WS0_W { w: self }
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws1(&mut self) -> WS1_W {
        WS1_W { w: self }
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws2(&mut self) -> WS2_W {
        WS2_W { w: self }
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws3(&mut self) -> WS3_W {
        WS3_W { w: self }
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws4(&mut self) -> WS4_W {
        WS4_W { w: self }
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws5(&mut self) -> WS5_W {
        WS5_W { w: self }
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws6(&mut self) -> WS6_W {
        WS6_W { w: self }
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws7(&mut self) -> WS7_W {
        WS7_W { w: self }
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws8(&mut self) -> WS8_W {
        WS8_W { w: self }
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws9(&mut self) -> WS9_W {
        WS9_W { w: self }
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws10(&mut self) -> WS10_W {
        WS10_W { w: self }
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws11(&mut self) -> WS11_W {
        WS11_W { w: self }
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws12(&mut self) -> WS12_W {
        WS12_W { w: self }
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws13(&mut self) -> WS13_W {
        WS13_W { w: self }
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws14(&mut self) -> WS14_W {
        WS14_W { w: self }
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws15(&mut self) -> WS15_W {
        WS15_W { w: self }
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws16(&mut self) -> WS16_W {
        WS16_W { w: self }
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws17(&mut self) -> WS17_W {
        WS17_W { w: self }
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws18(&mut self) -> WS18_W {
        WS18_W { w: self }
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws19(&mut self) -> WS19_W {
        WS19_W { w: self }
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws20(&mut self) -> WS20_W {
        WS20_W { w: self }
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws21(&mut self) -> WS21_W {
        WS21_W { w: self }
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws22(&mut self) -> WS22_W {
        WS22_W { w: self }
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws23(&mut self) -> WS23_W {
        WS23_W { w: self }
    }
}
