#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub hsmci_cr: HSMCI_CR,
    #[doc = "0x04 - Mode Register"]
    pub hsmci_mr: HSMCI_MR,
    #[doc = "0x08 - Data Timeout Register"]
    pub hsmci_dtor: HSMCI_DTOR,
    #[doc = "0x0c - SD/SDIO Card Register"]
    pub hsmci_sdcr: HSMCI_SDCR,
    #[doc = "0x10 - Argument Register"]
    pub hsmci_argr: HSMCI_ARGR,
    #[doc = "0x14 - Command Register"]
    pub hsmci_cmdr: HSMCI_CMDR,
    #[doc = "0x18 - Block Register"]
    pub hsmci_blkr: HSMCI_BLKR,
    #[doc = "0x1c - Completion Signal Timeout Register"]
    pub hsmci_cstor: HSMCI_CSTOR,
    #[doc = "0x20 - Response Register 0"]
    pub hsmci_rspr: [HSMCI_RSPR; 4],
    #[doc = "0x30 - Receive Data Register"]
    pub hsmci_rdr: HSMCI_RDR,
    #[doc = "0x34 - Transmit Data Register"]
    pub hsmci_tdr: HSMCI_TDR,
    _reserved11: [u8; 8usize],
    #[doc = "0x40 - Status Register"]
    pub hsmci_sr: HSMCI_SR,
    #[doc = "0x44 - Interrupt Enable Register"]
    pub hsmci_ier: HSMCI_IER,
    #[doc = "0x48 - Interrupt Disable Register"]
    pub hsmci_idr: HSMCI_IDR,
    #[doc = "0x4c - Interrupt Mask Register"]
    pub hsmci_imr: HSMCI_IMR,
    #[doc = "0x50 - DMA Configuration Register"]
    pub hsmci_dma: HSMCI_DMA,
    #[doc = "0x54 - Configuration Register"]
    pub hsmci_cfg: HSMCI_CFG,
    _reserved17: [u8; 140usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub hsmci_wpmr: HSMCI_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub hsmci_wpsr: HSMCI_WPSR,
    _reserved19: [u8; 276usize],
    #[doc = "0x200 - FIFO Memory Aperture0 0"]
    pub hsmci_fifo: [HSMCI_FIFO; 256],
}
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_cr](hsmci_cr) module"]
pub type HSMCI_CR = crate::Reg<u32, _HSMCI_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_CR;
#[doc = "`write(|w| ..)` method takes [hsmci_cr::W](hsmci_cr::W) writer structure"]
impl crate::Writable for HSMCI_CR {}
#[doc = "Control Register"]
pub mod hsmci_cr;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_mr](hsmci_mr) module"]
pub type HSMCI_MR = crate::Reg<u32, _HSMCI_MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_MR;
#[doc = "`read()` method returns [hsmci_mr::R](hsmci_mr::R) reader structure"]
impl crate::Readable for HSMCI_MR {}
#[doc = "`write(|w| ..)` method takes [hsmci_mr::W](hsmci_mr::W) writer structure"]
impl crate::Writable for HSMCI_MR {}
#[doc = "Mode Register"]
pub mod hsmci_mr;
#[doc = "Data Timeout Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_dtor](hsmci_dtor) module"]
pub type HSMCI_DTOR = crate::Reg<u32, _HSMCI_DTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_DTOR;
#[doc = "`read()` method returns [hsmci_dtor::R](hsmci_dtor::R) reader structure"]
impl crate::Readable for HSMCI_DTOR {}
#[doc = "`write(|w| ..)` method takes [hsmci_dtor::W](hsmci_dtor::W) writer structure"]
impl crate::Writable for HSMCI_DTOR {}
#[doc = "Data Timeout Register"]
pub mod hsmci_dtor;
#[doc = "SD/SDIO Card Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_sdcr](hsmci_sdcr) module"]
pub type HSMCI_SDCR = crate::Reg<u32, _HSMCI_SDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_SDCR;
#[doc = "`read()` method returns [hsmci_sdcr::R](hsmci_sdcr::R) reader structure"]
impl crate::Readable for HSMCI_SDCR {}
#[doc = "`write(|w| ..)` method takes [hsmci_sdcr::W](hsmci_sdcr::W) writer structure"]
impl crate::Writable for HSMCI_SDCR {}
#[doc = "SD/SDIO Card Register"]
pub mod hsmci_sdcr;
#[doc = "Argument Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_argr](hsmci_argr) module"]
pub type HSMCI_ARGR = crate::Reg<u32, _HSMCI_ARGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_ARGR;
#[doc = "`read()` method returns [hsmci_argr::R](hsmci_argr::R) reader structure"]
impl crate::Readable for HSMCI_ARGR {}
#[doc = "`write(|w| ..)` method takes [hsmci_argr::W](hsmci_argr::W) writer structure"]
impl crate::Writable for HSMCI_ARGR {}
#[doc = "Argument Register"]
pub mod hsmci_argr;
#[doc = "Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_cmdr](hsmci_cmdr) module"]
pub type HSMCI_CMDR = crate::Reg<u32, _HSMCI_CMDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_CMDR;
#[doc = "`write(|w| ..)` method takes [hsmci_cmdr::W](hsmci_cmdr::W) writer structure"]
impl crate::Writable for HSMCI_CMDR {}
#[doc = "Command Register"]
pub mod hsmci_cmdr;
#[doc = "Block Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_blkr](hsmci_blkr) module"]
pub type HSMCI_BLKR = crate::Reg<u32, _HSMCI_BLKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_BLKR;
#[doc = "`read()` method returns [hsmci_blkr::R](hsmci_blkr::R) reader structure"]
impl crate::Readable for HSMCI_BLKR {}
#[doc = "`write(|w| ..)` method takes [hsmci_blkr::W](hsmci_blkr::W) writer structure"]
impl crate::Writable for HSMCI_BLKR {}
#[doc = "Block Register"]
pub mod hsmci_blkr;
#[doc = "Completion Signal Timeout Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_cstor](hsmci_cstor) module"]
pub type HSMCI_CSTOR = crate::Reg<u32, _HSMCI_CSTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_CSTOR;
#[doc = "`read()` method returns [hsmci_cstor::R](hsmci_cstor::R) reader structure"]
impl crate::Readable for HSMCI_CSTOR {}
#[doc = "`write(|w| ..)` method takes [hsmci_cstor::W](hsmci_cstor::W) writer structure"]
impl crate::Writable for HSMCI_CSTOR {}
#[doc = "Completion Signal Timeout Register"]
pub mod hsmci_cstor;
#[doc = "Response Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_rspr](hsmci_rspr) module"]
pub type HSMCI_RSPR = crate::Reg<u32, _HSMCI_RSPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_RSPR;
#[doc = "`read()` method returns [hsmci_rspr::R](hsmci_rspr::R) reader structure"]
impl crate::Readable for HSMCI_RSPR {}
#[doc = "Response Register 0"]
pub mod hsmci_rspr;
#[doc = "Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_rdr](hsmci_rdr) module"]
pub type HSMCI_RDR = crate::Reg<u32, _HSMCI_RDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_RDR;
#[doc = "`read()` method returns [hsmci_rdr::R](hsmci_rdr::R) reader structure"]
impl crate::Readable for HSMCI_RDR {}
#[doc = "Receive Data Register"]
pub mod hsmci_rdr;
#[doc = "Transmit Data Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_tdr](hsmci_tdr) module"]
pub type HSMCI_TDR = crate::Reg<u32, _HSMCI_TDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_TDR;
#[doc = "`write(|w| ..)` method takes [hsmci_tdr::W](hsmci_tdr::W) writer structure"]
impl crate::Writable for HSMCI_TDR {}
#[doc = "Transmit Data Register"]
pub mod hsmci_tdr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_sr](hsmci_sr) module"]
pub type HSMCI_SR = crate::Reg<u32, _HSMCI_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_SR;
#[doc = "`read()` method returns [hsmci_sr::R](hsmci_sr::R) reader structure"]
impl crate::Readable for HSMCI_SR {}
#[doc = "Status Register"]
pub mod hsmci_sr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_ier](hsmci_ier) module"]
pub type HSMCI_IER = crate::Reg<u32, _HSMCI_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_IER;
#[doc = "`write(|w| ..)` method takes [hsmci_ier::W](hsmci_ier::W) writer structure"]
impl crate::Writable for HSMCI_IER {}
#[doc = "Interrupt Enable Register"]
pub mod hsmci_ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_idr](hsmci_idr) module"]
pub type HSMCI_IDR = crate::Reg<u32, _HSMCI_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_IDR;
#[doc = "`write(|w| ..)` method takes [hsmci_idr::W](hsmci_idr::W) writer structure"]
impl crate::Writable for HSMCI_IDR {}
#[doc = "Interrupt Disable Register"]
pub mod hsmci_idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_imr](hsmci_imr) module"]
pub type HSMCI_IMR = crate::Reg<u32, _HSMCI_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_IMR;
#[doc = "`read()` method returns [hsmci_imr::R](hsmci_imr::R) reader structure"]
impl crate::Readable for HSMCI_IMR {}
#[doc = "Interrupt Mask Register"]
pub mod hsmci_imr;
#[doc = "DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_dma](hsmci_dma) module"]
pub type HSMCI_DMA = crate::Reg<u32, _HSMCI_DMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_DMA;
#[doc = "`read()` method returns [hsmci_dma::R](hsmci_dma::R) reader structure"]
impl crate::Readable for HSMCI_DMA {}
#[doc = "`write(|w| ..)` method takes [hsmci_dma::W](hsmci_dma::W) writer structure"]
impl crate::Writable for HSMCI_DMA {}
#[doc = "DMA Configuration Register"]
pub mod hsmci_dma;
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_cfg](hsmci_cfg) module"]
pub type HSMCI_CFG = crate::Reg<u32, _HSMCI_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_CFG;
#[doc = "`read()` method returns [hsmci_cfg::R](hsmci_cfg::R) reader structure"]
impl crate::Readable for HSMCI_CFG {}
#[doc = "`write(|w| ..)` method takes [hsmci_cfg::W](hsmci_cfg::W) writer structure"]
impl crate::Writable for HSMCI_CFG {}
#[doc = "Configuration Register"]
pub mod hsmci_cfg;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_wpmr](hsmci_wpmr) module"]
pub type HSMCI_WPMR = crate::Reg<u32, _HSMCI_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_WPMR;
#[doc = "`read()` method returns [hsmci_wpmr::R](hsmci_wpmr::R) reader structure"]
impl crate::Readable for HSMCI_WPMR {}
#[doc = "`write(|w| ..)` method takes [hsmci_wpmr::W](hsmci_wpmr::W) writer structure"]
impl crate::Writable for HSMCI_WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod hsmci_wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_wpsr](hsmci_wpsr) module"]
pub type HSMCI_WPSR = crate::Reg<u32, _HSMCI_WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_WPSR;
#[doc = "`read()` method returns [hsmci_wpsr::R](hsmci_wpsr::R) reader structure"]
impl crate::Readable for HSMCI_WPSR {}
#[doc = "Write Protection Status Register"]
pub mod hsmci_wpsr;
#[doc = "FIFO Memory Aperture0 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsmci_fifo](hsmci_fifo) module"]
pub type HSMCI_FIFO = crate::Reg<u32, _HSMCI_FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HSMCI_FIFO;
#[doc = "`read()` method returns [hsmci_fifo::R](hsmci_fifo::R) reader structure"]
impl crate::Readable for HSMCI_FIFO {}
#[doc = "`write(|w| ..)` method takes [hsmci_fifo::W](hsmci_fifo::W) writer structure"]
impl crate::Writable for HSMCI_FIFO {}
#[doc = "FIFO Memory Aperture0 0"]
pub mod hsmci_fifo;
