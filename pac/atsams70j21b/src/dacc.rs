#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub dacc_cr: DACC_CR,
    #[doc = "0x04 - Mode Register"]
    pub dacc_mr: DACC_MR,
    #[doc = "0x08 - Trigger Register"]
    pub dacc_trigr: DACC_TRIGR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Channel Enable Register"]
    pub dacc_cher: DACC_CHER,
    #[doc = "0x14 - Channel Disable Register"]
    pub dacc_chdr: DACC_CHDR,
    #[doc = "0x18 - Channel Status Register"]
    pub dacc_chsr: DACC_CHSR,
    #[doc = "0x1c - Conversion Data Register 0"]
    pub dacc_cdr: [DACC_CDR; 2],
    #[doc = "0x24 - Interrupt Enable Register"]
    pub dacc_ier: DACC_IER,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub dacc_idr: DACC_IDR,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub dacc_imr: DACC_IMR,
    #[doc = "0x30 - Interrupt Status Register"]
    pub dacc_isr: DACC_ISR,
    _reserved11: [u8; 96usize],
    #[doc = "0x94 - Analog Current Register"]
    pub dacc_acr: DACC_ACR,
    _reserved12: [u8; 76usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub dacc_wpmr: DACC_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub dacc_wpsr: DACC_WPSR,
}
#[doc = "Control Register"]
pub struct DACC_CR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod dacc_cr;
#[doc = "Mode Register"]
pub struct DACC_MR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod dacc_mr;
#[doc = "Trigger Register"]
pub struct DACC_TRIGR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Trigger Register"]
pub mod dacc_trigr;
#[doc = "Channel Enable Register"]
pub struct DACC_CHER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Channel Enable Register"]
pub mod dacc_cher;
#[doc = "Channel Disable Register"]
pub struct DACC_CHDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Channel Disable Register"]
pub mod dacc_chdr;
#[doc = "Channel Status Register"]
pub struct DACC_CHSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Channel Status Register"]
pub mod dacc_chsr;
#[doc = "Conversion Data Register 0"]
pub struct DACC_CDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Conversion Data Register 0"]
pub mod dacc_cdr;
#[doc = "Interrupt Enable Register"]
pub struct DACC_IER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod dacc_ier;
#[doc = "Interrupt Disable Register"]
pub struct DACC_IDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod dacc_idr;
#[doc = "Interrupt Mask Register"]
pub struct DACC_IMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod dacc_imr;
#[doc = "Interrupt Status Register"]
pub struct DACC_ISR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod dacc_isr;
#[doc = "Analog Current Register"]
pub struct DACC_ACR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Analog Current Register"]
pub mod dacc_acr;
#[doc = "Write Protection Mode Register"]
pub struct DACC_WPMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Mode Register"]
pub mod dacc_wpmr;
#[doc = "Write Protection Status Register"]
pub struct DACC_WPSR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Status Register"]
pub mod dacc_wpsr;
