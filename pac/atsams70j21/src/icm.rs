#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration Register"]
    pub icm_cfg: ICM_CFG,
    #[doc = "0x04 - Control Register"]
    pub icm_ctrl: ICM_CTRL,
    #[doc = "0x08 - Status Register"]
    pub icm_sr: ICM_SR,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - Interrupt Enable Register"]
    pub icm_ier: ICM_IER,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub icm_idr: ICM_IDR,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub icm_imr: ICM_IMR,
    #[doc = "0x1c - Interrupt Status Register"]
    pub icm_isr: ICM_ISR,
    #[doc = "0x20 - Undefined Access Status Register"]
    pub icm_uasr: ICM_UASR,
    _reserved1: [u8; 12usize],
    #[doc = "0x30 - Region Descriptor Area Start Address Register"]
    pub icm_dscr: ICM_DSCR,
    #[doc = "0x34 - Region Hash Area Start Address Register"]
    pub icm_hash: ICM_HASH,
    #[doc = "0x38 - User Initial Hash Value 0 Register 0"]
    pub icm_uihval: [ICM_UIHVAL; 8],
}
#[doc = "Configuration Register"]
pub struct ICM_CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Register"]
pub mod icm_cfg;
#[doc = "Control Register"]
pub struct ICM_CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod icm_ctrl;
#[doc = "Status Register"]
pub struct ICM_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod icm_sr;
#[doc = "Interrupt Enable Register"]
pub struct ICM_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod icm_ier;
#[doc = "Interrupt Disable Register"]
pub struct ICM_IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod icm_idr;
#[doc = "Interrupt Mask Register"]
pub struct ICM_IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod icm_imr;
#[doc = "Interrupt Status Register"]
pub struct ICM_ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod icm_isr;
#[doc = "Undefined Access Status Register"]
pub struct ICM_UASR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Undefined Access Status Register"]
pub mod icm_uasr;
#[doc = "Region Descriptor Area Start Address Register"]
pub struct ICM_DSCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Region Descriptor Area Start Address Register"]
pub mod icm_dscr;
#[doc = "Region Hash Area Start Address Register"]
pub struct ICM_HASH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Region Hash Area Start Address Register"]
pub mod icm_hash;
#[doc = "User Initial Hash Value 0 Register 0"]
pub struct ICM_UIHVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "User Initial Hash Value 0 Register 0"]
pub mod icm_uihval;
