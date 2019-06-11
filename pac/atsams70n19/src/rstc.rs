#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub rstc_cr: RSTC_CR,
    #[doc = "0x04 - Status Register"]
    pub rstc_sr: RSTC_SR,
    #[doc = "0x08 - Mode Register"]
    pub rstc_mr: RSTC_MR,
}
#[doc = "Control Register"]
pub struct RSTC_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod rstc_cr;
#[doc = "Status Register"]
pub struct RSTC_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod rstc_sr;
#[doc = "Mode Register"]
pub struct RSTC_MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod rstc_mr;
