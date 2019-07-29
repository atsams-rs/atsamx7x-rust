#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub rswdt_cr: RSWDT_CR,
    #[doc = "0x04 - Mode Register"]
    pub rswdt_mr: RSWDT_MR,
    #[doc = "0x08 - Status Register"]
    pub rswdt_sr: RSWDT_SR,
}
#[doc = "Control Register"]
pub struct RSWDT_CR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod rswdt_cr;
#[doc = "Mode Register"]
pub struct RSWDT_MR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod rswdt_mr;
#[doc = "Status Register"]
pub struct RSWDT_SR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod rswdt_sr;
