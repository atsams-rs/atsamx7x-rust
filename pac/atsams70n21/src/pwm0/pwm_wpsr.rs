#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PWM_WPSR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type WPSWS0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type WPSWS1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type WPSWS2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type WPSWS3_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type WPSWS4_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type WPSWS5_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type WPVS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type WPHWS0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type WPHWS1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type WPHWS2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type WPHWS3_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type WPHWS4_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type WPHWS5_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type WPVSRC_R = crate::FR<u16, u16>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws0(&self) -> WPSWS0_R {
        WPSWS0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws1(&self) -> WPSWS1_R {
        WPSWS1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws2(&self) -> WPSWS2_R {
        WPSWS2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws3(&self) -> WPSWS3_R {
        WPSWS3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws4(&self) -> WPSWS4_R {
        WPSWS4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write Protect SW Status"]
    #[inline(always)]
    pub fn wpsws5(&self) -> WPSWS5_R {
        WPSWS5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write Protect Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WPVS_R {
        WPVS_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws0(&self) -> WPHWS0_R {
        WPHWS0_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws1(&self) -> WPHWS1_R {
        WPHWS1_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws2(&self) -> WPHWS2_R {
        WPHWS2_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws3(&self) -> WPHWS3_R {
        WPHWS3_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws4(&self) -> WPHWS4_R {
        WPHWS4_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Write Protect HW Status"]
    #[inline(always)]
    pub fn wphws5(&self) -> WPHWS5_R {
        WPHWS5_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Write Protect Violation Source"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WPVSRC_R {
        WPVSRC_R::new(((self.bits() >> 16) & 0xffff) as u16)
    }
}
