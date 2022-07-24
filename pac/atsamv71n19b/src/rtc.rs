#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - Mode Register"]
    pub mr: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x08 - Time Register"]
    pub timr: crate::Reg<timr::TIMR_SPEC>,
    #[doc = "0x0c - Calendar Register"]
    pub calr: crate::Reg<calr::CALR_SPEC>,
    #[doc = "0x10 - Time Alarm Register"]
    pub timalr: crate::Reg<timalr::TIMALR_SPEC>,
    #[doc = "0x14 - Calendar Alarm Register"]
    pub calalr: crate::Reg<calalr::CALALR_SPEC>,
    #[doc = "0x18 - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x1c - Status Clear Command Register"]
    pub sccr: crate::Reg<sccr::SCCR_SPEC>,
    #[doc = "0x20 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x24 - Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x28 - Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x2c - Valid Entry Register"]
    pub ver: crate::Reg<ver::VER_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "TIMR register accessor: an alias for `Reg<TIMR_SPEC>`"]
pub type TIMR = crate::Reg<timr::TIMR_SPEC>;
#[doc = "Time Register"]
pub mod timr;
#[doc = "CALR register accessor: an alias for `Reg<CALR_SPEC>`"]
pub type CALR = crate::Reg<calr::CALR_SPEC>;
#[doc = "Calendar Register"]
pub mod calr;
#[doc = "TIMALR register accessor: an alias for `Reg<TIMALR_SPEC>`"]
pub type TIMALR = crate::Reg<timalr::TIMALR_SPEC>;
#[doc = "Time Alarm Register"]
pub mod timalr;
#[doc = "CALALR register accessor: an alias for `Reg<CALALR_SPEC>`"]
pub type CALALR = crate::Reg<calalr::CALALR_SPEC>;
#[doc = "Calendar Alarm Register"]
pub mod calalr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "SCCR register accessor: an alias for `Reg<SCCR_SPEC>`"]
pub type SCCR = crate::Reg<sccr::SCCR_SPEC>;
#[doc = "Status Clear Command Register"]
pub mod sccr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "VER register accessor: an alias for `Reg<VER_SPEC>`"]
pub type VER = crate::Reg<ver::VER_SPEC>;
#[doc = "Valid Entry Register"]
pub mod ver;
