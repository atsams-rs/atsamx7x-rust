#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::QSPI_SR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type RDRF_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TDRE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TXEMPTY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVRES_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CSR_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CSS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type INSTRE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type QSPIENS_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Receive Data Register Full (cleared by reading SPI_RDR)"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty (cleared by writing SPI_TDR)"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmission Registers Empty (cleared by writing SPI_TDR)"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Status (cleared on read)"]
    #[inline(always)]
    pub fn ovres(&self) -> OVRES_R {
        OVRES_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Chip Select Rise (cleared on read)"]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Chip Select Status"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Instruction End Status (cleared on read)"]
    #[inline(always)]
    pub fn instre(&self) -> INSTRE_R {
        INSTRE_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 24 - QSPI Enable Status"]
    #[inline(always)]
    pub fn qspiens(&self) -> QSPIENS_R {
        QSPIENS_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
}
