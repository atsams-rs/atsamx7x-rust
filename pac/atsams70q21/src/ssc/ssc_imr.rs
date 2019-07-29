#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SSC_IMR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type TXRDY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TXEMPTY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXRDY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVRUN_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CP0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CP1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TXSYN_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RXSYN_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit Ready Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Empty Interrupt Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Ready Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn ovrun(&self) -> OVRUN_R {
        OVRUN_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Compare 0 Interrupt Mask"]
    #[inline(always)]
    pub fn cp0(&self) -> CP0_R {
        CP0_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Compare 1 Interrupt Mask"]
    #[inline(always)]
    pub fn cp1(&self) -> CP1_R {
        CP1_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Tx Sync Interrupt Mask"]
    #[inline(always)]
    pub fn txsyn(&self) -> TXSYN_R {
        TXSYN_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Rx Sync Interrupt Mask"]
    #[inline(always)]
    pub fn rxsyn(&self) -> RXSYN_R {
        RXSYN_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
}
