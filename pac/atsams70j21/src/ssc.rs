#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ssc_cr: SSC_CR,
    #[doc = "0x04 - Clock Mode Register"]
    pub ssc_cmr: SSC_CMR,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Receive Clock Mode Register"]
    pub ssc_rcmr: SSC_RCMR,
    #[doc = "0x14 - Receive Frame Mode Register"]
    pub ssc_rfmr: SSC_RFMR,
    #[doc = "0x18 - Transmit Clock Mode Register"]
    pub ssc_tcmr: SSC_TCMR,
    #[doc = "0x1c - Transmit Frame Mode Register"]
    pub ssc_tfmr: SSC_TFMR,
    #[doc = "0x20 - Receive Holding Register"]
    pub ssc_rhr: SSC_RHR,
    #[doc = "0x24 - Transmit Holding Register"]
    pub ssc_thr: SSC_THR,
    _reserved1: [u8; 8usize],
    #[doc = "0x30 - Receive Sync. Holding Register"]
    pub ssc_rshr: SSC_RSHR,
    #[doc = "0x34 - Transmit Sync. Holding Register"]
    pub ssc_tshr: SSC_TSHR,
    #[doc = "0x38 - Receive Compare 0 Register"]
    pub ssc_rc0r: SSC_RC0R,
    #[doc = "0x3c - Receive Compare 1 Register"]
    pub ssc_rc1r: SSC_RC1R,
    #[doc = "0x40 - Status Register"]
    pub ssc_sr: SSC_SR,
    #[doc = "0x44 - Interrupt Enable Register"]
    pub ssc_ier: SSC_IER,
    #[doc = "0x48 - Interrupt Disable Register"]
    pub ssc_idr: SSC_IDR,
    #[doc = "0x4c - Interrupt Mask Register"]
    pub ssc_imr: SSC_IMR,
    _reserved2: [u8; 148usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub ssc_wpmr: SSC_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub ssc_wpsr: SSC_WPSR,
}
#[doc = "Control Register"]
pub struct SSC_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod ssc_cr;
#[doc = "Clock Mode Register"]
pub struct SSC_CMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Mode Register"]
pub mod ssc_cmr;
#[doc = "Receive Clock Mode Register"]
pub struct SSC_RCMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Clock Mode Register"]
pub mod ssc_rcmr;
#[doc = "Receive Frame Mode Register"]
pub struct SSC_RFMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Mode Register"]
pub mod ssc_rfmr;
#[doc = "Transmit Clock Mode Register"]
pub struct SSC_TCMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Clock Mode Register"]
pub mod ssc_tcmr;
#[doc = "Transmit Frame Mode Register"]
pub struct SSC_TFMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Frame Mode Register"]
pub mod ssc_tfmr;
#[doc = "Receive Holding Register"]
pub struct SSC_RHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Holding Register"]
pub mod ssc_rhr;
#[doc = "Transmit Holding Register"]
pub struct SSC_THR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Holding Register"]
pub mod ssc_thr;
#[doc = "Receive Sync. Holding Register"]
pub struct SSC_RSHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Sync. Holding Register"]
pub mod ssc_rshr;
#[doc = "Transmit Sync. Holding Register"]
pub struct SSC_TSHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Sync. Holding Register"]
pub mod ssc_tshr;
#[doc = "Receive Compare 0 Register"]
pub struct SSC_RC0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Compare 0 Register"]
pub mod ssc_rc0r;
#[doc = "Receive Compare 1 Register"]
pub struct SSC_RC1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Compare 1 Register"]
pub mod ssc_rc1r;
#[doc = "Status Register"]
pub struct SSC_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod ssc_sr;
#[doc = "Interrupt Enable Register"]
pub struct SSC_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ssc_ier;
#[doc = "Interrupt Disable Register"]
pub struct SSC_IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod ssc_idr;
#[doc = "Interrupt Mask Register"]
pub struct SSC_IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod ssc_imr;
#[doc = "Write Protection Mode Register"]
pub struct SSC_WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Mode Register"]
pub mod ssc_wpmr;
#[doc = "Write Protection Status Register"]
pub struct SSC_WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Status Register"]
pub mod ssc_wpsr;
