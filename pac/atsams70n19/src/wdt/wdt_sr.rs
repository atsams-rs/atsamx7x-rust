#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::WDT_SR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type WDUNF_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type WDERR_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Watchdog Underflow (cleared on read)"]
    #[inline(always)]
    pub fn wdunf(&self) -> WDUNF_R {
        WDUNF_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Watchdog Error (cleared on read)"]
    #[inline(always)]
    pub fn wderr(&self) -> WDERR_R {
        WDERR_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
}
