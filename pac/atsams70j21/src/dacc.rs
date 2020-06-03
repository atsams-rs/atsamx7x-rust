#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub dacc_cr: DACC_CR,
    #[doc = "0x04 - Mode Register"]
    pub dacc_mr: DACC_MR,
    #[doc = "0x08 - Trigger Register"]
    pub dacc_trigr: DACC_TRIGR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Channel Enable Register"]
    pub dacc_cher: DACC_CHER,
    #[doc = "0x14 - Channel Disable Register"]
    pub dacc_chdr: DACC_CHDR,
    #[doc = "0x18 - Channel Status Register"]
    pub dacc_chsr: DACC_CHSR,
    #[doc = "0x1c - Conversion Data Register 0"]
    pub dacc_cdr: [DACC_CDR; 2],
    #[doc = "0x24 - Interrupt Enable Register"]
    pub dacc_ier: DACC_IER,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub dacc_idr: DACC_IDR,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub dacc_imr: DACC_IMR,
    #[doc = "0x30 - Interrupt Status Register"]
    pub dacc_isr: DACC_ISR,
    _reserved11: [u8; 96usize],
    #[doc = "0x94 - Analog Current Register"]
    pub dacc_acr: DACC_ACR,
    _reserved12: [u8; 76usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub dacc_wpmr: DACC_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub dacc_wpsr: DACC_WPSR,
}
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_cr](dacc_cr) module"]
pub type DACC_CR = crate::Reg<u32, _DACC_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACC_CR;
#[doc = "`write(|w| ..)` method takes [dacc_cr::W](dacc_cr::W) writer structure"]
impl crate::Writable for DACC_CR {}
#[doc = "Control Register"]
pub mod dacc_cr;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_mr](dacc_mr) module"]
pub type DACC_MR = crate::Reg<u32, _DACC_MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACC_MR;
#[doc = "`read()` method returns [dacc_mr::R](dacc_mr::R) reader structure"]
impl crate::Readable for DACC_MR {}
#[doc = "`write(|w| ..)` method takes [dacc_mr::W](dacc_mr::W) writer structure"]
impl crate::Writable for DACC_MR {}
#[doc = "Mode Register"]
pub mod dacc_mr;
#[doc = "Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_trigr](dacc_trigr) module"]
pub type DACC_TRIGR = crate::Reg<u32, _DACC_TRIGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACC_TRIGR;
#[doc = "`read()` method returns [dacc_trigr::R](dacc_trigr::R) reader structure"]
impl crate::Readable for DACC_TRIGR {}
#[doc = "`write(|w| ..)` method takes [dacc_trigr::W](dacc_trigr::W) writer structure"]
impl crate::Writable for DACC_TRIGR {}
#[doc = "Trigger Register"]
pub mod dacc_trigr;
#[doc = "Channel Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_cher](dacc_cher) module"]
pub type DACC_CHER = crate::Reg<u32, _DACC_CHER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACC_CHER;
#[doc = "`write(|w| ..)` method takes [dacc_cher::W](dacc_cher::W) writer structure"]
impl crate::Writable for DACC_CHER {}
#[doc = "Channel Enable Register"]
pub mod dacc_cher;
#[doc = "Channel Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_chdr](dacc_chdr) module"]
pub type DACC_CHDR = crate::Reg<u32, _DACC_CHDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACC_CHDR;
#[doc = "`write(|w| ..)` method takes [dacc_chdr::W](dacc_chdr::W) writer structure"]
impl crate::Writable for DACC_CHDR {}
#[doc = "Channel Disable Register"]
pub mod dacc_chdr;
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_chsr](dacc_chsr) module"]
pub type DACC_CHSR = crate::Reg<u32, _DACC_CHSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACC_CHSR;
#[doc = "`read()` method returns [dacc_chsr::R](dacc_chsr::R) reader structure"]
impl crate::Readable for DACC_CHSR {}
#[doc = "Channel Status Register"]
pub mod dacc_chsr;
#[doc = "Conversion Data Register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_cdr](dacc_cdr) module"]
pub type DACC_CDR = crate::Reg<u32, _DACC_CDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACC_CDR;
#[doc = "`write(|w| ..)` method takes [dacc_cdr::W](dacc_cdr::W) writer structure"]
impl crate::Writable for DACC_CDR {}
#[doc = "Conversion Data Register 0"]
pub mod dacc_cdr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_ier](dacc_ier) module"]
pub type DACC_IER = crate::Reg<u32, _DACC_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACC_IER;
#[doc = "`write(|w| ..)` method takes [dacc_ier::W](dacc_ier::W) writer structure"]
impl crate::Writable for DACC_IER {}
#[doc = "Interrupt Enable Register"]
pub mod dacc_ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_idr](dacc_idr) module"]
pub type DACC_IDR = crate::Reg<u32, _DACC_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACC_IDR;
#[doc = "`write(|w| ..)` method takes [dacc_idr::W](dacc_idr::W) writer structure"]
impl crate::Writable for DACC_IDR {}
#[doc = "Interrupt Disable Register"]
pub mod dacc_idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_imr](dacc_imr) module"]
pub type DACC_IMR = crate::Reg<u32, _DACC_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACC_IMR;
#[doc = "`read()` method returns [dacc_imr::R](dacc_imr::R) reader structure"]
impl crate::Readable for DACC_IMR {}
#[doc = "Interrupt Mask Register"]
pub mod dacc_imr;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_isr](dacc_isr) module"]
pub type DACC_ISR = crate::Reg<u32, _DACC_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACC_ISR;
#[doc = "`read()` method returns [dacc_isr::R](dacc_isr::R) reader structure"]
impl crate::Readable for DACC_ISR {}
#[doc = "Interrupt Status Register"]
pub mod dacc_isr;
#[doc = "Analog Current Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_acr](dacc_acr) module"]
pub type DACC_ACR = crate::Reg<u32, _DACC_ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACC_ACR;
#[doc = "`read()` method returns [dacc_acr::R](dacc_acr::R) reader structure"]
impl crate::Readable for DACC_ACR {}
#[doc = "`write(|w| ..)` method takes [dacc_acr::W](dacc_acr::W) writer structure"]
impl crate::Writable for DACC_ACR {}
#[doc = "Analog Current Register"]
pub mod dacc_acr;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_wpmr](dacc_wpmr) module"]
pub type DACC_WPMR = crate::Reg<u32, _DACC_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACC_WPMR;
#[doc = "`read()` method returns [dacc_wpmr::R](dacc_wpmr::R) reader structure"]
impl crate::Readable for DACC_WPMR {}
#[doc = "`write(|w| ..)` method takes [dacc_wpmr::W](dacc_wpmr::W) writer structure"]
impl crate::Writable for DACC_WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod dacc_wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacc_wpsr](dacc_wpsr) module"]
pub type DACC_WPSR = crate::Reg<u32, _DACC_WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DACC_WPSR;
#[doc = "`read()` method returns [dacc_wpsr::R](dacc_wpsr::R) reader structure"]
impl crate::Readable for DACC_WPSR {}
#[doc = "Write Protection Status Register"]
pub mod dacc_wpsr;
