#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::EEFC_FSR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type FRDY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type FCMDE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type FLOCKE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type FLERR_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type UECCELSB_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MECCELSB_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type UECCEMSB_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MECCEMSB_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Flash Ready Status (cleared when Flash is busy)"]
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Flash Command Error Status (cleared on read or by writing EEFC_FCR)"]
    #[inline(always)]
    pub fn fcmde(&self) -> FCMDE_R {
        FCMDE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Flash Lock Error Status (cleared on read)"]
    #[inline(always)]
    pub fn flocke(&self) -> FLOCKE_R {
        FLOCKE_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Flash Error Status (cleared when a programming operation starts)"]
    #[inline(always)]
    pub fn flerr(&self) -> FLERR_R {
        FLERR_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Unique ECC Error on LSB Part of the Memory Flash Data Bus (cleared on read)"]
    #[inline(always)]
    pub fn ueccelsb(&self) -> UECCELSB_R {
        UECCELSB_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Multiple ECC Error on LSB Part of the Memory Flash Data Bus (cleared on read)"]
    #[inline(always)]
    pub fn meccelsb(&self) -> MECCELSB_R {
        MECCELSB_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Unique ECC Error on MSB Part of the Memory Flash Data Bus (cleared on read)"]
    #[inline(always)]
    pub fn ueccemsb(&self) -> UECCEMSB_R {
        UECCEMSB_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Multiple ECC Error on MSB Part of the Memory Flash Data Bus (cleared on read)"]
    #[inline(always)]
    pub fn meccemsb(&self) -> MECCEMSB_R {
        MECCEMSB_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
}
