#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Core Release Register"]
    pub mcan_crel: MCAN_CREL,
    #[doc = "0x04 - Endian Register"]
    pub mcan_endn: MCAN_ENDN,
    #[doc = "0x08 - Customer Register"]
    pub mcan_cust: MCAN_CUST,
    #[doc = "0x0c - Data Bit Timing and Prescaler Register"]
    pub mcan_dbtp: MCAN_DBTP,
    #[doc = "0x10 - Test Register"]
    pub mcan_test: MCAN_TEST,
    #[doc = "0x14 - RAM Watchdog Register"]
    pub mcan_rwd: MCAN_RWD,
    #[doc = "0x18 - CC Control Register"]
    pub mcan_cccr: MCAN_CCCR,
    #[doc = "0x1c - Nominal Bit Timing and Prescaler Register"]
    pub mcan_nbtp: MCAN_NBTP,
    #[doc = "0x20 - Timestamp Counter Configuration Register"]
    pub mcan_tscc: MCAN_TSCC,
    #[doc = "0x24 - Timestamp Counter Value Register"]
    pub mcan_tscv: MCAN_TSCV,
    #[doc = "0x28 - Timeout Counter Configuration Register"]
    pub mcan_tocc: MCAN_TOCC,
    #[doc = "0x2c - Timeout Counter Value Register"]
    pub mcan_tocv: MCAN_TOCV,
    _reserved12: [u8; 16usize],
    #[doc = "0x40 - Error Counter Register"]
    pub mcan_ecr: MCAN_ECR,
    #[doc = "0x44 - Protocol Status Register"]
    pub mcan_psr: MCAN_PSR,
    #[doc = "0x48 - Transmit Delay Compensation Register"]
    pub mcan_tdcr: MCAN_TDCR,
    _reserved15: [u8; 4usize],
    #[doc = "0x50 - Interrupt Register"]
    pub mcan_ir: MCAN_IR,
    #[doc = "0x54 - Interrupt Enable Register"]
    pub mcan_ie: MCAN_IE,
    #[doc = "0x58 - Interrupt Line Select Register"]
    pub mcan_ils: MCAN_ILS,
    #[doc = "0x5c - Interrupt Line Enable Register"]
    pub mcan_ile: MCAN_ILE,
    _reserved19: [u8; 32usize],
    #[doc = "0x80 - Global Filter Configuration Register"]
    pub mcan_gfc: MCAN_GFC,
    #[doc = "0x84 - Standard ID Filter Configuration Register"]
    pub mcan_sidfc: MCAN_SIDFC,
    #[doc = "0x88 - Extended ID Filter Configuration Register"]
    pub mcan_xidfc: MCAN_XIDFC,
    _reserved22: [u8; 4usize],
    #[doc = "0x90 - Extended ID AND Mask Register"]
    pub mcan_xidam: MCAN_XIDAM,
    #[doc = "0x94 - High Priority Message Status Register"]
    pub mcan_hpms: MCAN_HPMS,
    #[doc = "0x98 - New Data 1 Register"]
    pub mcan_ndat1: MCAN_NDAT1,
    #[doc = "0x9c - New Data 2 Register"]
    pub mcan_ndat2: MCAN_NDAT2,
    #[doc = "0xa0 - Receive FIFO 0 Configuration Register"]
    pub mcan_rxf0c: MCAN_RXF0C,
    #[doc = "0xa4 - Receive FIFO 0 Status Register"]
    pub mcan_rxf0s: MCAN_RXF0S,
    #[doc = "0xa8 - Receive FIFO 0 Acknowledge Register"]
    pub mcan_rxf0a: MCAN_RXF0A,
    #[doc = "0xac - Receive Rx Buffer Configuration Register"]
    pub mcan_rxbc: MCAN_RXBC,
    #[doc = "0xb0 - Receive FIFO 1 Configuration Register"]
    pub mcan_rxf1c: MCAN_RXF1C,
    #[doc = "0xb4 - Receive FIFO 1 Status Register"]
    pub mcan_rxf1s: MCAN_RXF1S,
    #[doc = "0xb8 - Receive FIFO 1 Acknowledge Register"]
    pub mcan_rxf1a: MCAN_RXF1A,
    #[doc = "0xbc - Receive Buffer / FIFO Element Size Configuration Register"]
    pub mcan_rxesc: MCAN_RXESC,
    #[doc = "0xc0 - Transmit Buffer Configuration Register"]
    pub mcan_txbc: MCAN_TXBC,
    #[doc = "0xc4 - Transmit FIFO/Queue Status Register"]
    pub mcan_txfqs: MCAN_TXFQS,
    #[doc = "0xc8 - Transmit Buffer Element Size Configuration Register"]
    pub mcan_txesc: MCAN_TXESC,
    #[doc = "0xcc - Transmit Buffer Request Pending Register"]
    pub mcan_txbrp: MCAN_TXBRP,
    #[doc = "0xd0 - Transmit Buffer Add Request Register"]
    pub mcan_txbar: MCAN_TXBAR,
    #[doc = "0xd4 - Transmit Buffer Cancellation Request Register"]
    pub mcan_txbcr: MCAN_TXBCR,
    #[doc = "0xd8 - Transmit Buffer Transmission Occurred Register"]
    pub mcan_txbto: MCAN_TXBTO,
    #[doc = "0xdc - Transmit Buffer Cancellation Finished Register"]
    pub mcan_txbcf: MCAN_TXBCF,
    #[doc = "0xe0 - Transmit Buffer Transmission Interrupt Enable Register"]
    pub mcan_txbtie: MCAN_TXBTIE,
    #[doc = "0xe4 - Transmit Buffer Cancellation Finished Interrupt Enable Register"]
    pub mcan_txbcie: MCAN_TXBCIE,
    _reserved44: [u8; 8usize],
    #[doc = "0xf0 - Transmit Event FIFO Configuration Register"]
    pub mcan_txefc: MCAN_TXEFC,
    #[doc = "0xf4 - Transmit Event FIFO Status Register"]
    pub mcan_txefs: MCAN_TXEFS,
    #[doc = "0xf8 - Transmit Event FIFO Acknowledge Register"]
    pub mcan_txefa: MCAN_TXEFA,
}
#[doc = "Core Release Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_crel](mcan_crel) module"]
pub type MCAN_CREL = crate::Reg<u32, _MCAN_CREL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_CREL;
#[doc = "`read()` method returns [mcan_crel::R](mcan_crel::R) reader structure"]
impl crate::Readable for MCAN_CREL {}
#[doc = "Core Release Register"]
pub mod mcan_crel;
#[doc = "Endian Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_endn](mcan_endn) module"]
pub type MCAN_ENDN = crate::Reg<u32, _MCAN_ENDN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_ENDN;
#[doc = "`read()` method returns [mcan_endn::R](mcan_endn::R) reader structure"]
impl crate::Readable for MCAN_ENDN {}
#[doc = "Endian Register"]
pub mod mcan_endn;
#[doc = "Customer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_cust](mcan_cust) module"]
pub type MCAN_CUST = crate::Reg<u32, _MCAN_CUST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_CUST;
#[doc = "`read()` method returns [mcan_cust::R](mcan_cust::R) reader structure"]
impl crate::Readable for MCAN_CUST {}
#[doc = "`write(|w| ..)` method takes [mcan_cust::W](mcan_cust::W) writer structure"]
impl crate::Writable for MCAN_CUST {}
#[doc = "Customer Register"]
pub mod mcan_cust;
#[doc = "Data Bit Timing and Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_dbtp](mcan_dbtp) module"]
pub type MCAN_DBTP = crate::Reg<u32, _MCAN_DBTP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_DBTP;
#[doc = "`read()` method returns [mcan_dbtp::R](mcan_dbtp::R) reader structure"]
impl crate::Readable for MCAN_DBTP {}
#[doc = "`write(|w| ..)` method takes [mcan_dbtp::W](mcan_dbtp::W) writer structure"]
impl crate::Writable for MCAN_DBTP {}
#[doc = "Data Bit Timing and Prescaler Register"]
pub mod mcan_dbtp;
#[doc = "Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_test](mcan_test) module"]
pub type MCAN_TEST = crate::Reg<u32, _MCAN_TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TEST;
#[doc = "`read()` method returns [mcan_test::R](mcan_test::R) reader structure"]
impl crate::Readable for MCAN_TEST {}
#[doc = "`write(|w| ..)` method takes [mcan_test::W](mcan_test::W) writer structure"]
impl crate::Writable for MCAN_TEST {}
#[doc = "Test Register"]
pub mod mcan_test;
#[doc = "RAM Watchdog Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_rwd](mcan_rwd) module"]
pub type MCAN_RWD = crate::Reg<u32, _MCAN_RWD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_RWD;
#[doc = "`read()` method returns [mcan_rwd::R](mcan_rwd::R) reader structure"]
impl crate::Readable for MCAN_RWD {}
#[doc = "`write(|w| ..)` method takes [mcan_rwd::W](mcan_rwd::W) writer structure"]
impl crate::Writable for MCAN_RWD {}
#[doc = "RAM Watchdog Register"]
pub mod mcan_rwd;
#[doc = "CC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_cccr](mcan_cccr) module"]
pub type MCAN_CCCR = crate::Reg<u32, _MCAN_CCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_CCCR;
#[doc = "`read()` method returns [mcan_cccr::R](mcan_cccr::R) reader structure"]
impl crate::Readable for MCAN_CCCR {}
#[doc = "`write(|w| ..)` method takes [mcan_cccr::W](mcan_cccr::W) writer structure"]
impl crate::Writable for MCAN_CCCR {}
#[doc = "CC Control Register"]
pub mod mcan_cccr;
#[doc = "Nominal Bit Timing and Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_nbtp](mcan_nbtp) module"]
pub type MCAN_NBTP = crate::Reg<u32, _MCAN_NBTP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_NBTP;
#[doc = "`read()` method returns [mcan_nbtp::R](mcan_nbtp::R) reader structure"]
impl crate::Readable for MCAN_NBTP {}
#[doc = "`write(|w| ..)` method takes [mcan_nbtp::W](mcan_nbtp::W) writer structure"]
impl crate::Writable for MCAN_NBTP {}
#[doc = "Nominal Bit Timing and Prescaler Register"]
pub mod mcan_nbtp;
#[doc = "Timestamp Counter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_tscc](mcan_tscc) module"]
pub type MCAN_TSCC = crate::Reg<u32, _MCAN_TSCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TSCC;
#[doc = "`read()` method returns [mcan_tscc::R](mcan_tscc::R) reader structure"]
impl crate::Readable for MCAN_TSCC {}
#[doc = "`write(|w| ..)` method takes [mcan_tscc::W](mcan_tscc::W) writer structure"]
impl crate::Writable for MCAN_TSCC {}
#[doc = "Timestamp Counter Configuration Register"]
pub mod mcan_tscc;
#[doc = "Timestamp Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_tscv](mcan_tscv) module"]
pub type MCAN_TSCV = crate::Reg<u32, _MCAN_TSCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TSCV;
#[doc = "`read()` method returns [mcan_tscv::R](mcan_tscv::R) reader structure"]
impl crate::Readable for MCAN_TSCV {}
#[doc = "`write(|w| ..)` method takes [mcan_tscv::W](mcan_tscv::W) writer structure"]
impl crate::Writable for MCAN_TSCV {}
#[doc = "Timestamp Counter Value Register"]
pub mod mcan_tscv;
#[doc = "Timeout Counter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_tocc](mcan_tocc) module"]
pub type MCAN_TOCC = crate::Reg<u32, _MCAN_TOCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TOCC;
#[doc = "`read()` method returns [mcan_tocc::R](mcan_tocc::R) reader structure"]
impl crate::Readable for MCAN_TOCC {}
#[doc = "`write(|w| ..)` method takes [mcan_tocc::W](mcan_tocc::W) writer structure"]
impl crate::Writable for MCAN_TOCC {}
#[doc = "Timeout Counter Configuration Register"]
pub mod mcan_tocc;
#[doc = "Timeout Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_tocv](mcan_tocv) module"]
pub type MCAN_TOCV = crate::Reg<u32, _MCAN_TOCV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TOCV;
#[doc = "`read()` method returns [mcan_tocv::R](mcan_tocv::R) reader structure"]
impl crate::Readable for MCAN_TOCV {}
#[doc = "`write(|w| ..)` method takes [mcan_tocv::W](mcan_tocv::W) writer structure"]
impl crate::Writable for MCAN_TOCV {}
#[doc = "Timeout Counter Value Register"]
pub mod mcan_tocv;
#[doc = "Error Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_ecr](mcan_ecr) module"]
pub type MCAN_ECR = crate::Reg<u32, _MCAN_ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_ECR;
#[doc = "`read()` method returns [mcan_ecr::R](mcan_ecr::R) reader structure"]
impl crate::Readable for MCAN_ECR {}
#[doc = "Error Counter Register"]
pub mod mcan_ecr;
#[doc = "Protocol Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_psr](mcan_psr) module"]
pub type MCAN_PSR = crate::Reg<u32, _MCAN_PSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_PSR;
#[doc = "`read()` method returns [mcan_psr::R](mcan_psr::R) reader structure"]
impl crate::Readable for MCAN_PSR {}
#[doc = "Protocol Status Register"]
pub mod mcan_psr;
#[doc = "Transmit Delay Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_tdcr](mcan_tdcr) module"]
pub type MCAN_TDCR = crate::Reg<u32, _MCAN_TDCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TDCR;
#[doc = "`read()` method returns [mcan_tdcr::R](mcan_tdcr::R) reader structure"]
impl crate::Readable for MCAN_TDCR {}
#[doc = "`write(|w| ..)` method takes [mcan_tdcr::W](mcan_tdcr::W) writer structure"]
impl crate::Writable for MCAN_TDCR {}
#[doc = "Transmit Delay Compensation Register"]
pub mod mcan_tdcr;
#[doc = "Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_ir](mcan_ir) module"]
pub type MCAN_IR = crate::Reg<u32, _MCAN_IR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_IR;
#[doc = "`read()` method returns [mcan_ir::R](mcan_ir::R) reader structure"]
impl crate::Readable for MCAN_IR {}
#[doc = "`write(|w| ..)` method takes [mcan_ir::W](mcan_ir::W) writer structure"]
impl crate::Writable for MCAN_IR {}
#[doc = "Interrupt Register"]
pub mod mcan_ir;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_ie](mcan_ie) module"]
pub type MCAN_IE = crate::Reg<u32, _MCAN_IE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_IE;
#[doc = "`read()` method returns [mcan_ie::R](mcan_ie::R) reader structure"]
impl crate::Readable for MCAN_IE {}
#[doc = "`write(|w| ..)` method takes [mcan_ie::W](mcan_ie::W) writer structure"]
impl crate::Writable for MCAN_IE {}
#[doc = "Interrupt Enable Register"]
pub mod mcan_ie;
#[doc = "Interrupt Line Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_ils](mcan_ils) module"]
pub type MCAN_ILS = crate::Reg<u32, _MCAN_ILS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_ILS;
#[doc = "`read()` method returns [mcan_ils::R](mcan_ils::R) reader structure"]
impl crate::Readable for MCAN_ILS {}
#[doc = "`write(|w| ..)` method takes [mcan_ils::W](mcan_ils::W) writer structure"]
impl crate::Writable for MCAN_ILS {}
#[doc = "Interrupt Line Select Register"]
pub mod mcan_ils;
#[doc = "Interrupt Line Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_ile](mcan_ile) module"]
pub type MCAN_ILE = crate::Reg<u32, _MCAN_ILE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_ILE;
#[doc = "`read()` method returns [mcan_ile::R](mcan_ile::R) reader structure"]
impl crate::Readable for MCAN_ILE {}
#[doc = "`write(|w| ..)` method takes [mcan_ile::W](mcan_ile::W) writer structure"]
impl crate::Writable for MCAN_ILE {}
#[doc = "Interrupt Line Enable Register"]
pub mod mcan_ile;
#[doc = "Global Filter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_gfc](mcan_gfc) module"]
pub type MCAN_GFC = crate::Reg<u32, _MCAN_GFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_GFC;
#[doc = "`read()` method returns [mcan_gfc::R](mcan_gfc::R) reader structure"]
impl crate::Readable for MCAN_GFC {}
#[doc = "`write(|w| ..)` method takes [mcan_gfc::W](mcan_gfc::W) writer structure"]
impl crate::Writable for MCAN_GFC {}
#[doc = "Global Filter Configuration Register"]
pub mod mcan_gfc;
#[doc = "Standard ID Filter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_sidfc](mcan_sidfc) module"]
pub type MCAN_SIDFC = crate::Reg<u32, _MCAN_SIDFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_SIDFC;
#[doc = "`read()` method returns [mcan_sidfc::R](mcan_sidfc::R) reader structure"]
impl crate::Readable for MCAN_SIDFC {}
#[doc = "`write(|w| ..)` method takes [mcan_sidfc::W](mcan_sidfc::W) writer structure"]
impl crate::Writable for MCAN_SIDFC {}
#[doc = "Standard ID Filter Configuration Register"]
pub mod mcan_sidfc;
#[doc = "Extended ID Filter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_xidfc](mcan_xidfc) module"]
pub type MCAN_XIDFC = crate::Reg<u32, _MCAN_XIDFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_XIDFC;
#[doc = "`read()` method returns [mcan_xidfc::R](mcan_xidfc::R) reader structure"]
impl crate::Readable for MCAN_XIDFC {}
#[doc = "`write(|w| ..)` method takes [mcan_xidfc::W](mcan_xidfc::W) writer structure"]
impl crate::Writable for MCAN_XIDFC {}
#[doc = "Extended ID Filter Configuration Register"]
pub mod mcan_xidfc;
#[doc = "Extended ID AND Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_xidam](mcan_xidam) module"]
pub type MCAN_XIDAM = crate::Reg<u32, _MCAN_XIDAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_XIDAM;
#[doc = "`read()` method returns [mcan_xidam::R](mcan_xidam::R) reader structure"]
impl crate::Readable for MCAN_XIDAM {}
#[doc = "`write(|w| ..)` method takes [mcan_xidam::W](mcan_xidam::W) writer structure"]
impl crate::Writable for MCAN_XIDAM {}
#[doc = "Extended ID AND Mask Register"]
pub mod mcan_xidam;
#[doc = "High Priority Message Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_hpms](mcan_hpms) module"]
pub type MCAN_HPMS = crate::Reg<u32, _MCAN_HPMS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_HPMS;
#[doc = "`read()` method returns [mcan_hpms::R](mcan_hpms::R) reader structure"]
impl crate::Readable for MCAN_HPMS {}
#[doc = "High Priority Message Status Register"]
pub mod mcan_hpms;
#[doc = "New Data 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_ndat1](mcan_ndat1) module"]
pub type MCAN_NDAT1 = crate::Reg<u32, _MCAN_NDAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_NDAT1;
#[doc = "`read()` method returns [mcan_ndat1::R](mcan_ndat1::R) reader structure"]
impl crate::Readable for MCAN_NDAT1 {}
#[doc = "`write(|w| ..)` method takes [mcan_ndat1::W](mcan_ndat1::W) writer structure"]
impl crate::Writable for MCAN_NDAT1 {}
#[doc = "New Data 1 Register"]
pub mod mcan_ndat1;
#[doc = "New Data 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_ndat2](mcan_ndat2) module"]
pub type MCAN_NDAT2 = crate::Reg<u32, _MCAN_NDAT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_NDAT2;
#[doc = "`read()` method returns [mcan_ndat2::R](mcan_ndat2::R) reader structure"]
impl crate::Readable for MCAN_NDAT2 {}
#[doc = "`write(|w| ..)` method takes [mcan_ndat2::W](mcan_ndat2::W) writer structure"]
impl crate::Writable for MCAN_NDAT2 {}
#[doc = "New Data 2 Register"]
pub mod mcan_ndat2;
#[doc = "Receive FIFO 0 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_rxf0c](mcan_rxf0c) module"]
pub type MCAN_RXF0C = crate::Reg<u32, _MCAN_RXF0C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_RXF0C;
#[doc = "`read()` method returns [mcan_rxf0c::R](mcan_rxf0c::R) reader structure"]
impl crate::Readable for MCAN_RXF0C {}
#[doc = "`write(|w| ..)` method takes [mcan_rxf0c::W](mcan_rxf0c::W) writer structure"]
impl crate::Writable for MCAN_RXF0C {}
#[doc = "Receive FIFO 0 Configuration Register"]
pub mod mcan_rxf0c;
#[doc = "Receive FIFO 0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_rxf0s](mcan_rxf0s) module"]
pub type MCAN_RXF0S = crate::Reg<u32, _MCAN_RXF0S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_RXF0S;
#[doc = "`read()` method returns [mcan_rxf0s::R](mcan_rxf0s::R) reader structure"]
impl crate::Readable for MCAN_RXF0S {}
#[doc = "Receive FIFO 0 Status Register"]
pub mod mcan_rxf0s;
#[doc = "Receive FIFO 0 Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_rxf0a](mcan_rxf0a) module"]
pub type MCAN_RXF0A = crate::Reg<u32, _MCAN_RXF0A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_RXF0A;
#[doc = "`read()` method returns [mcan_rxf0a::R](mcan_rxf0a::R) reader structure"]
impl crate::Readable for MCAN_RXF0A {}
#[doc = "`write(|w| ..)` method takes [mcan_rxf0a::W](mcan_rxf0a::W) writer structure"]
impl crate::Writable for MCAN_RXF0A {}
#[doc = "Receive FIFO 0 Acknowledge Register"]
pub mod mcan_rxf0a;
#[doc = "Receive Rx Buffer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_rxbc](mcan_rxbc) module"]
pub type MCAN_RXBC = crate::Reg<u32, _MCAN_RXBC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_RXBC;
#[doc = "`read()` method returns [mcan_rxbc::R](mcan_rxbc::R) reader structure"]
impl crate::Readable for MCAN_RXBC {}
#[doc = "`write(|w| ..)` method takes [mcan_rxbc::W](mcan_rxbc::W) writer structure"]
impl crate::Writable for MCAN_RXBC {}
#[doc = "Receive Rx Buffer Configuration Register"]
pub mod mcan_rxbc;
#[doc = "Receive FIFO 1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_rxf1c](mcan_rxf1c) module"]
pub type MCAN_RXF1C = crate::Reg<u32, _MCAN_RXF1C>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_RXF1C;
#[doc = "`read()` method returns [mcan_rxf1c::R](mcan_rxf1c::R) reader structure"]
impl crate::Readable for MCAN_RXF1C {}
#[doc = "`write(|w| ..)` method takes [mcan_rxf1c::W](mcan_rxf1c::W) writer structure"]
impl crate::Writable for MCAN_RXF1C {}
#[doc = "Receive FIFO 1 Configuration Register"]
pub mod mcan_rxf1c;
#[doc = "Receive FIFO 1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_rxf1s](mcan_rxf1s) module"]
pub type MCAN_RXF1S = crate::Reg<u32, _MCAN_RXF1S>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_RXF1S;
#[doc = "`read()` method returns [mcan_rxf1s::R](mcan_rxf1s::R) reader structure"]
impl crate::Readable for MCAN_RXF1S {}
#[doc = "Receive FIFO 1 Status Register"]
pub mod mcan_rxf1s;
#[doc = "Receive FIFO 1 Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_rxf1a](mcan_rxf1a) module"]
pub type MCAN_RXF1A = crate::Reg<u32, _MCAN_RXF1A>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_RXF1A;
#[doc = "`read()` method returns [mcan_rxf1a::R](mcan_rxf1a::R) reader structure"]
impl crate::Readable for MCAN_RXF1A {}
#[doc = "`write(|w| ..)` method takes [mcan_rxf1a::W](mcan_rxf1a::W) writer structure"]
impl crate::Writable for MCAN_RXF1A {}
#[doc = "Receive FIFO 1 Acknowledge Register"]
pub mod mcan_rxf1a;
#[doc = "Receive Buffer / FIFO Element Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_rxesc](mcan_rxesc) module"]
pub type MCAN_RXESC = crate::Reg<u32, _MCAN_RXESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_RXESC;
#[doc = "`read()` method returns [mcan_rxesc::R](mcan_rxesc::R) reader structure"]
impl crate::Readable for MCAN_RXESC {}
#[doc = "`write(|w| ..)` method takes [mcan_rxesc::W](mcan_rxesc::W) writer structure"]
impl crate::Writable for MCAN_RXESC {}
#[doc = "Receive Buffer / FIFO Element Size Configuration Register"]
pub mod mcan_rxesc;
#[doc = "Transmit Buffer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txbc](mcan_txbc) module"]
pub type MCAN_TXBC = crate::Reg<u32, _MCAN_TXBC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TXBC;
#[doc = "`read()` method returns [mcan_txbc::R](mcan_txbc::R) reader structure"]
impl crate::Readable for MCAN_TXBC {}
#[doc = "`write(|w| ..)` method takes [mcan_txbc::W](mcan_txbc::W) writer structure"]
impl crate::Writable for MCAN_TXBC {}
#[doc = "Transmit Buffer Configuration Register"]
pub mod mcan_txbc;
#[doc = "Transmit FIFO/Queue Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txfqs](mcan_txfqs) module"]
pub type MCAN_TXFQS = crate::Reg<u32, _MCAN_TXFQS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TXFQS;
#[doc = "`read()` method returns [mcan_txfqs::R](mcan_txfqs::R) reader structure"]
impl crate::Readable for MCAN_TXFQS {}
#[doc = "Transmit FIFO/Queue Status Register"]
pub mod mcan_txfqs;
#[doc = "Transmit Buffer Element Size Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txesc](mcan_txesc) module"]
pub type MCAN_TXESC = crate::Reg<u32, _MCAN_TXESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TXESC;
#[doc = "`read()` method returns [mcan_txesc::R](mcan_txesc::R) reader structure"]
impl crate::Readable for MCAN_TXESC {}
#[doc = "`write(|w| ..)` method takes [mcan_txesc::W](mcan_txesc::W) writer structure"]
impl crate::Writable for MCAN_TXESC {}
#[doc = "Transmit Buffer Element Size Configuration Register"]
pub mod mcan_txesc;
#[doc = "Transmit Buffer Request Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txbrp](mcan_txbrp) module"]
pub type MCAN_TXBRP = crate::Reg<u32, _MCAN_TXBRP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TXBRP;
#[doc = "`read()` method returns [mcan_txbrp::R](mcan_txbrp::R) reader structure"]
impl crate::Readable for MCAN_TXBRP {}
#[doc = "Transmit Buffer Request Pending Register"]
pub mod mcan_txbrp;
#[doc = "Transmit Buffer Add Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txbar](mcan_txbar) module"]
pub type MCAN_TXBAR = crate::Reg<u32, _MCAN_TXBAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TXBAR;
#[doc = "`read()` method returns [mcan_txbar::R](mcan_txbar::R) reader structure"]
impl crate::Readable for MCAN_TXBAR {}
#[doc = "`write(|w| ..)` method takes [mcan_txbar::W](mcan_txbar::W) writer structure"]
impl crate::Writable for MCAN_TXBAR {}
#[doc = "Transmit Buffer Add Request Register"]
pub mod mcan_txbar;
#[doc = "Transmit Buffer Cancellation Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txbcr](mcan_txbcr) module"]
pub type MCAN_TXBCR = crate::Reg<u32, _MCAN_TXBCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TXBCR;
#[doc = "`read()` method returns [mcan_txbcr::R](mcan_txbcr::R) reader structure"]
impl crate::Readable for MCAN_TXBCR {}
#[doc = "`write(|w| ..)` method takes [mcan_txbcr::W](mcan_txbcr::W) writer structure"]
impl crate::Writable for MCAN_TXBCR {}
#[doc = "Transmit Buffer Cancellation Request Register"]
pub mod mcan_txbcr;
#[doc = "Transmit Buffer Transmission Occurred Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txbto](mcan_txbto) module"]
pub type MCAN_TXBTO = crate::Reg<u32, _MCAN_TXBTO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TXBTO;
#[doc = "`read()` method returns [mcan_txbto::R](mcan_txbto::R) reader structure"]
impl crate::Readable for MCAN_TXBTO {}
#[doc = "Transmit Buffer Transmission Occurred Register"]
pub mod mcan_txbto;
#[doc = "Transmit Buffer Cancellation Finished Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txbcf](mcan_txbcf) module"]
pub type MCAN_TXBCF = crate::Reg<u32, _MCAN_TXBCF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TXBCF;
#[doc = "`read()` method returns [mcan_txbcf::R](mcan_txbcf::R) reader structure"]
impl crate::Readable for MCAN_TXBCF {}
#[doc = "Transmit Buffer Cancellation Finished Register"]
pub mod mcan_txbcf;
#[doc = "Transmit Buffer Transmission Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txbtie](mcan_txbtie) module"]
pub type MCAN_TXBTIE = crate::Reg<u32, _MCAN_TXBTIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TXBTIE;
#[doc = "`read()` method returns [mcan_txbtie::R](mcan_txbtie::R) reader structure"]
impl crate::Readable for MCAN_TXBTIE {}
#[doc = "`write(|w| ..)` method takes [mcan_txbtie::W](mcan_txbtie::W) writer structure"]
impl crate::Writable for MCAN_TXBTIE {}
#[doc = "Transmit Buffer Transmission Interrupt Enable Register"]
pub mod mcan_txbtie;
#[doc = "Transmit Buffer Cancellation Finished Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txbcie](mcan_txbcie) module"]
pub type MCAN_TXBCIE = crate::Reg<u32, _MCAN_TXBCIE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TXBCIE;
#[doc = "`read()` method returns [mcan_txbcie::R](mcan_txbcie::R) reader structure"]
impl crate::Readable for MCAN_TXBCIE {}
#[doc = "`write(|w| ..)` method takes [mcan_txbcie::W](mcan_txbcie::W) writer structure"]
impl crate::Writable for MCAN_TXBCIE {}
#[doc = "Transmit Buffer Cancellation Finished Interrupt Enable Register"]
pub mod mcan_txbcie;
#[doc = "Transmit Event FIFO Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txefc](mcan_txefc) module"]
pub type MCAN_TXEFC = crate::Reg<u32, _MCAN_TXEFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TXEFC;
#[doc = "`read()` method returns [mcan_txefc::R](mcan_txefc::R) reader structure"]
impl crate::Readable for MCAN_TXEFC {}
#[doc = "`write(|w| ..)` method takes [mcan_txefc::W](mcan_txefc::W) writer structure"]
impl crate::Writable for MCAN_TXEFC {}
#[doc = "Transmit Event FIFO Configuration Register"]
pub mod mcan_txefc;
#[doc = "Transmit Event FIFO Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txefs](mcan_txefs) module"]
pub type MCAN_TXEFS = crate::Reg<u32, _MCAN_TXEFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TXEFS;
#[doc = "`read()` method returns [mcan_txefs::R](mcan_txefs::R) reader structure"]
impl crate::Readable for MCAN_TXEFS {}
#[doc = "Transmit Event FIFO Status Register"]
pub mod mcan_txefs;
#[doc = "Transmit Event FIFO Acknowledge Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcan_txefa](mcan_txefa) module"]
pub type MCAN_TXEFA = crate::Reg<u32, _MCAN_TXEFA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCAN_TXEFA;
#[doc = "`read()` method returns [mcan_txefa::R](mcan_txefa::R) reader structure"]
impl crate::Readable for MCAN_TXEFA {}
#[doc = "`write(|w| ..)` method takes [mcan_txefa::W](mcan_txefa::W) writer structure"]
impl crate::Writable for MCAN_TXEFA {}
#[doc = "Transmit Event FIFO Acknowledge Register"]
pub mod mcan_txefa;
