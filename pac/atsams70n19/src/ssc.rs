#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ssc_cr: SSC_CR,
    #[doc = "0x04 - Clock Mode Register"]
    pub ssc_cmr: SSC_CMR,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Receive Clock Mode Register"]
    pub ssc_rcmr: SSC_RCMR,
    #[doc = "0x14 - Receive Frame Mode Register"]
    pub ssc_rfmr: SSC_RFMR,
    #[doc = "0x18 - Transmit Clock Mode Register"]
    pub ssc_tcmr: SSC_TCMR,
    #[doc = "0x1c - Transmit Frame Mode Register"]
    pub ssc_tfmr: SSC_TFMR,
    #[doc = "0x20 - Receive Holding Register"]
    pub ssc_rhr: SSC_RHR,
    #[doc = "0x24 - Transmit Holding Register"]
    pub ssc_thr: SSC_THR,
    _reserved8: [u8; 8usize],
    #[doc = "0x30 - Receive Sync. Holding Register"]
    pub ssc_rshr: SSC_RSHR,
    #[doc = "0x34 - Transmit Sync. Holding Register"]
    pub ssc_tshr: SSC_TSHR,
    #[doc = "0x38 - Receive Compare 0 Register"]
    pub ssc_rc0r: SSC_RC0R,
    #[doc = "0x3c - Receive Compare 1 Register"]
    pub ssc_rc1r: SSC_RC1R,
    #[doc = "0x40 - Status Register"]
    pub ssc_sr: SSC_SR,
    #[doc = "0x44 - Interrupt Enable Register"]
    pub ssc_ier: SSC_IER,
    #[doc = "0x48 - Interrupt Disable Register"]
    pub ssc_idr: SSC_IDR,
    #[doc = "0x4c - Interrupt Mask Register"]
    pub ssc_imr: SSC_IMR,
    _reserved16: [u8; 148usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub ssc_wpmr: SSC_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub ssc_wpsr: SSC_WPSR,
}
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_cr](ssc_cr) module"]
pub type SSC_CR = crate::Reg<u32, _SSC_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC_CR;
#[doc = "`write(|w| ..)` method takes [ssc_cr::W](ssc_cr::W) writer structure"]
impl crate::Writable for SSC_CR {}
#[doc = "Control Register"]
pub mod ssc_cr;
#[doc = "Clock Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_cmr](ssc_cmr) module"]
pub type SSC_CMR = crate::Reg<u32, _SSC_CMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC_CMR;
#[doc = "`read()` method returns [ssc_cmr::R](ssc_cmr::R) reader structure"]
impl crate::Readable for SSC_CMR {}
#[doc = "`write(|w| ..)` method takes [ssc_cmr::W](ssc_cmr::W) writer structure"]
impl crate::Writable for SSC_CMR {}
#[doc = "Clock Mode Register"]
pub mod ssc_cmr;
#[doc = "Receive Clock Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_rcmr](ssc_rcmr) module"]
pub type SSC_RCMR = crate::Reg<u32, _SSC_RCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC_RCMR;
#[doc = "`read()` method returns [ssc_rcmr::R](ssc_rcmr::R) reader structure"]
impl crate::Readable for SSC_RCMR {}
#[doc = "`write(|w| ..)` method takes [ssc_rcmr::W](ssc_rcmr::W) writer structure"]
impl crate::Writable for SSC_RCMR {}
#[doc = "Receive Clock Mode Register"]
pub mod ssc_rcmr;
#[doc = "Receive Frame Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_rfmr](ssc_rfmr) module"]
pub type SSC_RFMR = crate::Reg<u32, _SSC_RFMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC_RFMR;
#[doc = "`read()` method returns [ssc_rfmr::R](ssc_rfmr::R) reader structure"]
impl crate::Readable for SSC_RFMR {}
#[doc = "`write(|w| ..)` method takes [ssc_rfmr::W](ssc_rfmr::W) writer structure"]
impl crate::Writable for SSC_RFMR {}
#[doc = "Receive Frame Mode Register"]
pub mod ssc_rfmr;
#[doc = "Transmit Clock Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_tcmr](ssc_tcmr) module"]
pub type SSC_TCMR = crate::Reg<u32, _SSC_TCMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC_TCMR;
#[doc = "`read()` method returns [ssc_tcmr::R](ssc_tcmr::R) reader structure"]
impl crate::Readable for SSC_TCMR {}
#[doc = "`write(|w| ..)` method takes [ssc_tcmr::W](ssc_tcmr::W) writer structure"]
impl crate::Writable for SSC_TCMR {}
#[doc = "Transmit Clock Mode Register"]
pub mod ssc_tcmr;
#[doc = "Transmit Frame Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_tfmr](ssc_tfmr) module"]
pub type SSC_TFMR = crate::Reg<u32, _SSC_TFMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC_TFMR;
#[doc = "`read()` method returns [ssc_tfmr::R](ssc_tfmr::R) reader structure"]
impl crate::Readable for SSC_TFMR {}
#[doc = "`write(|w| ..)` method takes [ssc_tfmr::W](ssc_tfmr::W) writer structure"]
impl crate::Writable for SSC_TFMR {}
#[doc = "Transmit Frame Mode Register"]
pub mod ssc_tfmr;
#[doc = "Receive Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_rhr](ssc_rhr) module"]
pub type SSC_RHR = crate::Reg<u32, _SSC_RHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC_RHR;
#[doc = "`read()` method returns [ssc_rhr::R](ssc_rhr::R) reader structure"]
impl crate::Readable for SSC_RHR {}
#[doc = "Receive Holding Register"]
pub mod ssc_rhr;
#[doc = "Transmit Holding Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_thr](ssc_thr) module"]
pub type SSC_THR = crate::Reg<u32, _SSC_THR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC_THR;
#[doc = "`write(|w| ..)` method takes [ssc_thr::W](ssc_thr::W) writer structure"]
impl crate::Writable for SSC_THR {}
#[doc = "Transmit Holding Register"]
pub mod ssc_thr;
#[doc = "Receive Sync. Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_rshr](ssc_rshr) module"]
pub type SSC_RSHR = crate::Reg<u32, _SSC_RSHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC_RSHR;
#[doc = "`read()` method returns [ssc_rshr::R](ssc_rshr::R) reader structure"]
impl crate::Readable for SSC_RSHR {}
#[doc = "Receive Sync. Holding Register"]
pub mod ssc_rshr;
#[doc = "Transmit Sync. Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_tshr](ssc_tshr) module"]
pub type SSC_TSHR = crate::Reg<u32, _SSC_TSHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC_TSHR;
#[doc = "`read()` method returns [ssc_tshr::R](ssc_tshr::R) reader structure"]
impl crate::Readable for SSC_TSHR {}
#[doc = "`write(|w| ..)` method takes [ssc_tshr::W](ssc_tshr::W) writer structure"]
impl crate::Writable for SSC_TSHR {}
#[doc = "Transmit Sync. Holding Register"]
pub mod ssc_tshr;
#[doc = "Receive Compare 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_rc0r](ssc_rc0r) module"]
pub type SSC_RC0R = crate::Reg<u32, _SSC_RC0R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC_RC0R;
#[doc = "`read()` method returns [ssc_rc0r::R](ssc_rc0r::R) reader structure"]
impl crate::Readable for SSC_RC0R {}
#[doc = "`write(|w| ..)` method takes [ssc_rc0r::W](ssc_rc0r::W) writer structure"]
impl crate::Writable for SSC_RC0R {}
#[doc = "Receive Compare 0 Register"]
pub mod ssc_rc0r;
#[doc = "Receive Compare 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_rc1r](ssc_rc1r) module"]
pub type SSC_RC1R = crate::Reg<u32, _SSC_RC1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC_RC1R;
#[doc = "`read()` method returns [ssc_rc1r::R](ssc_rc1r::R) reader structure"]
impl crate::Readable for SSC_RC1R {}
#[doc = "`write(|w| ..)` method takes [ssc_rc1r::W](ssc_rc1r::W) writer structure"]
impl crate::Writable for SSC_RC1R {}
#[doc = "Receive Compare 1 Register"]
pub mod ssc_rc1r;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_sr](ssc_sr) module"]
pub type SSC_SR = crate::Reg<u32, _SSC_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC_SR;
#[doc = "`read()` method returns [ssc_sr::R](ssc_sr::R) reader structure"]
impl crate::Readable for SSC_SR {}
#[doc = "Status Register"]
pub mod ssc_sr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_ier](ssc_ier) module"]
pub type SSC_IER = crate::Reg<u32, _SSC_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC_IER;
#[doc = "`write(|w| ..)` method takes [ssc_ier::W](ssc_ier::W) writer structure"]
impl crate::Writable for SSC_IER {}
#[doc = "Interrupt Enable Register"]
pub mod ssc_ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_idr](ssc_idr) module"]
pub type SSC_IDR = crate::Reg<u32, _SSC_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC_IDR;
#[doc = "`write(|w| ..)` method takes [ssc_idr::W](ssc_idr::W) writer structure"]
impl crate::Writable for SSC_IDR {}
#[doc = "Interrupt Disable Register"]
pub mod ssc_idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_imr](ssc_imr) module"]
pub type SSC_IMR = crate::Reg<u32, _SSC_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC_IMR;
#[doc = "`read()` method returns [ssc_imr::R](ssc_imr::R) reader structure"]
impl crate::Readable for SSC_IMR {}
#[doc = "Interrupt Mask Register"]
pub mod ssc_imr;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_wpmr](ssc_wpmr) module"]
pub type SSC_WPMR = crate::Reg<u32, _SSC_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC_WPMR;
#[doc = "`read()` method returns [ssc_wpmr::R](ssc_wpmr::R) reader structure"]
impl crate::Readable for SSC_WPMR {}
#[doc = "`write(|w| ..)` method takes [ssc_wpmr::W](ssc_wpmr::W) writer structure"]
impl crate::Writable for SSC_WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod ssc_wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssc_wpsr](ssc_wpsr) module"]
pub type SSC_WPSR = crate::Reg<u32, _SSC_WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSC_WPSR;
#[doc = "`read()` method returns [ssc_wpsr::R](ssc_wpsr::R) reader structure"]
impl crate::Readable for SSC_WPSR {}
#[doc = "Write Protection Status Register"]
pub mod ssc_wpsr;
