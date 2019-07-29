#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub uart_cr: UART_CR,
    #[doc = "0x04 - Mode Register"]
    pub uart_mr: UART_MR,
    #[doc = "0x08 - Interrupt Enable Register"]
    pub uart_ier: UART_IER,
    #[doc = "0x0c - Interrupt Disable Register"]
    pub uart_idr: UART_IDR,
    #[doc = "0x10 - Interrupt Mask Register"]
    pub uart_imr: UART_IMR,
    #[doc = "0x14 - Status Register"]
    pub uart_sr: UART_SR,
    #[doc = "0x18 - Receive Holding Register"]
    pub uart_rhr: UART_RHR,
    #[doc = "0x1c - Transmit Holding Register"]
    pub uart_thr: UART_THR,
    #[doc = "0x20 - Baud Rate Generator Register"]
    pub uart_brgr: UART_BRGR,
    #[doc = "0x24 - Comparison Register"]
    pub uart_cmpr: UART_CMPR,
    _reserved10: [u8; 188usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub uart_wpmr: UART_WPMR,
}
#[doc = "Control Register"]
pub struct UART_CR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod uart_cr;
#[doc = "Mode Register"]
pub struct UART_MR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod uart_mr;
#[doc = "Interrupt Enable Register"]
pub struct UART_IER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod uart_ier;
#[doc = "Interrupt Disable Register"]
pub struct UART_IDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod uart_idr;
#[doc = "Interrupt Mask Register"]
pub struct UART_IMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod uart_imr;
#[doc = "Status Register"]
pub struct UART_SR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod uart_sr;
#[doc = "Receive Holding Register"]
pub struct UART_RHR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Receive Holding Register"]
pub mod uart_rhr;
#[doc = "Transmit Holding Register"]
pub struct UART_THR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Transmit Holding Register"]
pub mod uart_thr;
#[doc = "Baud Rate Generator Register"]
pub struct UART_BRGR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Baud Rate Generator Register"]
pub mod uart_brgr;
#[doc = "Comparison Register"]
pub struct UART_CMPR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Comparison Register"]
pub mod uart_cmpr;
#[doc = "Write Protection Mode Register"]
pub struct UART_WPMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Mode Register"]
pub mod uart_wpmr;
