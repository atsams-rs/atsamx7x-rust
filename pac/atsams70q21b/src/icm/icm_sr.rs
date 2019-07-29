#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ICM_SR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type ENABLE_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RAWRMDIS_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type RMDIS_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - ICM Controller Enable Register"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Region Monitoring Disabled Raw Status"]
    #[inline(always)]
    pub fn rawrmdis(&self) -> RAWRMDIS_R {
        RAWRMDIS_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Region Monitoring Disabled Status"]
    #[inline(always)]
    pub fn rmdis(&self) -> RMDIS_R {
        RMDIS_R::new(((self.bits() >> 12) & 0x0f) as u8)
    }
}
