#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AFEC_ISR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type EOC0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EOC1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EOC2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EOC3_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EOC4_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EOC5_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EOC6_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EOC7_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EOC8_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EOC9_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EOC10_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type EOC11_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DRDY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type GOVRE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type COMPE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TEMPCHG_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - End of Conversion 0 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc0(&self) -> EOC0_R {
        EOC0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of Conversion 1 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of Conversion 2 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc2(&self) -> EOC2_R {
        EOC2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of Conversion 3 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc3(&self) -> EOC3_R {
        EOC3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End of Conversion 4 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc4(&self) -> EOC4_R {
        EOC4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of Conversion 5 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc5(&self) -> EOC5_R {
        EOC5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - End of Conversion 6 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc6(&self) -> EOC6_R {
        EOC6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - End of Conversion 7 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc7(&self) -> EOC7_R {
        EOC7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - End of Conversion 8 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc8(&self) -> EOC8_R {
        EOC8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - End of Conversion 9 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc9(&self) -> EOC9_R {
        EOC9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - End of Conversion 10 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc10(&self) -> EOC10_R {
        EOC10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - End of Conversion 11 (cleared by reading AFEC_CDRx)"]
    #[inline(always)]
    pub fn eoc11(&self) -> EOC11_R {
        EOC11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Data Ready (cleared by reading AFEC_LCDR)"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - General Overrun Error (cleared by reading AFEC_ISR)"]
    #[inline(always)]
    pub fn govre(&self) -> GOVRE_R {
        GOVRE_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Comparison Error (cleared by reading AFEC_ISR)"]
    #[inline(always)]
    pub fn compe(&self) -> COMPE_R {
        COMPE_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Temperature Change (cleared on read)"]
    #[inline(always)]
    pub fn tempchg(&self) -> TEMPCHG_R {
        TEMPCHG_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
}
