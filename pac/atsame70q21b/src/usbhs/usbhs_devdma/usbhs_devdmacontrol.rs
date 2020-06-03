#[doc = "Reader of register USBHS_DEVDMACONTROL"]
pub type R = crate::R<u32, super::USBHS_DEVDMACONTROL>;
#[doc = "Writer for register USBHS_DEVDMACONTROL"]
pub type W = crate::W<u32, super::USBHS_DEVDMACONTROL>;
#[doc = "Register USBHS_DEVDMACONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::USBHS_DEVDMACONTROL {
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
#[doc = "Reader of field `LDNXT_DSC`"]
pub type LDNXT_DSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LDNXT_DSC`"]
pub struct LDNXT_DSC_W<'a> {
    w: &'a mut W,
}
impl<'a> LDNXT_DSC_W<'a> {
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
#[doc = "Reader of field `END_TR_EN`"]
pub type END_TR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `END_TR_EN`"]
pub struct END_TR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> END_TR_EN_W<'a> {
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
#[doc = "Reader of field `END_B_EN`"]
pub type END_B_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `END_B_EN`"]
pub struct END_B_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> END_B_EN_W<'a> {
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
#[doc = "Reader of field `END_TR_IT`"]
pub type END_TR_IT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `END_TR_IT`"]
pub struct END_TR_IT_W<'a> {
    w: &'a mut W,
}
impl<'a> END_TR_IT_W<'a> {
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
#[doc = "Reader of field `END_BUFFIT`"]
pub type END_BUFFIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `END_BUFFIT`"]
pub struct END_BUFFIT_W<'a> {
    w: &'a mut W,
}
impl<'a> END_BUFFIT_W<'a> {
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
#[doc = "Reader of field `DESC_LD_IT`"]
pub type DESC_LD_IT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DESC_LD_IT`"]
pub struct DESC_LD_IT_W<'a> {
    w: &'a mut W,
}
impl<'a> DESC_LD_IT_W<'a> {
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
#[doc = "Reader of field `BURST_LCK`"]
pub type BURST_LCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BURST_LCK`"]
pub struct BURST_LCK_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_LCK_W<'a> {
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
#[doc = "Reader of field `BUFF_LENGTH`"]
pub type BUFF_LENGTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BUFF_LENGTH`"]
pub struct BUFF_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFF_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel Enable Command"]
    #[inline(always)]
    pub fn chann_enb(&self) -> CHANN_ENB_R {
        CHANN_ENB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Load Next Channel Transfer Descriptor Enable Command"]
    #[inline(always)]
    pub fn ldnxt_dsc(&self) -> LDNXT_DSC_R {
        LDNXT_DSC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of Transfer Enable Control (OUT transfers only)"]
    #[inline(always)]
    pub fn end_tr_en(&self) -> END_TR_EN_R {
        END_TR_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of Buffer Enable Control"]
    #[inline(always)]
    pub fn end_b_en(&self) -> END_B_EN_R {
        END_B_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End of Transfer Interrupt Enable"]
    #[inline(always)]
    pub fn end_tr_it(&self) -> END_TR_IT_R {
        END_TR_IT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn end_buffit(&self) -> END_BUFFIT_R {
        END_BUFFIT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Descriptor Loaded Interrupt Enable"]
    #[inline(always)]
    pub fn desc_ld_it(&self) -> DESC_LD_IT_R {
        DESC_LD_IT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Burst Lock Enable"]
    #[inline(always)]
    pub fn burst_lck(&self) -> BURST_LCK_R {
        BURST_LCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Buffer Byte Length (Write-only)"]
    #[inline(always)]
    pub fn buff_length(&self) -> BUFF_LENGTH_R {
        BUFF_LENGTH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Enable Command"]
    #[inline(always)]
    pub fn chann_enb(&mut self) -> CHANN_ENB_W {
        CHANN_ENB_W { w: self }
    }
    #[doc = "Bit 1 - Load Next Channel Transfer Descriptor Enable Command"]
    #[inline(always)]
    pub fn ldnxt_dsc(&mut self) -> LDNXT_DSC_W {
        LDNXT_DSC_W { w: self }
    }
    #[doc = "Bit 2 - End of Transfer Enable Control (OUT transfers only)"]
    #[inline(always)]
    pub fn end_tr_en(&mut self) -> END_TR_EN_W {
        END_TR_EN_W { w: self }
    }
    #[doc = "Bit 3 - End of Buffer Enable Control"]
    #[inline(always)]
    pub fn end_b_en(&mut self) -> END_B_EN_W {
        END_B_EN_W { w: self }
    }
    #[doc = "Bit 4 - End of Transfer Interrupt Enable"]
    #[inline(always)]
    pub fn end_tr_it(&mut self) -> END_TR_IT_W {
        END_TR_IT_W { w: self }
    }
    #[doc = "Bit 5 - End of Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn end_buffit(&mut self) -> END_BUFFIT_W {
        END_BUFFIT_W { w: self }
    }
    #[doc = "Bit 6 - Descriptor Loaded Interrupt Enable"]
    #[inline(always)]
    pub fn desc_ld_it(&mut self) -> DESC_LD_IT_W {
        DESC_LD_IT_W { w: self }
    }
    #[doc = "Bit 7 - Burst Lock Enable"]
    #[inline(always)]
    pub fn burst_lck(&mut self) -> BURST_LCK_W {
        BURST_LCK_W { w: self }
    }
    #[doc = "Bits 16:31 - Buffer Byte Length (Write-only)"]
    #[inline(always)]
    pub fn buff_length(&mut self) -> BUFF_LENGTH_W {
        BUFF_LENGTH_W { w: self }
    }
}
