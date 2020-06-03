#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub wdt_cr: WDT_CR,
    #[doc = "0x04 - Mode Register"]
    pub wdt_mr: WDT_MR,
    #[doc = "0x08 - Status Register"]
    pub wdt_sr: WDT_SR,
}
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_cr](wdt_cr) module"]
pub type WDT_CR = crate::Reg<u32, _WDT_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDT_CR;
#[doc = "`write(|w| ..)` method takes [wdt_cr::W](wdt_cr::W) writer structure"]
impl crate::Writable for WDT_CR {}
#[doc = "Control Register"]
pub mod wdt_cr;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_mr](wdt_mr) module"]
pub type WDT_MR = crate::Reg<u32, _WDT_MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDT_MR;
#[doc = "`read()` method returns [wdt_mr::R](wdt_mr::R) reader structure"]
impl crate::Readable for WDT_MR {}
#[doc = "`write(|w| ..)` method takes [wdt_mr::W](wdt_mr::W) writer structure"]
impl crate::Writable for WDT_MR {}
#[doc = "Mode Register"]
pub mod wdt_mr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_sr](wdt_sr) module"]
pub type WDT_SR = crate::Reg<u32, _WDT_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDT_SR;
#[doc = "`read()` method returns [wdt_sr::R](wdt_sr::R) reader structure"]
impl crate::Readable for WDT_SR {}
#[doc = "Status Register"]
pub mod wdt_sr;
