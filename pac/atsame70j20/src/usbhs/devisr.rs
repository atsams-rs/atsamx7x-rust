#[doc = "Register `DEVISR` reader"]
pub struct R(crate::R<DEVISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SUSP` reader - Suspend Interrupt"]
pub type SUSP_R = crate::BitReader<bool>;
#[doc = "Field `MSOF` reader - Micro Start of Frame Interrupt"]
pub type MSOF_R = crate::BitReader<bool>;
#[doc = "Field `SOF` reader - Start of Frame Interrupt"]
pub type SOF_R = crate::BitReader<bool>;
#[doc = "Field `EORST` reader - End of Reset Interrupt"]
pub type EORST_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUP` reader - Wake-Up Interrupt"]
pub type WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `EORSM` reader - End of Resume Interrupt"]
pub type EORSM_R = crate::BitReader<bool>;
#[doc = "Field `UPRSM` reader - Upstream Resume Interrupt"]
pub type UPRSM_R = crate::BitReader<bool>;
#[doc = "Field `PEP_0` reader - Endpoint 0 Interrupt"]
pub type PEP_0_R = crate::BitReader<bool>;
#[doc = "Field `PEP_1` reader - Endpoint 1 Interrupt"]
pub type PEP_1_R = crate::BitReader<bool>;
#[doc = "Field `PEP_2` reader - Endpoint 2 Interrupt"]
pub type PEP_2_R = crate::BitReader<bool>;
#[doc = "Field `PEP_3` reader - Endpoint 3 Interrupt"]
pub type PEP_3_R = crate::BitReader<bool>;
#[doc = "Field `PEP_4` reader - Endpoint 4 Interrupt"]
pub type PEP_4_R = crate::BitReader<bool>;
#[doc = "Field `PEP_5` reader - Endpoint 5 Interrupt"]
pub type PEP_5_R = crate::BitReader<bool>;
#[doc = "Field `PEP_6` reader - Endpoint 6 Interrupt"]
pub type PEP_6_R = crate::BitReader<bool>;
#[doc = "Field `PEP_7` reader - Endpoint 7 Interrupt"]
pub type PEP_7_R = crate::BitReader<bool>;
#[doc = "Field `PEP_8` reader - Endpoint 8 Interrupt"]
pub type PEP_8_R = crate::BitReader<bool>;
#[doc = "Field `PEP_9` reader - Endpoint 9 Interrupt"]
pub type PEP_9_R = crate::BitReader<bool>;
#[doc = "Field `PEP_10` reader - Endpoint 10 Interrupt"]
pub type PEP_10_R = crate::BitReader<bool>;
#[doc = "Field `PEP_11` reader - Endpoint 11 Interrupt"]
pub type PEP_11_R = crate::BitReader<bool>;
#[doc = "Field `DMA_1` reader - DMA Channel 1 Interrupt"]
pub type DMA_1_R = crate::BitReader<bool>;
#[doc = "Field `DMA_2` reader - DMA Channel 2 Interrupt"]
pub type DMA_2_R = crate::BitReader<bool>;
#[doc = "Field `DMA_3` reader - DMA Channel 3 Interrupt"]
pub type DMA_3_R = crate::BitReader<bool>;
#[doc = "Field `DMA_4` reader - DMA Channel 4 Interrupt"]
pub type DMA_4_R = crate::BitReader<bool>;
#[doc = "Field `DMA_5` reader - DMA Channel 5 Interrupt"]
pub type DMA_5_R = crate::BitReader<bool>;
#[doc = "Field `DMA_6` reader - DMA Channel 6 Interrupt"]
pub type DMA_6_R = crate::BitReader<bool>;
#[doc = "Field `DMA_7` reader - DMA Channel 7 Interrupt"]
pub type DMA_7_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Suspend Interrupt"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt"]
    #[inline(always)]
    pub fn msof(&self) -> MSOF_R {
        MSOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start of Frame Interrupt"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Reset Interrupt"]
    #[inline(always)]
    pub fn eorst(&self) -> EORST_R {
        EORST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Resume Interrupt"]
    #[inline(always)]
    pub fn eorsm(&self) -> EORSM_R {
        EORSM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt"]
    #[inline(always)]
    pub fn uprsm(&self) -> UPRSM_R {
        UPRSM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - Endpoint 0 Interrupt"]
    #[inline(always)]
    pub fn pep_0(&self) -> PEP_0_R {
        PEP_0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn pep_1(&self) -> PEP_1_R {
        PEP_1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Endpoint 2 Interrupt"]
    #[inline(always)]
    pub fn pep_2(&self) -> PEP_2_R {
        PEP_2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn pep_3(&self) -> PEP_3_R {
        PEP_3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn pep_4(&self) -> PEP_4_R {
        PEP_4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn pep_5(&self) -> PEP_5_R {
        PEP_5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn pep_6(&self) -> PEP_6_R {
        PEP_6_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Endpoint 7 Interrupt"]
    #[inline(always)]
    pub fn pep_7(&self) -> PEP_7_R {
        PEP_7_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Endpoint 8 Interrupt"]
    #[inline(always)]
    pub fn pep_8(&self) -> PEP_8_R {
        PEP_8_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Endpoint 9 Interrupt"]
    #[inline(always)]
    pub fn pep_9(&self) -> PEP_9_R {
        PEP_9_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Endpoint 10 Interrupt"]
    #[inline(always)]
    pub fn pep_10(&self) -> PEP_10_R {
        PEP_10_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Endpoint 11 Interrupt"]
    #[inline(always)]
    pub fn pep_11(&self) -> PEP_11_R {
        PEP_11_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA Channel 1 Interrupt"]
    #[inline(always)]
    pub fn dma_1(&self) -> DMA_1_R {
        DMA_1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMA Channel 2 Interrupt"]
    #[inline(always)]
    pub fn dma_2(&self) -> DMA_2_R {
        DMA_2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DMA Channel 3 Interrupt"]
    #[inline(always)]
    pub fn dma_3(&self) -> DMA_3_R {
        DMA_3_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA Channel 4 Interrupt"]
    #[inline(always)]
    pub fn dma_4(&self) -> DMA_4_R {
        DMA_4_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA Channel 5 Interrupt"]
    #[inline(always)]
    pub fn dma_5(&self) -> DMA_5_R {
        DMA_5_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA Channel 6 Interrupt"]
    #[inline(always)]
    pub fn dma_6(&self) -> DMA_6_R {
        DMA_6_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA Channel 7 Interrupt"]
    #[inline(always)]
    pub fn dma_7(&self) -> DMA_7_R {
        DMA_7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Device Global Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devisr](index.html) module"]
pub struct DEVISR_SPEC;
impl crate::RegisterSpec for DEVISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devisr::R](R) reader structure"]
impl crate::Readable for DEVISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVISR to value 0"]
impl crate::Resettable for DEVISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
