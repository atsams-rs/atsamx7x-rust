#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Interrupt Controller Type Register"]
    pub ictr: ICTR,
    #[doc = "0x08 - Auxiliary Control Register"]
    pub actlr: ACTLR,
}
#[doc = "Interrupt Controller Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ictr](ictr) module"]
pub type ICTR = crate::Reg<u32, _ICTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICTR;
#[doc = "`read()` method returns [ictr::R](ictr::R) reader structure"]
impl crate::Readable for ICTR {}
#[doc = "Interrupt Controller Type Register"]
pub mod ictr;
#[doc = "Auxiliary Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actlr](actlr) module"]
pub type ACTLR = crate::Reg<u32, _ACTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACTLR;
#[doc = "`read()` method returns [actlr::R](actlr::R) reader structure"]
impl crate::Readable for ACTLR {}
#[doc = "`write(|w| ..)` method takes [actlr::W](actlr::W) writer structure"]
impl crate::Writable for ACTLR {}
#[doc = "Auxiliary Control Register"]
pub mod actlr;
