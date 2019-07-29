#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::DACC_CHSR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type CH0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type CH1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DACRDY0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DACRDY1_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Channel 0 Status"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Status"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DAC Ready Flag"]
    #[inline(always)]
    pub fn dacrdy0(&self) -> DACRDY0_R {
        DACRDY0_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DAC Ready Flag"]
    #[inline(always)]
    pub fn dacrdy1(&self) -> DACRDY1_R {
        DACRDY1_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
}
