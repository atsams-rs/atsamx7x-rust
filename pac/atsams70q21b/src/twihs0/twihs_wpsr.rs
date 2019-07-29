#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TWIHS_WPSR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type WPVS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type WPVSRC_R = crate::FR<u32, u32>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Write Protection Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WPVS_R {
        WPVS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 8:31 - Write Protection Violation Source"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WPVSRC_R {
        WPVSRC_R::new(((self.bits() >> 8) & 0x00ff_ffff) as u32)
    }
}
