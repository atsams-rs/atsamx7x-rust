#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    pub mr: crate::Reg<mr::MR_SPEC>,
    #[doc = "0x04 - Alarm Register"]
    pub ar: crate::Reg<ar::AR_SPEC>,
    #[doc = "0x08 - Value Register"]
    pub vr: crate::Reg<vr::VR_SPEC>,
    #[doc = "0x0c - Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
}
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "AR register accessor: an alias for `Reg<AR_SPEC>`"]
pub type AR = crate::Reg<ar::AR_SPEC>;
#[doc = "Alarm Register"]
pub mod ar;
#[doc = "VR register accessor: an alias for `Reg<VR_SPEC>`"]
pub type VR = crate::Reg<vr::VR_SPEC>;
#[doc = "Value Register"]
pub mod vr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "Status Register"]
pub mod sr;
