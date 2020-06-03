#[doc = "Priority Register A for Slave 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_pras](matrix_pras) module"]
pub type MATRIX_PRAS = crate::Reg<u32, _MATRIX_PRAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRAS;
#[doc = "`read()` method returns [matrix_pras::R](matrix_pras::R) reader structure"]
impl crate::Readable for MATRIX_PRAS {}
#[doc = "`write(|w| ..)` method takes [matrix_pras::W](matrix_pras::W) writer structure"]
impl crate::Writable for MATRIX_PRAS {}
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pras;
#[doc = "Priority Register B for Slave 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_prbs](matrix_prbs) module"]
pub type MATRIX_PRBS = crate::Reg<u32, _MATRIX_PRBS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MATRIX_PRBS;
#[doc = "`read()` method returns [matrix_prbs::R](matrix_prbs::R) reader structure"]
impl crate::Readable for MATRIX_PRBS {}
#[doc = "`write(|w| ..)` method takes [matrix_prbs::W](matrix_prbs::W) writer structure"]
impl crate::Writable for MATRIX_PRBS {}
#[doc = "Priority Register B for Slave 0"]
pub mod matrix_prbs;
