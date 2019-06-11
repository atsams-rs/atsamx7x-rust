#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDRAMC Mode Register"]
    pub sdramc_mr: SDRAMC_MR,
    #[doc = "0x04 - SDRAMC Refresh Timer Register"]
    pub sdramc_tr: SDRAMC_TR,
    #[doc = "0x08 - SDRAMC Configuration Register"]
    pub sdramc_cr: SDRAMC_CR,
    _reserved0: [u8; 4usize],
    #[doc = "0x10 - SDRAMC Low Power Register"]
    pub sdramc_lpr: SDRAMC_LPR,
    #[doc = "0x14 - SDRAMC Interrupt Enable Register"]
    pub sdramc_ier: SDRAMC_IER,
    #[doc = "0x18 - SDRAMC Interrupt Disable Register"]
    pub sdramc_idr: SDRAMC_IDR,
    #[doc = "0x1c - SDRAMC Interrupt Mask Register"]
    pub sdramc_imr: SDRAMC_IMR,
    #[doc = "0x20 - SDRAMC Interrupt Status Register"]
    pub sdramc_isr: SDRAMC_ISR,
    #[doc = "0x24 - SDRAMC Memory Device Register"]
    pub sdramc_mdr: SDRAMC_MDR,
    #[doc = "0x28 - SDRAMC Configuration Register 1"]
    pub sdramc_cfr1: SDRAMC_CFR1,
    #[doc = "0x2c - SDRAMC OCMS Register"]
    pub sdramc_ocms: SDRAMC_OCMS,
    #[doc = "0x30 - SDRAMC OCMS KEY1 Register"]
    pub sdramc_ocms_key1: SDRAMC_OCMS_KEY1,
    #[doc = "0x34 - SDRAMC OCMS KEY2 Register"]
    pub sdramc_ocms_key2: SDRAMC_OCMS_KEY2,
}
#[doc = "SDRAMC Mode Register"]
pub struct SDRAMC_MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAMC Mode Register"]
pub mod sdramc_mr;
#[doc = "SDRAMC Refresh Timer Register"]
pub struct SDRAMC_TR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAMC Refresh Timer Register"]
pub mod sdramc_tr;
#[doc = "SDRAMC Configuration Register"]
pub struct SDRAMC_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAMC Configuration Register"]
pub mod sdramc_cr;
#[doc = "SDRAMC Low Power Register"]
pub struct SDRAMC_LPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAMC Low Power Register"]
pub mod sdramc_lpr;
#[doc = "SDRAMC Interrupt Enable Register"]
pub struct SDRAMC_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAMC Interrupt Enable Register"]
pub mod sdramc_ier;
#[doc = "SDRAMC Interrupt Disable Register"]
pub struct SDRAMC_IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAMC Interrupt Disable Register"]
pub mod sdramc_idr;
#[doc = "SDRAMC Interrupt Mask Register"]
pub struct SDRAMC_IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAMC Interrupt Mask Register"]
pub mod sdramc_imr;
#[doc = "SDRAMC Interrupt Status Register"]
pub struct SDRAMC_ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAMC Interrupt Status Register"]
pub mod sdramc_isr;
#[doc = "SDRAMC Memory Device Register"]
pub struct SDRAMC_MDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAMC Memory Device Register"]
pub mod sdramc_mdr;
#[doc = "SDRAMC Configuration Register 1"]
pub struct SDRAMC_CFR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAMC Configuration Register 1"]
pub mod sdramc_cfr1;
#[doc = "SDRAMC OCMS Register"]
pub struct SDRAMC_OCMS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAMC OCMS Register"]
pub mod sdramc_ocms;
#[doc = "SDRAMC OCMS KEY1 Register"]
pub struct SDRAMC_OCMS_KEY1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAMC OCMS KEY1 Register"]
pub mod sdramc_ocms_key1;
#[doc = "SDRAMC OCMS KEY2 Register"]
pub struct SDRAMC_OCMS_KEY2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SDRAMC OCMS KEY2 Register"]
pub mod sdramc_ocms_key2;
