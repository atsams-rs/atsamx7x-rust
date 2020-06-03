#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub twihs_cr: TWIHS_CR,
    #[doc = "0x04 - Master Mode Register"]
    pub twihs_mmr: TWIHS_MMR,
    #[doc = "0x08 - Slave Mode Register"]
    pub twihs_smr: TWIHS_SMR,
    #[doc = "0x0c - Internal Address Register"]
    pub twihs_iadr: TWIHS_IADR,
    #[doc = "0x10 - Clock Waveform Generator Register"]
    pub twihs_cwgr: TWIHS_CWGR,
    _reserved5: [u8; 12usize],
    #[doc = "0x20 - Status Register"]
    pub twihs_sr: TWIHS_SR,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub twihs_ier: TWIHS_IER,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub twihs_idr: TWIHS_IDR,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub twihs_imr: TWIHS_IMR,
    #[doc = "0x30 - Receive Holding Register"]
    pub twihs_rhr: TWIHS_RHR,
    #[doc = "0x34 - Transmit Holding Register"]
    pub twihs_thr: TWIHS_THR,
    #[doc = "0x38 - SMBus Timing Register"]
    pub twihs_smbtr: TWIHS_SMBTR,
    _reserved12: [u8; 8usize],
    #[doc = "0x44 - Filter Register"]
    pub twihs_filtr: TWIHS_FILTR,
    _reserved13: [u8; 4usize],
    #[doc = "0x4c - SleepWalking Matching Register"]
    pub twihs_swmr: TWIHS_SWMR,
    _reserved14: [u8; 148usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub twihs_wpmr: TWIHS_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub twihs_wpsr: TWIHS_WPSR,
}
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_cr](twihs_cr) module"]
pub type TWIHS_CR = crate::Reg<u32, _TWIHS_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWIHS_CR;
#[doc = "`write(|w| ..)` method takes [twihs_cr::W](twihs_cr::W) writer structure"]
impl crate::Writable for TWIHS_CR {}
#[doc = "Control Register"]
pub mod twihs_cr;
#[doc = "Master Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_mmr](twihs_mmr) module"]
pub type TWIHS_MMR = crate::Reg<u32, _TWIHS_MMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWIHS_MMR;
#[doc = "`read()` method returns [twihs_mmr::R](twihs_mmr::R) reader structure"]
impl crate::Readable for TWIHS_MMR {}
#[doc = "`write(|w| ..)` method takes [twihs_mmr::W](twihs_mmr::W) writer structure"]
impl crate::Writable for TWIHS_MMR {}
#[doc = "Master Mode Register"]
pub mod twihs_mmr;
#[doc = "Slave Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_smr](twihs_smr) module"]
pub type TWIHS_SMR = crate::Reg<u32, _TWIHS_SMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWIHS_SMR;
#[doc = "`read()` method returns [twihs_smr::R](twihs_smr::R) reader structure"]
impl crate::Readable for TWIHS_SMR {}
#[doc = "`write(|w| ..)` method takes [twihs_smr::W](twihs_smr::W) writer structure"]
impl crate::Writable for TWIHS_SMR {}
#[doc = "Slave Mode Register"]
pub mod twihs_smr;
#[doc = "Internal Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_iadr](twihs_iadr) module"]
pub type TWIHS_IADR = crate::Reg<u32, _TWIHS_IADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWIHS_IADR;
#[doc = "`read()` method returns [twihs_iadr::R](twihs_iadr::R) reader structure"]
impl crate::Readable for TWIHS_IADR {}
#[doc = "`write(|w| ..)` method takes [twihs_iadr::W](twihs_iadr::W) writer structure"]
impl crate::Writable for TWIHS_IADR {}
#[doc = "Internal Address Register"]
pub mod twihs_iadr;
#[doc = "Clock Waveform Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_cwgr](twihs_cwgr) module"]
pub type TWIHS_CWGR = crate::Reg<u32, _TWIHS_CWGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWIHS_CWGR;
#[doc = "`read()` method returns [twihs_cwgr::R](twihs_cwgr::R) reader structure"]
impl crate::Readable for TWIHS_CWGR {}
#[doc = "`write(|w| ..)` method takes [twihs_cwgr::W](twihs_cwgr::W) writer structure"]
impl crate::Writable for TWIHS_CWGR {}
#[doc = "Clock Waveform Generator Register"]
pub mod twihs_cwgr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_sr](twihs_sr) module"]
pub type TWIHS_SR = crate::Reg<u32, _TWIHS_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWIHS_SR;
#[doc = "`read()` method returns [twihs_sr::R](twihs_sr::R) reader structure"]
impl crate::Readable for TWIHS_SR {}
#[doc = "Status Register"]
pub mod twihs_sr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_ier](twihs_ier) module"]
pub type TWIHS_IER = crate::Reg<u32, _TWIHS_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWIHS_IER;
#[doc = "`write(|w| ..)` method takes [twihs_ier::W](twihs_ier::W) writer structure"]
impl crate::Writable for TWIHS_IER {}
#[doc = "Interrupt Enable Register"]
pub mod twihs_ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_idr](twihs_idr) module"]
pub type TWIHS_IDR = crate::Reg<u32, _TWIHS_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWIHS_IDR;
#[doc = "`write(|w| ..)` method takes [twihs_idr::W](twihs_idr::W) writer structure"]
impl crate::Writable for TWIHS_IDR {}
#[doc = "Interrupt Disable Register"]
pub mod twihs_idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_imr](twihs_imr) module"]
pub type TWIHS_IMR = crate::Reg<u32, _TWIHS_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWIHS_IMR;
#[doc = "`read()` method returns [twihs_imr::R](twihs_imr::R) reader structure"]
impl crate::Readable for TWIHS_IMR {}
#[doc = "Interrupt Mask Register"]
pub mod twihs_imr;
#[doc = "Receive Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_rhr](twihs_rhr) module"]
pub type TWIHS_RHR = crate::Reg<u32, _TWIHS_RHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWIHS_RHR;
#[doc = "`read()` method returns [twihs_rhr::R](twihs_rhr::R) reader structure"]
impl crate::Readable for TWIHS_RHR {}
#[doc = "Receive Holding Register"]
pub mod twihs_rhr;
#[doc = "Transmit Holding Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_thr](twihs_thr) module"]
pub type TWIHS_THR = crate::Reg<u32, _TWIHS_THR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWIHS_THR;
#[doc = "`write(|w| ..)` method takes [twihs_thr::W](twihs_thr::W) writer structure"]
impl crate::Writable for TWIHS_THR {}
#[doc = "Transmit Holding Register"]
pub mod twihs_thr;
#[doc = "SMBus Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_smbtr](twihs_smbtr) module"]
pub type TWIHS_SMBTR = crate::Reg<u32, _TWIHS_SMBTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWIHS_SMBTR;
#[doc = "`read()` method returns [twihs_smbtr::R](twihs_smbtr::R) reader structure"]
impl crate::Readable for TWIHS_SMBTR {}
#[doc = "`write(|w| ..)` method takes [twihs_smbtr::W](twihs_smbtr::W) writer structure"]
impl crate::Writable for TWIHS_SMBTR {}
#[doc = "SMBus Timing Register"]
pub mod twihs_smbtr;
#[doc = "Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_filtr](twihs_filtr) module"]
pub type TWIHS_FILTR = crate::Reg<u32, _TWIHS_FILTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWIHS_FILTR;
#[doc = "`read()` method returns [twihs_filtr::R](twihs_filtr::R) reader structure"]
impl crate::Readable for TWIHS_FILTR {}
#[doc = "`write(|w| ..)` method takes [twihs_filtr::W](twihs_filtr::W) writer structure"]
impl crate::Writable for TWIHS_FILTR {}
#[doc = "Filter Register"]
pub mod twihs_filtr;
#[doc = "SleepWalking Matching Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_swmr](twihs_swmr) module"]
pub type TWIHS_SWMR = crate::Reg<u32, _TWIHS_SWMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWIHS_SWMR;
#[doc = "`read()` method returns [twihs_swmr::R](twihs_swmr::R) reader structure"]
impl crate::Readable for TWIHS_SWMR {}
#[doc = "`write(|w| ..)` method takes [twihs_swmr::W](twihs_swmr::W) writer structure"]
impl crate::Writable for TWIHS_SWMR {}
#[doc = "SleepWalking Matching Register"]
pub mod twihs_swmr;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_wpmr](twihs_wpmr) module"]
pub type TWIHS_WPMR = crate::Reg<u32, _TWIHS_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWIHS_WPMR;
#[doc = "`read()` method returns [twihs_wpmr::R](twihs_wpmr::R) reader structure"]
impl crate::Readable for TWIHS_WPMR {}
#[doc = "`write(|w| ..)` method takes [twihs_wpmr::W](twihs_wpmr::W) writer structure"]
impl crate::Writable for TWIHS_WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod twihs_wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twihs_wpsr](twihs_wpsr) module"]
pub type TWIHS_WPSR = crate::Reg<u32, _TWIHS_WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TWIHS_WPSR;
#[doc = "`read()` method returns [twihs_wpsr::R](twihs_wpsr::R) reader structure"]
impl crate::Readable for TWIHS_WPSR {}
#[doc = "Write Protection Status Register"]
pub mod twihs_wpsr;
