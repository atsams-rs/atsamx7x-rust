#[doc = "Channel Control Register (channel = 0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_ccr](tc_ccr) module"]
pub type TC_CCR = crate::Reg<u32, _TC_CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_CCR;
#[doc = "`write(|w| ..)` method takes [tc_ccr::W](tc_ccr::W) writer structure"]
impl crate::Writable for TC_CCR {}
#[doc = "Channel Control Register (channel = 0)"]
pub mod tc_ccr;
#[doc = "Channel Mode Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_cmr_capture_mode](tc_cmr_capture_mode) module"]
pub type TC_CMR_CAPTURE_MODE = crate::Reg<u32, _TC_CMR_CAPTURE_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_CMR_CAPTURE_MODE;
#[doc = "`read()` method returns [tc_cmr_capture_mode::R](tc_cmr_capture_mode::R) reader structure"]
impl crate::Readable for TC_CMR_CAPTURE_MODE {}
#[doc = "`write(|w| ..)` method takes [tc_cmr_capture_mode::W](tc_cmr_capture_mode::W) writer structure"]
impl crate::Writable for TC_CMR_CAPTURE_MODE {}
#[doc = "Channel Mode Register (channel = 0)"]
pub mod tc_cmr_capture_mode;
#[doc = "Channel Mode Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_cmr_waveform_mode](tc_cmr_waveform_mode) module"]
pub type TC_CMR_WAVEFORM_MODE = crate::Reg<u32, _TC_CMR_WAVEFORM_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_CMR_WAVEFORM_MODE;
#[doc = "`read()` method returns [tc_cmr_waveform_mode::R](tc_cmr_waveform_mode::R) reader structure"]
impl crate::Readable for TC_CMR_WAVEFORM_MODE {}
#[doc = "`write(|w| ..)` method takes [tc_cmr_waveform_mode::W](tc_cmr_waveform_mode::W) writer structure"]
impl crate::Writable for TC_CMR_WAVEFORM_MODE {}
#[doc = "Channel Mode Register (channel = 0)"]
pub mod tc_cmr_waveform_mode;
#[doc = "Stepper Motor Mode Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_smmr](tc_smmr) module"]
pub type TC_SMMR = crate::Reg<u32, _TC_SMMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_SMMR;
#[doc = "`read()` method returns [tc_smmr::R](tc_smmr::R) reader structure"]
impl crate::Readable for TC_SMMR {}
#[doc = "`write(|w| ..)` method takes [tc_smmr::W](tc_smmr::W) writer structure"]
impl crate::Writable for TC_SMMR {}
#[doc = "Stepper Motor Mode Register (channel = 0)"]
pub mod tc_smmr;
#[doc = "Register AB (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_rab](tc_rab) module"]
pub type TC_RAB = crate::Reg<u32, _TC_RAB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_RAB;
#[doc = "`read()` method returns [tc_rab::R](tc_rab::R) reader structure"]
impl crate::Readable for TC_RAB {}
#[doc = "Register AB (channel = 0)"]
pub mod tc_rab;
#[doc = "Counter Value (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_cv](tc_cv) module"]
pub type TC_CV = crate::Reg<u32, _TC_CV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_CV;
#[doc = "`read()` method returns [tc_cv::R](tc_cv::R) reader structure"]
impl crate::Readable for TC_CV {}
#[doc = "Counter Value (channel = 0)"]
pub mod tc_cv;
#[doc = "Register A (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_ra](tc_ra) module"]
pub type TC_RA = crate::Reg<u32, _TC_RA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_RA;
#[doc = "`read()` method returns [tc_ra::R](tc_ra::R) reader structure"]
impl crate::Readable for TC_RA {}
#[doc = "`write(|w| ..)` method takes [tc_ra::W](tc_ra::W) writer structure"]
impl crate::Writable for TC_RA {}
#[doc = "Register A (channel = 0)"]
pub mod tc_ra;
#[doc = "Register B (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_rb](tc_rb) module"]
pub type TC_RB = crate::Reg<u32, _TC_RB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_RB;
#[doc = "`read()` method returns [tc_rb::R](tc_rb::R) reader structure"]
impl crate::Readable for TC_RB {}
#[doc = "`write(|w| ..)` method takes [tc_rb::W](tc_rb::W) writer structure"]
impl crate::Writable for TC_RB {}
#[doc = "Register B (channel = 0)"]
pub mod tc_rb;
#[doc = "Register C (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_rc](tc_rc) module"]
pub type TC_RC = crate::Reg<u32, _TC_RC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_RC;
#[doc = "`read()` method returns [tc_rc::R](tc_rc::R) reader structure"]
impl crate::Readable for TC_RC {}
#[doc = "`write(|w| ..)` method takes [tc_rc::W](tc_rc::W) writer structure"]
impl crate::Writable for TC_RC {}
#[doc = "Register C (channel = 0)"]
pub mod tc_rc;
#[doc = "Status Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_sr](tc_sr) module"]
pub type TC_SR = crate::Reg<u32, _TC_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_SR;
#[doc = "`read()` method returns [tc_sr::R](tc_sr::R) reader structure"]
impl crate::Readable for TC_SR {}
#[doc = "Status Register (channel = 0)"]
pub mod tc_sr;
#[doc = "Interrupt Enable Register (channel = 0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_ier](tc_ier) module"]
pub type TC_IER = crate::Reg<u32, _TC_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_IER;
#[doc = "`write(|w| ..)` method takes [tc_ier::W](tc_ier::W) writer structure"]
impl crate::Writable for TC_IER {}
#[doc = "Interrupt Enable Register (channel = 0)"]
pub mod tc_ier;
#[doc = "Interrupt Disable Register (channel = 0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_idr](tc_idr) module"]
pub type TC_IDR = crate::Reg<u32, _TC_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_IDR;
#[doc = "`write(|w| ..)` method takes [tc_idr::W](tc_idr::W) writer structure"]
impl crate::Writable for TC_IDR {}
#[doc = "Interrupt Disable Register (channel = 0)"]
pub mod tc_idr;
#[doc = "Interrupt Mask Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_imr](tc_imr) module"]
pub type TC_IMR = crate::Reg<u32, _TC_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_IMR;
#[doc = "`read()` method returns [tc_imr::R](tc_imr::R) reader structure"]
impl crate::Readable for TC_IMR {}
#[doc = "Interrupt Mask Register (channel = 0)"]
pub mod tc_imr;
#[doc = "Extended Mode Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tc_emr](tc_emr) module"]
pub type TC_EMR = crate::Reg<u32, _TC_EMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC_EMR;
#[doc = "`read()` method returns [tc_emr::R](tc_emr::R) reader structure"]
impl crate::Readable for TC_EMR {}
#[doc = "`write(|w| ..)` method takes [tc_emr::W](tc_emr::W) writer structure"]
impl crate::Writable for TC_EMR {}
#[doc = "Extended Mode Register (channel = 0)"]
pub mod tc_emr;
