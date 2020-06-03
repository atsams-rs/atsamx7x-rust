#[doc = "Specific Address 1 Bottom Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_sab](gmac_sab) module"]
pub type GMAC_SAB = crate::Reg<u32, _GMAC_SAB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_SAB;
#[doc = "`read()` method returns [gmac_sab::R](gmac_sab::R) reader structure"]
impl crate::Readable for GMAC_SAB {}
#[doc = "`write(|w| ..)` method takes [gmac_sab::W](gmac_sab::W) writer structure"]
impl crate::Writable for GMAC_SAB {}
#[doc = "Specific Address 1 Bottom Register"]
pub mod gmac_sab;
#[doc = "Specific Address 1 Top Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gmac_sat](gmac_sat) module"]
pub type GMAC_SAT = crate::Reg<u32, _GMAC_SAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GMAC_SAT;
#[doc = "`read()` method returns [gmac_sat::R](gmac_sat::R) reader structure"]
impl crate::Readable for GMAC_SAT {}
#[doc = "`write(|w| ..)` method takes [gmac_sat::W](gmac_sat::W) writer structure"]
impl crate::Writable for GMAC_SAT {}
#[doc = "Specific Address 1 Top Register"]
pub mod gmac_sat;
