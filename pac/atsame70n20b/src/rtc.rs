#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub rtc_cr: RTC_CR,
    #[doc = "0x04 - Mode Register"]
    pub rtc_mr: RTC_MR,
    #[doc = "0x08 - Time Register"]
    pub rtc_timr: RTC_TIMR,
    #[doc = "0x0c - Calendar Register"]
    pub rtc_calr: RTC_CALR,
    #[doc = "0x10 - Time Alarm Register"]
    pub rtc_timalr: RTC_TIMALR,
    #[doc = "0x14 - Calendar Alarm Register"]
    pub rtc_calalr: RTC_CALALR,
    #[doc = "0x18 - Status Register"]
    pub rtc_sr: RTC_SR,
    #[doc = "0x1c - Status Clear Command Register"]
    pub rtc_sccr: RTC_SCCR,
    #[doc = "0x20 - Interrupt Enable Register"]
    pub rtc_ier: RTC_IER,
    #[doc = "0x24 - Interrupt Disable Register"]
    pub rtc_idr: RTC_IDR,
    #[doc = "0x28 - Interrupt Mask Register"]
    pub rtc_imr: RTC_IMR,
    #[doc = "0x2c - Valid Entry Register"]
    pub rtc_ver: RTC_VER,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cr](rtc_cr) module"]
pub type RTC_CR = crate::Reg<u32, _RTC_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CR;
#[doc = "`read()` method returns [rtc_cr::R](rtc_cr::R) reader structure"]
impl crate::Readable for RTC_CR {}
#[doc = "`write(|w| ..)` method takes [rtc_cr::W](rtc_cr::W) writer structure"]
impl crate::Writable for RTC_CR {}
#[doc = "Control Register"]
pub mod rtc_cr;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_mr](rtc_mr) module"]
pub type RTC_MR = crate::Reg<u32, _RTC_MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_MR;
#[doc = "`read()` method returns [rtc_mr::R](rtc_mr::R) reader structure"]
impl crate::Readable for RTC_MR {}
#[doc = "`write(|w| ..)` method takes [rtc_mr::W](rtc_mr::W) writer structure"]
impl crate::Writable for RTC_MR {}
#[doc = "Mode Register"]
pub mod rtc_mr;
#[doc = "Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_timr](rtc_timr) module"]
pub type RTC_TIMR = crate::Reg<u32, _RTC_TIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_TIMR;
#[doc = "`read()` method returns [rtc_timr::R](rtc_timr::R) reader structure"]
impl crate::Readable for RTC_TIMR {}
#[doc = "`write(|w| ..)` method takes [rtc_timr::W](rtc_timr::W) writer structure"]
impl crate::Writable for RTC_TIMR {}
#[doc = "Time Register"]
pub mod rtc_timr;
#[doc = "Calendar Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_calr](rtc_calr) module"]
pub type RTC_CALR = crate::Reg<u32, _RTC_CALR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CALR;
#[doc = "`read()` method returns [rtc_calr::R](rtc_calr::R) reader structure"]
impl crate::Readable for RTC_CALR {}
#[doc = "`write(|w| ..)` method takes [rtc_calr::W](rtc_calr::W) writer structure"]
impl crate::Writable for RTC_CALR {}
#[doc = "Calendar Register"]
pub mod rtc_calr;
#[doc = "Time Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_timalr](rtc_timalr) module"]
pub type RTC_TIMALR = crate::Reg<u32, _RTC_TIMALR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_TIMALR;
#[doc = "`read()` method returns [rtc_timalr::R](rtc_timalr::R) reader structure"]
impl crate::Readable for RTC_TIMALR {}
#[doc = "`write(|w| ..)` method takes [rtc_timalr::W](rtc_timalr::W) writer structure"]
impl crate::Writable for RTC_TIMALR {}
#[doc = "Time Alarm Register"]
pub mod rtc_timalr;
#[doc = "Calendar Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_calalr](rtc_calalr) module"]
pub type RTC_CALALR = crate::Reg<u32, _RTC_CALALR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CALALR;
#[doc = "`read()` method returns [rtc_calalr::R](rtc_calalr::R) reader structure"]
impl crate::Readable for RTC_CALALR {}
#[doc = "`write(|w| ..)` method takes [rtc_calalr::W](rtc_calalr::W) writer structure"]
impl crate::Writable for RTC_CALALR {}
#[doc = "Calendar Alarm Register"]
pub mod rtc_calalr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_sr](rtc_sr) module"]
pub type RTC_SR = crate::Reg<u32, _RTC_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_SR;
#[doc = "`read()` method returns [rtc_sr::R](rtc_sr::R) reader structure"]
impl crate::Readable for RTC_SR {}
#[doc = "Status Register"]
pub mod rtc_sr;
#[doc = "Status Clear Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_sccr](rtc_sccr) module"]
pub type RTC_SCCR = crate::Reg<u32, _RTC_SCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_SCCR;
#[doc = "`write(|w| ..)` method takes [rtc_sccr::W](rtc_sccr::W) writer structure"]
impl crate::Writable for RTC_SCCR {}
#[doc = "Status Clear Command Register"]
pub mod rtc_sccr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_ier](rtc_ier) module"]
pub type RTC_IER = crate::Reg<u32, _RTC_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IER;
#[doc = "`write(|w| ..)` method takes [rtc_ier::W](rtc_ier::W) writer structure"]
impl crate::Writable for RTC_IER {}
#[doc = "Interrupt Enable Register"]
pub mod rtc_ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_idr](rtc_idr) module"]
pub type RTC_IDR = crate::Reg<u32, _RTC_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IDR;
#[doc = "`write(|w| ..)` method takes [rtc_idr::W](rtc_idr::W) writer structure"]
impl crate::Writable for RTC_IDR {}
#[doc = "Interrupt Disable Register"]
pub mod rtc_idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_imr](rtc_imr) module"]
pub type RTC_IMR = crate::Reg<u32, _RTC_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IMR;
#[doc = "`read()` method returns [rtc_imr::R](rtc_imr::R) reader structure"]
impl crate::Readable for RTC_IMR {}
#[doc = "Interrupt Mask Register"]
pub mod rtc_imr;
#[doc = "Valid Entry Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_ver](rtc_ver) module"]
pub type RTC_VER = crate::Reg<u32, _RTC_VER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_VER;
#[doc = "`read()` method returns [rtc_ver::R](rtc_ver::R) reader structure"]
impl crate::Readable for RTC_VER {}
#[doc = "Valid Entry Register"]
pub mod rtc_ver;
