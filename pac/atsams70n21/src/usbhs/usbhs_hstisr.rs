#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USBHS_HSTISR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type DCONNI_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DDISCI_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RSTI_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RSMEDI_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXRSMI_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type HSOFI_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type HWUPI_R = crate::FR<bool, bool>;
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
pub type PEP_10_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PEP_11_R = crate::FR<bool, bool>;
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
#[doc = r"Reader of the field"]
pub type DMA_7_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Device Connection Interrupt"]
    #[inline(always)]
    pub fn dconni(&self) -> DCONNI_R {
        DCONNI_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt"]
    #[inline(always)]
    pub fn ddisci(&self) -> DDISCI_R {
        DDISCI_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt"]
    #[inline(always)]
    pub fn rsti(&self) -> RSTI_R {
        RSTI_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt"]
    #[inline(always)]
    pub fn rsmedi(&self) -> RSMEDI_R {
        RSMEDI_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt"]
    #[inline(always)]
    pub fn rxrsmi(&self) -> RXRSMI_R {
        RXRSMI_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt"]
    #[inline(always)]
    pub fn hsofi(&self) -> HSOFI_R {
        HSOFI_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt"]
    #[inline(always)]
    pub fn hwupi(&self) -> HWUPI_R {
        HWUPI_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pipe 0 Interrupt"]
    #[inline(always)]
    pub fn pep_0(&self) -> PEP_0_R {
        PEP_0_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pipe 1 Interrupt"]
    #[inline(always)]
    pub fn pep_1(&self) -> PEP_1_R {
        PEP_1_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pipe 2 Interrupt"]
    #[inline(always)]
    pub fn pep_2(&self) -> PEP_2_R {
        PEP_2_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pipe 3 Interrupt"]
    #[inline(always)]
    pub fn pep_3(&self) -> PEP_3_R {
        PEP_3_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pipe 4 Interrupt"]
    #[inline(always)]
    pub fn pep_4(&self) -> PEP_4_R {
        PEP_4_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pipe 5 Interrupt"]
    #[inline(always)]
    pub fn pep_5(&self) -> PEP_5_R {
        PEP_5_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pipe 6 Interrupt"]
    #[inline(always)]
    pub fn pep_6(&self) -> PEP_6_R {
        PEP_6_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pipe 7 Interrupt"]
    #[inline(always)]
    pub fn pep_7(&self) -> PEP_7_R {
        PEP_7_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pipe 8 Interrupt"]
    #[inline(always)]
    pub fn pep_8(&self) -> PEP_8_R {
        PEP_8_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pipe 9 Interrupt"]
    #[inline(always)]
    pub fn pep_9(&self) -> PEP_9_R {
        PEP_9_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Pipe 10 Interrupt"]
    #[inline(always)]
    pub fn pep_10(&self) -> PEP_10_R {
        PEP_10_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Pipe 11 Interrupt"]
    #[inline(always)]
    pub fn pep_11(&self) -> PEP_11_R {
        PEP_11_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt"]
    #[inline(always)]
    pub fn dma_1(&self) -> DMA_1_R {
        DMA_1_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt"]
    #[inline(always)]
    pub fn dma_2(&self) -> DMA_2_R {
        DMA_2_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt"]
    #[inline(always)]
    pub fn dma_3(&self) -> DMA_3_R {
        DMA_3_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt"]
    #[inline(always)]
    pub fn dma_4(&self) -> DMA_4_R {
        DMA_4_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt"]
    #[inline(always)]
    pub fn dma_5(&self) -> DMA_5_R {
        DMA_5_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt"]
    #[inline(always)]
    pub fn dma_6(&self) -> DMA_6_R {
        DMA_6_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DMA Channel 7 Interrupt"]
    #[inline(always)]
    pub fn dma_7(&self) -> DMA_7_R {
        DMA_7_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
