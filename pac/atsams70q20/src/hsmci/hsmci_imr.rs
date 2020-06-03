#[doc = "Reader of register HSMCI_IMR"]
pub type R = crate::R<u32, super::HSMCI_IMR>;
#[doc = "Reader of field `CMDRDY`"]
pub type CMDRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXRDY`"]
pub type RXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRDY`"]
pub type TXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `BLKE`"]
pub type BLKE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTIP`"]
pub type DTIP_R = crate::R<bool, bool>;
#[doc = "Reader of field `NOTBUSY`"]
pub type NOTBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDIOIRQA`"]
pub type SDIOIRQA_R = crate::R<bool, bool>;
#[doc = "Reader of field `SDIOWAIT`"]
pub type SDIOWAIT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSRCV`"]
pub type CSRCV_R = crate::R<bool, bool>;
#[doc = "Reader of field `RINDE`"]
pub type RINDE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RDIRE`"]
pub type RDIRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RCRCE`"]
pub type RCRCE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RENDE`"]
pub type RENDE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTOE`"]
pub type RTOE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DCRCE`"]
pub type DCRCE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTOE`"]
pub type DTOE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSTOE`"]
pub type CSTOE_R = crate::R<bool, bool>;
#[doc = "Reader of field `BLKOVRE`"]
pub type BLKOVRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `FIFOEMPTY`"]
pub type FIFOEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `XFRDONE`"]
pub type XFRDONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACKRCV`"]
pub type ACKRCV_R = crate::R<bool, bool>;
#[doc = "Reader of field `ACKRCVE`"]
pub type ACKRCVE_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE`"]
pub type OVRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `UNRE`"]
pub type UNRE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Command Ready Interrupt Mask"]
    #[inline(always)]
    pub fn cmdrdy(&self) -> CMDRDY_R {
        CMDRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receiver Ready Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Ready Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data Block Ended Interrupt Mask"]
    #[inline(always)]
    pub fn blke(&self) -> BLKE_R {
        BLKE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Transfer in Progress Interrupt Mask"]
    #[inline(always)]
    pub fn dtip(&self) -> DTIP_R {
        DTIP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data Not Busy Interrupt Mask"]
    #[inline(always)]
    pub fn notbusy(&self) -> NOTBUSY_R {
        NOTBUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SDIO Interrupt for Slot A Interrupt Mask"]
    #[inline(always)]
    pub fn sdioirqa(&self) -> SDIOIRQA_R {
        SDIOIRQA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SDIO Read Wait Operation Status Interrupt Mask"]
    #[inline(always)]
    pub fn sdiowait(&self) -> SDIOWAIT_R {
        SDIOWAIT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Completion Signal Received Interrupt Mask"]
    #[inline(always)]
    pub fn csrcv(&self) -> CSRCV_R {
        CSRCV_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Response Index Error Interrupt Mask"]
    #[inline(always)]
    pub fn rinde(&self) -> RINDE_R {
        RINDE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Response Direction Error Interrupt Mask"]
    #[inline(always)]
    pub fn rdire(&self) -> RDIRE_R {
        RDIRE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Response CRC Error Interrupt Mask"]
    #[inline(always)]
    pub fn rcrce(&self) -> RCRCE_R {
        RCRCE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Response End Bit Error Interrupt Mask"]
    #[inline(always)]
    pub fn rende(&self) -> RENDE_R {
        RENDE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Response Time-out Error Interrupt Mask"]
    #[inline(always)]
    pub fn rtoe(&self) -> RTOE_R {
        RTOE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error Interrupt Mask"]
    #[inline(always)]
    pub fn dcrce(&self) -> DCRCE_R {
        DCRCE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Data Time-out Error Interrupt Mask"]
    #[inline(always)]
    pub fn dtoe(&self) -> DTOE_R {
        DTOE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Completion Signal Time-out Error Interrupt Mask"]
    #[inline(always)]
    pub fn cstoe(&self) -> CSTOE_R {
        CSTOE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMA Block Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn blkovre(&self) -> BLKOVRE_R {
        BLKOVRE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn fifoempty(&self) -> FIFOEMPTY_R {
        FIFOEMPTY_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Transfer Done Interrupt Mask"]
    #[inline(always)]
    pub fn xfrdone(&self) -> XFRDONE_R {
        XFRDONE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Boot Operation Acknowledge Received Interrupt Mask"]
    #[inline(always)]
    pub fn ackrcv(&self) -> ACKRCV_R {
        ACKRCV_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Boot Operation Acknowledge Error Interrupt Mask"]
    #[inline(always)]
    pub fn ackrcve(&self) -> ACKRCVE_R {
        ACKRCVE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Underrun Interrupt Mask"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
