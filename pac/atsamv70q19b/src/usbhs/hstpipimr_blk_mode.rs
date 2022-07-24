#[doc = "Register `HSTPIPIMR_BLK_MODE[%s]` reader"]
pub struct R(crate::R<HSTPIPIMR_BLK_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTPIPIMR_BLK_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTPIPIMR_BLK_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTPIPIMR_BLK_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXINE` reader - Received IN Data Interrupt Enable"]
pub type RXINE_R = crate::BitReader<bool>;
#[doc = "Field `TXOUTE` reader - Transmitted OUT Data Interrupt Enable"]
pub type TXOUTE_R = crate::BitReader<bool>;
#[doc = "Field `TXSTPE` reader - Transmitted SETUP Interrupt Enable"]
pub type TXSTPE_R = crate::BitReader<bool>;
#[doc = "Field `PERRE` reader - Pipe Error Interrupt Enable"]
pub type PERRE_R = crate::BitReader<bool>;
#[doc = "Field `NAKEDE` reader - NAKed Interrupt Enable"]
pub type NAKEDE_R = crate::BitReader<bool>;
#[doc = "Field `OVERFIE` reader - Overflow Interrupt Enable"]
pub type OVERFIE_R = crate::BitReader<bool>;
#[doc = "Field `RXSTALLDE` reader - Received STALLed Interrupt Enable"]
pub type RXSTALLDE_R = crate::BitReader<bool>;
#[doc = "Field `SHORTPACKETIE` reader - Short Packet Interrupt Enable"]
pub type SHORTPACKETIE_R = crate::BitReader<bool>;
#[doc = "Field `NBUSYBKE` reader - Number of Busy Banks Interrupt Enable"]
pub type NBUSYBKE_R = crate::BitReader<bool>;
#[doc = "Field `FIFOCON` reader - FIFO Control"]
pub type FIFOCON_R = crate::BitReader<bool>;
#[doc = "Field `PDISHDMA` reader - Pipe Interrupts Disable HDMA Request Enable"]
pub type PDISHDMA_R = crate::BitReader<bool>;
#[doc = "Field `PFREEZE` reader - Pipe Freeze"]
pub type PFREEZE_R = crate::BitReader<bool>;
#[doc = "Field `RSTDT` reader - Reset Data Toggle"]
pub type RSTDT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Received IN Data Interrupt Enable"]
    #[inline(always)]
    pub fn rxine(&self) -> RXINE_R {
        RXINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Enable"]
    #[inline(always)]
    pub fn txoute(&self) -> TXOUTE_R {
        TXOUTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt Enable"]
    #[inline(always)]
    pub fn txstpe(&self) -> TXSTPE_R {
        TXSTPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Enable"]
    #[inline(always)]
    pub fn perre(&self) -> PERRE_R {
        PERRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAKed Interrupt Enable"]
    #[inline(always)]
    pub fn nakede(&self) -> NAKEDE_R {
        NAKEDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn overfie(&self) -> OVERFIE_R {
        OVERFIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Enable"]
    #[inline(always)]
    pub fn rxstallde(&self) -> RXSTALLDE_R {
        RXSTALLDE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    pub fn shortpacketie(&self) -> SHORTPACKETIE_R {
        SHORTPACKETIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Enable"]
    #[inline(always)]
    pub fn nbusybke(&self) -> NBUSYBKE_R {
        NBUSYBKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    pub fn fifocon(&self) -> FIFOCON_R {
        FIFOCON_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Pipe Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    pub fn pdishdma(&self) -> PDISHDMA_R {
        PDISHDMA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pipe Freeze"]
    #[inline(always)]
    pub fn pfreeze(&self) -> PFREEZE_R {
        PFREEZE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset Data Toggle"]
    #[inline(always)]
    pub fn rstdt(&self) -> RSTDT_R {
        RSTDT_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "Host Pipe Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipimr_blk_mode](index.html) module"]
pub struct HSTPIPIMR_BLK_MODE_SPEC;
impl crate::RegisterSpec for HSTPIPIMR_BLK_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstpipimr_blk_mode::R](R) reader structure"]
impl crate::Readable for HSTPIPIMR_BLK_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HSTPIPIMR_BLK_MODE[%s]
to value 0"]
impl crate::Resettable for HSTPIPIMR_BLK_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
