#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ICTR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct INTLINESNUMR {
    bits: u8,
}
impl INTLINESNUMR {
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
    #[doc = "Bits 0:3 - Total number of interrupt lines supported by an implementation, defined in groups of 32"]
    #[inline]
    pub fn intlinesnum(&self) -> INTLINESNUMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INTLINESNUMR { bits }
    }
}
