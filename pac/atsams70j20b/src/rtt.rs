#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    pub rtt_mr: RTT_MR,
    #[doc = "0x04 - Alarm Register"]
    pub rtt_ar: RTT_AR,
    #[doc = "0x08 - Value Register"]
    pub rtt_vr: RTT_VR,
    #[doc = "0x0c - Status Register"]
    pub rtt_sr: RTT_SR,
}
#[doc = "Mode Register"]
pub struct RTT_MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod rtt_mr;
#[doc = "Alarm Register"]
pub struct RTT_AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm Register"]
pub mod rtt_ar;
#[doc = "Value Register"]
pub struct RTT_VR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value Register"]
pub mod rtt_vr;
#[doc = "Status Register"]
pub struct RTT_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod rtt_sr;
