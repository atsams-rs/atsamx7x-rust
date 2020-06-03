#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub spi_cr: SPI_CR,
    #[doc = "0x04 - Mode Register"]
    pub spi_mr: SPI_MR,
    #[doc = "0x08 - Receive Data Register"]
    pub spi_rdr: SPI_RDR,
    #[doc = "0x0c - Transmit Data Register"]
    pub spi_tdr: SPI_TDR,
    #[doc = "0x10 - Status Register"]
    pub spi_sr: SPI_SR,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub spi_ier: SPI_IER,
    #[doc = "0x18 - Interrupt Disable Register"]
    pub spi_idr: SPI_IDR,
    #[doc = "0x1c - Interrupt Mask Register"]
    pub spi_imr: SPI_IMR,
    _reserved8: [u8; 16usize],
    #[doc = "0x30 - Chip Select Register 0"]
    pub spi_csr: [SPI_CSR; 4],
    _reserved9: [u8; 164usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub spi_wpmr: SPI_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub spi_wpsr: SPI_WPSR,
}
#[doc = "Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cr](spi_cr) module"]
pub type SPI_CR = crate::Reg<u32, _SPI_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CR;
#[doc = "`write(|w| ..)` method takes [spi_cr::W](spi_cr::W) writer structure"]
impl crate::Writable for SPI_CR {}
#[doc = "Control Register"]
pub mod spi_cr;
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mr](spi_mr) module"]
pub type SPI_MR = crate::Reg<u32, _SPI_MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_MR;
#[doc = "`read()` method returns [spi_mr::R](spi_mr::R) reader structure"]
impl crate::Readable for SPI_MR {}
#[doc = "`write(|w| ..)` method takes [spi_mr::W](spi_mr::W) writer structure"]
impl crate::Writable for SPI_MR {}
#[doc = "Mode Register"]
pub mod spi_mr;
#[doc = "Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_rdr](spi_rdr) module"]
pub type SPI_RDR = crate::Reg<u32, _SPI_RDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_RDR;
#[doc = "`read()` method returns [spi_rdr::R](spi_rdr::R) reader structure"]
impl crate::Readable for SPI_RDR {}
#[doc = "Receive Data Register"]
pub mod spi_rdr;
#[doc = "Transmit Data Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_tdr](spi_tdr) module"]
pub type SPI_TDR = crate::Reg<u32, _SPI_TDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_TDR;
#[doc = "`write(|w| ..)` method takes [spi_tdr::W](spi_tdr::W) writer structure"]
impl crate::Writable for SPI_TDR {}
#[doc = "Transmit Data Register"]
pub mod spi_tdr;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_sr](spi_sr) module"]
pub type SPI_SR = crate::Reg<u32, _SPI_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_SR;
#[doc = "`read()` method returns [spi_sr::R](spi_sr::R) reader structure"]
impl crate::Readable for SPI_SR {}
#[doc = "Status Register"]
pub mod spi_sr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ier](spi_ier) module"]
pub type SPI_IER = crate::Reg<u32, _SPI_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_IER;
#[doc = "`write(|w| ..)` method takes [spi_ier::W](spi_ier::W) writer structure"]
impl crate::Writable for SPI_IER {}
#[doc = "Interrupt Enable Register"]
pub mod spi_ier;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_idr](spi_idr) module"]
pub type SPI_IDR = crate::Reg<u32, _SPI_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_IDR;
#[doc = "`write(|w| ..)` method takes [spi_idr::W](spi_idr::W) writer structure"]
impl crate::Writable for SPI_IDR {}
#[doc = "Interrupt Disable Register"]
pub mod spi_idr;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_imr](spi_imr) module"]
pub type SPI_IMR = crate::Reg<u32, _SPI_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_IMR;
#[doc = "`read()` method returns [spi_imr::R](spi_imr::R) reader structure"]
impl crate::Readable for SPI_IMR {}
#[doc = "Interrupt Mask Register"]
pub mod spi_imr;
#[doc = "Chip Select Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_csr](spi_csr) module"]
pub type SPI_CSR = crate::Reg<u32, _SPI_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_CSR;
#[doc = "`read()` method returns [spi_csr::R](spi_csr::R) reader structure"]
impl crate::Readable for SPI_CSR {}
#[doc = "`write(|w| ..)` method takes [spi_csr::W](spi_csr::W) writer structure"]
impl crate::Writable for SPI_CSR {}
#[doc = "Chip Select Register 0"]
pub mod spi_csr;
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_wpmr](spi_wpmr) module"]
pub type SPI_WPMR = crate::Reg<u32, _SPI_WPMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_WPMR;
#[doc = "`read()` method returns [spi_wpmr::R](spi_wpmr::R) reader structure"]
impl crate::Readable for SPI_WPMR {}
#[doc = "`write(|w| ..)` method takes [spi_wpmr::W](spi_wpmr::W) writer structure"]
impl crate::Writable for SPI_WPMR {}
#[doc = "Write Protection Mode Register"]
pub mod spi_wpmr;
#[doc = "Write Protection Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_wpsr](spi_wpsr) module"]
pub type SPI_WPSR = crate::Reg<u32, _SPI_WPSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPI_WPSR;
#[doc = "`read()` method returns [spi_wpsr::R](spi_wpsr::R) reader structure"]
impl crate::Readable for SPI_WPSR {}
#[doc = "Write Protection Status Register"]
pub mod spi_wpsr;
