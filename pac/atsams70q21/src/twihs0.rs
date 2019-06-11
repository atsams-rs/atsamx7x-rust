#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub twihs_cr: TWIHS_CR,
    #[doc = "0x04 - Master Mode Register"]
    pub twihs_mmr: TWIHS_MMR,
    #[doc = "0x08 - Slave Mode Register"]
    pub twihs_smr: TWIHS_SMR,
    #[doc = "0x0c - Internal Address Register"]
    pub twihs_iadr: TWIHS_IADR,
    #[doc = "0x10 - Clock Waveform Generator Register"]
    pub twihs_cwgr: TWIHS_CWGR,
    _reserved0: [u8; 12usize],
    #[doc = "0x20 - Status Register"]
    pub twihs_sr: TWIHS_SR,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub twihs_ier: TWIHS_IER,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub twihs_idr: TWIHS_IDR,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub twihs_imr: TWIHS_IMR,
    #[doc = "0x30 - Receive Holding Register"]
    pub twihs_rhr: TWIHS_RHR,
    #[doc = "0x34 - Transmit Holding Register"]
    pub twihs_thr: TWIHS_THR,
    #[doc = "0x38 - SMBus Timing Register"]
    pub twihs_smbtr: TWIHS_SMBTR,
    _reserved1: [u8; 8usize],
    #[doc = "0x44 - Filter Register"]
    pub twihs_filtr: TWIHS_FILTR,
    _reserved2: [u8; 4usize],
    #[doc = "0x4c - SleepWalking Matching Register"]
    pub twihs_swmr: TWIHS_SWMR,
    _reserved3: [u8; 148usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub twihs_wpmr: TWIHS_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub twihs_wpsr: TWIHS_WPSR,
}
#[doc = "Control Register"]
pub struct TWIHS_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod twihs_cr;
#[doc = "Master Mode Register"]
pub struct TWIHS_MMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Mode Register"]
pub mod twihs_mmr;
#[doc = "Slave Mode Register"]
pub struct TWIHS_SMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Mode Register"]
pub mod twihs_smr;
#[doc = "Internal Address Register"]
pub struct TWIHS_IADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal Address Register"]
pub mod twihs_iadr;
#[doc = "Clock Waveform Generator Register"]
pub struct TWIHS_CWGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Waveform Generator Register"]
pub mod twihs_cwgr;
#[doc = "Status Register"]
pub struct TWIHS_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod twihs_sr;
#[doc = "Interrupt Enable Register"]
pub struct TWIHS_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod twihs_ier;
#[doc = "Interrupt Disable Register"]
pub struct TWIHS_IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod twihs_idr;
#[doc = "Interrupt Mask Register"]
pub struct TWIHS_IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod twihs_imr;
#[doc = "Receive Holding Register"]
pub struct TWIHS_RHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Holding Register"]
pub mod twihs_rhr;
#[doc = "Transmit Holding Register"]
pub struct TWIHS_THR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Holding Register"]
pub mod twihs_thr;
#[doc = "SMBus Timing Register"]
pub struct TWIHS_SMBTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMBus Timing Register"]
pub mod twihs_smbtr;
#[doc = "Filter Register"]
pub struct TWIHS_FILTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Filter Register"]
pub mod twihs_filtr;
#[doc = "SleepWalking Matching Register"]
pub struct TWIHS_SWMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SleepWalking Matching Register"]
pub mod twihs_swmr;
#[doc = "Write Protection Mode Register"]
pub struct TWIHS_WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Mode Register"]
pub mod twihs_wpmr;
#[doc = "Write Protection Status Register"]
pub struct TWIHS_WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Status Register"]
pub mod twihs_wpsr;
