#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub trng_cr: TRNG_CR,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Interrupt Enable Register"]
    pub trng_ier: TRNG_IER,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub trng_idr: TRNG_IDR,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub trng_imr: TRNG_IMR,
    #[doc = "0x1c - Interrupt Status Register"]
    pub trng_isr: TRNG_ISR,
    _reserved5: [u8; 48usize],
    #[doc = "0x50 - Output Data Register"]
    pub trng_odata: TRNG_ODATA,
}
#[doc = "Control Register"]
pub struct TRNG_CR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod trng_cr;
#[doc = "Interrupt Enable Register"]
pub struct TRNG_IER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod trng_ier;
#[doc = "Interrupt Disable Register"]
pub struct TRNG_IDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod trng_idr;
#[doc = "Interrupt Mask Register"]
pub struct TRNG_IMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod trng_imr;
#[doc = "Interrupt Status Register"]
pub struct TRNG_ISR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod trng_isr;
#[doc = "Output Data Register"]
pub struct TRNG_ODATA {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Output Data Register"]
pub mod trng_odata;
