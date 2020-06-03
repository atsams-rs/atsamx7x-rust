#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub us_cr: US_CR,
    #[doc = "0x04 - Mode Register"]
    pub us_mr: US_MR,
    #[doc = "0x08 - Interrupt Enable Register"]
    pub us_ier: US_IER,
    #[doc = "0x0c - Interrupt Disable Register"]
    pub us_idr: US_IDR,
    #[doc = "0x10 - Interrupt Mask Register"]
    pub us_imr: US_IMR,
    #[doc = "0x14 - Channel Status Register"]
    pub us_csr: US_CSR,
    #[doc = "0x18 - Receive Holding Register"]
    pub us_rhr: US_RHR,
    #[doc = "0x1c - Transmit Holding Register"]
    pub us_thr: US_THR,
    #[doc = "0x20 - Baud Rate Generator Register"]
    pub us_brgr: US_BRGR,
    #[doc = "0x24 - Receiver Time-out Register"]
    pub us_rtor: US_RTOR,
    #[doc = "0x28 - Transmitter Timeguard Register"]
    pub us_ttgr: US_TTGR,
    _reserved11: [u8; 20usize],
    #[doc = "0x40 - FI DI Ratio Register"]
    pub us_fidi: US_FIDI,
    #[doc = "0x44 - Number of Errors Register"]
    pub us_ner: US_NER,
    _reserved13: [u8; 4usize],
    #[doc = "0x4c - IrDA Filter Register"]
    pub us_if: US_IF,
    #[doc = "0x50 - Manchester Configuration Register"]
    pub us_man: US_MAN,
    #[doc = "0x54 - LIN Mode Register"]
    pub us_linmr: US_LINMR,
    #[doc = "0x58 - LIN Identifier Register"]
    pub us_linir: US_LINIR,
    #[doc = "0x5c - LIN Baud Rate Register"]
    pub us_linbrr: US_LINBRR,
    #[doc = "0x60 - LON Mode Register"]
    pub us_lonmr: US_LONMR,
    #[doc = "0x64 - LON Preamble Register"]
    pub us_lonpr: US_LONPR,
    #[doc = "0x68 - LON Data Length Register"]
    pub us_londl: US_LONDL,
    #[doc = "0x6c - LON L2HDR Register"]
    pub us_lonl2hdr: US_LONL2HDR,
    #[doc = "0x70 - LON Backlog Register"]
    pub us_lonbl: US_LONBL,
    #[doc = "0x74 - LON Beta1 Tx Register"]
    pub us_lonb1tx: US_LONB1TX,
    #[doc = "0x78 - LON Beta1 Rx Register"]
    pub us_lonb1rx: US_LONB1RX,
    #[doc = "0x7c - LON Priority Register"]
    pub us_lonprio: US_LONPRIO,
    #[doc = "0x80 - LON IDT Tx Register"]
    pub us_idttx: US_IDTTX,
    #[doc = "0x84 - LON IDT Rx Register"]
    pub us_idtrx: US_IDTRX,
    #[doc = "0x88 - IC DIFF Register"]
    pub us_icdiff: US_ICDIFF,
    _reserved29: [u8; 88usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub us_wpmr: US_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub us_wpsr: US_WPSR,
}
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_cr](us_cr) module"]
pub type US_CR = crate::Reg<u32, _US_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_CR;
#[doc = "`write(|w| ..)` method takes [us_cr::W](us_cr::W) writer structure"]
impl crate::Writable for US_CR {}
#[doc = "Control Register"]
pub mod us_cr;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_mr](us_mr) module"]
pub type US_MR = crate::Reg<u32, _US_MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_MR;
#[doc = "`read()` method returns [us_mr::R](us_mr::R) reader structure"]
impl crate::Readable for US_MR {}
#[doc = "`write(|w| ..)` method takes [us_mr::W](us_mr::W) writer structure"]
impl crate::Writable for US_MR {}
#[doc = "Mode Register"]
pub mod us_mr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_ier](us_ier) module"]
pub type US_IER = crate::Reg<u32, _US_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_IER;
#[doc = "`write(|w| ..)` method takes [us_ier::W](us_ier::W) writer structure"]
impl crate::Writable for US_IER {}
#[doc = "Interrupt Enable Register"]
pub mod us_ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_idr](us_idr) module"]
pub type US_IDR = crate::Reg<u32, _US_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_IDR;
#[doc = "`write(|w| ..)` method takes [us_idr::W](us_idr::W) writer structure"]
impl crate::Writable for US_IDR {}
#[doc = "Interrupt Disable Register"]
pub mod us_idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_imr](us_imr) module"]
pub type US_IMR = crate::Reg<u32, _US_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_IMR;
#[doc = "`read()` method returns [us_imr::R](us_imr::R) reader structure"]
impl crate::Readable for US_IMR {}
#[doc = "Interrupt Mask Register"]
pub mod us_imr;
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_csr](us_csr) module"]
pub type US_CSR = crate::Reg<u32, _US_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_CSR;
#[doc = "`read()` method returns [us_csr::R](us_csr::R) reader structure"]
impl crate::Readable for US_CSR {}
#[doc = "Channel Status Register"]
pub mod us_csr;
#[doc = "Receive Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_rhr](us_rhr) module"]
pub type US_RHR = crate::Reg<u32, _US_RHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_RHR;
#[doc = "`read()` method returns [us_rhr::R](us_rhr::R) reader structure"]
impl crate::Readable for US_RHR {}
#[doc = "Receive Holding Register"]
pub mod us_rhr;
#[doc = "Transmit Holding Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_thr](us_thr) module"]
pub type US_THR = crate::Reg<u32, _US_THR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_THR;
#[doc = "`write(|w| ..)` method takes [us_thr::W](us_thr::W) writer structure"]
impl crate::Writable for US_THR {}
#[doc = "Transmit Holding Register"]
pub mod us_thr;
#[doc = "Baud Rate Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_brgr](us_brgr) module"]
pub type US_BRGR = crate::Reg<u32, _US_BRGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_BRGR;
#[doc = "`read()` method returns [us_brgr::R](us_brgr::R) reader structure"]
impl crate::Readable for US_BRGR {}
#[doc = "`write(|w| ..)` method takes [us_brgr::W](us_brgr::W) writer structure"]
impl crate::Writable for US_BRGR {}
#[doc = "Baud Rate Generator Register"]
pub mod us_brgr;
#[doc = "Receiver Time-out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_rtor](us_rtor) module"]
pub type US_RTOR = crate::Reg<u32, _US_RTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_RTOR;
#[doc = "`read()` method returns [us_rtor::R](us_rtor::R) reader structure"]
impl crate::Readable for US_RTOR {}
#[doc = "`write(|w| ..)` method takes [us_rtor::W](us_rtor::W) writer structure"]
impl crate::Writable for US_RTOR {}
#[doc = "Receiver Time-out Register"]
pub mod us_rtor;
#[doc = "Transmitter Timeguard Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_ttgr](us_ttgr) module"]
pub type US_TTGR = crate::Reg<u32, _US_TTGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_TTGR;
#[doc = "`read()` method returns [us_ttgr::R](us_ttgr::R) reader structure"]
impl crate::Readable for US_TTGR {}
#[doc = "`write(|w| ..)` method takes [us_ttgr::W](us_ttgr::W) writer structure"]
impl crate::Writable for US_TTGR {}
#[doc = "Transmitter Timeguard Register"]
pub mod us_ttgr;
#[doc = "FI DI Ratio Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_fidi](us_fidi) module"]
pub type US_FIDI = crate::Reg<u32, _US_FIDI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_FIDI;
#[doc = "`read()` method returns [us_fidi::R](us_fidi::R) reader structure"]
impl crate::Readable for US_FIDI {}
#[doc = "`write(|w| ..)` method takes [us_fidi::W](us_fidi::W) writer structure"]
impl crate::Writable for US_FIDI {}
#[doc = "FI DI Ratio Register"]
pub mod us_fidi;
#[doc = "Number of Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_ner](us_ner) module"]
pub type US_NER = crate::Reg<u32, _US_NER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_NER;
#[doc = "`read()` method returns [us_ner::R](us_ner::R) reader structure"]
impl crate::Readable for US_NER {}
#[doc = "Number of Errors Register"]
pub mod us_ner;
#[doc = "IrDA Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_if](us_if) module"]
pub type US_IF = crate::Reg<u32, _US_IF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_IF;
#[doc = "`read()` method returns [us_if::R](us_if::R) reader structure"]
impl crate::Readable for US_IF {}
#[doc = "`write(|w| ..)` method takes [us_if::W](us_if::W) writer structure"]
impl crate::Writable for US_IF {}
#[doc = "IrDA Filter Register"]
pub mod us_if;
#[doc = "Manchester Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_man](us_man) module"]
pub type US_MAN = crate::Reg<u32, _US_MAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_MAN;
#[doc = "`read()` method returns [us_man::R](us_man::R) reader structure"]
impl crate::Readable for US_MAN {}
#[doc = "`write(|w| ..)` method takes [us_man::W](us_man::W) writer structure"]
impl crate::Writable for US_MAN {}
#[doc = "Manchester Configuration Register"]
pub mod us_man;
#[doc = "LIN Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_linmr](us_linmr) module"]
pub type US_LINMR = crate::Reg<u32, _US_LINMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_LINMR;
#[doc = "`read()` method returns [us_linmr::R](us_linmr::R) reader structure"]
impl crate::Readable for US_LINMR {}
#[doc = "`write(|w| ..)` method takes [us_linmr::W](us_linmr::W) writer structure"]
impl crate::Writable for US_LINMR {}
#[doc = "LIN Mode Register"]
pub mod us_linmr;
#[doc = "LIN Identifier Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_linir](us_linir) module"]
pub type US_LINIR = crate::Reg<u32, _US_LINIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_LINIR;
#[doc = "`read()` method returns [us_linir::R](us_linir::R) reader structure"]
impl crate::Readable for US_LINIR {}
#[doc = "`write(|w| ..)` method takes [us_linir::W](us_linir::W) writer structure"]
impl crate::Writable for US_LINIR {}
#[doc = "LIN Identifier Register"]
pub mod us_linir;
#[doc = "LIN Baud Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_linbrr](us_linbrr) module"]
pub type US_LINBRR = crate::Reg<u32, _US_LINBRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_LINBRR;
#[doc = "`read()` method returns [us_linbrr::R](us_linbrr::R) reader structure"]
impl crate::Readable for US_LINBRR {}
#[doc = "LIN Baud Rate Register"]
pub mod us_linbrr;
#[doc = "LON Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_lonmr](us_lonmr) module"]
pub type US_LONMR = crate::Reg<u32, _US_LONMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_LONMR;
#[doc = "`read()` method returns [us_lonmr::R](us_lonmr::R) reader structure"]
impl crate::Readable for US_LONMR {}
#[doc = "`write(|w| ..)` method takes [us_lonmr::W](us_lonmr::W) writer structure"]
impl crate::Writable for US_LONMR {}
#[doc = "LON Mode Register"]
pub mod us_lonmr;
#[doc = "LON Preamble Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_lonpr](us_lonpr) module"]
pub type US_LONPR = crate::Reg<u32, _US_LONPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_LONPR;
#[doc = "`read()` method returns [us_lonpr::R](us_lonpr::R) reader structure"]
impl crate::Readable for US_LONPR {}
#[doc = "`write(|w| ..)` method takes [us_lonpr::W](us_lonpr::W) writer structure"]
impl crate::Writable for US_LONPR {}
#[doc = "LON Preamble Register"]
pub mod us_lonpr;
#[doc = "LON Data Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_londl](us_londl) module"]
pub type US_LONDL = crate::Reg<u32, _US_LONDL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_LONDL;
#[doc = "`read()` method returns [us_londl::R](us_londl::R) reader structure"]
impl crate::Readable for US_LONDL {}
#[doc = "`write(|w| ..)` method takes [us_londl::W](us_londl::W) writer structure"]
impl crate::Writable for US_LONDL {}
#[doc = "LON Data Length Register"]
pub mod us_londl;
#[doc = "LON L2HDR Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_lonl2hdr](us_lonl2hdr) module"]
pub type US_LONL2HDR = crate::Reg<u32, _US_LONL2HDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_LONL2HDR;
#[doc = "`read()` method returns [us_lonl2hdr::R](us_lonl2hdr::R) reader structure"]
impl crate::Readable for US_LONL2HDR {}
#[doc = "`write(|w| ..)` method takes [us_lonl2hdr::W](us_lonl2hdr::W) writer structure"]
impl crate::Writable for US_LONL2HDR {}
#[doc = "LON L2HDR Register"]
pub mod us_lonl2hdr;
#[doc = "LON Backlog Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_lonbl](us_lonbl) module"]
pub type US_LONBL = crate::Reg<u32, _US_LONBL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_LONBL;
#[doc = "`read()` method returns [us_lonbl::R](us_lonbl::R) reader structure"]
impl crate::Readable for US_LONBL {}
#[doc = "LON Backlog Register"]
pub mod us_lonbl;
#[doc = "LON Beta1 Tx Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_lonb1tx](us_lonb1tx) module"]
pub type US_LONB1TX = crate::Reg<u32, _US_LONB1TX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_LONB1TX;
#[doc = "`read()` method returns [us_lonb1tx::R](us_lonb1tx::R) reader structure"]
impl crate::Readable for US_LONB1TX {}
#[doc = "`write(|w| ..)` method takes [us_lonb1tx::W](us_lonb1tx::W) writer structure"]
impl crate::Writable for US_LONB1TX {}
#[doc = "LON Beta1 Tx Register"]
pub mod us_lonb1tx;
#[doc = "LON Beta1 Rx Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_lonb1rx](us_lonb1rx) module"]
pub type US_LONB1RX = crate::Reg<u32, _US_LONB1RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_LONB1RX;
#[doc = "`read()` method returns [us_lonb1rx::R](us_lonb1rx::R) reader structure"]
impl crate::Readable for US_LONB1RX {}
#[doc = "`write(|w| ..)` method takes [us_lonb1rx::W](us_lonb1rx::W) writer structure"]
impl crate::Writable for US_LONB1RX {}
#[doc = "LON Beta1 Rx Register"]
pub mod us_lonb1rx;
#[doc = "LON Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_lonprio](us_lonprio) module"]
pub type US_LONPRIO = crate::Reg<u32, _US_LONPRIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_LONPRIO;
#[doc = "`read()` method returns [us_lonprio::R](us_lonprio::R) reader structure"]
impl crate::Readable for US_LONPRIO {}
#[doc = "`write(|w| ..)` method takes [us_lonprio::W](us_lonprio::W) writer structure"]
impl crate::Writable for US_LONPRIO {}
#[doc = "LON Priority Register"]
pub mod us_lonprio;
#[doc = "LON IDT Tx Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_idttx](us_idttx) module"]
pub type US_IDTTX = crate::Reg<u32, _US_IDTTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_IDTTX;
#[doc = "`read()` method returns [us_idttx::R](us_idttx::R) reader structure"]
impl crate::Readable for US_IDTTX {}
#[doc = "`write(|w| ..)` method takes [us_idttx::W](us_idttx::W) writer structure"]
impl crate::Writable for US_IDTTX {}
#[doc = "LON IDT Tx Register"]
pub mod us_idttx;
#[doc = "LON IDT Rx Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_idtrx](us_idtrx) module"]
pub type US_IDTRX = crate::Reg<u32, _US_IDTRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_IDTRX;
#[doc = "`read()` method returns [us_idtrx::R](us_idtrx::R) reader structure"]
impl crate::Readable for US_IDTRX {}
#[doc = "`write(|w| ..)` method takes [us_idtrx::W](us_idtrx::W) writer structure"]
impl crate::Writable for US_IDTRX {}
#[doc = "LON IDT Rx Register"]
pub mod us_idtrx;
#[doc = "IC DIFF Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_icdiff](us_icdiff) module"]
pub type US_ICDIFF = crate::Reg<u32, _US_ICDIFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_ICDIFF;
#[doc = "`read()` method returns [us_icdiff::R](us_icdiff::R) reader structure"]
impl crate::Readable for US_ICDIFF {}
#[doc = "`write(|w| ..)` method takes [us_icdiff::W](us_icdiff::W) writer structure"]
impl crate::Writable for US_ICDIFF {}
#[doc = "IC DIFF Register"]
pub mod us_icdiff;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_wpmr](us_wpmr) module"]
pub type US_WPMR = crate::Reg<u32, _US_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_WPMR;
#[doc = "`read()` method returns [us_wpmr::R](us_wpmr::R) reader structure"]
impl crate::Readable for US_WPMR {}
#[doc = "`write(|w| ..)` method takes [us_wpmr::W](us_wpmr::W) writer structure"]
impl crate::Writable for US_WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod us_wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_wpsr](us_wpsr) module"]
pub type US_WPSR = crate::Reg<u32, _US_WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _US_WPSR;
#[doc = "`read()` method returns [us_wpsr::R](us_wpsr::R) reader structure"]
impl crate::Readable for US_WPSR {}
#[doc = "Write Protection Status Register"]
pub mod us_wpsr;
