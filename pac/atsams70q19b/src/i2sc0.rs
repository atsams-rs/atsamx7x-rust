#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub i2sc_cr: I2SC_CR,
    #[doc = "0x04 - Mode Register"]
    pub i2sc_mr: I2SC_MR,
    #[doc = "0x08 - Status Register"]
    pub i2sc_sr: I2SC_SR,
    #[doc = "0x0c - Status Clear Register"]
    pub i2sc_scr: I2SC_SCR,
    #[doc = "0x10 - Status Set Register"]
    pub i2sc_ssr: I2SC_SSR,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub i2sc_ier: I2SC_IER,
    #[doc = "0x18 - Interrupt Disable Register"]
    pub i2sc_idr: I2SC_IDR,
    #[doc = "0x1c - Interrupt Mask Register"]
    pub i2sc_imr: I2SC_IMR,
    #[doc = "0x20 - Receiver Holding Register"]
    pub i2sc_rhr: I2SC_RHR,
    #[doc = "0x24 - Transmitter Holding Register"]
    pub i2sc_thr: I2SC_THR,
}
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sc_cr](i2sc_cr) module"]
pub type I2SC_CR = crate::Reg<u32, _I2SC_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SC_CR;
#[doc = "`write(|w| ..)` method takes [i2sc_cr::W](i2sc_cr::W) writer structure"]
impl crate::Writable for I2SC_CR {}
#[doc = "Control Register"]
pub mod i2sc_cr;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sc_mr](i2sc_mr) module"]
pub type I2SC_MR = crate::Reg<u32, _I2SC_MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SC_MR;
#[doc = "`read()` method returns [i2sc_mr::R](i2sc_mr::R) reader structure"]
impl crate::Readable for I2SC_MR {}
#[doc = "`write(|w| ..)` method takes [i2sc_mr::W](i2sc_mr::W) writer structure"]
impl crate::Writable for I2SC_MR {}
#[doc = "Mode Register"]
pub mod i2sc_mr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sc_sr](i2sc_sr) module"]
pub type I2SC_SR = crate::Reg<u32, _I2SC_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SC_SR;
#[doc = "`read()` method returns [i2sc_sr::R](i2sc_sr::R) reader structure"]
impl crate::Readable for I2SC_SR {}
#[doc = "Status Register"]
pub mod i2sc_sr;
#[doc = "Status Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sc_scr](i2sc_scr) module"]
pub type I2SC_SCR = crate::Reg<u32, _I2SC_SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SC_SCR;
#[doc = "`write(|w| ..)` method takes [i2sc_scr::W](i2sc_scr::W) writer structure"]
impl crate::Writable for I2SC_SCR {}
#[doc = "Status Clear Register"]
pub mod i2sc_scr;
#[doc = "Status Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sc_ssr](i2sc_ssr) module"]
pub type I2SC_SSR = crate::Reg<u32, _I2SC_SSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SC_SSR;
#[doc = "`write(|w| ..)` method takes [i2sc_ssr::W](i2sc_ssr::W) writer structure"]
impl crate::Writable for I2SC_SSR {}
#[doc = "Status Set Register"]
pub mod i2sc_ssr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sc_ier](i2sc_ier) module"]
pub type I2SC_IER = crate::Reg<u32, _I2SC_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SC_IER;
#[doc = "`write(|w| ..)` method takes [i2sc_ier::W](i2sc_ier::W) writer structure"]
impl crate::Writable for I2SC_IER {}
#[doc = "Interrupt Enable Register"]
pub mod i2sc_ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sc_idr](i2sc_idr) module"]
pub type I2SC_IDR = crate::Reg<u32, _I2SC_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SC_IDR;
#[doc = "`write(|w| ..)` method takes [i2sc_idr::W](i2sc_idr::W) writer structure"]
impl crate::Writable for I2SC_IDR {}
#[doc = "Interrupt Disable Register"]
pub mod i2sc_idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sc_imr](i2sc_imr) module"]
pub type I2SC_IMR = crate::Reg<u32, _I2SC_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SC_IMR;
#[doc = "`read()` method returns [i2sc_imr::R](i2sc_imr::R) reader structure"]
impl crate::Readable for I2SC_IMR {}
#[doc = "Interrupt Mask Register"]
pub mod i2sc_imr;
#[doc = "Receiver Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sc_rhr](i2sc_rhr) module"]
pub type I2SC_RHR = crate::Reg<u32, _I2SC_RHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SC_RHR;
#[doc = "`read()` method returns [i2sc_rhr::R](i2sc_rhr::R) reader structure"]
impl crate::Readable for I2SC_RHR {}
#[doc = "Receiver Holding Register"]
pub mod i2sc_rhr;
#[doc = "Transmitter Holding Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sc_thr](i2sc_thr) module"]
pub type I2SC_THR = crate::Reg<u32, _I2SC_THR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2SC_THR;
#[doc = "`write(|w| ..)` method takes [i2sc_thr::W](i2sc_thr::W) writer structure"]
impl crate::Writable for I2SC_THR {}
#[doc = "Transmitter Holding Register"]
pub mod i2sc_thr;
