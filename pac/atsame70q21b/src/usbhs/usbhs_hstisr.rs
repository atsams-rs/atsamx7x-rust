#[doc = "Reader of register USBHS_HSTISR"]
pub type R = crate::R<u32, super::USBHS_HSTISR>;
#[doc = "Reader of field `DCONNI`"]
pub type DCONNI_R = crate::R<bool, bool>;
#[doc = "Reader of field `DDISCI`"]
pub type DDISCI_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSTI`"]
pub type RSTI_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSMEDI`"]
pub type RSMEDI_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXRSMI`"]
pub type RXRSMI_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSOFI`"]
pub type HSOFI_R = crate::R<bool, bool>;
#[doc = "Reader of field `HWUPI`"]
pub type HWUPI_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEP_0`"]
pub type PEP_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEP_1`"]
pub type PEP_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEP_2`"]
pub type PEP_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEP_3`"]
pub type PEP_3_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEP_4`"]
pub type PEP_4_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEP_5`"]
pub type PEP_5_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEP_6`"]
pub type PEP_6_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEP_7`"]
pub type PEP_7_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEP_8`"]
pub type PEP_8_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEP_9`"]
pub type PEP_9_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_0`"]
pub type DMA_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_1`"]
pub type DMA_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_2`"]
pub type DMA_2_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_3`"]
pub type DMA_3_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_4`"]
pub type DMA_4_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_5`"]
pub type DMA_5_R = crate::R<bool, bool>;
#[doc = "Reader of field `DMA_6`"]
pub type DMA_6_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Device Connection Interrupt"]
    #[inline(always)]
    pub fn dconni(&self) -> DCONNI_R {
        DCONNI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt"]
    #[inline(always)]
    pub fn ddisci(&self) -> DDISCI_R {
        DDISCI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt"]
    #[inline(always)]
    pub fn rsti(&self) -> RSTI_R {
        RSTI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt"]
    #[inline(always)]
    pub fn rsmedi(&self) -> RSMEDI_R {
        RSMEDI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt"]
    #[inline(always)]
    pub fn rxrsmi(&self) -> RXRSMI_R {
        RXRSMI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt"]
    #[inline(always)]
    pub fn hsofi(&self) -> HSOFI_R {
        HSOFI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt"]
    #[inline(always)]
    pub fn hwupi(&self) -> HWUPI_R {
        HWUPI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pipe 0 Interrupt"]
    #[inline(always)]
    pub fn pep_0(&self) -> PEP_0_R {
        PEP_0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pipe 1 Interrupt"]
    #[inline(always)]
    pub fn pep_1(&self) -> PEP_1_R {
        PEP_1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pipe 2 Interrupt"]
    #[inline(always)]
    pub fn pep_2(&self) -> PEP_2_R {
        PEP_2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pipe 3 Interrupt"]
    #[inline(always)]
    pub fn pep_3(&self) -> PEP_3_R {
        PEP_3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pipe 4 Interrupt"]
    #[inline(always)]
    pub fn pep_4(&self) -> PEP_4_R {
        PEP_4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pipe 5 Interrupt"]
    #[inline(always)]
    pub fn pep_5(&self) -> PEP_5_R {
        PEP_5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pipe 6 Interrupt"]
    #[inline(always)]
    pub fn pep_6(&self) -> PEP_6_R {
        PEP_6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Pipe 7 Interrupt"]
    #[inline(always)]
    pub fn pep_7(&self) -> PEP_7_R {
        PEP_7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Pipe 8 Interrupt"]
    #[inline(always)]
    pub fn pep_8(&self) -> PEP_8_R {
        PEP_8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pipe 9 Interrupt"]
    #[inline(always)]
    pub fn pep_9(&self) -> PEP_9_R {
        PEP_9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DMA Channel 0 Interrupt"]
    #[inline(always)]
    pub fn dma_0(&self) -> DMA_0_R {
        DMA_0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DMA Channel 1 Interrupt"]
    #[inline(always)]
    pub fn dma_1(&self) -> DMA_1_R {
        DMA_1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DMA Channel 2 Interrupt"]
    #[inline(always)]
    pub fn dma_2(&self) -> DMA_2_R {
        DMA_2_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DMA Channel 3 Interrupt"]
    #[inline(always)]
    pub fn dma_3(&self) -> DMA_3_R {
        DMA_3_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DMA Channel 4 Interrupt"]
    #[inline(always)]
    pub fn dma_4(&self) -> DMA_4_R {
        DMA_4_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DMA Channel 5 Interrupt"]
    #[inline(always)]
    pub fn dma_5(&self) -> DMA_5_R {
        DMA_5_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DMA Channel 6 Interrupt"]
    #[inline(always)]
    pub fn dma_6(&self) -> DMA_6_R {
        DMA_6_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
