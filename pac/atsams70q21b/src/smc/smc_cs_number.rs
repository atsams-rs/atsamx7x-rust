#[doc = "SMC Setup Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smc_setup](smc_setup) module"]
pub type SMC_SETUP = crate::Reg<u32, _SMC_SETUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMC_SETUP;
#[doc = "`read()` method returns [smc_setup::R](smc_setup::R) reader structure"]
impl crate::Readable for SMC_SETUP {}
#[doc = "`write(|w| ..)` method takes [smc_setup::W](smc_setup::W) writer structure"]
impl crate::Writable for SMC_SETUP {}
#[doc = "SMC Setup Register"]
pub mod smc_setup;
#[doc = "SMC Pulse Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smc_pulse](smc_pulse) module"]
pub type SMC_PULSE = crate::Reg<u32, _SMC_PULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMC_PULSE;
#[doc = "`read()` method returns [smc_pulse::R](smc_pulse::R) reader structure"]
impl crate::Readable for SMC_PULSE {}
#[doc = "`write(|w| ..)` method takes [smc_pulse::W](smc_pulse::W) writer structure"]
impl crate::Writable for SMC_PULSE {}
#[doc = "SMC Pulse Register"]
pub mod smc_pulse;
#[doc = "SMC Cycle Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smc_cycle](smc_cycle) module"]
pub type SMC_CYCLE = crate::Reg<u32, _SMC_CYCLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMC_CYCLE;
#[doc = "`read()` method returns [smc_cycle::R](smc_cycle::R) reader structure"]
impl crate::Readable for SMC_CYCLE {}
#[doc = "`write(|w| ..)` method takes [smc_cycle::W](smc_cycle::W) writer structure"]
impl crate::Writable for SMC_CYCLE {}
#[doc = "SMC Cycle Register"]
pub mod smc_cycle;
#[doc = "SMC Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smc_mode](smc_mode) module"]
pub type SMC_MODE = crate::Reg<u32, _SMC_MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMC_MODE;
#[doc = "`read()` method returns [smc_mode::R](smc_mode::R) reader structure"]
impl crate::Readable for SMC_MODE {}
#[doc = "`write(|w| ..)` method takes [smc_mode::W](smc_mode::W) writer structure"]
impl crate::Writable for SMC_MODE {}
#[doc = "SMC Mode Register"]
pub mod smc_mode;
