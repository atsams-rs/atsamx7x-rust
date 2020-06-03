#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - OHCI Interrupt Configuration Register"]
    pub utmi_ohciicr: UTMI_OHCIICR,
    _reserved1: [u8; 28usize],
    #[doc = "0x30 - UTMI Clock Trimming Register"]
    pub utmi_cktrim: UTMI_CKTRIM,
}
#[doc = "OHCI Interrupt Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [utmi_ohciicr](utmi_ohciicr) module"]
pub type UTMI_OHCIICR = crate::Reg<u32, _UTMI_OHCIICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UTMI_OHCIICR;
#[doc = "`read()` method returns [utmi_ohciicr::R](utmi_ohciicr::R) reader structure"]
impl crate::Readable for UTMI_OHCIICR {}
#[doc = "`write(|w| ..)` method takes [utmi_ohciicr::W](utmi_ohciicr::W) writer structure"]
impl crate::Writable for UTMI_OHCIICR {}
#[doc = "OHCI Interrupt Configuration Register"]
pub mod utmi_ohciicr;
#[doc = "UTMI Clock Trimming Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [utmi_cktrim](utmi_cktrim) module"]
pub type UTMI_CKTRIM = crate::Reg<u32, _UTMI_CKTRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UTMI_CKTRIM;
#[doc = "`read()` method returns [utmi_cktrim::R](utmi_cktrim::R) reader structure"]
impl crate::Readable for UTMI_CKTRIM {}
#[doc = "`write(|w| ..)` method takes [utmi_cktrim::W](utmi_cktrim::W) writer structure"]
impl crate::Writable for UTMI_CKTRIM {}
#[doc = "UTMI Clock Trimming Register"]
pub mod utmi_cktrim;
