#[doc = r"Register block"]
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
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtt_mr](rtt_mr) module"]
pub type RTT_MR = crate::Reg<u32, _RTT_MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTT_MR;
#[doc = "`read()` method returns [rtt_mr::R](rtt_mr::R) reader structure"]
impl crate::Readable for RTT_MR {}
#[doc = "`write(|w| ..)` method takes [rtt_mr::W](rtt_mr::W) writer structure"]
impl crate::Writable for RTT_MR {}
#[doc = "Mode Register"]
pub mod rtt_mr;
#[doc = "Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtt_ar](rtt_ar) module"]
pub type RTT_AR = crate::Reg<u32, _RTT_AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTT_AR;
#[doc = "`read()` method returns [rtt_ar::R](rtt_ar::R) reader structure"]
impl crate::Readable for RTT_AR {}
#[doc = "`write(|w| ..)` method takes [rtt_ar::W](rtt_ar::W) writer structure"]
impl crate::Writable for RTT_AR {}
#[doc = "Alarm Register"]
pub mod rtt_ar;
#[doc = "Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtt_vr](rtt_vr) module"]
pub type RTT_VR = crate::Reg<u32, _RTT_VR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTT_VR;
#[doc = "`read()` method returns [rtt_vr::R](rtt_vr::R) reader structure"]
impl crate::Readable for RTT_VR {}
#[doc = "Value Register"]
pub mod rtt_vr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtt_sr](rtt_sr) module"]
pub type RTT_SR = crate::Reg<u32, _RTT_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTT_SR;
#[doc = "`read()` method returns [rtt_sr::R](rtt_sr::R) reader structure"]
impl crate::Readable for RTT_SR {}
#[doc = "Status Register"]
pub mod rtt_sr;
