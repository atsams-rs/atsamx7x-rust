#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub rstc_cr: RSTC_CR,
    #[doc = "0x04 - Status Register"]
    pub rstc_sr: RSTC_SR,
    #[doc = "0x08 - Mode Register"]
    pub rstc_mr: RSTC_MR,
}
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstc_cr](rstc_cr) module"]
pub type RSTC_CR = crate::Reg<u32, _RSTC_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTC_CR;
#[doc = "`write(|w| ..)` method takes [rstc_cr::W](rstc_cr::W) writer structure"]
impl crate::Writable for RSTC_CR {}
#[doc = "Control Register"]
pub mod rstc_cr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstc_sr](rstc_sr) module"]
pub type RSTC_SR = crate::Reg<u32, _RSTC_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTC_SR;
#[doc = "`read()` method returns [rstc_sr::R](rstc_sr::R) reader structure"]
impl crate::Readable for RSTC_SR {}
#[doc = "Status Register"]
pub mod rstc_sr;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstc_mr](rstc_mr) module"]
pub type RSTC_MR = crate::Reg<u32, _RSTC_MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTC_MR;
#[doc = "`read()` method returns [rstc_mr::R](rstc_mr::R) reader structure"]
impl crate::Readable for RSTC_MR {}
#[doc = "`write(|w| ..)` method takes [rstc_mr::W](rstc_mr::W) writer structure"]
impl crate::Writable for RSTC_MR {}
#[doc = "Mode Register"]
pub mod rstc_mr;
