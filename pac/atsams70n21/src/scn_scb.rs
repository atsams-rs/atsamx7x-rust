#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Interrupt Controller Type Register"]
    pub ictr: ICTR,
    #[doc = "0x08 - Auxiliary Control Register"]
    pub actlr: ACTLR,
}
#[doc = "Interrupt Controller Type Register"]
pub struct ICTR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Controller Type Register"]
pub mod ictr;
#[doc = "Auxiliary Control Register"]
pub struct ACTLR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Auxiliary Control Register"]
pub mod actlr;
