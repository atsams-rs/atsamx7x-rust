#[doc = "Reader of register ISI_IMR"]
pub type R = crate::R<u32, super::ISI_IMR>;
#[doc = "Reader of field `DIS_DONE`"]
pub type DIS_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRST`"]
pub type SRST_R = crate::R<bool, bool>;
#[doc = "Reader of field `VSYNC`"]
pub type VSYNC_R = crate::R<bool, bool>;
#[doc = "Reader of field `PXFR_DONE`"]
pub type PXFR_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CXFR_DONE`"]
pub type CXFR_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `P_OVR`"]
pub type P_OVR_R = crate::R<bool, bool>;
#[doc = "Reader of field `C_OVR`"]
pub type C_OVR_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRC_ERR`"]
pub type CRC_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `FR_OVR`"]
pub type FR_OVR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - Module Disable Operation Completed"]
    #[inline(always)]
    pub fn dis_done(&self) -> DIS_DONE_R {
        DIS_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software Reset Completed"]
    #[inline(always)]
    pub fn srst(&self) -> SRST_R {
        SRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Vertical Synchronization"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Preview DMA Transfer Completed"]
    #[inline(always)]
    pub fn pxfr_done(&self) -> PXFR_DONE_R {
        PXFR_DONE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Codec DMA Transfer Completed"]
    #[inline(always)]
    pub fn cxfr_done(&self) -> CXFR_DONE_R {
        CXFR_DONE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Preview FIFO Overflow"]
    #[inline(always)]
    pub fn p_ovr(&self) -> P_OVR_R {
        P_OVR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Codec FIFO Overflow"]
    #[inline(always)]
    pub fn c_ovr(&self) -> C_OVR_R {
        C_OVR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CRC Synchronization Error"]
    #[inline(always)]
    pub fn crc_err(&self) -> CRC_ERR_R {
        CRC_ERR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Frame Rate Overrun"]
    #[inline(always)]
    pub fn fr_ovr(&self) -> FR_OVR_R {
        FR_OVR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
