#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Core Release Register"]
    pub crel: crate::Reg<crel::CREL_SPEC>,
    #[doc = "0x04 - Endian Register"]
    pub endn: crate::Reg<endn::ENDN_SPEC>,
    #[doc = "0x08 - Customer Register"]
    pub cust: crate::Reg<cust::CUST_SPEC>,
    #[doc = "0x0c - Data Bit Timing and Prescaler Register"]
    pub dbtp: crate::Reg<dbtp::DBTP_SPEC>,
    #[doc = "0x10 - Test Register"]
    pub test: crate::Reg<test::TEST_SPEC>,
    #[doc = "0x14 - RAM Watchdog Register"]
    pub rwd: crate::Reg<rwd::RWD_SPEC>,
    #[doc = "0x18 - CC Control Register"]
    pub cccr: crate::Reg<cccr::CCCR_SPEC>,
    #[doc = "0x1c - Nominal Bit Timing and Prescaler Register"]
    pub nbtp: crate::Reg<nbtp::NBTP_SPEC>,
    #[doc = "0x20 - Timestamp Counter Configuration Register"]
    pub tscc: crate::Reg<tscc::TSCC_SPEC>,
    #[doc = "0x24 - Timestamp Counter Value Register"]
    pub tscv: crate::Reg<tscv::TSCV_SPEC>,
    #[doc = "0x28 - Timeout Counter Configuration Register"]
    pub tocc: crate::Reg<tocc::TOCC_SPEC>,
    #[doc = "0x2c - Timeout Counter Value Register"]
    pub tocv: crate::Reg<tocv::TOCV_SPEC>,
    _reserved12: [u8; 0x10],
    #[doc = "0x40 - Error Counter Register"]
    pub ecr: crate::Reg<ecr::ECR_SPEC>,
    #[doc = "0x44 - Protocol Status Register"]
    pub psr: crate::Reg<psr::PSR_SPEC>,
    #[doc = "0x48 - Transmit Delay Compensation Register"]
    pub tdcr: crate::Reg<tdcr::TDCR_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x50 - Interrupt Register"]
    pub ir: crate::Reg<ir::IR_SPEC>,
    #[doc = "0x54 - Interrupt Enable Register"]
    pub ie: crate::Reg<ie::IE_SPEC>,
    #[doc = "0x58 - Interrupt Line Select Register"]
    pub ils: crate::Reg<ils::ILS_SPEC>,
    #[doc = "0x5c - Interrupt Line Enable Register"]
    pub ile: crate::Reg<ile::ILE_SPEC>,
    _reserved19: [u8; 0x20],
    #[doc = "0x80 - Global Filter Configuration Register"]
    pub gfc: crate::Reg<gfc::GFC_SPEC>,
    #[doc = "0x84 - Standard ID Filter Configuration Register"]
    pub sidfc: crate::Reg<sidfc::SIDFC_SPEC>,
    #[doc = "0x88 - Extended ID Filter Configuration Register"]
    pub xidfc: crate::Reg<xidfc::XIDFC_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x90 - Extended ID AND Mask Register"]
    pub xidam: crate::Reg<xidam::XIDAM_SPEC>,
    #[doc = "0x94 - High Priority Message Status Register"]
    pub hpms: crate::Reg<hpms::HPMS_SPEC>,
    #[doc = "0x98 - New Data 1 Register"]
    pub ndat1: crate::Reg<ndat1::NDAT1_SPEC>,
    #[doc = "0x9c - New Data 2 Register"]
    pub ndat2: crate::Reg<ndat2::NDAT2_SPEC>,
    #[doc = "0xa0 - Receive FIFO 0 Configuration Register"]
    pub rxf0c: crate::Reg<rxf0c::RXF0C_SPEC>,
    #[doc = "0xa4 - Receive FIFO 0 Status Register"]
    pub rxf0s: crate::Reg<rxf0s::RXF0S_SPEC>,
    #[doc = "0xa8 - Receive FIFO 0 Acknowledge Register"]
    pub rxf0a: crate::Reg<rxf0a::RXF0A_SPEC>,
    #[doc = "0xac - Receive Rx Buffer Configuration Register"]
    pub rxbc: crate::Reg<rxbc::RXBC_SPEC>,
    #[doc = "0xb0 - Receive FIFO 1 Configuration Register"]
    pub rxf1c: crate::Reg<rxf1c::RXF1C_SPEC>,
    #[doc = "0xb4 - Receive FIFO 1 Status Register"]
    pub rxf1s: crate::Reg<rxf1s::RXF1S_SPEC>,
    #[doc = "0xb8 - Receive FIFO 1 Acknowledge Register"]
    pub rxf1a: crate::Reg<rxf1a::RXF1A_SPEC>,
    #[doc = "0xbc - Receive Buffer / FIFO Element Size Configuration Register"]
    pub rxesc: crate::Reg<rxesc::RXESC_SPEC>,
    #[doc = "0xc0 - Transmit Buffer Configuration Register"]
    pub txbc: crate::Reg<txbc::TXBC_SPEC>,
    #[doc = "0xc4 - Transmit FIFO/Queue Status Register"]
    pub txfqs: crate::Reg<txfqs::TXFQS_SPEC>,
    #[doc = "0xc8 - Transmit Buffer Element Size Configuration Register"]
    pub txesc: crate::Reg<txesc::TXESC_SPEC>,
    #[doc = "0xcc - Transmit Buffer Request Pending Register"]
    pub txbrp: crate::Reg<txbrp::TXBRP_SPEC>,
    #[doc = "0xd0 - Transmit Buffer Add Request Register"]
    pub txbar: crate::Reg<txbar::TXBAR_SPEC>,
    #[doc = "0xd4 - Transmit Buffer Cancellation Request Register"]
    pub txbcr: crate::Reg<txbcr::TXBCR_SPEC>,
    #[doc = "0xd8 - Transmit Buffer Transmission Occurred Register"]
    pub txbto: crate::Reg<txbto::TXBTO_SPEC>,
    #[doc = "0xdc - Transmit Buffer Cancellation Finished Register"]
    pub txbcf: crate::Reg<txbcf::TXBCF_SPEC>,
    #[doc = "0xe0 - Transmit Buffer Transmission Interrupt Enable Register"]
    pub txbtie: crate::Reg<txbtie::TXBTIE_SPEC>,
    #[doc = "0xe4 - Transmit Buffer Cancellation Finished Interrupt Enable Register"]
    pub txbcie: crate::Reg<txbcie::TXBCIE_SPEC>,
    _reserved44: [u8; 0x08],
    #[doc = "0xf0 - Transmit Event FIFO Configuration Register"]
    pub txefc: crate::Reg<txefc::TXEFC_SPEC>,
    #[doc = "0xf4 - Transmit Event FIFO Status Register"]
    pub txefs: crate::Reg<txefs::TXEFS_SPEC>,
    #[doc = "0xf8 - Transmit Event FIFO Acknowledge Register"]
    pub txefa: crate::Reg<txefa::TXEFA_SPEC>,
}
#[doc = "CREL register accessor: an alias for `Reg<CREL_SPEC>`"]
pub type CREL = crate::Reg<crel::CREL_SPEC>;
#[doc = "Core Release Register"]
pub mod crel;
#[doc = "ENDN register accessor: an alias for `Reg<ENDN_SPEC>`"]
pub type ENDN = crate::Reg<endn::ENDN_SPEC>;
#[doc = "Endian Register"]
pub mod endn;
#[doc = "CUST register accessor: an alias for `Reg<CUST_SPEC>`"]
pub type CUST = crate::Reg<cust::CUST_SPEC>;
#[doc = "Customer Register"]
pub mod cust;
#[doc = "DBTP register accessor: an alias for `Reg<DBTP_SPEC>`"]
pub type DBTP = crate::Reg<dbtp::DBTP_SPEC>;
#[doc = "Data Bit Timing and Prescaler Register"]
pub mod dbtp;
#[doc = "TEST register accessor: an alias for `Reg<TEST_SPEC>`"]
pub type TEST = crate::Reg<test::TEST_SPEC>;
#[doc = "Test Register"]
pub mod test;
#[doc = "RWD register accessor: an alias for `Reg<RWD_SPEC>`"]
pub type RWD = crate::Reg<rwd::RWD_SPEC>;
#[doc = "RAM Watchdog Register"]
pub mod rwd;
#[doc = "CCCR register accessor: an alias for `Reg<CCCR_SPEC>`"]
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
#[doc = "CC Control Register"]
pub mod cccr;
#[doc = "NBTP register accessor: an alias for `Reg<NBTP_SPEC>`"]
pub type NBTP = crate::Reg<nbtp::NBTP_SPEC>;
#[doc = "Nominal Bit Timing and Prescaler Register"]
pub mod nbtp;
#[doc = "TSCC register accessor: an alias for `Reg<TSCC_SPEC>`"]
pub type TSCC = crate::Reg<tscc::TSCC_SPEC>;
#[doc = "Timestamp Counter Configuration Register"]
pub mod tscc;
#[doc = "TSCV register accessor: an alias for `Reg<TSCV_SPEC>`"]
pub type TSCV = crate::Reg<tscv::TSCV_SPEC>;
#[doc = "Timestamp Counter Value Register"]
pub mod tscv;
#[doc = "TOCC register accessor: an alias for `Reg<TOCC_SPEC>`"]
pub type TOCC = crate::Reg<tocc::TOCC_SPEC>;
#[doc = "Timeout Counter Configuration Register"]
pub mod tocc;
#[doc = "TOCV register accessor: an alias for `Reg<TOCV_SPEC>`"]
pub type TOCV = crate::Reg<tocv::TOCV_SPEC>;
#[doc = "Timeout Counter Value Register"]
pub mod tocv;
#[doc = "ECR register accessor: an alias for `Reg<ECR_SPEC>`"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Error Counter Register"]
pub mod ecr;
#[doc = "PSR register accessor: an alias for `Reg<PSR_SPEC>`"]
pub type PSR = crate::Reg<psr::PSR_SPEC>;
#[doc = "Protocol Status Register"]
pub mod psr;
#[doc = "TDCR register accessor: an alias for `Reg<TDCR_SPEC>`"]
pub type TDCR = crate::Reg<tdcr::TDCR_SPEC>;
#[doc = "Transmit Delay Compensation Register"]
pub mod tdcr;
#[doc = "IR register accessor: an alias for `Reg<IR_SPEC>`"]
pub type IR = crate::Reg<ir::IR_SPEC>;
#[doc = "Interrupt Register"]
pub mod ir;
#[doc = "IE register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ie;
#[doc = "ILS register accessor: an alias for `Reg<ILS_SPEC>`"]
pub type ILS = crate::Reg<ils::ILS_SPEC>;
#[doc = "Interrupt Line Select Register"]
pub mod ils;
#[doc = "ILE register accessor: an alias for `Reg<ILE_SPEC>`"]
pub type ILE = crate::Reg<ile::ILE_SPEC>;
#[doc = "Interrupt Line Enable Register"]
pub mod ile;
#[doc = "GFC register accessor: an alias for `Reg<GFC_SPEC>`"]
pub type GFC = crate::Reg<gfc::GFC_SPEC>;
#[doc = "Global Filter Configuration Register"]
pub mod gfc;
#[doc = "SIDFC register accessor: an alias for `Reg<SIDFC_SPEC>`"]
pub type SIDFC = crate::Reg<sidfc::SIDFC_SPEC>;
#[doc = "Standard ID Filter Configuration Register"]
pub mod sidfc;
#[doc = "XIDFC register accessor: an alias for `Reg<XIDFC_SPEC>`"]
pub type XIDFC = crate::Reg<xidfc::XIDFC_SPEC>;
#[doc = "Extended ID Filter Configuration Register"]
pub mod xidfc;
#[doc = "XIDAM register accessor: an alias for `Reg<XIDAM_SPEC>`"]
pub type XIDAM = crate::Reg<xidam::XIDAM_SPEC>;
#[doc = "Extended ID AND Mask Register"]
pub mod xidam;
#[doc = "HPMS register accessor: an alias for `Reg<HPMS_SPEC>`"]
pub type HPMS = crate::Reg<hpms::HPMS_SPEC>;
#[doc = "High Priority Message Status Register"]
pub mod hpms;
#[doc = "NDAT1 register accessor: an alias for `Reg<NDAT1_SPEC>`"]
pub type NDAT1 = crate::Reg<ndat1::NDAT1_SPEC>;
#[doc = "New Data 1 Register"]
pub mod ndat1;
#[doc = "NDAT2 register accessor: an alias for `Reg<NDAT2_SPEC>`"]
pub type NDAT2 = crate::Reg<ndat2::NDAT2_SPEC>;
#[doc = "New Data 2 Register"]
pub mod ndat2;
#[doc = "RXF0C register accessor: an alias for `Reg<RXF0C_SPEC>`"]
pub type RXF0C = crate::Reg<rxf0c::RXF0C_SPEC>;
#[doc = "Receive FIFO 0 Configuration Register"]
pub mod rxf0c;
#[doc = "RXF0S register accessor: an alias for `Reg<RXF0S_SPEC>`"]
pub type RXF0S = crate::Reg<rxf0s::RXF0S_SPEC>;
#[doc = "Receive FIFO 0 Status Register"]
pub mod rxf0s;
#[doc = "RXF0A register accessor: an alias for `Reg<RXF0A_SPEC>`"]
pub type RXF0A = crate::Reg<rxf0a::RXF0A_SPEC>;
#[doc = "Receive FIFO 0 Acknowledge Register"]
pub mod rxf0a;
#[doc = "RXBC register accessor: an alias for `Reg<RXBC_SPEC>`"]
pub type RXBC = crate::Reg<rxbc::RXBC_SPEC>;
#[doc = "Receive Rx Buffer Configuration Register"]
pub mod rxbc;
#[doc = "RXF1C register accessor: an alias for `Reg<RXF1C_SPEC>`"]
pub type RXF1C = crate::Reg<rxf1c::RXF1C_SPEC>;
#[doc = "Receive FIFO 1 Configuration Register"]
pub mod rxf1c;
#[doc = "RXF1S register accessor: an alias for `Reg<RXF1S_SPEC>`"]
pub type RXF1S = crate::Reg<rxf1s::RXF1S_SPEC>;
#[doc = "Receive FIFO 1 Status Register"]
pub mod rxf1s;
#[doc = "RXF1A register accessor: an alias for `Reg<RXF1A_SPEC>`"]
pub type RXF1A = crate::Reg<rxf1a::RXF1A_SPEC>;
#[doc = "Receive FIFO 1 Acknowledge Register"]
pub mod rxf1a;
#[doc = "RXESC register accessor: an alias for `Reg<RXESC_SPEC>`"]
pub type RXESC = crate::Reg<rxesc::RXESC_SPEC>;
#[doc = "Receive Buffer / FIFO Element Size Configuration Register"]
pub mod rxesc;
#[doc = "TXBC register accessor: an alias for `Reg<TXBC_SPEC>`"]
pub type TXBC = crate::Reg<txbc::TXBC_SPEC>;
#[doc = "Transmit Buffer Configuration Register"]
pub mod txbc;
#[doc = "TXFQS register accessor: an alias for `Reg<TXFQS_SPEC>`"]
pub type TXFQS = crate::Reg<txfqs::TXFQS_SPEC>;
#[doc = "Transmit FIFO/Queue Status Register"]
pub mod txfqs;
#[doc = "TXESC register accessor: an alias for `Reg<TXESC_SPEC>`"]
pub type TXESC = crate::Reg<txesc::TXESC_SPEC>;
#[doc = "Transmit Buffer Element Size Configuration Register"]
pub mod txesc;
#[doc = "TXBRP register accessor: an alias for `Reg<TXBRP_SPEC>`"]
pub type TXBRP = crate::Reg<txbrp::TXBRP_SPEC>;
#[doc = "Transmit Buffer Request Pending Register"]
pub mod txbrp;
#[doc = "TXBAR register accessor: an alias for `Reg<TXBAR_SPEC>`"]
pub type TXBAR = crate::Reg<txbar::TXBAR_SPEC>;
#[doc = "Transmit Buffer Add Request Register"]
pub mod txbar;
#[doc = "TXBCR register accessor: an alias for `Reg<TXBCR_SPEC>`"]
pub type TXBCR = crate::Reg<txbcr::TXBCR_SPEC>;
#[doc = "Transmit Buffer Cancellation Request Register"]
pub mod txbcr;
#[doc = "TXBTO register accessor: an alias for `Reg<TXBTO_SPEC>`"]
pub type TXBTO = crate::Reg<txbto::TXBTO_SPEC>;
#[doc = "Transmit Buffer Transmission Occurred Register"]
pub mod txbto;
#[doc = "TXBCF register accessor: an alias for `Reg<TXBCF_SPEC>`"]
pub type TXBCF = crate::Reg<txbcf::TXBCF_SPEC>;
#[doc = "Transmit Buffer Cancellation Finished Register"]
pub mod txbcf;
#[doc = "TXBTIE register accessor: an alias for `Reg<TXBTIE_SPEC>`"]
pub type TXBTIE = crate::Reg<txbtie::TXBTIE_SPEC>;
#[doc = "Transmit Buffer Transmission Interrupt Enable Register"]
pub mod txbtie;
#[doc = "TXBCIE register accessor: an alias for `Reg<TXBCIE_SPEC>`"]
pub type TXBCIE = crate::Reg<txbcie::TXBCIE_SPEC>;
#[doc = "Transmit Buffer Cancellation Finished Interrupt Enable Register"]
pub mod txbcie;
#[doc = "TXEFC register accessor: an alias for `Reg<TXEFC_SPEC>`"]
pub type TXEFC = crate::Reg<txefc::TXEFC_SPEC>;
#[doc = "Transmit Event FIFO Configuration Register"]
pub mod txefc;
#[doc = "TXEFS register accessor: an alias for `Reg<TXEFS_SPEC>`"]
pub type TXEFS = crate::Reg<txefs::TXEFS_SPEC>;
#[doc = "Transmit Event FIFO Status Register"]
pub mod txefs;
#[doc = "TXEFA register accessor: an alias for `Reg<TXEFA_SPEC>`"]
pub type TXEFA = crate::Reg<txefa::TXEFA_SPEC>;
#[doc = "Transmit Event FIFO Acknowledge Register"]
pub mod txefa;
