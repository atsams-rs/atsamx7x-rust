#[doc = r" Register block"]
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
    _reserved0: [u8; 136usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub isi_wpmr: ISI_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub isi_wpsr: ISI_WPSR,
}
#[doc = "ISI Configuration 1 Register"]
pub struct ISI_CFG1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISI Configuration 1 Register"]
pub mod isi_cfg1;
#[doc = "ISI Configuration 2 Register"]
pub struct ISI_CFG2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISI Configuration 2 Register"]
pub mod isi_cfg2;
#[doc = "ISI Preview Size Register"]
pub struct ISI_PSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISI Preview Size Register"]
pub mod isi_psize;
#[doc = "ISI Preview Decimation Factor Register"]
pub struct ISI_PDECF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISI Preview Decimation Factor Register"]
pub mod isi_pdecf;
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 0 Register"]
pub struct ISI_Y2R_SET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 0 Register"]
pub mod isi_y2r_set0;
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 1 Register"]
pub struct ISI_Y2R_SET1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISI Color Space Conversion YCrCb To RGB Set 1 Register"]
pub mod isi_y2r_set1;
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 0 Register"]
pub struct ISI_R2Y_SET0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 0 Register"]
pub mod isi_r2y_set0;
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 1 Register"]
pub struct ISI_R2Y_SET1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 1 Register"]
pub mod isi_r2y_set1;
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 2 Register"]
pub struct ISI_R2Y_SET2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 2 Register"]
pub mod isi_r2y_set2;
#[doc = "ISI Control Register"]
pub struct ISI_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISI Control Register"]
pub mod isi_cr;
#[doc = "ISI Status Register"]
pub struct ISI_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISI Status Register"]
pub mod isi_sr;
#[doc = "ISI Interrupt Enable Register"]
pub struct ISI_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISI Interrupt Enable Register"]
pub mod isi_ier;
#[doc = "ISI Interrupt Disable Register"]
pub struct ISI_IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISI Interrupt Disable Register"]
pub mod isi_idr;
#[doc = "ISI Interrupt Mask Register"]
pub struct ISI_IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "ISI Interrupt Mask Register"]
pub mod isi_imr;
#[doc = "DMA Channel Enable Register"]
pub struct ISI_DMA_CHER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Enable Register"]
pub mod isi_dma_cher;
#[doc = "DMA Channel Disable Register"]
pub struct ISI_DMA_CHDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Disable Register"]
pub mod isi_dma_chdr;
#[doc = "DMA Channel Status Register"]
pub struct ISI_DMA_CHSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel Status Register"]
pub mod isi_dma_chsr;
#[doc = "DMA Preview Base Address Register"]
pub struct ISI_DMA_P_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Preview Base Address Register"]
pub mod isi_dma_p_addr;
#[doc = "DMA Preview Control Register"]
pub struct ISI_DMA_P_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Preview Control Register"]
pub mod isi_dma_p_ctrl;
#[doc = "DMA Preview Descriptor Address Register"]
pub struct ISI_DMA_P_DSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Preview Descriptor Address Register"]
pub mod isi_dma_p_dscr;
#[doc = "DMA Codec Base Address Register"]
pub struct ISI_DMA_C_ADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Codec Base Address Register"]
pub mod isi_dma_c_addr;
#[doc = "DMA Codec Control Register"]
pub struct ISI_DMA_C_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Codec Control Register"]
pub mod isi_dma_c_ctrl;
#[doc = "DMA Codec Descriptor Address Register"]
pub struct ISI_DMA_C_DSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Codec Descriptor Address Register"]
pub mod isi_dma_c_dscr;
#[doc = "Write Protection Mode Register"]
pub struct ISI_WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Mode Register"]
pub mod isi_wpmr;
#[doc = "Write Protection Status Register"]
pub struct ISI_WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Status Register"]
pub mod isi_wpsr;
