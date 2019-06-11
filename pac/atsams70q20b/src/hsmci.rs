#[doc = r" Register block"]
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
    _reserved0: [u8; 8usize],
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
    _reserved1: [u8; 140usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub hsmci_wpmr: HSMCI_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub hsmci_wpsr: HSMCI_WPSR,
    _reserved2: [u8; 276usize],
    #[doc = "0x200 - FIFO Memory Aperture0 0"]
    pub hsmci_fifo: [HSMCI_FIFO; 256],
}
#[doc = "Control Register"]
pub struct HSMCI_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod hsmci_cr;
#[doc = "Mode Register"]
pub struct HSMCI_MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod hsmci_mr;
#[doc = "Data Timeout Register"]
pub struct HSMCI_DTOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Timeout Register"]
pub mod hsmci_dtor;
#[doc = "SD/SDIO Card Register"]
pub struct HSMCI_SDCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SD/SDIO Card Register"]
pub mod hsmci_sdcr;
#[doc = "Argument Register"]
pub struct HSMCI_ARGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Argument Register"]
pub mod hsmci_argr;
#[doc = "Command Register"]
pub struct HSMCI_CMDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod hsmci_cmdr;
#[doc = "Block Register"]
pub struct HSMCI_BLKR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Block Register"]
pub mod hsmci_blkr;
#[doc = "Completion Signal Timeout Register"]
pub struct HSMCI_CSTOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Completion Signal Timeout Register"]
pub mod hsmci_cstor;
#[doc = "Response Register 0"]
pub struct HSMCI_RSPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response Register 0"]
pub mod hsmci_rspr;
#[doc = "Receive Data Register"]
pub struct HSMCI_RDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Data Register"]
pub mod hsmci_rdr;
#[doc = "Transmit Data Register"]
pub struct HSMCI_TDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Data Register"]
pub mod hsmci_tdr;
#[doc = "Status Register"]
pub struct HSMCI_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod hsmci_sr;
#[doc = "Interrupt Enable Register"]
pub struct HSMCI_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod hsmci_ier;
#[doc = "Interrupt Disable Register"]
pub struct HSMCI_IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod hsmci_idr;
#[doc = "Interrupt Mask Register"]
pub struct HSMCI_IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod hsmci_imr;
#[doc = "DMA Configuration Register"]
pub struct HSMCI_DMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Configuration Register"]
pub mod hsmci_dma;
#[doc = "Configuration Register"]
pub struct HSMCI_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Register"]
pub mod hsmci_cfg;
#[doc = "Write Protection Mode Register"]
pub struct HSMCI_WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Mode Register"]
pub mod hsmci_wpmr;
#[doc = "Write Protection Status Register"]
pub struct HSMCI_WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Status Register"]
pub mod hsmci_wpsr;
#[doc = "FIFO Memory Aperture0 0"]
pub struct HSMCI_FIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Memory Aperture0 0"]
pub mod hsmci_fifo;
