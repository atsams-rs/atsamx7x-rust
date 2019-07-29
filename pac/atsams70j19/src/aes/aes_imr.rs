#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AES_IMR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type DATRDY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type URAD_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type TAGRDY_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Data Ready Interrupt Mask"]
    #[inline(always)]
    pub fn datrdy(&self) -> DATRDY_R {
        DATRDY_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 8 - Unspecified Register Access Detection Interrupt Mask"]
    #[inline(always)]
    pub fn urad(&self) -> URAD_R {
        URAD_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - GCM Tag Ready Interrupt Mask"]
    #[inline(always)]
    pub fn tagrdy(&self) -> TAGRDY_R {
        TAGRDY_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
}
