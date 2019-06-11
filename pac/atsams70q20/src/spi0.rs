#[doc = r" Register block"]
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
    _reserved0: [u8; 16usize],
    #[doc = "0x30 - Chip Select Register 0"]
    pub spi_csr: [SPI_CSR; 4],
    _reserved1: [u8; 164usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub spi_wpmr: SPI_WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub spi_wpsr: SPI_WPSR,
}
#[doc = "Control Register"]
pub struct SPI_CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod spi_cr;
#[doc = "Mode Register"]
pub struct SPI_MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod spi_mr;
#[doc = "Receive Data Register"]
pub struct SPI_RDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Data Register"]
pub mod spi_rdr;
#[doc = "Transmit Data Register"]
pub struct SPI_TDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Data Register"]
pub mod spi_tdr;
#[doc = "Status Register"]
pub struct SPI_SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod spi_sr;
#[doc = "Interrupt Enable Register"]
pub struct SPI_IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod spi_ier;
#[doc = "Interrupt Disable Register"]
pub struct SPI_IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod spi_idr;
#[doc = "Interrupt Mask Register"]
pub struct SPI_IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod spi_imr;
#[doc = "Chip Select Register 0"]
pub struct SPI_CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip Select Register 0"]
pub mod spi_csr;
#[doc = "Write Protection Mode Register"]
pub struct SPI_WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Mode Register"]
pub mod spi_wpmr;
#[doc = "Write Protection Status Register"]
pub struct SPI_WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Status Register"]
pub mod spi_wpsr;
