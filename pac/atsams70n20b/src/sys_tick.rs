#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control and Status Register"]
    pub csr: CSR,
    #[doc = "0x04 - Reload Value Register"]
    pub rvr: RVR,
    #[doc = "0x08 - Current Value Register"]
    pub cvr: CVR,
    #[doc = "0x0c - Calibration Value Register"]
    pub calib: CALIB,
}
#[doc = "Control and Status Register"]
pub struct CSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Control and Status Register"]
pub mod csr;
#[doc = "Reload Value Register"]
pub struct RVR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Reload Value Register"]
pub mod rvr;
#[doc = "Current Value Register"]
pub struct CVR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Current Value Register"]
pub mod cvr;
#[doc = "Calibration Value Register"]
pub struct CALIB {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Calibration Value Register"]
pub mod calib;
