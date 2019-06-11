#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AFEC_LCDR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct LDATAR {
    bits: u16,
}
impl LDATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CHNBR {
    bits: u8,
}
impl CHNBR {
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
    #[doc = "Bits 0:15 - Last Data Converted"]
    #[inline]
    pub fn ldata(&self) -> LDATAR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        LDATAR { bits }
    }
    #[doc = "Bits 24:27 - Channel Number"]
    #[inline]
    pub fn chnb(&self) -> CHNBR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CHNBR { bits }
    }
}
