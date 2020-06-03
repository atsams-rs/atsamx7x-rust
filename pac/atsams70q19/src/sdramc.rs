#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDRAMC Mode Register"]
    pub sdramc_mr: SDRAMC_MR,
    #[doc = "0x04 - SDRAMC Refresh Timer Register"]
    pub sdramc_tr: SDRAMC_TR,
    #[doc = "0x08 - SDRAMC Configuration Register"]
    pub sdramc_cr: SDRAMC_CR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - SDRAMC Low Power Register"]
    pub sdramc_lpr: SDRAMC_LPR,
    #[doc = "0x14 - SDRAMC Interrupt Enable Register"]
    pub sdramc_ier: SDRAMC_IER,
    #[doc = "0x18 - SDRAMC Interrupt Disable Register"]
    pub sdramc_idr: SDRAMC_IDR,
    #[doc = "0x1c - SDRAMC Interrupt Mask Register"]
    pub sdramc_imr: SDRAMC_IMR,
    #[doc = "0x20 - SDRAMC Interrupt Status Register"]
    pub sdramc_isr: SDRAMC_ISR,
    #[doc = "0x24 - SDRAMC Memory Device Register"]
    pub sdramc_mdr: SDRAMC_MDR,
    #[doc = "0x28 - SDRAMC Configuration Register 1"]
    pub sdramc_cfr1: SDRAMC_CFR1,
    #[doc = "0x2c - SDRAMC OCMS Register"]
    pub sdramc_ocms: SDRAMC_OCMS,
    #[doc = "0x30 - SDRAMC OCMS KEY1 Register"]
    pub sdramc_ocms_key1: SDRAMC_OCMS_KEY1,
    #[doc = "0x34 - SDRAMC OCMS KEY2 Register"]
    pub sdramc_ocms_key2: SDRAMC_OCMS_KEY2,
}
#[doc = "SDRAMC Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramc_mr](sdramc_mr) module"]
pub type SDRAMC_MR = crate::Reg<u32, _SDRAMC_MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRAMC_MR;
#[doc = "`read()` method returns [sdramc_mr::R](sdramc_mr::R) reader structure"]
impl crate::Readable for SDRAMC_MR {}
#[doc = "`write(|w| ..)` method takes [sdramc_mr::W](sdramc_mr::W) writer structure"]
impl crate::Writable for SDRAMC_MR {}
#[doc = "SDRAMC Mode Register"]
pub mod sdramc_mr;
#[doc = "SDRAMC Refresh Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramc_tr](sdramc_tr) module"]
pub type SDRAMC_TR = crate::Reg<u32, _SDRAMC_TR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRAMC_TR;
#[doc = "`read()` method returns [sdramc_tr::R](sdramc_tr::R) reader structure"]
impl crate::Readable for SDRAMC_TR {}
#[doc = "`write(|w| ..)` method takes [sdramc_tr::W](sdramc_tr::W) writer structure"]
impl crate::Writable for SDRAMC_TR {}
#[doc = "SDRAMC Refresh Timer Register"]
pub mod sdramc_tr;
#[doc = "SDRAMC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramc_cr](sdramc_cr) module"]
pub type SDRAMC_CR = crate::Reg<u32, _SDRAMC_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRAMC_CR;
#[doc = "`read()` method returns [sdramc_cr::R](sdramc_cr::R) reader structure"]
impl crate::Readable for SDRAMC_CR {}
#[doc = "`write(|w| ..)` method takes [sdramc_cr::W](sdramc_cr::W) writer structure"]
impl crate::Writable for SDRAMC_CR {}
#[doc = "SDRAMC Configuration Register"]
pub mod sdramc_cr;
#[doc = "SDRAMC Low Power Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramc_lpr](sdramc_lpr) module"]
pub type SDRAMC_LPR = crate::Reg<u32, _SDRAMC_LPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRAMC_LPR;
#[doc = "`read()` method returns [sdramc_lpr::R](sdramc_lpr::R) reader structure"]
impl crate::Readable for SDRAMC_LPR {}
#[doc = "`write(|w| ..)` method takes [sdramc_lpr::W](sdramc_lpr::W) writer structure"]
impl crate::Writable for SDRAMC_LPR {}
#[doc = "SDRAMC Low Power Register"]
pub mod sdramc_lpr;
#[doc = "SDRAMC Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramc_ier](sdramc_ier) module"]
pub type SDRAMC_IER = crate::Reg<u32, _SDRAMC_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRAMC_IER;
#[doc = "`write(|w| ..)` method takes [sdramc_ier::W](sdramc_ier::W) writer structure"]
impl crate::Writable for SDRAMC_IER {}
#[doc = "SDRAMC Interrupt Enable Register"]
pub mod sdramc_ier;
#[doc = "SDRAMC Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramc_idr](sdramc_idr) module"]
pub type SDRAMC_IDR = crate::Reg<u32, _SDRAMC_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRAMC_IDR;
#[doc = "`write(|w| ..)` method takes [sdramc_idr::W](sdramc_idr::W) writer structure"]
impl crate::Writable for SDRAMC_IDR {}
#[doc = "SDRAMC Interrupt Disable Register"]
pub mod sdramc_idr;
#[doc = "SDRAMC Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramc_imr](sdramc_imr) module"]
pub type SDRAMC_IMR = crate::Reg<u32, _SDRAMC_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRAMC_IMR;
#[doc = "`read()` method returns [sdramc_imr::R](sdramc_imr::R) reader structure"]
impl crate::Readable for SDRAMC_IMR {}
#[doc = "SDRAMC Interrupt Mask Register"]
pub mod sdramc_imr;
#[doc = "SDRAMC Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramc_isr](sdramc_isr) module"]
pub type SDRAMC_ISR = crate::Reg<u32, _SDRAMC_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRAMC_ISR;
#[doc = "`read()` method returns [sdramc_isr::R](sdramc_isr::R) reader structure"]
impl crate::Readable for SDRAMC_ISR {}
#[doc = "SDRAMC Interrupt Status Register"]
pub mod sdramc_isr;
#[doc = "SDRAMC Memory Device Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramc_mdr](sdramc_mdr) module"]
pub type SDRAMC_MDR = crate::Reg<u32, _SDRAMC_MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRAMC_MDR;
#[doc = "`read()` method returns [sdramc_mdr::R](sdramc_mdr::R) reader structure"]
impl crate::Readable for SDRAMC_MDR {}
#[doc = "`write(|w| ..)` method takes [sdramc_mdr::W](sdramc_mdr::W) writer structure"]
impl crate::Writable for SDRAMC_MDR {}
#[doc = "SDRAMC Memory Device Register"]
pub mod sdramc_mdr;
#[doc = "SDRAMC Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramc_cfr1](sdramc_cfr1) module"]
pub type SDRAMC_CFR1 = crate::Reg<u32, _SDRAMC_CFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRAMC_CFR1;
#[doc = "`read()` method returns [sdramc_cfr1::R](sdramc_cfr1::R) reader structure"]
impl crate::Readable for SDRAMC_CFR1 {}
#[doc = "`write(|w| ..)` method takes [sdramc_cfr1::W](sdramc_cfr1::W) writer structure"]
impl crate::Writable for SDRAMC_CFR1 {}
#[doc = "SDRAMC Configuration Register 1"]
pub mod sdramc_cfr1;
#[doc = "SDRAMC OCMS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramc_ocms](sdramc_ocms) module"]
pub type SDRAMC_OCMS = crate::Reg<u32, _SDRAMC_OCMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRAMC_OCMS;
#[doc = "`read()` method returns [sdramc_ocms::R](sdramc_ocms::R) reader structure"]
impl crate::Readable for SDRAMC_OCMS {}
#[doc = "`write(|w| ..)` method takes [sdramc_ocms::W](sdramc_ocms::W) writer structure"]
impl crate::Writable for SDRAMC_OCMS {}
#[doc = "SDRAMC OCMS Register"]
pub mod sdramc_ocms;
#[doc = "SDRAMC OCMS KEY1 Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramc_ocms_key1](sdramc_ocms_key1) module"]
pub type SDRAMC_OCMS_KEY1 = crate::Reg<u32, _SDRAMC_OCMS_KEY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRAMC_OCMS_KEY1;
#[doc = "`write(|w| ..)` method takes [sdramc_ocms_key1::W](sdramc_ocms_key1::W) writer structure"]
impl crate::Writable for SDRAMC_OCMS_KEY1 {}
#[doc = "SDRAMC OCMS KEY1 Register"]
pub mod sdramc_ocms_key1;
#[doc = "SDRAMC OCMS KEY2 Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdramc_ocms_key2](sdramc_ocms_key2) module"]
pub type SDRAMC_OCMS_KEY2 = crate::Reg<u32, _SDRAMC_OCMS_KEY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDRAMC_OCMS_KEY2;
#[doc = "`write(|w| ..)` method takes [sdramc_ocms_key2::W](sdramc_ocms_key2::W) writer structure"]
impl crate::Writable for SDRAMC_OCMS_KEY2 {}
#[doc = "SDRAMC OCMS KEY2 Register"]
pub mod sdramc_ocms_key2;
