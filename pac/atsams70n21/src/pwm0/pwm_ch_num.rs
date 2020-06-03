#[doc = "PWM Channel Mode Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cmr](pwm_cmr) module"]
pub type PWM_CMR = crate::Reg<u32, _PWM_CMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_CMR;
#[doc = "`read()` method returns [pwm_cmr::R](pwm_cmr::R) reader structure"]
impl crate::Readable for PWM_CMR {}
#[doc = "`write(|w| ..)` method takes [pwm_cmr::W](pwm_cmr::W) writer structure"]
impl crate::Writable for PWM_CMR {}
#[doc = "PWM Channel Mode Register (ch_num = 0)"]
pub mod pwm_cmr;
#[doc = "PWM Channel Duty Cycle Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cdty](pwm_cdty) module"]
pub type PWM_CDTY = crate::Reg<u32, _PWM_CDTY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_CDTY;
#[doc = "`read()` method returns [pwm_cdty::R](pwm_cdty::R) reader structure"]
impl crate::Readable for PWM_CDTY {}
#[doc = "`write(|w| ..)` method takes [pwm_cdty::W](pwm_cdty::W) writer structure"]
impl crate::Writable for PWM_CDTY {}
#[doc = "PWM Channel Duty Cycle Register (ch_num = 0)"]
pub mod pwm_cdty;
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cdtyupd](pwm_cdtyupd) module"]
pub type PWM_CDTYUPD = crate::Reg<u32, _PWM_CDTYUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_CDTYUPD;
#[doc = "`write(|w| ..)` method takes [pwm_cdtyupd::W](pwm_cdtyupd::W) writer structure"]
impl crate::Writable for PWM_CDTYUPD {}
#[doc = "PWM Channel Duty Cycle Update Register (ch_num = 0)"]
pub mod pwm_cdtyupd;
#[doc = "PWM Channel Period Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cprd](pwm_cprd) module"]
pub type PWM_CPRD = crate::Reg<u32, _PWM_CPRD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_CPRD;
#[doc = "`read()` method returns [pwm_cprd::R](pwm_cprd::R) reader structure"]
impl crate::Readable for PWM_CPRD {}
#[doc = "`write(|w| ..)` method takes [pwm_cprd::W](pwm_cprd::W) writer structure"]
impl crate::Writable for PWM_CPRD {}
#[doc = "PWM Channel Period Register (ch_num = 0)"]
pub mod pwm_cprd;
#[doc = "PWM Channel Period Update Register (ch_num = 0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cprdupd](pwm_cprdupd) module"]
pub type PWM_CPRDUPD = crate::Reg<u32, _PWM_CPRDUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_CPRDUPD;
#[doc = "`write(|w| ..)` method takes [pwm_cprdupd::W](pwm_cprdupd::W) writer structure"]
impl crate::Writable for PWM_CPRDUPD {}
#[doc = "PWM Channel Period Update Register (ch_num = 0)"]
pub mod pwm_cprdupd;
#[doc = "PWM Channel Counter Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ccnt](pwm_ccnt) module"]
pub type PWM_CCNT = crate::Reg<u32, _PWM_CCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_CCNT;
#[doc = "`read()` method returns [pwm_ccnt::R](pwm_ccnt::R) reader structure"]
impl crate::Readable for PWM_CCNT {}
#[doc = "PWM Channel Counter Register (ch_num = 0)"]
pub mod pwm_ccnt;
#[doc = "PWM Channel Dead Time Register (ch_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_dt](pwm_dt) module"]
pub type PWM_DT = crate::Reg<u32, _PWM_DT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_DT;
#[doc = "`read()` method returns [pwm_dt::R](pwm_dt::R) reader structure"]
impl crate::Readable for PWM_DT {}
#[doc = "`write(|w| ..)` method takes [pwm_dt::W](pwm_dt::W) writer structure"]
impl crate::Writable for PWM_DT {}
#[doc = "PWM Channel Dead Time Register (ch_num = 0)"]
pub mod pwm_dt;
#[doc = "PWM Channel Dead Time Update Register (ch_num = 0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_dtupd](pwm_dtupd) module"]
pub type PWM_DTUPD = crate::Reg<u32, _PWM_DTUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_DTUPD;
#[doc = "`write(|w| ..)` method takes [pwm_dtupd::W](pwm_dtupd::W) writer structure"]
impl crate::Writable for PWM_DTUPD {}
#[doc = "PWM Channel Dead Time Update Register (ch_num = 0)"]
pub mod pwm_dtupd;
