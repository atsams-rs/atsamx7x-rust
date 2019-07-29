#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USBHS_HSTPIPIMR_CTRL_MODE {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type RXINE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TXOUTE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TXSTPE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PERRE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type NAKEDE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVERFIE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXSTALLDE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SHORTPACKETIE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type NBUSYBKE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type FIFOCON_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PDISHDMA_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PFREEZE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RSTDT_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Received IN Data Interrupt Enable"]
    #[inline(always)]
    pub fn rxine(&self) -> RXINE_R {
        RXINE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Enable"]
    #[inline(always)]
    pub fn txoute(&self) -> TXOUTE_R {
        TXOUTE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt Enable"]
    #[inline(always)]
    pub fn txstpe(&self) -> TXSTPE_R {
        TXSTPE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Enable"]
    #[inline(always)]
    pub fn perre(&self) -> PERRE_R {
        PERRE_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAKed Interrupt Enable"]
    #[inline(always)]
    pub fn nakede(&self) -> NAKEDE_R {
        NAKEDE_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn overfie(&self) -> OVERFIE_R {
        OVERFIE_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Enable"]
    #[inline(always)]
    pub fn rxstallde(&self) -> RXSTALLDE_R {
        RXSTALLDE_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    pub fn shortpacketie(&self) -> SHORTPACKETIE_R {
        SHORTPACKETIE_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Enable"]
    #[inline(always)]
    pub fn nbusybke(&self) -> NBUSYBKE_R {
        NBUSYBKE_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    pub fn fifocon(&self) -> FIFOCON_R {
        FIFOCON_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pipe Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    pub fn pdishdma(&self) -> PDISHDMA_R {
        PDISHDMA_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pipe Freeze"]
    #[inline(always)]
    pub fn pfreeze(&self) -> PFREEZE_R {
        PFREEZE_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Reset Data Toggle"]
    #[inline(always)]
    pub fn rstdt(&self) -> RSTDT_R {
        RSTDT_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
}
