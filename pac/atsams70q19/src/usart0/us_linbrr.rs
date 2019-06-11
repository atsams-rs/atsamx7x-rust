#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::US_LINBRR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct LINCDR {
    bits: u16,
}
impl LINCDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LINFPR {
    bits: u8,
}
impl LINFPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - Clock Divider after Synchronization"]
    #[inline]
    pub fn lincd(&self) -> LINCDR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        LINCDR { bits }
    }
    #[doc = "Bits 16:18 - Fractional Part after Synchronization"]
    #[inline]
    pub fn linfp(&self) -> LINFPR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LINFPR { bits }
    }
}
