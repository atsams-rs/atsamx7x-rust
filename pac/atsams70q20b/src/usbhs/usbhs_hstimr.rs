#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USBHS_HSTIMR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type DCONNIE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DDISCIE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RSTIE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RSMEDIE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXRSMIE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type HSOFIE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type HWUPIE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PEP_0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PEP_1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PEP_2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PEP_3_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PEP_4_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PEP_5_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PEP_6_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PEP_7_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PEP_8_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PEP_9_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DMA_0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DMA_1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DMA_2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DMA_3_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DMA_4_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DMA_5_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DMA_6_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Device Connection Interrupt Enable"]
    #[inline(always)]
    pub fn dconnie(&self) -> DCONNIE_R {
        DCONNIE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt Enable"]
    #[inline(always)]
    pub fn ddiscie(&self) -> DDISCIE_R {
        DDISCIE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt Enable"]
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt Enable"]
    #[inline(always)]
    pub fn rsmedie(&self) -> RSMEDIE_R {
        RSMEDIE_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt Enable"]
    #[inline(always)]
    pub fn rxrsmie(&self) -> RXRSMIE_R {
        RXRSMIE_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn hsofie(&self) -> HSOFIE_R {
        HSOFIE_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt Enable"]
    #[inline(always)]
    pub fn hwupie(&self) -> HWUPIE_R {
        HWUPIE_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pipe 0 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_0(&self) -> PEP_0_R {
        PEP_0_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pipe 1 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_1(&self) -> PEP_1_R {
        PEP_1_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pipe 2 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_2(&self) -> PEP_2_R {
        PEP_2_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pipe 3 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_3(&self) -> PEP_3_R {
        PEP_3_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pipe 4 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_4(&self) -> PEP_4_R {
        PEP_4_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pipe 5 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_5(&self) -> PEP_5_R {
        PEP_5_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pipe 6 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_6(&self) -> PEP_6_R {
        PEP_6_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pipe 7 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_7(&self) -> PEP_7_R {
        PEP_7_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pipe 8 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_8(&self) -> PEP_8_R {
        PEP_8_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pipe 9 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_9(&self) -> PEP_9_R {
        PEP_9_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DMA Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_0(&self) -> DMA_0_R {
        DMA_0_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DMA Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_1(&self) -> DMA_1_R {
        DMA_1_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DMA Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_2(&self) -> DMA_2_R {
        DMA_2_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DMA Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_3(&self) -> DMA_3_R {
        DMA_3_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DMA Channel 4 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_4(&self) -> DMA_4_R {
        DMA_4_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DMA Channel 5 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_5(&self) -> DMA_5_R {
        DMA_5_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DMA Channel 6 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_6(&self) -> DMA_6_R {
        DMA_6_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
