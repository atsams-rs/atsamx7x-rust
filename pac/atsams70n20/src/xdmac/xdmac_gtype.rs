#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::XDMAC_GTYPE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct NB_CHR {
    bits: u8,
}
impl NB_CHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FIFO_SZR {
    bits: u16,
}
impl FIFO_SZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NB_REQR {
    bits: u8,
}
impl NB_REQR {
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
    #[doc = "Bits 0:4 - Number of Channels Minus One"]
    #[inline]
    pub fn nb_ch(&self) -> NB_CHR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NB_CHR { bits }
    }
    #[doc = "Bits 5:15 - Number of Bytes"]
    #[inline]
    pub fn fifo_sz(&self) -> FIFO_SZR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FIFO_SZR { bits }
    }
    #[doc = "Bits 16:22 - Number of Peripheral Requests Minus One"]
    #[inline]
    pub fn nb_req(&self) -> NB_REQR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NB_REQR { bits }
    }
}
