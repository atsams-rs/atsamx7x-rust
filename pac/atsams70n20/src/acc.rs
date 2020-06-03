#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub acc_cr: ACC_CR,
    #[doc = "0x04 - Mode Register"]
    pub acc_mr: ACC_MR,
    _reserved2: [u8; 28usize],
    #[doc = "0x24 - Interrupt Enable Register"]
    pub acc_ier: ACC_IER,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub acc_idr: ACC_IDR,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub acc_imr: ACC_IMR,
    #[doc = "0x30 - Interrupt Status Register"]
    pub acc_isr: ACC_ISR,
    _reserved6: [u8; 96usize],
    #[doc = "0x94 - Analog Control Register"]
    pub acc_acr: ACC_ACR,
    _reserved7: [u8; 76usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub acc_wpmr: ACC_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub acc_wpsr: ACC_WPSR,
}
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc_cr](acc_cr) module"]
pub type ACC_CR = crate::Reg<u32, _ACC_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC_CR;
#[doc = "`write(|w| ..)` method takes [acc_cr::W](acc_cr::W) writer structure"]
impl crate::Writable for ACC_CR {}
#[doc = "Control Register"]
pub mod acc_cr;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc_mr](acc_mr) module"]
pub type ACC_MR = crate::Reg<u32, _ACC_MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC_MR;
#[doc = "`read()` method returns [acc_mr::R](acc_mr::R) reader structure"]
impl crate::Readable for ACC_MR {}
#[doc = "`write(|w| ..)` method takes [acc_mr::W](acc_mr::W) writer structure"]
impl crate::Writable for ACC_MR {}
#[doc = "Mode Register"]
pub mod acc_mr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc_ier](acc_ier) module"]
pub type ACC_IER = crate::Reg<u32, _ACC_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC_IER;
#[doc = "`write(|w| ..)` method takes [acc_ier::W](acc_ier::W) writer structure"]
impl crate::Writable for ACC_IER {}
#[doc = "Interrupt Enable Register"]
pub mod acc_ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc_idr](acc_idr) module"]
pub type ACC_IDR = crate::Reg<u32, _ACC_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC_IDR;
#[doc = "`write(|w| ..)` method takes [acc_idr::W](acc_idr::W) writer structure"]
impl crate::Writable for ACC_IDR {}
#[doc = "Interrupt Disable Register"]
pub mod acc_idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc_imr](acc_imr) module"]
pub type ACC_IMR = crate::Reg<u32, _ACC_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC_IMR;
#[doc = "`read()` method returns [acc_imr::R](acc_imr::R) reader structure"]
impl crate::Readable for ACC_IMR {}
#[doc = "Interrupt Mask Register"]
pub mod acc_imr;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc_isr](acc_isr) module"]
pub type ACC_ISR = crate::Reg<u32, _ACC_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC_ISR;
#[doc = "`read()` method returns [acc_isr::R](acc_isr::R) reader structure"]
impl crate::Readable for ACC_ISR {}
#[doc = "Interrupt Status Register"]
pub mod acc_isr;
#[doc = "Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc_acr](acc_acr) module"]
pub type ACC_ACR = crate::Reg<u32, _ACC_ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC_ACR;
#[doc = "`read()` method returns [acc_acr::R](acc_acr::R) reader structure"]
impl crate::Readable for ACC_ACR {}
#[doc = "`write(|w| ..)` method takes [acc_acr::W](acc_acr::W) writer structure"]
impl crate::Writable for ACC_ACR {}
#[doc = "Analog Control Register"]
pub mod acc_acr;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc_wpmr](acc_wpmr) module"]
pub type ACC_WPMR = crate::Reg<u32, _ACC_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC_WPMR;
#[doc = "`read()` method returns [acc_wpmr::R](acc_wpmr::R) reader structure"]
impl crate::Readable for ACC_WPMR {}
#[doc = "`write(|w| ..)` method takes [acc_wpmr::W](acc_wpmr::W) writer structure"]
impl crate::Writable for ACC_WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod acc_wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc_wpsr](acc_wpsr) module"]
pub type ACC_WPSR = crate::Reg<u32, _ACC_WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC_WPSR;
#[doc = "`read()` method returns [acc_wpsr::R](acc_wpsr::R) reader structure"]
impl crate::Readable for ACC_WPSR {}
#[doc = "Write Protection Status Register"]
pub mod acc_wpsr;
