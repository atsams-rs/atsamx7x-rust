#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDRDY` reader - Command Ready (cleared by writing in HSMCI_CMDR)"]
pub type CMDRDY_R = crate::BitReader<bool>;
#[doc = "Field `RXRDY` reader - Receiver Ready (cleared by reading HSMCI_RDR)"]
pub type RXRDY_R = crate::BitReader<bool>;
#[doc = "Field `TXRDY` reader - Transmit Ready (cleared by writing in HSMCI_TDR)"]
pub type TXRDY_R = crate::BitReader<bool>;
#[doc = "Field `BLKE` reader - Data Block Ended (cleared on read)"]
pub type BLKE_R = crate::BitReader<bool>;
#[doc = "Field `DTIP` reader - Data Transfer in Progress (cleared at the end of CRC16 calculation)"]
pub type DTIP_R = crate::BitReader<bool>;
#[doc = "Field `NOTBUSY` reader - HSMCI Not Busy"]
pub type NOTBUSY_R = crate::BitReader<bool>;
#[doc = "Field `SDIOIRQA` reader - SDIO Interrupt for Slot A (cleared on read)"]
pub type SDIOIRQA_R = crate::BitReader<bool>;
#[doc = "Field `SDIOWAIT` reader - SDIO Read Wait Operation Status"]
pub type SDIOWAIT_R = crate::BitReader<bool>;
#[doc = "Field `CSRCV` reader - CE-ATA Completion Signal Received (cleared on read)"]
pub type CSRCV_R = crate::BitReader<bool>;
#[doc = "Field `RINDE` reader - Response Index Error (cleared by writing in HSMCI_CMDR)"]
pub type RINDE_R = crate::BitReader<bool>;
#[doc = "Field `RDIRE` reader - Response Direction Error (cleared by writing in HSMCI_CMDR)"]
pub type RDIRE_R = crate::BitReader<bool>;
#[doc = "Field `RCRCE` reader - Response CRC Error (cleared by writing in HSMCI_CMDR)"]
pub type RCRCE_R = crate::BitReader<bool>;
#[doc = "Field `RENDE` reader - Response End Bit Error (cleared by writing in HSMCI_CMDR)"]
pub type RENDE_R = crate::BitReader<bool>;
#[doc = "Field `RTOE` reader - Response Time-out Error (cleared by writing in HSMCI_CMDR)"]
pub type RTOE_R = crate::BitReader<bool>;
#[doc = "Field `DCRCE` reader - Data CRC Error (cleared on read)"]
pub type DCRCE_R = crate::BitReader<bool>;
#[doc = "Field `DTOE` reader - Data Time-out Error (cleared on read)"]
pub type DTOE_R = crate::BitReader<bool>;
#[doc = "Field `CSTOE` reader - Completion Signal Time-out Error (cleared on read)"]
pub type CSTOE_R = crate::BitReader<bool>;
#[doc = "Field `BLKOVRE` reader - DMA Block Overrun Error (cleared on read)"]
pub type BLKOVRE_R = crate::BitReader<bool>;
#[doc = "Field `FIFOEMPTY` reader - FIFO empty flag"]
pub type FIFOEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `XFRDONE` reader - Transfer Done flag"]
pub type XFRDONE_R = crate::BitReader<bool>;
#[doc = "Field `ACKRCV` reader - Boot Operation Acknowledge Received (cleared on read)"]
pub type ACKRCV_R = crate::BitReader<bool>;
#[doc = "Field `ACKRCVE` reader - Boot Operation Acknowledge Error (cleared on read)"]
pub type ACKRCVE_R = crate::BitReader<bool>;
#[doc = "Field `OVRE` reader - Overrun (if FERRCTRL = 1, cleared by writing in HSMCI_CMDR or cleared on read if FERRCTRL = 0)"]
pub type OVRE_R = crate::BitReader<bool>;
#[doc = "Field `UNRE` reader - Underrun (if FERRCTRL = 1, cleared by writing in HSMCI_CMDR or cleared on read if FERRCTRL = 0)"]
pub type UNRE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Command Ready (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn cmdrdy(&self) -> CMDRDY_R {
        CMDRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver Ready (cleared by reading HSMCI_RDR)"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Ready (cleared by writing in HSMCI_TDR)"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Block Ended (cleared on read)"]
    #[inline(always)]
    pub fn blke(&self) -> BLKE_R {
        BLKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Transfer in Progress (cleared at the end of CRC16 calculation)"]
    #[inline(always)]
    pub fn dtip(&self) -> DTIP_R {
        DTIP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSMCI Not Busy"]
    #[inline(always)]
    pub fn notbusy(&self) -> NOTBUSY_R {
        NOTBUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - SDIO Interrupt for Slot A (cleared on read)"]
    #[inline(always)]
    pub fn sdioirqa(&self) -> SDIOIRQA_R {
        SDIOIRQA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - SDIO Read Wait Operation Status"]
    #[inline(always)]
    pub fn sdiowait(&self) -> SDIOWAIT_R {
        SDIOWAIT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CE-ATA Completion Signal Received (cleared on read)"]
    #[inline(always)]
    pub fn csrcv(&self) -> CSRCV_R {
        CSRCV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Response Index Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rinde(&self) -> RINDE_R {
        RINDE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Response Direction Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rdire(&self) -> RDIRE_R {
        RDIRE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Response CRC Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rcrce(&self) -> RCRCE_R {
        RCRCE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Response End Bit Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rende(&self) -> RENDE_R {
        RENDE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Response Time-out Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rtoe(&self) -> RTOE_R {
        RTOE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error (cleared on read)"]
    #[inline(always)]
    pub fn dcrce(&self) -> DCRCE_R {
        DCRCE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data Time-out Error (cleared on read)"]
    #[inline(always)]
    pub fn dtoe(&self) -> DTOE_R {
        DTOE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Completion Signal Time-out Error (cleared on read)"]
    #[inline(always)]
    pub fn cstoe(&self) -> CSTOE_R {
        CSTOE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA Block Overrun Error (cleared on read)"]
    #[inline(always)]
    pub fn blkovre(&self) -> BLKOVRE_R {
        BLKOVRE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - FIFO empty flag"]
    #[inline(always)]
    pub fn fifoempty(&self) -> FIFOEMPTY_R {
        FIFOEMPTY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transfer Done flag"]
    #[inline(always)]
    pub fn xfrdone(&self) -> XFRDONE_R {
        XFRDONE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Boot Operation Acknowledge Received (cleared on read)"]
    #[inline(always)]
    pub fn ackrcv(&self) -> ACKRCV_R {
        ACKRCV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Boot Operation Acknowledge Error (cleared on read)"]
    #[inline(always)]
    pub fn ackrcve(&self) -> ACKRCVE_R {
        ACKRCVE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Overrun (if FERRCTRL = 1, cleared by writing in HSMCI_CMDR or cleared on read if FERRCTRL = 0)"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Underrun (if FERRCTRL = 1, cleared by writing in HSMCI_CMDR or cleared on read if FERRCTRL = 0)"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
