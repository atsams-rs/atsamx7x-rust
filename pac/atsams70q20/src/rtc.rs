#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub rtc_cr: RTC_CR,
    #[doc = "0x04 - Mode Register"]
    pub rtc_mr: RTC_MR,
    #[doc = "0x08 - Time Register"]
    pub rtc_timr: RTC_TIMR,
    #[doc = "0x0c - Calendar Register"]
    pub rtc_calr: RTC_CALR,
    #[doc = "0x10 - Time Alarm Register"]
    pub rtc_timalr: RTC_TIMALR,
    #[doc = "0x14 - Calendar Alarm Register"]
    pub rtc_calalr: RTC_CALALR,
    #[doc = "0x18 - Status Register"]
    pub rtc_sr: RTC_SR,
    #[doc = "0x1c - Status Clear Command Register"]
    pub rtc_sccr: RTC_SCCR,
    #[doc = "0x20 - Interrupt Enable Register"]
    pub rtc_ier: RTC_IER,
    #[doc = "0x24 - Interrupt Disable Register"]
    pub rtc_idr: RTC_IDR,
    #[doc = "0x28 - Interrupt Mask Register"]
    pub rtc_imr: RTC_IMR,
    #[doc = "0x2c - Valid Entry Register"]
    pub rtc_ver: RTC_VER,
}
#[doc = "Control Register"]
pub struct RTC_CR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod rtc_cr;
#[doc = "Mode Register"]
pub struct RTC_MR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod rtc_mr;
#[doc = "Time Register"]
pub struct RTC_TIMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Time Register"]
pub mod rtc_timr;
#[doc = "Calendar Register"]
pub struct RTC_CALR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Calendar Register"]
pub mod rtc_calr;
#[doc = "Time Alarm Register"]
pub struct RTC_TIMALR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Time Alarm Register"]
pub mod rtc_timalr;
#[doc = "Calendar Alarm Register"]
pub struct RTC_CALALR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Calendar Alarm Register"]
pub mod rtc_calalr;
#[doc = "Status Register"]
pub struct RTC_SR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod rtc_sr;
#[doc = "Status Clear Command Register"]
pub struct RTC_SCCR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Status Clear Command Register"]
pub mod rtc_sccr;
#[doc = "Interrupt Enable Register"]
pub struct RTC_IER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod rtc_ier;
#[doc = "Interrupt Disable Register"]
pub struct RTC_IDR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod rtc_idr;
#[doc = "Interrupt Mask Register"]
pub struct RTC_IMR {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod rtc_imr;
#[doc = "Valid Entry Register"]
pub struct RTC_VER {
    register: vcell::VolatileCell<u32>,
}
#[doc = "Valid Entry Register"]
pub mod rtc_ver;
