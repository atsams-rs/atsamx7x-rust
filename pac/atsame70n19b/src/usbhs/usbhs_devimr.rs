#[doc = "Reader of register USBHS_DEVIMR"]
pub type R = crate::R<u32, super::USBHS_DEVIMR>;
#[doc = "Reader of field `SUSPE`"]
pub type SUSPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `MSOFE`"]
pub type MSOFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SOFE`"]
pub type SOFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EORSTE`"]
pub type EORSTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `WAKEUPE`"]
pub type WAKEUPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EORSME`"]
pub type EORSME_R = crate::R<bool, bool>;
#[doc = "Reader of field `UPRSME`"]
pub type UPRSME_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `DMA_7`"]
pub type DMA_7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Suspend Interrupt Mask"]
    #[inline(always)]
    pub fn suspe(&self) -> SUSPE_R {
        SUSPE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt Mask"]
    #[inline(always)]
    pub fn msofe(&self) -> MSOFE_R {
        MSOFE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start of Frame Interrupt Mask"]
    #[inline(always)]
    pub fn sofe(&self) -> SOFE_R {
        SOFE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of Reset Interrupt Mask"]
    #[inline(always)]
    pub fn eorste(&self) -> EORSTE_R {
        EORSTE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn wakeupe(&self) -> WAKEUPE_R {
        WAKEUPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of Resume Interrupt Mask"]
    #[inline(always)]
    pub fn eorsme(&self) -> EORSME_R {
        EORSME_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt Mask"]
    #[inline(always)]
    pub fn uprsme(&self) -> UPRSME_R {
        UPRSME_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint 0 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_0(&self) -> PEP_0_R {
        PEP_0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint 1 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_1(&self) -> PEP_1_R {
        PEP_1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint 2 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_2(&self) -> PEP_2_R {
        PEP_2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Endpoint 3 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_3(&self) -> PEP_3_R {
        PEP_3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint 4 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_4(&self) -> PEP_4_R {
        PEP_4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint 5 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_5(&self) -> PEP_5_R {
        PEP_5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Endpoint 6 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_6(&self) -> PEP_6_R {
        PEP_6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Endpoint 7 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_7(&self) -> PEP_7_R {
        PEP_7_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Endpoint 8 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_8(&self) -> PEP_8_R {
        PEP_8_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Endpoint 9 Interrupt Mask"]
    #[inline(always)]
    pub fn pep_9(&self) -> PEP_9_R {
        PEP_9_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt Mask"]
    #[inline(always)]
    pub fn dma_1(&self) -> DMA_1_R {
        DMA_1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt Mask"]
    #[inline(always)]
    pub fn dma_2(&self) -> DMA_2_R {
        DMA_2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt Mask"]
    #[inline(always)]
    pub fn dma_3(&self) -> DMA_3_R {
        DMA_3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt Mask"]
    #[inline(always)]
    pub fn dma_4(&self) -> DMA_4_R {
        DMA_4_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt Mask"]
    #[inline(always)]
    pub fn dma_5(&self) -> DMA_5_R {
        DMA_5_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt Mask"]
    #[inline(always)]
    pub fn dma_6(&self) -> DMA_6_R {
        DMA_6_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DMA Channel 7 Interrupt Mask"]
    #[inline(always)]
    pub fn dma_7(&self) -> DMA_7_R {
        DMA_7_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
