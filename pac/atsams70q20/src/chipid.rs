#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip ID Register"]
    pub chipid_cidr: CHIPID_CIDR,
    #[doc = "0x04 - Chip ID Extension Register"]
    pub chipid_exid: CHIPID_EXID,
}
#[doc = "Chip ID Register"]
pub struct CHIPID_CIDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Chip ID Register"]
pub mod chipid_cidr;
#[doc = "Chip ID Extension Register"]
pub struct CHIPID_EXID {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Chip ID Extension Register"]
pub mod chipid_exid;
