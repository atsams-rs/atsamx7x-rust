#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub mr: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x08 - Data Timeout Register"]
    pub dtor: crate::Reg<dtor::DTOR_SPEC>,
    #[doc = "0x0c - SD/SDIO Card Register"]
    pub sdcr: crate::Reg<sdcr::SDCR_SPEC>,
    #[doc = "0x10 - Argument Register"]
    pub argr: crate::Reg<argr::ARGR_SPEC>,
    #[doc = "0x14 - Command Register"]
    pub cmdr: crate::Reg<cmdr::CMDR_SPEC>,
    #[doc = "0x18 - Block Register"]
    pub blkr: crate::Reg<blkr::BLKR_SPEC>,
    #[doc = "0x1c - Completion Signal Timeout Register"]
    pub cstor: crate::Reg<cstor::CSTOR_SPEC>,
    #[doc = "0x20..0x30 - Response Register 0"]
    pub rspr: [crate::Reg<rspr::RSPR_SPEC>; 4],
    #[doc = "0x30 - Receive Data Register"]
    pub rdr: crate::Reg<rdr::RDR_SPEC>,
    #[doc = "0x34 - Transmit Data Register"]
    pub tdr: crate::Reg<tdr::TDR_SPEC>,
    _reserved11: [u8; 0x08],
    #[doc = "0x40 - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x44 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x48 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x4c - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x50 - DMA Configuration Register"]
    pub dma: crate::Reg<dma::DMA_SPEC>,
    #[doc = "0x54 - Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    _reserved17: [u8; 0x8c],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
    _reserved19: [u8; 0x0114],
    #[doc = "0x200..0x600 - FIFO Memory Aperture0 0"]
    pub fifo: [crate::Reg<fifo::FIFO_SPEC>; 256],
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "DTOR register accessor: an alias for `Reg<DTOR_SPEC>`"]
pub type DTOR = crate::Reg<dtor::DTOR_SPEC>;
#[doc = "Data Timeout Register"]
pub mod dtor;
#[doc = "SDCR register accessor: an alias for `Reg<SDCR_SPEC>`"]
pub type SDCR = crate::Reg<sdcr::SDCR_SPEC>;
#[doc = "SD/SDIO Card Register"]
pub mod sdcr;
#[doc = "ARGR register accessor: an alias for `Reg<ARGR_SPEC>`"]
pub type ARGR = crate::Reg<argr::ARGR_SPEC>;
#[doc = "Argument Register"]
pub mod argr;
#[doc = "CMDR register accessor: an alias for `Reg<CMDR_SPEC>`"]
pub type CMDR = crate::Reg<cmdr::CMDR_SPEC>;
#[doc = "Command Register"]
pub mod cmdr;
#[doc = "BLKR register accessor: an alias for `Reg<BLKR_SPEC>`"]
pub type BLKR = crate::Reg<blkr::BLKR_SPEC>;
#[doc = "Block Register"]
pub mod blkr;
#[doc = "CSTOR register accessor: an alias for `Reg<CSTOR_SPEC>`"]
pub type CSTOR = crate::Reg<cstor::CSTOR_SPEC>;
#[doc = "Completion Signal Timeout Register"]
pub mod cstor;
#[doc = "RSPR register accessor: an alias for `Reg<RSPR_SPEC>`"]
pub type RSPR = crate::Reg<rspr::RSPR_SPEC>;
#[doc = "Response Register 0"]
pub mod rspr;
#[doc = "RDR register accessor: an alias for `Reg<RDR_SPEC>`"]
pub type RDR = crate::Reg<rdr::RDR_SPEC>;
#[doc = "Receive Data Register"]
pub mod rdr;
#[doc = "TDR register accessor: an alias for `Reg<TDR_SPEC>`"]
pub type TDR = crate::Reg<tdr::TDR_SPEC>;
#[doc = "Transmit Data Register"]
pub mod tdr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "DMA register accessor: an alias for `Reg<DMA_SPEC>`"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "DMA Configuration Register"]
pub mod dma;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
#[doc = "FIFO register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "FIFO Memory Aperture0 0"]
pub mod fifo;
