#[doc = "Writer for register ISI_IDR"]
pub type W = crate::W<u32, super::ISI_IDR>;
#[doc = "Register ISI_IDR `reset()`'s with value 0"]
impl crate::ResetValue for super::ISI_IDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DIS_DONE`"]
pub struct DIS_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_DONE_W<'a> {
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
#[doc = "Write proxy for field `SRST`"]
pub struct SRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SRST_W<'a> {
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
#[doc = "Write proxy for field `VSYNC`"]
pub struct VSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> VSYNC_W<'a> {
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
#[doc = "Write proxy for field `PXFR_DONE`"]
pub struct PXFR_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> PXFR_DONE_W<'a> {
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
#[doc = "Write proxy for field `CXFR_DONE`"]
pub struct CXFR_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CXFR_DONE_W<'a> {
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
#[doc = "Write proxy for field `P_OVR`"]
pub struct P_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> P_OVR_W<'a> {
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
#[doc = "Write proxy for field `C_OVR`"]
pub struct C_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> C_OVR_W<'a> {
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
#[doc = "Write proxy for field `CRC_ERR`"]
pub struct CRC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Write proxy for field `FR_OVR`"]
pub struct FR_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> FR_OVR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl W {
    #[doc = "Bit 1 - Disable Done Interrupt Disable"]
    #[inline(always)]
    pub fn dis_done(&mut self) -> DIS_DONE_W {
        DIS_DONE_W { w: self }
    }
    #[doc = "Bit 2 - Software Reset Interrupt Disable"]
    #[inline(always)]
    pub fn srst(&mut self) -> SRST_W {
        SRST_W { w: self }
    }
    #[doc = "Bit 10 - Vertical Synchronization Interrupt Disable"]
    #[inline(always)]
    pub fn vsync(&mut self) -> VSYNC_W {
        VSYNC_W { w: self }
    }
    #[doc = "Bit 16 - Preview DMA Transfer Done Interrupt Disable"]
    #[inline(always)]
    pub fn pxfr_done(&mut self) -> PXFR_DONE_W {
        PXFR_DONE_W { w: self }
    }
    #[doc = "Bit 17 - Codec DMA Transfer Done Interrupt Disable"]
    #[inline(always)]
    pub fn cxfr_done(&mut self) -> CXFR_DONE_W {
        CXFR_DONE_W { w: self }
    }
    #[doc = "Bit 24 - Preview Datapath Overflow Interrupt Disable"]
    #[inline(always)]
    pub fn p_ovr(&mut self) -> P_OVR_W {
        P_OVR_W { w: self }
    }
    #[doc = "Bit 25 - Codec Datapath Overflow Interrupt Disable"]
    #[inline(always)]
    pub fn c_ovr(&mut self) -> C_OVR_W {
        C_OVR_W { w: self }
    }
    #[doc = "Bit 26 - Embedded Synchronization CRC Error Interrupt Disable"]
    #[inline(always)]
    pub fn crc_err(&mut self) -> CRC_ERR_W {
        CRC_ERR_W { w: self }
    }
    #[doc = "Bit 27 - Frame Rate Overflow Interrupt Disable"]
    #[inline(always)]
    pub fn fr_ovr(&mut self) -> FR_OVR_W {
        FR_OVR_W { w: self }
    }
}
