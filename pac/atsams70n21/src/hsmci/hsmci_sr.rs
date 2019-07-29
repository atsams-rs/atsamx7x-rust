#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HSMCI_SR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type CMDRDY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXRDY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TXRDY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type BLKE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DTIP_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type NOTBUSY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SDIOIRQA_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SDIOWAIT_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CSRCV_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RINDE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RDIRE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RCRCE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RENDE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RTOE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DCRCE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DTOE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CSTOE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type BLKOVRE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type FIFOEMPTY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type XFRDONE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type ACKRCV_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type ACKRCVE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVRE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type UNRE_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Command Ready (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn cmdrdy(&self) -> CMDRDY_R {
        CMDRDY_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receiver Ready (cleared by reading HSMCI_RDR)"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit Ready (cleared by writing in HSMCI_TDR)"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data Block Ended (cleared on read)"]
    #[inline(always)]
    pub fn blke(&self) -> BLKE_R {
        BLKE_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Data Transfer in Progress (cleared at the end of CRC16 calculation)"]
    #[inline(always)]
    pub fn dtip(&self) -> DTIP_R {
        DTIP_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HSMCI Not Busy"]
    #[inline(always)]
    pub fn notbusy(&self) -> NOTBUSY_R {
        NOTBUSY_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SDIO Interrupt for Slot A (cleared on read)"]
    #[inline(always)]
    pub fn sdioirqa(&self) -> SDIOIRQA_R {
        SDIOIRQA_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SDIO Read Wait Operation Status"]
    #[inline(always)]
    pub fn sdiowait(&self) -> SDIOWAIT_R {
        SDIOWAIT_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CE-ATA Completion Signal Received (cleared on read)"]
    #[inline(always)]
    pub fn csrcv(&self) -> CSRCV_R {
        CSRCV_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Response Index Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rinde(&self) -> RINDE_R {
        RINDE_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Response Direction Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rdire(&self) -> RDIRE_R {
        RDIRE_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Response CRC Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rcrce(&self) -> RCRCE_R {
        RCRCE_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Response End Bit Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rende(&self) -> RENDE_R {
        RENDE_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Response Time-out Error (cleared by writing in HSMCI_CMDR)"]
    #[inline(always)]
    pub fn rtoe(&self) -> RTOE_R {
        RTOE_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error (cleared on read)"]
    #[inline(always)]
    pub fn dcrce(&self) -> DCRCE_R {
        DCRCE_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Data Time-out Error (cleared on read)"]
    #[inline(always)]
    pub fn dtoe(&self) -> DTOE_R {
        DTOE_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Completion Signal Time-out Error (cleared on read)"]
    #[inline(always)]
    pub fn cstoe(&self) -> CSTOE_R {
        CSTOE_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - DMA Block Overrun Error (cleared on read)"]
    #[inline(always)]
    pub fn blkovre(&self) -> BLKOVRE_R {
        BLKOVRE_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - FIFO empty flag"]
    #[inline(always)]
    pub fn fifoempty(&self) -> FIFOEMPTY_R {
        FIFOEMPTY_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Transfer Done flag"]
    #[inline(always)]
    pub fn xfrdone(&self) -> XFRDONE_R {
        XFRDONE_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Boot Operation Acknowledge Received (cleared on read)"]
    #[inline(always)]
    pub fn ackrcv(&self) -> ACKRCV_R {
        ACKRCV_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Boot Operation Acknowledge Error (cleared on read)"]
    #[inline(always)]
    pub fn ackrcve(&self) -> ACKRCVE_R {
        ACKRCVE_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Overrun (if FERRCTRL = 1, cleared by writing in HSMCI_CMDR or cleared on read if FERRCTRL = 0)"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Underrun (if FERRCTRL = 1, cleared by writing in HSMCI_CMDR or cleared on read if FERRCTRL = 0)"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
