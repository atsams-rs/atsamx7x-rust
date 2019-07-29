#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - OHCI Interrupt Configuration Register"]
    pub utmi_ohciicr: UTMI_OHCIICR,
    _reserved1: [u8; 28usize],
    #[doc = "0x30 - UTMI Clock Trimming Register"]
    pub utmi_cktrim: UTMI_CKTRIM,
}
#[doc = "OHCI Interrupt Configuration Register"]
pub struct UTMI_OHCIICR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "OHCI Interrupt Configuration Register"]
pub mod utmi_ohciicr;
#[doc = "UTMI Clock Trimming Register"]
pub struct UTMI_CKTRIM {
    register: vcell::VolatileCell<u32>,
}
#[doc = "UTMI Clock Trimming Register"]
pub mod utmi_cktrim;
