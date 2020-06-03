#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ISI Configuration 1 Register"]
    pub isi_cfg1: ISI_CFG1,
    #[doc = "0x04 - ISI Configuration 2 Register"]
    pub isi_cfg2: ISI_CFG2,
    #[doc = "0x08 - ISI Preview Size Register"]
    pub isi_psize: ISI_PSIZE,
    #[doc = "0x0c - ISI Preview Decimation Factor Register"]
    pub isi_pdecf: ISI_PDECF,
    #[doc = "0x10 - ISI Color Space Conversion YCrCb To RGB Set 0 Register"]
    pub isi_y2r_set0: ISI_Y2R_SET0,
    #[doc = "0x14 - ISI Color Space Conversion YCrCb To RGB Set 1 Register"]
    pub isi_y2r_set1: ISI_Y2R_SET1,
    #[doc = "0x18 - ISI Color Space Conversion RGB To YCrCb Set 0 Register"]
    pub isi_r2y_set0: ISI_R2Y_SET0,
    #[doc = "0x1c - ISI Color Space Conversion RGB To YCrCb Set 1 Register"]
    pub isi_r2y_set1: ISI_R2Y_SET1,
    #[doc = "0x20 - ISI Color Space Conversion RGB To YCrCb Set 2 Register"]
    pub isi_r2y_set2: ISI_R2Y_SET2,
    #[doc = "0x24 - ISI Control Register"]
    pub isi_cr: ISI_CR,
    #[doc = "0x28 - ISI Status Register"]
    pub isi_sr: ISI_SR,
    #[doc = "0x2c - ISI Interrupt Enable Register"]
    pub isi_ier: ISI_IER,
    #[doc = "0x30 - ISI Interrupt Disable Register"]
    pub isi_idr: ISI_IDR,
    #[doc = "0x34 - ISI Interrupt Mask Register"]
    pub isi_imr: ISI_IMR,
    #[doc = "0x38 - DMA Channel Enable Register"]
    pub isi_dma_cher: ISI_DMA_CHER,
    #[doc = "0x3c - DMA Channel Disable Register"]
    pub isi_dma_chdr: ISI_DMA_CHDR,
    #[doc = "0x40 - DMA Channel Status Register"]
    pub isi_dma_chsr: ISI_DMA_CHSR,
    #[doc = "0x44 - DMA Preview Base Address Register"]
    pub isi_dma_p_addr: ISI_DMA_P_ADDR,
    #[doc = "0x48 - DMA Preview Control Register"]
    pub isi_dma_p_ctrl: ISI_DMA_P_CTRL,
    #[doc = "0x4c - DMA Preview Descriptor Address Register"]
    pub isi_dma_p_dscr: ISI_DMA_P_DSCR,
    #[doc = "0x50 - DMA Codec Base Address Register"]
    pub isi_dma_c_addr: ISI_DMA_C_ADDR,
    #[doc = "0x54 - DMA Codec Control Register"]
    pub isi_dma_c_ctrl: ISI_DMA_C_CTRL,
    #[doc = "0x58 - DMA Codec Descriptor Address Register"]
    pub isi_dma_c_dscr: ISI_DMA_C_DSCR,
    _reserved23: [u8; 136usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub isi_wpmr: ISI_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub isi_wpsr: ISI_WPSR,
}
#[doc = "ISI Configuration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_cfg1](isi_cfg1) module"]
pub type ISI_CFG1 = crate::Reg<u32, _ISI_CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_CFG1;
#[doc = "`read()` method returns [isi_cfg1::R](isi_cfg1::R) reader structure"]
impl crate::Readable for ISI_CFG1 {}
#[doc = "`write(|w| ..)` method takes [isi_cfg1::W](isi_cfg1::W) writer structure"]
impl crate::Writable for ISI_CFG1 {}
#[doc = "ISI Configuration 1 Register"]
pub mod isi_cfg1;
#[doc = "ISI Configuration 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_cfg2](isi_cfg2) module"]
pub type ISI_CFG2 = crate::Reg<u32, _ISI_CFG2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_CFG2;
#[doc = "`read()` method returns [isi_cfg2::R](isi_cfg2::R) reader structure"]
impl crate::Readable for ISI_CFG2 {}
#[doc = "`write(|w| ..)` method takes [isi_cfg2::W](isi_cfg2::W) writer structure"]
impl crate::Writable for ISI_CFG2 {}
#[doc = "ISI Configuration 2 Register"]
pub mod isi_cfg2;
#[doc = "ISI Preview Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_psize](isi_psize) module"]
pub type ISI_PSIZE = crate::Reg<u32, _ISI_PSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_PSIZE;
#[doc = "`read()` method returns [isi_psize::R](isi_psize::R) reader structure"]
impl crate::Readable for ISI_PSIZE {}
#[doc = "`write(|w| ..)` method takes [isi_psize::W](isi_psize::W) writer structure"]
impl crate::Writable for ISI_PSIZE {}
#[doc = "ISI Preview Size Register"]
pub mod isi_psize;
#[doc = "ISI Preview Decimation Factor Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_pdecf](isi_pdecf) module"]
pub type ISI_PDECF = crate::Reg<u32, _ISI_PDECF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_PDECF;
#[doc = "`read()` method returns [isi_pdecf::R](isi_pdecf::R) reader structure"]
impl crate::Readable for ISI_PDECF {}
#[doc = "`write(|w| ..)` method takes [isi_pdecf::W](isi_pdecf::W) writer structure"]
impl crate::Writable for ISI_PDECF {}
#[doc = "ISI Preview Decimation Factor Register"]
pub mod isi_pdecf;
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_y2r_set0](isi_y2r_set0) module"]
pub type ISI_Y2R_SET0 = crate::Reg<u32, _ISI_Y2R_SET0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_Y2R_SET0;
#[doc = "`read()` method returns [isi_y2r_set0::R](isi_y2r_set0::R) reader structure"]
impl crate::Readable for ISI_Y2R_SET0 {}
#[doc = "`write(|w| ..)` method takes [isi_y2r_set0::W](isi_y2r_set0::W) writer structure"]
impl crate::Writable for ISI_Y2R_SET0 {}
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 0 Register"]
pub mod isi_y2r_set0;
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_y2r_set1](isi_y2r_set1) module"]
pub type ISI_Y2R_SET1 = crate::Reg<u32, _ISI_Y2R_SET1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_Y2R_SET1;
#[doc = "`read()` method returns [isi_y2r_set1::R](isi_y2r_set1::R) reader structure"]
impl crate::Readable for ISI_Y2R_SET1 {}
#[doc = "`write(|w| ..)` method takes [isi_y2r_set1::W](isi_y2r_set1::W) writer structure"]
impl crate::Writable for ISI_Y2R_SET1 {}
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 1 Register"]
pub mod isi_y2r_set1;
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_r2y_set0](isi_r2y_set0) module"]
pub type ISI_R2Y_SET0 = crate::Reg<u32, _ISI_R2Y_SET0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_R2Y_SET0;
#[doc = "`read()` method returns [isi_r2y_set0::R](isi_r2y_set0::R) reader structure"]
impl crate::Readable for ISI_R2Y_SET0 {}
#[doc = "`write(|w| ..)` method takes [isi_r2y_set0::W](isi_r2y_set0::W) writer structure"]
impl crate::Writable for ISI_R2Y_SET0 {}
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 0 Register"]
pub mod isi_r2y_set0;
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_r2y_set1](isi_r2y_set1) module"]
pub type ISI_R2Y_SET1 = crate::Reg<u32, _ISI_R2Y_SET1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_R2Y_SET1;
#[doc = "`read()` method returns [isi_r2y_set1::R](isi_r2y_set1::R) reader structure"]
impl crate::Readable for ISI_R2Y_SET1 {}
#[doc = "`write(|w| ..)` method takes [isi_r2y_set1::W](isi_r2y_set1::W) writer structure"]
impl crate::Writable for ISI_R2Y_SET1 {}
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 1 Register"]
pub mod isi_r2y_set1;
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_r2y_set2](isi_r2y_set2) module"]
pub type ISI_R2Y_SET2 = crate::Reg<u32, _ISI_R2Y_SET2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_R2Y_SET2;
#[doc = "`read()` method returns [isi_r2y_set2::R](isi_r2y_set2::R) reader structure"]
impl crate::Readable for ISI_R2Y_SET2 {}
#[doc = "`write(|w| ..)` method takes [isi_r2y_set2::W](isi_r2y_set2::W) writer structure"]
impl crate::Writable for ISI_R2Y_SET2 {}
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 2 Register"]
pub mod isi_r2y_set2;
#[doc = "ISI Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_cr](isi_cr) module"]
pub type ISI_CR = crate::Reg<u32, _ISI_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_CR;
#[doc = "`write(|w| ..)` method takes [isi_cr::W](isi_cr::W) writer structure"]
impl crate::Writable for ISI_CR {}
#[doc = "ISI Control Register"]
pub mod isi_cr;
#[doc = "ISI Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_sr](isi_sr) module"]
pub type ISI_SR = crate::Reg<u32, _ISI_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_SR;
#[doc = "`read()` method returns [isi_sr::R](isi_sr::R) reader structure"]
impl crate::Readable for ISI_SR {}
#[doc = "ISI Status Register"]
pub mod isi_sr;
#[doc = "ISI Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_ier](isi_ier) module"]
pub type ISI_IER = crate::Reg<u32, _ISI_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_IER;
#[doc = "`write(|w| ..)` method takes [isi_ier::W](isi_ier::W) writer structure"]
impl crate::Writable for ISI_IER {}
#[doc = "ISI Interrupt Enable Register"]
pub mod isi_ier;
#[doc = "ISI Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_idr](isi_idr) module"]
pub type ISI_IDR = crate::Reg<u32, _ISI_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_IDR;
#[doc = "`write(|w| ..)` method takes [isi_idr::W](isi_idr::W) writer structure"]
impl crate::Writable for ISI_IDR {}
#[doc = "ISI Interrupt Disable Register"]
pub mod isi_idr;
#[doc = "ISI Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_imr](isi_imr) module"]
pub type ISI_IMR = crate::Reg<u32, _ISI_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_IMR;
#[doc = "`read()` method returns [isi_imr::R](isi_imr::R) reader structure"]
impl crate::Readable for ISI_IMR {}
#[doc = "ISI Interrupt Mask Register"]
pub mod isi_imr;
#[doc = "DMA Channel Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_dma_cher](isi_dma_cher) module"]
pub type ISI_DMA_CHER = crate::Reg<u32, _ISI_DMA_CHER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_DMA_CHER;
#[doc = "`write(|w| ..)` method takes [isi_dma_cher::W](isi_dma_cher::W) writer structure"]
impl crate::Writable for ISI_DMA_CHER {}
#[doc = "DMA Channel Enable Register"]
pub mod isi_dma_cher;
#[doc = "DMA Channel Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_dma_chdr](isi_dma_chdr) module"]
pub type ISI_DMA_CHDR = crate::Reg<u32, _ISI_DMA_CHDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_DMA_CHDR;
#[doc = "`write(|w| ..)` method takes [isi_dma_chdr::W](isi_dma_chdr::W) writer structure"]
impl crate::Writable for ISI_DMA_CHDR {}
#[doc = "DMA Channel Disable Register"]
pub mod isi_dma_chdr;
#[doc = "DMA Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_dma_chsr](isi_dma_chsr) module"]
pub type ISI_DMA_CHSR = crate::Reg<u32, _ISI_DMA_CHSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_DMA_CHSR;
#[doc = "`read()` method returns [isi_dma_chsr::R](isi_dma_chsr::R) reader structure"]
impl crate::Readable for ISI_DMA_CHSR {}
#[doc = "DMA Channel Status Register"]
pub mod isi_dma_chsr;
#[doc = "DMA Preview Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_dma_p_addr](isi_dma_p_addr) module"]
pub type ISI_DMA_P_ADDR = crate::Reg<u32, _ISI_DMA_P_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_DMA_P_ADDR;
#[doc = "`read()` method returns [isi_dma_p_addr::R](isi_dma_p_addr::R) reader structure"]
impl crate::Readable for ISI_DMA_P_ADDR {}
#[doc = "`write(|w| ..)` method takes [isi_dma_p_addr::W](isi_dma_p_addr::W) writer structure"]
impl crate::Writable for ISI_DMA_P_ADDR {}
#[doc = "DMA Preview Base Address Register"]
pub mod isi_dma_p_addr;
#[doc = "DMA Preview Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_dma_p_ctrl](isi_dma_p_ctrl) module"]
pub type ISI_DMA_P_CTRL = crate::Reg<u32, _ISI_DMA_P_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_DMA_P_CTRL;
#[doc = "`read()` method returns [isi_dma_p_ctrl::R](isi_dma_p_ctrl::R) reader structure"]
impl crate::Readable for ISI_DMA_P_CTRL {}
#[doc = "`write(|w| ..)` method takes [isi_dma_p_ctrl::W](isi_dma_p_ctrl::W) writer structure"]
impl crate::Writable for ISI_DMA_P_CTRL {}
#[doc = "DMA Preview Control Register"]
pub mod isi_dma_p_ctrl;
#[doc = "DMA Preview Descriptor Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_dma_p_dscr](isi_dma_p_dscr) module"]
pub type ISI_DMA_P_DSCR = crate::Reg<u32, _ISI_DMA_P_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_DMA_P_DSCR;
#[doc = "`read()` method returns [isi_dma_p_dscr::R](isi_dma_p_dscr::R) reader structure"]
impl crate::Readable for ISI_DMA_P_DSCR {}
#[doc = "`write(|w| ..)` method takes [isi_dma_p_dscr::W](isi_dma_p_dscr::W) writer structure"]
impl crate::Writable for ISI_DMA_P_DSCR {}
#[doc = "DMA Preview Descriptor Address Register"]
pub mod isi_dma_p_dscr;
#[doc = "DMA Codec Base Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_dma_c_addr](isi_dma_c_addr) module"]
pub type ISI_DMA_C_ADDR = crate::Reg<u32, _ISI_DMA_C_ADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_DMA_C_ADDR;
#[doc = "`read()` method returns [isi_dma_c_addr::R](isi_dma_c_addr::R) reader structure"]
impl crate::Readable for ISI_DMA_C_ADDR {}
#[doc = "`write(|w| ..)` method takes [isi_dma_c_addr::W](isi_dma_c_addr::W) writer structure"]
impl crate::Writable for ISI_DMA_C_ADDR {}
#[doc = "DMA Codec Base Address Register"]
pub mod isi_dma_c_addr;
#[doc = "DMA Codec Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_dma_c_ctrl](isi_dma_c_ctrl) module"]
pub type ISI_DMA_C_CTRL = crate::Reg<u32, _ISI_DMA_C_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_DMA_C_CTRL;
#[doc = "`read()` method returns [isi_dma_c_ctrl::R](isi_dma_c_ctrl::R) reader structure"]
impl crate::Readable for ISI_DMA_C_CTRL {}
#[doc = "`write(|w| ..)` method takes [isi_dma_c_ctrl::W](isi_dma_c_ctrl::W) writer structure"]
impl crate::Writable for ISI_DMA_C_CTRL {}
#[doc = "DMA Codec Control Register"]
pub mod isi_dma_c_ctrl;
#[doc = "DMA Codec Descriptor Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_dma_c_dscr](isi_dma_c_dscr) module"]
pub type ISI_DMA_C_DSCR = crate::Reg<u32, _ISI_DMA_C_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_DMA_C_DSCR;
#[doc = "`read()` method returns [isi_dma_c_dscr::R](isi_dma_c_dscr::R) reader structure"]
impl crate::Readable for ISI_DMA_C_DSCR {}
#[doc = "`write(|w| ..)` method takes [isi_dma_c_dscr::W](isi_dma_c_dscr::W) writer structure"]
impl crate::Writable for ISI_DMA_C_DSCR {}
#[doc = "DMA Codec Descriptor Address Register"]
pub mod isi_dma_c_dscr;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_wpmr](isi_wpmr) module"]
pub type ISI_WPMR = crate::Reg<u32, _ISI_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_WPMR;
#[doc = "`read()` method returns [isi_wpmr::R](isi_wpmr::R) reader structure"]
impl crate::Readable for ISI_WPMR {}
#[doc = "`write(|w| ..)` method takes [isi_wpmr::W](isi_wpmr::W) writer structure"]
impl crate::Writable for ISI_WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod isi_wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isi_wpsr](isi_wpsr) module"]
pub type ISI_WPSR = crate::Reg<u32, _ISI_WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISI_WPSR;
#[doc = "`read()` method returns [isi_wpsr::R](isi_wpsr::R) reader structure"]
impl crate::Readable for ISI_WPSR {}
#[doc = "Write Protection Status Register"]
pub mod isi_wpsr;
