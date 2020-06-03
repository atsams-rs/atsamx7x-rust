#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Supply Controller Control Register"]
    pub supc_cr: SUPC_CR,
    #[doc = "0x04 - Supply Controller Supply Monitor Mode Register"]
    pub supc_smmr: SUPC_SMMR,
    #[doc = "0x08 - Supply Controller Mode Register"]
    pub supc_mr: SUPC_MR,
    #[doc = "0x0c - Supply Controller Wake-up Mode Register"]
    pub supc_wumr: SUPC_WUMR,
    #[doc = "0x10 - Supply Controller Wake-up Inputs Register"]
    pub supc_wuir: SUPC_WUIR,
    #[doc = "0x14 - Supply Controller Status Register"]
    pub supc_sr: SUPC_SR,
}
#[doc = "Supply Controller Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [supc_cr](supc_cr) module"]
pub type SUPC_CR = crate::Reg<u32, _SUPC_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUPC_CR;
#[doc = "`write(|w| ..)` method takes [supc_cr::W](supc_cr::W) writer structure"]
impl crate::Writable for SUPC_CR {}
#[doc = "Supply Controller Control Register"]
pub mod supc_cr;
#[doc = "Supply Controller Supply Monitor Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [supc_smmr](supc_smmr) module"]
pub type SUPC_SMMR = crate::Reg<u32, _SUPC_SMMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUPC_SMMR;
#[doc = "`read()` method returns [supc_smmr::R](supc_smmr::R) reader structure"]
impl crate::Readable for SUPC_SMMR {}
#[doc = "`write(|w| ..)` method takes [supc_smmr::W](supc_smmr::W) writer structure"]
impl crate::Writable for SUPC_SMMR {}
#[doc = "Supply Controller Supply Monitor Mode Register"]
pub mod supc_smmr;
#[doc = "Supply Controller Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [supc_mr](supc_mr) module"]
pub type SUPC_MR = crate::Reg<u32, _SUPC_MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUPC_MR;
#[doc = "`read()` method returns [supc_mr::R](supc_mr::R) reader structure"]
impl crate::Readable for SUPC_MR {}
#[doc = "`write(|w| ..)` method takes [supc_mr::W](supc_mr::W) writer structure"]
impl crate::Writable for SUPC_MR {}
#[doc = "Supply Controller Mode Register"]
pub mod supc_mr;
#[doc = "Supply Controller Wake-up Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [supc_wumr](supc_wumr) module"]
pub type SUPC_WUMR = crate::Reg<u32, _SUPC_WUMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUPC_WUMR;
#[doc = "`read()` method returns [supc_wumr::R](supc_wumr::R) reader structure"]
impl crate::Readable for SUPC_WUMR {}
#[doc = "`write(|w| ..)` method takes [supc_wumr::W](supc_wumr::W) writer structure"]
impl crate::Writable for SUPC_WUMR {}
#[doc = "Supply Controller Wake-up Mode Register"]
pub mod supc_wumr;
#[doc = "Supply Controller Wake-up Inputs Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [supc_wuir](supc_wuir) module"]
pub type SUPC_WUIR = crate::Reg<u32, _SUPC_WUIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUPC_WUIR;
#[doc = "`read()` method returns [supc_wuir::R](supc_wuir::R) reader structure"]
impl crate::Readable for SUPC_WUIR {}
#[doc = "`write(|w| ..)` method takes [supc_wuir::W](supc_wuir::W) writer structure"]
impl crate::Writable for SUPC_WUIR {}
#[doc = "Supply Controller Wake-up Inputs Register"]
pub mod supc_wuir;
#[doc = "Supply Controller Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [supc_sr](supc_sr) module"]
pub type SUPC_SR = crate::Reg<u32, _SUPC_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUPC_SR;
#[doc = "`read()` method returns [supc_sr::R](supc_sr::R) reader structure"]
impl crate::Readable for SUPC_SR {}
#[doc = "Supply Controller Status Register"]
pub mod supc_sr;
