#[doc = "Reader of register USBHS_DEVDMASTATUS"]
pub type R = crate::R<u32, super::USBHS_DEVDMASTATUS>;
#[doc = "Writer for register USBHS_DEVDMASTATUS"]
pub type W = crate::W<u32, super::USBHS_DEVDMASTATUS>;
#[doc = "Register USBHS_DEVDMASTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::USBHS_DEVDMASTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHANN_ENB`"]
pub type CHANN_ENB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHANN_ENB`"]
pub struct CHANN_ENB_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANN_ENB_W<'a> {
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
#[doc = "Reader of field `CHANN_ACT`"]
pub type CHANN_ACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHANN_ACT`"]
pub struct CHANN_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANN_ACT_W<'a> {
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
#[doc = "Reader of field `END_TR_ST`"]
pub type END_TR_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `END_TR_ST`"]
pub struct END_TR_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> END_TR_ST_W<'a> {
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
#[doc = "Reader of field `END_BF_ST`"]
pub type END_BF_ST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `END_BF_ST`"]
pub struct END_BF_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> END_BF_ST_W<'a> {
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
#[doc = "Reader of field `DESC_LDST`"]
pub type DESC_LDST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DESC_LDST`"]
pub struct DESC_LDST_W<'a> {
    w: &'a mut W,
}
impl<'a> DESC_LDST_W<'a> {
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
#[doc = "Reader of field `BUFF_COUNT`"]
pub type BUFF_COUNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BUFF_COUNT`"]
pub struct BUFF_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFF_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel Enable Status"]
    #[inline(always)]
    pub fn chann_enb(&self) -> CHANN_ENB_R {
        CHANN_ENB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel Active Status"]
    #[inline(always)]
    pub fn chann_act(&self) -> CHANN_ACT_R {
        CHANN_ACT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End of Channel Transfer Status"]
    #[inline(always)]
    pub fn end_tr_st(&self) -> END_TR_ST_R {
        END_TR_ST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of Channel Buffer Status"]
    #[inline(always)]
    pub fn end_bf_st(&self) -> END_BF_ST_R {
        END_BF_ST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Descriptor Loaded Status"]
    #[inline(always)]
    pub fn desc_ldst(&self) -> DESC_LDST_R {
        DESC_LDST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Buffer Byte Count"]
    #[inline(always)]
    pub fn buff_count(&self) -> BUFF_COUNT_R {
        BUFF_COUNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Enable Status"]
    #[inline(always)]
    pub fn chann_enb(&mut self) -> CHANN_ENB_W {
        CHANN_ENB_W { w: self }
    }
    #[doc = "Bit 1 - Channel Active Status"]
    #[inline(always)]
    pub fn chann_act(&mut self) -> CHANN_ACT_W {
        CHANN_ACT_W { w: self }
    }
    #[doc = "Bit 4 - End of Channel Transfer Status"]
    #[inline(always)]
    pub fn end_tr_st(&mut self) -> END_TR_ST_W {
        END_TR_ST_W { w: self }
    }
    #[doc = "Bit 5 - End of Channel Buffer Status"]
    #[inline(always)]
    pub fn end_bf_st(&mut self) -> END_BF_ST_W {
        END_BF_ST_W { w: self }
    }
    #[doc = "Bit 6 - Descriptor Loaded Status"]
    #[inline(always)]
    pub fn desc_ldst(&mut self) -> DESC_LDST_W {
        DESC_LDST_W { w: self }
    }
    #[doc = "Bits 16:31 - Buffer Byte Count"]
    #[inline(always)]
    pub fn buff_count(&mut self) -> BUFF_COUNT_W {
        BUFF_COUNT_W { w: self }
    }
}
