#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub wdt_cr: WDT_CR,
    #[doc = "0x04 - Mode Register"]
    pub wdt_mr: WDT_MR,
    #[doc = "0x08 - Status Register"]
    pub wdt_sr: WDT_SR,
}
#[doc = "Control Register"]
pub struct WDT_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod wdt_cr;
#[doc = "Mode Register"]
pub struct WDT_MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod wdt_mr;
#[doc = "Status Register"]
pub struct WDT_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod wdt_sr;
