#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub trng_cr: TRNG_CR,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Interrupt Enable Register"]
    pub trng_ier: TRNG_IER,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub trng_idr: TRNG_IDR,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub trng_imr: TRNG_IMR,
    #[doc = "0x1c - Interrupt Status Register"]
    pub trng_isr: TRNG_ISR,
    _reserved5: [u8; 48usize],
    #[doc = "0x50 - Output Data Register"]
    pub trng_odata: TRNG_ODATA,
}
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng_cr](trng_cr) module"]
pub type TRNG_CR = crate::Reg<u32, _TRNG_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG_CR;
#[doc = "`write(|w| ..)` method takes [trng_cr::W](trng_cr::W) writer structure"]
impl crate::Writable for TRNG_CR {}
#[doc = "Control Register"]
pub mod trng_cr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng_ier](trng_ier) module"]
pub type TRNG_IER = crate::Reg<u32, _TRNG_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG_IER;
#[doc = "`write(|w| ..)` method takes [trng_ier::W](trng_ier::W) writer structure"]
impl crate::Writable for TRNG_IER {}
#[doc = "Interrupt Enable Register"]
pub mod trng_ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng_idr](trng_idr) module"]
pub type TRNG_IDR = crate::Reg<u32, _TRNG_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG_IDR;
#[doc = "`write(|w| ..)` method takes [trng_idr::W](trng_idr::W) writer structure"]
impl crate::Writable for TRNG_IDR {}
#[doc = "Interrupt Disable Register"]
pub mod trng_idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng_imr](trng_imr) module"]
pub type TRNG_IMR = crate::Reg<u32, _TRNG_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG_IMR;
#[doc = "`read()` method returns [trng_imr::R](trng_imr::R) reader structure"]
impl crate::Readable for TRNG_IMR {}
#[doc = "Interrupt Mask Register"]
pub mod trng_imr;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng_isr](trng_isr) module"]
pub type TRNG_ISR = crate::Reg<u32, _TRNG_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG_ISR;
#[doc = "`read()` method returns [trng_isr::R](trng_isr::R) reader structure"]
impl crate::Readable for TRNG_ISR {}
#[doc = "Interrupt Status Register"]
pub mod trng_isr;
#[doc = "Output Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trng_odata](trng_odata) module"]
pub type TRNG_ODATA = crate::Reg<u32, _TRNG_ODATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRNG_ODATA;
#[doc = "`read()` method returns [trng_odata::R](trng_odata::R) reader structure"]
impl crate::Readable for TRNG_ODATA {}
#[doc = "Output Data Register"]
pub mod trng_odata;
