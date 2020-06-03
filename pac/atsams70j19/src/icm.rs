#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration Register"]
    pub icm_cfg: ICM_CFG,
    #[doc = "0x04 - Control Register"]
    pub icm_ctrl: ICM_CTRL,
    #[doc = "0x08 - Status Register"]
    pub icm_sr: ICM_SR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Interrupt Enable Register"]
    pub icm_ier: ICM_IER,
    #[doc = "0x14 - Interrupt Disable Register"]
    pub icm_idr: ICM_IDR,
    #[doc = "0x18 - Interrupt Mask Register"]
    pub icm_imr: ICM_IMR,
    #[doc = "0x1c - Interrupt Status Register"]
    pub icm_isr: ICM_ISR,
    #[doc = "0x20 - Undefined Access Status Register"]
    pub icm_uasr: ICM_UASR,
    _reserved8: [u8; 12usize],
    #[doc = "0x30 - Region Descriptor Area Start Address Register"]
    pub icm_dscr: ICM_DSCR,
    #[doc = "0x34 - Region Hash Area Start Address Register"]
    pub icm_hash: ICM_HASH,
    #[doc = "0x38 - User Initial Hash Value 0 Register 0"]
    pub icm_uihval: [ICM_UIHVAL; 8],
}
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icm_cfg](icm_cfg) module"]
pub type ICM_CFG = crate::Reg<u32, _ICM_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICM_CFG;
#[doc = "`read()` method returns [icm_cfg::R](icm_cfg::R) reader structure"]
impl crate::Readable for ICM_CFG {}
#[doc = "`write(|w| ..)` method takes [icm_cfg::W](icm_cfg::W) writer structure"]
impl crate::Writable for ICM_CFG {}
#[doc = "Configuration Register"]
pub mod icm_cfg;
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icm_ctrl](icm_ctrl) module"]
pub type ICM_CTRL = crate::Reg<u32, _ICM_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICM_CTRL;
#[doc = "`write(|w| ..)` method takes [icm_ctrl::W](icm_ctrl::W) writer structure"]
impl crate::Writable for ICM_CTRL {}
#[doc = "Control Register"]
pub mod icm_ctrl;
#[doc = "Status Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icm_sr](icm_sr) module"]
pub type ICM_SR = crate::Reg<u32, _ICM_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICM_SR;
#[doc = "`write(|w| ..)` method takes [icm_sr::W](icm_sr::W) writer structure"]
impl crate::Writable for ICM_SR {}
#[doc = "Status Register"]
pub mod icm_sr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icm_ier](icm_ier) module"]
pub type ICM_IER = crate::Reg<u32, _ICM_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICM_IER;
#[doc = "`write(|w| ..)` method takes [icm_ier::W](icm_ier::W) writer structure"]
impl crate::Writable for ICM_IER {}
#[doc = "Interrupt Enable Register"]
pub mod icm_ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icm_idr](icm_idr) module"]
pub type ICM_IDR = crate::Reg<u32, _ICM_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICM_IDR;
#[doc = "`write(|w| ..)` method takes [icm_idr::W](icm_idr::W) writer structure"]
impl crate::Writable for ICM_IDR {}
#[doc = "Interrupt Disable Register"]
pub mod icm_idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icm_imr](icm_imr) module"]
pub type ICM_IMR = crate::Reg<u32, _ICM_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICM_IMR;
#[doc = "`read()` method returns [icm_imr::R](icm_imr::R) reader structure"]
impl crate::Readable for ICM_IMR {}
#[doc = "Interrupt Mask Register"]
pub mod icm_imr;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icm_isr](icm_isr) module"]
pub type ICM_ISR = crate::Reg<u32, _ICM_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICM_ISR;
#[doc = "`read()` method returns [icm_isr::R](icm_isr::R) reader structure"]
impl crate::Readable for ICM_ISR {}
#[doc = "Interrupt Status Register"]
pub mod icm_isr;
#[doc = "Undefined Access Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icm_uasr](icm_uasr) module"]
pub type ICM_UASR = crate::Reg<u32, _ICM_UASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICM_UASR;
#[doc = "`read()` method returns [icm_uasr::R](icm_uasr::R) reader structure"]
impl crate::Readable for ICM_UASR {}
#[doc = "Undefined Access Status Register"]
pub mod icm_uasr;
#[doc = "Region Descriptor Area Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icm_dscr](icm_dscr) module"]
pub type ICM_DSCR = crate::Reg<u32, _ICM_DSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICM_DSCR;
#[doc = "`read()` method returns [icm_dscr::R](icm_dscr::R) reader structure"]
impl crate::Readable for ICM_DSCR {}
#[doc = "`write(|w| ..)` method takes [icm_dscr::W](icm_dscr::W) writer structure"]
impl crate::Writable for ICM_DSCR {}
#[doc = "Region Descriptor Area Start Address Register"]
pub mod icm_dscr;
#[doc = "Region Hash Area Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icm_hash](icm_hash) module"]
pub type ICM_HASH = crate::Reg<u32, _ICM_HASH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICM_HASH;
#[doc = "`read()` method returns [icm_hash::R](icm_hash::R) reader structure"]
impl crate::Readable for ICM_HASH {}
#[doc = "`write(|w| ..)` method takes [icm_hash::W](icm_hash::W) writer structure"]
impl crate::Writable for ICM_HASH {}
#[doc = "Region Hash Area Start Address Register"]
pub mod icm_hash;
#[doc = "User Initial Hash Value 0 Register 0\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icm_uihval](icm_uihval) module"]
pub type ICM_UIHVAL = crate::Reg<u32, _ICM_UIHVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICM_UIHVAL;
#[doc = "`write(|w| ..)` method takes [icm_uihval::W](icm_uihval::W) writer structure"]
impl crate::Writable for ICM_UIHVAL {}
#[doc = "User Initial Hash Value 0 Register 0"]
pub mod icm_uihval;
