#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::US_IMR_LON_MODE {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type RXRDY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TXRDY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVRE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TXEMPTY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RIIC_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DSRIC_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DCDIC_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type LSFE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type LCRCE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type LTXD_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type LCOL_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type LFET_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type LRXD_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type LBLOVFE_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - RXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Mask"]
    #[inline(always)]
    pub fn riic(&self) -> RIIC_R {
        RIIC_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Mask"]
    #[inline(always)]
    pub fn dsric(&self) -> DSRIC_R {
        DSRIC_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Interrupt Mask"]
    #[inline(always)]
    pub fn dcdic(&self) -> DCDIC_R {
        DCDIC_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LON Short Frame Error Interrupt Mask"]
    #[inline(always)]
    pub fn lsfe(&self) -> LSFE_R {
        LSFE_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LON CRC Error Interrupt Mask"]
    #[inline(always)]
    pub fn lcrce(&self) -> LCRCE_R {
        LCRCE_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 24 - LON Transmission Done Interrupt Mask"]
    #[inline(always)]
    pub fn ltxd(&self) -> LTXD_R {
        LTXD_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - LON Collision Interrupt Mask"]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - LON Frame Early Termination Interrupt Mask"]
    #[inline(always)]
    pub fn lfet(&self) -> LFET_R {
        LFET_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LON Reception Done Interrupt Mask"]
    #[inline(always)]
    pub fn lrxd(&self) -> LRXD_R {
        LRXD_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - LON Backlog Overflow Error Interrupt Mask"]
    #[inline(always)]
    pub fn lblovfe(&self) -> LBLOVFE_R {
        LBLOVFE_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
}
