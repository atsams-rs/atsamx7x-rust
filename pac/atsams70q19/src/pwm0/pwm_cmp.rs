#[doc = "PWM Comparison 0 Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cmpv](pwm_cmpv) module"]
pub type PWM_CMPV = crate::Reg<u32, _PWM_CMPV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_CMPV;
#[doc = "`read()` method returns [pwm_cmpv::R](pwm_cmpv::R) reader structure"]
impl crate::Readable for PWM_CMPV {}
#[doc = "`write(|w| ..)` method takes [pwm_cmpv::W](pwm_cmpv::W) writer structure"]
impl crate::Writable for PWM_CMPV {}
#[doc = "PWM Comparison 0 Value Register"]
pub mod pwm_cmpv;
#[doc = "PWM Comparison 0 Value Update Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cmpvupd](pwm_cmpvupd) module"]
pub type PWM_CMPVUPD = crate::Reg<u32, _PWM_CMPVUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_CMPVUPD;
#[doc = "`write(|w| ..)` method takes [pwm_cmpvupd::W](pwm_cmpvupd::W) writer structure"]
impl crate::Writable for PWM_CMPVUPD {}
#[doc = "PWM Comparison 0 Value Update Register"]
pub mod pwm_cmpvupd;
#[doc = "PWM Comparison 0 Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cmpm](pwm_cmpm) module"]
pub type PWM_CMPM = crate::Reg<u32, _PWM_CMPM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_CMPM;
#[doc = "`read()` method returns [pwm_cmpm::R](pwm_cmpm::R) reader structure"]
impl crate::Readable for PWM_CMPM {}
#[doc = "`write(|w| ..)` method takes [pwm_cmpm::W](pwm_cmpm::W) writer structure"]
impl crate::Writable for PWM_CMPM {}
#[doc = "PWM Comparison 0 Mode Register"]
pub mod pwm_cmpm;
#[doc = "PWM Comparison 0 Mode Update Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_cmpmupd](pwm_cmpmupd) module"]
pub type PWM_CMPMUPD = crate::Reg<u32, _PWM_CMPMUPD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWM_CMPMUPD;
#[doc = "`write(|w| ..)` method takes [pwm_cmpmupd::W](pwm_cmpmupd::W) writer structure"]
impl crate::Writable for PWM_CMPMUPD {}
#[doc = "PWM Comparison 0 Mode Update Register"]
pub mod pwm_cmpmupd;
