#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TC_SR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type COVFS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type LOVRS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CPAS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CPBS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CPCS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type LDRAS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type LDRBS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type ETRGS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CLKSTA_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MTIOA_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MTIOB_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Counter Overflow Status (cleared on read)"]
    #[inline(always)]
    pub fn covfs(&self) -> COVFS_R {
        COVFS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Load Overrun Status (cleared on read)"]
    #[inline(always)]
    pub fn lovrs(&self) -> LOVRS_R {
        LOVRS_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RA Compare Status (cleared on read)"]
    #[inline(always)]
    pub fn cpas(&self) -> CPAS_R {
        CPAS_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RB Compare Status (cleared on read)"]
    #[inline(always)]
    pub fn cpbs(&self) -> CPBS_R {
        CPBS_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RC Compare Status (cleared on read)"]
    #[inline(always)]
    pub fn cpcs(&self) -> CPCS_R {
        CPCS_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RA Loading Status (cleared on read)"]
    #[inline(always)]
    pub fn ldras(&self) -> LDRAS_R {
        LDRAS_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RB Loading Status (cleared on read)"]
    #[inline(always)]
    pub fn ldrbs(&self) -> LDRBS_R {
        LDRBS_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Trigger Status (cleared on read)"]
    #[inline(always)]
    pub fn etrgs(&self) -> ETRGS_R {
        ETRGS_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Clock Enabling Status"]
    #[inline(always)]
    pub fn clksta(&self) -> CLKSTA_R {
        CLKSTA_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIOAx Mirror"]
    #[inline(always)]
    pub fn mtioa(&self) -> MTIOA_R {
        MTIOA_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIOBx Mirror"]
    #[inline(always)]
    pub fn mtiob(&self) -> MTIOB_R {
        MTIOB_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
}
