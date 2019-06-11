#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EEFC Flash Mode Register"]
    pub eefc_fmr: EEFC_FMR,
    #[doc = "0x04 - EEFC Flash Command Register"]
    pub eefc_fcr: EEFC_FCR,
    #[doc = "0x08 - EEFC Flash Status Register"]
    pub eefc_fsr: EEFC_FSR,
    #[doc = "0x0c - EEFC Flash Result Register"]
    pub eefc_frr: EEFC_FRR,
    _reserved0: [u8; 212usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub eefc_wpmr: EEFC_WPMR,
}
#[doc = "EEFC Flash Mode Register"]
pub struct EEFC_FMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEFC Flash Mode Register"]
pub mod eefc_fmr;
#[doc = "EEFC Flash Command Register"]
pub struct EEFC_FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEFC Flash Command Register"]
pub mod eefc_fcr;
#[doc = "EEFC Flash Status Register"]
pub struct EEFC_FSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEFC Flash Status Register"]
pub mod eefc_fsr;
#[doc = "EEFC Flash Result Register"]
pub struct EEFC_FRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEFC Flash Result Register"]
pub mod eefc_frr;
#[doc = "Write Protection Mode Register"]
pub struct EEFC_WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Mode Register"]
pub mod eefc_wpmr;
