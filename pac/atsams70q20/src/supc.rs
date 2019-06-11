#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Supply Controller Control Register"]
    pub supc_cr: SUPC_CR,
    #[doc = "0x04 - Supply Controller Supply Monitor Mode Register"]
    pub supc_smmr: SUPC_SMMR,
    #[doc = "0x08 - Supply Controller Mode Register"]
    pub supc_mr: SUPC_MR,
    #[doc = "0x0c - Supply Controller Wake-up Mode Register"]
    pub supc_wumr: SUPC_WUMR,
    #[doc = "0x10 - Supply Controller Wake-up Inputs Register"]
    pub supc_wuir: SUPC_WUIR,
    #[doc = "0x14 - Supply Controller Status Register"]
    pub supc_sr: SUPC_SR,
}
#[doc = "Supply Controller Control Register"]
pub struct SUPC_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Supply Controller Control Register"]
pub mod supc_cr;
#[doc = "Supply Controller Supply Monitor Mode Register"]
pub struct SUPC_SMMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Supply Controller Supply Monitor Mode Register"]
pub mod supc_smmr;
#[doc = "Supply Controller Mode Register"]
pub struct SUPC_MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Supply Controller Mode Register"]
pub mod supc_mr;
#[doc = "Supply Controller Wake-up Mode Register"]
pub struct SUPC_WUMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Supply Controller Wake-up Mode Register"]
pub mod supc_wumr;
#[doc = "Supply Controller Wake-up Inputs Register"]
pub struct SUPC_WUIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Supply Controller Wake-up Inputs Register"]
pub mod supc_wuir;
#[doc = "Supply Controller Status Register"]
pub struct SUPC_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Supply Controller Status Register"]
pub mod supc_sr;
