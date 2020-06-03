#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub rswdt_cr: RSWDT_CR,
    #[doc = "0x04 - Mode Register"]
    pub rswdt_mr: RSWDT_MR,
    #[doc = "0x08 - Status Register"]
    pub rswdt_sr: RSWDT_SR,
}
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rswdt_cr](rswdt_cr) module"]
pub type RSWDT_CR = crate::Reg<u32, _RSWDT_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSWDT_CR;
#[doc = "`write(|w| ..)` method takes [rswdt_cr::W](rswdt_cr::W) writer structure"]
impl crate::Writable for RSWDT_CR {}
#[doc = "Control Register"]
pub mod rswdt_cr;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rswdt_mr](rswdt_mr) module"]
pub type RSWDT_MR = crate::Reg<u32, _RSWDT_MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSWDT_MR;
#[doc = "`read()` method returns [rswdt_mr::R](rswdt_mr::R) reader structure"]
impl crate::Readable for RSWDT_MR {}
#[doc = "`write(|w| ..)` method takes [rswdt_mr::W](rswdt_mr::W) writer structure"]
impl crate::Writable for RSWDT_MR {}
#[doc = "Mode Register"]
pub mod rswdt_mr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rswdt_sr](rswdt_sr) module"]
pub type RSWDT_SR = crate::Reg<u32, _RSWDT_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSWDT_SR;
#[doc = "`read()` method returns [rswdt_sr::R](rswdt_sr::R) reader structure"]
impl crate::Readable for RSWDT_SR {}
#[doc = "Status Register"]
pub mod rswdt_sr;
