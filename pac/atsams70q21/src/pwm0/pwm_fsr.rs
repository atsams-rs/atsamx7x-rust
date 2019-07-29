#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PWM_FSR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type FIV_R = crate::FR<u8, u8>;
#[doc = r"Reader of the field"]
pub type FS_R = crate::FR<u8, u8>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Fault Input Value"]
    #[inline(always)]
    pub fn fiv(&self) -> FIV_R {
        FIV_R::new((self.bits() & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Status"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits() >> 8) & 0xff) as u8)
    }
}
