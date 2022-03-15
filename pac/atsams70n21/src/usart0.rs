#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub us_cr: crate::Reg<us_cr::US_CR_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub us_mr: crate::Reg<us_mr::US_MR_SPEC>,
    #[doc = "0x08 - Interrupt Enable Register"]
    pub us_ier: crate::Reg<us_ier::US_IER_SPEC>,
    #[doc = "0x0c - Interrupt Disable Register"]
    pub us_idr: crate::Reg<us_idr::US_IDR_SPEC>,
    #[doc = "0x10 - Interrupt Mask Register"]
    pub us_imr: crate::Reg<us_imr::US_IMR_SPEC>,
    #[doc = "0x14 - Channel Status Register"]
    pub us_csr: crate::Reg<us_csr::US_CSR_SPEC>,
    #[doc = "0x18 - Receive Holding Register"]
    pub us_rhr: crate::Reg<us_rhr::US_RHR_SPEC>,
    #[doc = "0x1c - Transmit Holding Register"]
    pub us_thr: crate::Reg<us_thr::US_THR_SPEC>,
    #[doc = "0x20 - Baud Rate Generator Register"]
    pub us_brgr: crate::Reg<us_brgr::US_BRGR_SPEC>,
    #[doc = "0x24 - Receiver Time-out Register"]
    pub us_rtor: crate::Reg<us_rtor::US_RTOR_SPEC>,
    #[doc = "0x28 - Transmitter Timeguard Register"]
    pub us_ttgr: crate::Reg<us_ttgr::US_TTGR_SPEC>,
    _reserved11: [u8; 0x14],
    #[doc = "0x40 - FI DI Ratio Register"]
    pub us_fidi: crate::Reg<us_fidi::US_FIDI_SPEC>,
    #[doc = "0x44 - Number of Errors Register"]
    pub us_ner: crate::Reg<us_ner::US_NER_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x4c - IrDA Filter Register"]
    pub us_if: crate::Reg<us_if::US_IF_SPEC>,
    #[doc = "0x50 - Manchester Configuration Register"]
    pub us_man: crate::Reg<us_man::US_MAN_SPEC>,
    #[doc = "0x54 - LIN Mode Register"]
    pub us_linmr: crate::Reg<us_linmr::US_LINMR_SPEC>,
    #[doc = "0x58 - LIN Identifier Register"]
    pub us_linir: crate::Reg<us_linir::US_LINIR_SPEC>,
    #[doc = "0x5c - LIN Baud Rate Register"]
    pub us_linbrr: crate::Reg<us_linbrr::US_LINBRR_SPEC>,
    #[doc = "0x60 - LON Mode Register"]
    pub us_lonmr: crate::Reg<us_lonmr::US_LONMR_SPEC>,
    #[doc = "0x64 - LON Preamble Register"]
    pub us_lonpr: crate::Reg<us_lonpr::US_LONPR_SPEC>,
    #[doc = "0x68 - LON Data Length Register"]
    pub us_londl: crate::Reg<us_londl::US_LONDL_SPEC>,
    #[doc = "0x6c - LON L2HDR Register"]
    pub us_lonl2hdr: crate::Reg<us_lonl2hdr::US_LONL2HDR_SPEC>,
    #[doc = "0x70 - LON Backlog Register"]
    pub us_lonbl: crate::Reg<us_lonbl::US_LONBL_SPEC>,
    #[doc = "0x74 - LON Beta1 Tx Register"]
    pub us_lonb1tx: crate::Reg<us_lonb1tx::US_LONB1TX_SPEC>,
    #[doc = "0x78 - LON Beta1 Rx Register"]
    pub us_lonb1rx: crate::Reg<us_lonb1rx::US_LONB1RX_SPEC>,
    #[doc = "0x7c - LON Priority Register"]
    pub us_lonprio: crate::Reg<us_lonprio::US_LONPRIO_SPEC>,
    #[doc = "0x80 - LON IDT Tx Register"]
    pub us_idttx: crate::Reg<us_idttx::US_IDTTX_SPEC>,
    #[doc = "0x84 - LON IDT Rx Register"]
    pub us_idtrx: crate::Reg<us_idtrx::US_IDTRX_SPEC>,
    #[doc = "0x88 - IC DIFF Register"]
    pub us_icdiff: crate::Reg<us_icdiff::US_ICDIFF_SPEC>,
    _reserved29: [u8; 0x58],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub us_wpmr: crate::Reg<us_wpmr::US_WPMR_SPEC>,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub us_wpsr: crate::Reg<us_wpsr::US_WPSR_SPEC>,
}
#[doc = "US_CR register accessor: an alias for `Reg<US_CR_SPEC>`"]
pub type US_CR = crate::Reg<us_cr::US_CR_SPEC>;
#[doc = "Control Register"]
pub mod us_cr;
#[doc = "US_MR register accessor: an alias for `Reg<US_MR_SPEC>`"]
pub type US_MR = crate::Reg<us_mr::US_MR_SPEC>;
#[doc = "Mode Register"]
pub mod us_mr;
#[doc = "US_IER register accessor: an alias for `Reg<US_IER_SPEC>`"]
pub type US_IER = crate::Reg<us_ier::US_IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod us_ier;
#[doc = "US_IDR register accessor: an alias for `Reg<US_IDR_SPEC>`"]
pub type US_IDR = crate::Reg<us_idr::US_IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod us_idr;
#[doc = "US_IMR register accessor: an alias for `Reg<US_IMR_SPEC>`"]
pub type US_IMR = crate::Reg<us_imr::US_IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod us_imr;
#[doc = "US_CSR register accessor: an alias for `Reg<US_CSR_SPEC>`"]
pub type US_CSR = crate::Reg<us_csr::US_CSR_SPEC>;
#[doc = "Channel Status Register"]
pub mod us_csr;
#[doc = "US_RHR register accessor: an alias for `Reg<US_RHR_SPEC>`"]
pub type US_RHR = crate::Reg<us_rhr::US_RHR_SPEC>;
#[doc = "Receive Holding Register"]
pub mod us_rhr;
#[doc = "US_THR register accessor: an alias for `Reg<US_THR_SPEC>`"]
pub type US_THR = crate::Reg<us_thr::US_THR_SPEC>;
#[doc = "Transmit Holding Register"]
pub mod us_thr;
#[doc = "US_BRGR register accessor: an alias for `Reg<US_BRGR_SPEC>`"]
pub type US_BRGR = crate::Reg<us_brgr::US_BRGR_SPEC>;
#[doc = "Baud Rate Generator Register"]
pub mod us_brgr;
#[doc = "US_RTOR register accessor: an alias for `Reg<US_RTOR_SPEC>`"]
pub type US_RTOR = crate::Reg<us_rtor::US_RTOR_SPEC>;
#[doc = "Receiver Time-out Register"]
pub mod us_rtor;
#[doc = "US_TTGR register accessor: an alias for `Reg<US_TTGR_SPEC>`"]
pub type US_TTGR = crate::Reg<us_ttgr::US_TTGR_SPEC>;
#[doc = "Transmitter Timeguard Register"]
pub mod us_ttgr;
#[doc = "US_FIDI register accessor: an alias for `Reg<US_FIDI_SPEC>`"]
pub type US_FIDI = crate::Reg<us_fidi::US_FIDI_SPEC>;
#[doc = "FI DI Ratio Register"]
pub mod us_fidi;
#[doc = "US_NER register accessor: an alias for `Reg<US_NER_SPEC>`"]
pub type US_NER = crate::Reg<us_ner::US_NER_SPEC>;
#[doc = "Number of Errors Register"]
pub mod us_ner;
#[doc = "US_IF register accessor: an alias for `Reg<US_IF_SPEC>`"]
pub type US_IF = crate::Reg<us_if::US_IF_SPEC>;
#[doc = "IrDA Filter Register"]
pub mod us_if;
#[doc = "US_MAN register accessor: an alias for `Reg<US_MAN_SPEC>`"]
pub type US_MAN = crate::Reg<us_man::US_MAN_SPEC>;
#[doc = "Manchester Configuration Register"]
pub mod us_man;
#[doc = "US_LINMR register accessor: an alias for `Reg<US_LINMR_SPEC>`"]
pub type US_LINMR = crate::Reg<us_linmr::US_LINMR_SPEC>;
#[doc = "LIN Mode Register"]
pub mod us_linmr;
#[doc = "US_LINIR register accessor: an alias for `Reg<US_LINIR_SPEC>`"]
pub type US_LINIR = crate::Reg<us_linir::US_LINIR_SPEC>;
#[doc = "LIN Identifier Register"]
pub mod us_linir;
#[doc = "US_LINBRR register accessor: an alias for `Reg<US_LINBRR_SPEC>`"]
pub type US_LINBRR = crate::Reg<us_linbrr::US_LINBRR_SPEC>;
#[doc = "LIN Baud Rate Register"]
pub mod us_linbrr;
#[doc = "US_LONMR register accessor: an alias for `Reg<US_LONMR_SPEC>`"]
pub type US_LONMR = crate::Reg<us_lonmr::US_LONMR_SPEC>;
#[doc = "LON Mode Register"]
pub mod us_lonmr;
#[doc = "US_LONPR register accessor: an alias for `Reg<US_LONPR_SPEC>`"]
pub type US_LONPR = crate::Reg<us_lonpr::US_LONPR_SPEC>;
#[doc = "LON Preamble Register"]
pub mod us_lonpr;
#[doc = "US_LONDL register accessor: an alias for `Reg<US_LONDL_SPEC>`"]
pub type US_LONDL = crate::Reg<us_londl::US_LONDL_SPEC>;
#[doc = "LON Data Length Register"]
pub mod us_londl;
#[doc = "US_LONL2HDR register accessor: an alias for `Reg<US_LONL2HDR_SPEC>`"]
pub type US_LONL2HDR = crate::Reg<us_lonl2hdr::US_LONL2HDR_SPEC>;
#[doc = "LON L2HDR Register"]
pub mod us_lonl2hdr;
#[doc = "US_LONBL register accessor: an alias for `Reg<US_LONBL_SPEC>`"]
pub type US_LONBL = crate::Reg<us_lonbl::US_LONBL_SPEC>;
#[doc = "LON Backlog Register"]
pub mod us_lonbl;
#[doc = "US_LONB1TX register accessor: an alias for `Reg<US_LONB1TX_SPEC>`"]
pub type US_LONB1TX = crate::Reg<us_lonb1tx::US_LONB1TX_SPEC>;
#[doc = "LON Beta1 Tx Register"]
pub mod us_lonb1tx;
#[doc = "US_LONB1RX register accessor: an alias for `Reg<US_LONB1RX_SPEC>`"]
pub type US_LONB1RX = crate::Reg<us_lonb1rx::US_LONB1RX_SPEC>;
#[doc = "LON Beta1 Rx Register"]
pub mod us_lonb1rx;
#[doc = "US_LONPRIO register accessor: an alias for `Reg<US_LONPRIO_SPEC>`"]
pub type US_LONPRIO = crate::Reg<us_lonprio::US_LONPRIO_SPEC>;
#[doc = "LON Priority Register"]
pub mod us_lonprio;
#[doc = "US_IDTTX register accessor: an alias for `Reg<US_IDTTX_SPEC>`"]
pub type US_IDTTX = crate::Reg<us_idttx::US_IDTTX_SPEC>;
#[doc = "LON IDT Tx Register"]
pub mod us_idttx;
#[doc = "US_IDTRX register accessor: an alias for `Reg<US_IDTRX_SPEC>`"]
pub type US_IDTRX = crate::Reg<us_idtrx::US_IDTRX_SPEC>;
#[doc = "LON IDT Rx Register"]
pub mod us_idtrx;
#[doc = "US_ICDIFF register accessor: an alias for `Reg<US_ICDIFF_SPEC>`"]
pub type US_ICDIFF = crate::Reg<us_icdiff::US_ICDIFF_SPEC>;
#[doc = "IC DIFF Register"]
pub mod us_icdiff;
#[doc = "US_WPMR register accessor: an alias for `Reg<US_WPMR_SPEC>`"]
pub type US_WPMR = crate::Reg<us_wpmr::US_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod us_wpmr;
#[doc = "US_WPSR register accessor: an alias for `Reg<US_WPSR_SPEC>`"]
pub type US_WPSR = crate::Reg<us_wpsr::US_WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod us_wpsr;
