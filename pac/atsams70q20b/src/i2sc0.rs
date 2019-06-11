#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub i2sc_cr: I2SC_CR,
    #[doc = "0x04 - Mode Register"]
    pub i2sc_mr: I2SC_MR,
    #[doc = "0x08 - Status Register"]
    pub i2sc_sr: I2SC_SR,
    #[doc = "0x0c - Status Clear Register"]
    pub i2sc_scr: I2SC_SCR,
    #[doc = "0x10 - Status Set Register"]
    pub i2sc_ssr: I2SC_SSR,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub i2sc_ier: I2SC_IER,
    #[doc = "0x18 - Interrupt Disable Register"]
    pub i2sc_idr: I2SC_IDR,
    #[doc = "0x1c - Interrupt Mask Register"]
    pub i2sc_imr: I2SC_IMR,
    #[doc = "0x20 - Receiver Holding Register"]
    pub i2sc_rhr: I2SC_RHR,
    #[doc = "0x24 - Transmitter Holding Register"]
    pub i2sc_thr: I2SC_THR,
}
#[doc = "Control Register"]
pub struct I2SC_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod i2sc_cr;
#[doc = "Mode Register"]
pub struct I2SC_MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod i2sc_mr;
#[doc = "Status Register"]
pub struct I2SC_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod i2sc_sr;
#[doc = "Status Clear Register"]
pub struct I2SC_SCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Clear Register"]
pub mod i2sc_scr;
#[doc = "Status Set Register"]
pub struct I2SC_SSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Set Register"]
pub mod i2sc_ssr;
#[doc = "Interrupt Enable Register"]
pub struct I2SC_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod i2sc_ier;
#[doc = "Interrupt Disable Register"]
pub struct I2SC_IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod i2sc_idr;
#[doc = "Interrupt Mask Register"]
pub struct I2SC_IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod i2sc_imr;
#[doc = "Receiver Holding Register"]
pub struct I2SC_RHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Holding Register"]
pub mod i2sc_rhr;
#[doc = "Transmitter Holding Register"]
pub struct I2SC_THR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmitter Holding Register"]
pub mod i2sc_thr;
