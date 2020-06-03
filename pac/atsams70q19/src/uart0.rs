#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub uart_cr: UART_CR,
    #[doc = "0x04 - Mode Register"]
    pub uart_mr: UART_MR,
    #[doc = "0x08 - Interrupt Enable Register"]
    pub uart_ier: UART_IER,
    #[doc = "0x0c - Interrupt Disable Register"]
    pub uart_idr: UART_IDR,
    #[doc = "0x10 - Interrupt Mask Register"]
    pub uart_imr: UART_IMR,
    #[doc = "0x14 - Status Register"]
    pub uart_sr: UART_SR,
    #[doc = "0x18 - Receive Holding Register"]
    pub uart_rhr: UART_RHR,
    #[doc = "0x1c - Transmit Holding Register"]
    pub uart_thr: UART_THR,
    #[doc = "0x20 - Baud Rate Generator Register"]
    pub uart_brgr: UART_BRGR,
    #[doc = "0x24 - Comparison Register"]
    pub uart_cmpr: UART_CMPR,
    _reserved10: [u8; 188usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub uart_wpmr: UART_WPMR,
}
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_cr](uart_cr) module"]
pub type UART_CR = crate::Reg<u32, _UART_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_CR;
#[doc = "`write(|w| ..)` method takes [uart_cr::W](uart_cr::W) writer structure"]
impl crate::Writable for UART_CR {}
#[doc = "Control Register"]
pub mod uart_cr;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_mr](uart_mr) module"]
pub type UART_MR = crate::Reg<u32, _UART_MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_MR;
#[doc = "`read()` method returns [uart_mr::R](uart_mr::R) reader structure"]
impl crate::Readable for UART_MR {}
#[doc = "`write(|w| ..)` method takes [uart_mr::W](uart_mr::W) writer structure"]
impl crate::Writable for UART_MR {}
#[doc = "Mode Register"]
pub mod uart_mr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_ier](uart_ier) module"]
pub type UART_IER = crate::Reg<u32, _UART_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_IER;
#[doc = "`write(|w| ..)` method takes [uart_ier::W](uart_ier::W) writer structure"]
impl crate::Writable for UART_IER {}
#[doc = "Interrupt Enable Register"]
pub mod uart_ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_idr](uart_idr) module"]
pub type UART_IDR = crate::Reg<u32, _UART_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_IDR;
#[doc = "`write(|w| ..)` method takes [uart_idr::W](uart_idr::W) writer structure"]
impl crate::Writable for UART_IDR {}
#[doc = "Interrupt Disable Register"]
pub mod uart_idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_imr](uart_imr) module"]
pub type UART_IMR = crate::Reg<u32, _UART_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_IMR;
#[doc = "`read()` method returns [uart_imr::R](uart_imr::R) reader structure"]
impl crate::Readable for UART_IMR {}
#[doc = "Interrupt Mask Register"]
pub mod uart_imr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_sr](uart_sr) module"]
pub type UART_SR = crate::Reg<u32, _UART_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_SR;
#[doc = "`read()` method returns [uart_sr::R](uart_sr::R) reader structure"]
impl crate::Readable for UART_SR {}
#[doc = "Status Register"]
pub mod uart_sr;
#[doc = "Receive Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_rhr](uart_rhr) module"]
pub type UART_RHR = crate::Reg<u32, _UART_RHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_RHR;
#[doc = "`read()` method returns [uart_rhr::R](uart_rhr::R) reader structure"]
impl crate::Readable for UART_RHR {}
#[doc = "Receive Holding Register"]
pub mod uart_rhr;
#[doc = "Transmit Holding Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_thr](uart_thr) module"]
pub type UART_THR = crate::Reg<u32, _UART_THR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_THR;
#[doc = "`write(|w| ..)` method takes [uart_thr::W](uart_thr::W) writer structure"]
impl crate::Writable for UART_THR {}
#[doc = "Transmit Holding Register"]
pub mod uart_thr;
#[doc = "Baud Rate Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_brgr](uart_brgr) module"]
pub type UART_BRGR = crate::Reg<u32, _UART_BRGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_BRGR;
#[doc = "`read()` method returns [uart_brgr::R](uart_brgr::R) reader structure"]
impl crate::Readable for UART_BRGR {}
#[doc = "`write(|w| ..)` method takes [uart_brgr::W](uart_brgr::W) writer structure"]
impl crate::Writable for UART_BRGR {}
#[doc = "Baud Rate Generator Register"]
pub mod uart_brgr;
#[doc = "Comparison Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_cmpr](uart_cmpr) module"]
pub type UART_CMPR = crate::Reg<u32, _UART_CMPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_CMPR;
#[doc = "`read()` method returns [uart_cmpr::R](uart_cmpr::R) reader structure"]
impl crate::Readable for UART_CMPR {}
#[doc = "`write(|w| ..)` method takes [uart_cmpr::W](uart_cmpr::W) writer structure"]
impl crate::Writable for UART_CMPR {}
#[doc = "Comparison Register"]
pub mod uart_cmpr;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_wpmr](uart_wpmr) module"]
pub type UART_WPMR = crate::Reg<u32, _UART_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART_WPMR;
#[doc = "`read()` method returns [uart_wpmr::R](uart_wpmr::R) reader structure"]
impl crate::Readable for UART_WPMR {}
#[doc = "`write(|w| ..)` method takes [uart_wpmr::W](uart_wpmr::W) writer structure"]
impl crate::Writable for UART_WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod uart_wpmr;
