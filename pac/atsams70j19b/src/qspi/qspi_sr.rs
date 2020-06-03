#[doc = "Reader of register QSPI_SR"]
pub type R = crate::R<u32, super::QSPI_SR>;
#[doc = "Reader of field `RDRF`"]
pub type RDRF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TDRE`"]
pub type TDRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXEMPTY`"]
pub type TXEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRES`"]
pub type OVRES_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSR`"]
pub type CSR_R = crate::R<bool, bool>;
#[doc = "Reader of field `CSS`"]
pub type CSS_R = crate::R<bool, bool>;
#[doc = "Reader of field `INSTRE`"]
pub type INSTRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `QSPIENS`"]
pub type QSPIENS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Receive Data Register Full (cleared by reading SPI_RDR)"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty (cleared by writing SPI_TDR)"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmission Registers Empty (cleared by writing SPI_TDR)"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Status (cleared on read)"]
    #[inline(always)]
    pub fn ovres(&self) -> OVRES_R {
        OVRES_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Chip Select Rise (cleared on read)"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Chip Select Status"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Instruction End Status (cleared on read)"]
    #[inline(always)]
    pub fn instre(&self) -> INSTRE_R {
        INSTRE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 24 - QSPI Enable Status"]
    #[inline(always)]
    pub fn qspiens(&self) -> QSPIENS_R {
        QSPIENS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
