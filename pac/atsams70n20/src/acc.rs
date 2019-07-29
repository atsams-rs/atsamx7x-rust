#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub acc_cr: ACC_CR,
    #[doc = "0x04 - Mode Register"]
    pub acc_mr: ACC_MR,
    _reserved2: [u8; 28usize],
    #[doc = "0x24 - Interrupt Enable Register"]
    pub acc_ier: ACC_IER,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub acc_idr: ACC_IDR,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub acc_imr: ACC_IMR,
    #[doc = "0x30 - Interrupt Status Register"]
    pub acc_isr: ACC_ISR,
    _reserved6: [u8; 96usize],
    #[doc = "0x94 - Analog Control Register"]
    pub acc_acr: ACC_ACR,
    _reserved7: [u8; 76usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub acc_wpmr: ACC_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub acc_wpsr: ACC_WPSR,
}
#[doc = "Control Register"]
pub struct ACC_CR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod acc_cr;
#[doc = "Mode Register"]
pub struct ACC_MR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod acc_mr;
#[doc = "Interrupt Enable Register"]
pub struct ACC_IER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod acc_ier;
#[doc = "Interrupt Disable Register"]
pub struct ACC_IDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod acc_idr;
#[doc = "Interrupt Mask Register"]
pub struct ACC_IMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod acc_imr;
#[doc = "Interrupt Status Register"]
pub struct ACC_ISR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod acc_isr;
#[doc = "Analog Control Register"]
pub struct ACC_ACR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Analog Control Register"]
pub mod acc_acr;
#[doc = "Write Protection Mode Register"]
pub struct ACC_WPMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Mode Register"]
pub mod acc_wpmr;
#[doc = "Write Protection Status Register"]
pub struct ACC_WPSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Status Register"]
pub mod acc_wpsr;
