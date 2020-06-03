#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub qspi_cr: QSPI_CR,
    #[doc = "0x04 - Mode Register"]
    pub qspi_mr: QSPI_MR,
    #[doc = "0x08 - Receive Data Register"]
    pub qspi_rdr: QSPI_RDR,
    #[doc = "0x0c - Transmit Data Register"]
    pub qspi_tdr: QSPI_TDR,
    #[doc = "0x10 - Status Register"]
    pub qspi_sr: QSPI_SR,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub qspi_ier: QSPI_IER,
    #[doc = "0x18 - Interrupt Disable Register"]
    pub qspi_idr: QSPI_IDR,
    #[doc = "0x1c - Interrupt Mask Register"]
    pub qspi_imr: QSPI_IMR,
    #[doc = "0x20 - Serial Clock Register"]
    pub qspi_scr: QSPI_SCR,
    _reserved9: [u8; 12usize],
    #[doc = "0x30 - Instruction Address Register"]
    pub qspi_iar: QSPI_IAR,
    #[doc = "0x34 - Instruction Code Register"]
    pub qspi_icr: QSPI_ICR,
    #[doc = "0x38 - Instruction Frame Register"]
    pub qspi_ifr: QSPI_IFR,
    _reserved12: [u8; 4usize],
    #[doc = "0x40 - Scrambling Mode Register"]
    pub qspi_smr: QSPI_SMR,
    #[doc = "0x44 - Scrambling Key Register"]
    pub qspi_skr: QSPI_SKR,
    _reserved14: [u8; 156usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub qspi_wpmr: QSPI_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub qspi_wpsr: QSPI_WPSR,
}
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_cr](qspi_cr) module"]
pub type QSPI_CR = crate::Reg<u32, _QSPI_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSPI_CR;
#[doc = "`write(|w| ..)` method takes [qspi_cr::W](qspi_cr::W) writer structure"]
impl crate::Writable for QSPI_CR {}
#[doc = "Control Register"]
pub mod qspi_cr;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_mr](qspi_mr) module"]
pub type QSPI_MR = crate::Reg<u32, _QSPI_MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSPI_MR;
#[doc = "`read()` method returns [qspi_mr::R](qspi_mr::R) reader structure"]
impl crate::Readable for QSPI_MR {}
#[doc = "`write(|w| ..)` method takes [qspi_mr::W](qspi_mr::W) writer structure"]
impl crate::Writable for QSPI_MR {}
#[doc = "Mode Register"]
pub mod qspi_mr;
#[doc = "Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_rdr](qspi_rdr) module"]
pub type QSPI_RDR = crate::Reg<u32, _QSPI_RDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSPI_RDR;
#[doc = "`read()` method returns [qspi_rdr::R](qspi_rdr::R) reader structure"]
impl crate::Readable for QSPI_RDR {}
#[doc = "Receive Data Register"]
pub mod qspi_rdr;
#[doc = "Transmit Data Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_tdr](qspi_tdr) module"]
pub type QSPI_TDR = crate::Reg<u32, _QSPI_TDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSPI_TDR;
#[doc = "`write(|w| ..)` method takes [qspi_tdr::W](qspi_tdr::W) writer structure"]
impl crate::Writable for QSPI_TDR {}
#[doc = "Transmit Data Register"]
pub mod qspi_tdr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_sr](qspi_sr) module"]
pub type QSPI_SR = crate::Reg<u32, _QSPI_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSPI_SR;
#[doc = "`read()` method returns [qspi_sr::R](qspi_sr::R) reader structure"]
impl crate::Readable for QSPI_SR {}
#[doc = "Status Register"]
pub mod qspi_sr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_ier](qspi_ier) module"]
pub type QSPI_IER = crate::Reg<u32, _QSPI_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSPI_IER;
#[doc = "`write(|w| ..)` method takes [qspi_ier::W](qspi_ier::W) writer structure"]
impl crate::Writable for QSPI_IER {}
#[doc = "Interrupt Enable Register"]
pub mod qspi_ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_idr](qspi_idr) module"]
pub type QSPI_IDR = crate::Reg<u32, _QSPI_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSPI_IDR;
#[doc = "`write(|w| ..)` method takes [qspi_idr::W](qspi_idr::W) writer structure"]
impl crate::Writable for QSPI_IDR {}
#[doc = "Interrupt Disable Register"]
pub mod qspi_idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_imr](qspi_imr) module"]
pub type QSPI_IMR = crate::Reg<u32, _QSPI_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSPI_IMR;
#[doc = "`read()` method returns [qspi_imr::R](qspi_imr::R) reader structure"]
impl crate::Readable for QSPI_IMR {}
#[doc = "Interrupt Mask Register"]
pub mod qspi_imr;
#[doc = "Serial Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_scr](qspi_scr) module"]
pub type QSPI_SCR = crate::Reg<u32, _QSPI_SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSPI_SCR;
#[doc = "`read()` method returns [qspi_scr::R](qspi_scr::R) reader structure"]
impl crate::Readable for QSPI_SCR {}
#[doc = "`write(|w| ..)` method takes [qspi_scr::W](qspi_scr::W) writer structure"]
impl crate::Writable for QSPI_SCR {}
#[doc = "Serial Clock Register"]
pub mod qspi_scr;
#[doc = "Instruction Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_iar](qspi_iar) module"]
pub type QSPI_IAR = crate::Reg<u32, _QSPI_IAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSPI_IAR;
#[doc = "`read()` method returns [qspi_iar::R](qspi_iar::R) reader structure"]
impl crate::Readable for QSPI_IAR {}
#[doc = "`write(|w| ..)` method takes [qspi_iar::W](qspi_iar::W) writer structure"]
impl crate::Writable for QSPI_IAR {}
#[doc = "Instruction Address Register"]
pub mod qspi_iar;
#[doc = "Instruction Code Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_icr](qspi_icr) module"]
pub type QSPI_ICR = crate::Reg<u32, _QSPI_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSPI_ICR;
#[doc = "`read()` method returns [qspi_icr::R](qspi_icr::R) reader structure"]
impl crate::Readable for QSPI_ICR {}
#[doc = "`write(|w| ..)` method takes [qspi_icr::W](qspi_icr::W) writer structure"]
impl crate::Writable for QSPI_ICR {}
#[doc = "Instruction Code Register"]
pub mod qspi_icr;
#[doc = "Instruction Frame Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_ifr](qspi_ifr) module"]
pub type QSPI_IFR = crate::Reg<u32, _QSPI_IFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSPI_IFR;
#[doc = "`read()` method returns [qspi_ifr::R](qspi_ifr::R) reader structure"]
impl crate::Readable for QSPI_IFR {}
#[doc = "`write(|w| ..)` method takes [qspi_ifr::W](qspi_ifr::W) writer structure"]
impl crate::Writable for QSPI_IFR {}
#[doc = "Instruction Frame Register"]
pub mod qspi_ifr;
#[doc = "Scrambling Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_smr](qspi_smr) module"]
pub type QSPI_SMR = crate::Reg<u32, _QSPI_SMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSPI_SMR;
#[doc = "`read()` method returns [qspi_smr::R](qspi_smr::R) reader structure"]
impl crate::Readable for QSPI_SMR {}
#[doc = "`write(|w| ..)` method takes [qspi_smr::W](qspi_smr::W) writer structure"]
impl crate::Writable for QSPI_SMR {}
#[doc = "Scrambling Mode Register"]
pub mod qspi_smr;
#[doc = "Scrambling Key Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_skr](qspi_skr) module"]
pub type QSPI_SKR = crate::Reg<u32, _QSPI_SKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSPI_SKR;
#[doc = "`write(|w| ..)` method takes [qspi_skr::W](qspi_skr::W) writer structure"]
impl crate::Writable for QSPI_SKR {}
#[doc = "Scrambling Key Register"]
pub mod qspi_skr;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_wpmr](qspi_wpmr) module"]
pub type QSPI_WPMR = crate::Reg<u32, _QSPI_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSPI_WPMR;
#[doc = "`read()` method returns [qspi_wpmr::R](qspi_wpmr::R) reader structure"]
impl crate::Readable for QSPI_WPMR {}
#[doc = "`write(|w| ..)` method takes [qspi_wpmr::W](qspi_wpmr::W) writer structure"]
impl crate::Writable for QSPI_WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod qspi_wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qspi_wpsr](qspi_wpsr) module"]
pub type QSPI_WPSR = crate::Reg<u32, _QSPI_WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QSPI_WPSR;
#[doc = "`read()` method returns [qspi_wpsr::R](qspi_wpsr::R) reader structure"]
impl crate::Readable for QSPI_WPSR {}
#[doc = "Write Protection Status Register"]
pub mod qspi_wpsr;
