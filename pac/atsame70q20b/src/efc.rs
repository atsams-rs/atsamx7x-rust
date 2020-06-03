#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EEFC Flash Mode Register"]
    pub eefc_fmr: EEFC_FMR,
    #[doc = "0x04 - EEFC Flash Command Register"]
    pub eefc_fcr: EEFC_FCR,
    #[doc = "0x08 - EEFC Flash Status Register"]
    pub eefc_fsr: EEFC_FSR,
    #[doc = "0x0c - EEFC Flash Result Register"]
    pub eefc_frr: EEFC_FRR,
    _reserved4: [u8; 212usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub eefc_wpmr: EEFC_WPMR,
}
#[doc = "EEFC Flash Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefc_fmr](eefc_fmr) module"]
pub type EEFC_FMR = crate::Reg<u32, _EEFC_FMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFC_FMR;
#[doc = "`read()` method returns [eefc_fmr::R](eefc_fmr::R) reader structure"]
impl crate::Readable for EEFC_FMR {}
#[doc = "`write(|w| ..)` method takes [eefc_fmr::W](eefc_fmr::W) writer structure"]
impl crate::Writable for EEFC_FMR {}
#[doc = "EEFC Flash Mode Register"]
pub mod eefc_fmr;
#[doc = "EEFC Flash Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefc_fcr](eefc_fcr) module"]
pub type EEFC_FCR = crate::Reg<u32, _EEFC_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFC_FCR;
#[doc = "`write(|w| ..)` method takes [eefc_fcr::W](eefc_fcr::W) writer structure"]
impl crate::Writable for EEFC_FCR {}
#[doc = "EEFC Flash Command Register"]
pub mod eefc_fcr;
#[doc = "EEFC Flash Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefc_fsr](eefc_fsr) module"]
pub type EEFC_FSR = crate::Reg<u32, _EEFC_FSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFC_FSR;
#[doc = "`read()` method returns [eefc_fsr::R](eefc_fsr::R) reader structure"]
impl crate::Readable for EEFC_FSR {}
#[doc = "EEFC Flash Status Register"]
pub mod eefc_fsr;
#[doc = "EEFC Flash Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefc_frr](eefc_frr) module"]
pub type EEFC_FRR = crate::Reg<u32, _EEFC_FRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFC_FRR;
#[doc = "`read()` method returns [eefc_frr::R](eefc_frr::R) reader structure"]
impl crate::Readable for EEFC_FRR {}
#[doc = "EEFC Flash Result Register"]
pub mod eefc_frr;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefc_wpmr](eefc_wpmr) module"]
pub type EEFC_WPMR = crate::Reg<u32, _EEFC_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFC_WPMR;
#[doc = "`read()` method returns [eefc_wpmr::R](eefc_wpmr::R) reader structure"]
impl crate::Readable for EEFC_WPMR {}
#[doc = "`write(|w| ..)` method takes [eefc_wpmr::W](eefc_wpmr::W) writer structure"]
impl crate::Writable for EEFC_WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod eefc_wpmr;
