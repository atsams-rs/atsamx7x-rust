#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub qspi_cr: QSPI_CR,
    #[doc = "0x04 - Mode Register"]
    pub qspi_mr: QSPI_MR,
    #[doc = "0x08 - Receive Data Register"]
    pub qspi_rdr: QSPI_RDR,
    #[doc = "0x0c - Transmit Data Register"]
    pub qspi_tdr: QSPI_TDR,
    #[doc = "0x10 - Status Register"]
    pub qspi_sr: QSPI_SR,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub qspi_ier: QSPI_IER,
    #[doc = "0x18 - Interrupt Disable Register"]
    pub qspi_idr: QSPI_IDR,
    #[doc = "0x1c - Interrupt Mask Register"]
    pub qspi_imr: QSPI_IMR,
    #[doc = "0x20 - Serial Clock Register"]
    pub qspi_scr: QSPI_SCR,
    _reserved0: [u8; 12usize],
    #[doc = "0x30 - Instruction Address Register"]
    pub qspi_iar: QSPI_IAR,
    #[doc = "0x34 - Instruction Code Register"]
    pub qspi_icr: QSPI_ICR,
    #[doc = "0x38 - Instruction Frame Register"]
    pub qspi_ifr: QSPI_IFR,
    _reserved1: [u8; 4usize],
    #[doc = "0x40 - Scrambling Mode Register"]
    pub qspi_smr: QSPI_SMR,
    #[doc = "0x44 - Scrambling Key Register"]
    pub qspi_skr: QSPI_SKR,
    _reserved2: [u8; 156usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub qspi_wpmr: QSPI_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub qspi_wpsr: QSPI_WPSR,
}
#[doc = "Control Register"]
pub struct QSPI_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod qspi_cr;
#[doc = "Mode Register"]
pub struct QSPI_MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod qspi_mr;
#[doc = "Receive Data Register"]
pub struct QSPI_RDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Data Register"]
pub mod qspi_rdr;
#[doc = "Transmit Data Register"]
pub struct QSPI_TDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Data Register"]
pub mod qspi_tdr;
#[doc = "Status Register"]
pub struct QSPI_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod qspi_sr;
#[doc = "Interrupt Enable Register"]
pub struct QSPI_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod qspi_ier;
#[doc = "Interrupt Disable Register"]
pub struct QSPI_IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod qspi_idr;
#[doc = "Interrupt Mask Register"]
pub struct QSPI_IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod qspi_imr;
#[doc = "Serial Clock Register"]
pub struct QSPI_SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Serial Clock Register"]
pub mod qspi_scr;
#[doc = "Instruction Address Register"]
pub struct QSPI_IAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction Address Register"]
pub mod qspi_iar;
#[doc = "Instruction Code Register"]
pub struct QSPI_ICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction Code Register"]
pub mod qspi_icr;
#[doc = "Instruction Frame Register"]
pub struct QSPI_IFR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Instruction Frame Register"]
pub mod qspi_ifr;
#[doc = "Scrambling Mode Register"]
pub struct QSPI_SMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scrambling Mode Register"]
pub mod qspi_smr;
#[doc = "Scrambling Key Register"]
pub struct QSPI_SKR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Scrambling Key Register"]
pub mod qspi_skr;
#[doc = "Write Protection Mode Register"]
pub struct QSPI_WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Mode Register"]
pub mod qspi_wpmr;
#[doc = "Write Protection Status Register"]
pub struct QSPI_WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Status Register"]
pub mod qspi_wpsr;
