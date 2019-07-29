#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ACC_ISR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type CE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SCO_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type MASK_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Comparison Edge (cleared on read)"]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Synchronized Comparator Output"]
    #[inline(always)]
    pub fn sco(&self) -> SCO_R {
        SCO_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Flag Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
