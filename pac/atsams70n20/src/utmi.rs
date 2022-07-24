#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - OHCI Interrupt Configuration Register"]
    pub ohciicr: crate::Reg<ohciicr::OHCIICR_SPEC>,
    _reserved1: [u8; 0x1c],
    #[doc = "0x30 - UTMI Clock Trimming Register"]
    pub cktrim: crate::Reg<cktrim::CKTRIM_SPEC>,
}
#[doc = "OHCIICR register accessor: an alias for `Reg<OHCIICR_SPEC>`"]
pub type OHCIICR = crate::Reg<ohciicr::OHCIICR_SPEC>;
#[doc = "OHCI Interrupt Configuration Register"]
pub mod ohciicr;
#[doc = "CKTRIM register accessor: an alias for `Reg<CKTRIM_SPEC>`"]
pub type CKTRIM = crate::Reg<cktrim::CKTRIM_SPEC>;
#[doc = "UTMI Clock Trimming Register"]
pub mod cktrim;
