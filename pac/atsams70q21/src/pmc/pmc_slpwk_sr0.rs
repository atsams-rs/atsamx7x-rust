#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PMC_SLPWK_SR0 {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type PID7_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID8_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID9_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID10_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID11_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID12_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID13_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID14_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID15_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID16_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID17_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID18_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID19_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID20_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID21_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID22_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID23_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID24_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID25_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID26_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID27_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID28_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID29_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID30_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID31_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 7 - Peripheral 7 SleepWalking Status"]
    #[inline(always)]
    pub fn pid7(&self) -> PID7_R {
        PID7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Peripheral 8 SleepWalking Status"]
    #[inline(always)]
    pub fn pid8(&self) -> PID8_R {
        PID8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Peripheral 9 SleepWalking Status"]
    #[inline(always)]
    pub fn pid9(&self) -> PID9_R {
        PID9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Peripheral 10 SleepWalking Status"]
    #[inline(always)]
    pub fn pid10(&self) -> PID10_R {
        PID10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Peripheral 11 SleepWalking Status"]
    #[inline(always)]
    pub fn pid11(&self) -> PID11_R {
        PID11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Peripheral 12 SleepWalking Status"]
    #[inline(always)]
    pub fn pid12(&self) -> PID12_R {
        PID12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Peripheral 13 SleepWalking Status"]
    #[inline(always)]
    pub fn pid13(&self) -> PID13_R {
        PID13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Peripheral 14 SleepWalking Status"]
    #[inline(always)]
    pub fn pid14(&self) -> PID14_R {
        PID14_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Peripheral 15 SleepWalking Status"]
    #[inline(always)]
    pub fn pid15(&self) -> PID15_R {
        PID15_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Peripheral 16 SleepWalking Status"]
    #[inline(always)]
    pub fn pid16(&self) -> PID16_R {
        PID16_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Peripheral 17 SleepWalking Status"]
    #[inline(always)]
    pub fn pid17(&self) -> PID17_R {
        PID17_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Peripheral 18 SleepWalking Status"]
    #[inline(always)]
    pub fn pid18(&self) -> PID18_R {
        PID18_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Peripheral 19 SleepWalking Status"]
    #[inline(always)]
    pub fn pid19(&self) -> PID19_R {
        PID19_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Peripheral 20 SleepWalking Status"]
    #[inline(always)]
    pub fn pid20(&self) -> PID20_R {
        PID20_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Peripheral 21 SleepWalking Status"]
    #[inline(always)]
    pub fn pid21(&self) -> PID21_R {
        PID21_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Peripheral 22 SleepWalking Status"]
    #[inline(always)]
    pub fn pid22(&self) -> PID22_R {
        PID22_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Peripheral 23 SleepWalking Status"]
    #[inline(always)]
    pub fn pid23(&self) -> PID23_R {
        PID23_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Peripheral 24 SleepWalking Status"]
    #[inline(always)]
    pub fn pid24(&self) -> PID24_R {
        PID24_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Peripheral 25 SleepWalking Status"]
    #[inline(always)]
    pub fn pid25(&self) -> PID25_R {
        PID25_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Peripheral 26 SleepWalking Status"]
    #[inline(always)]
    pub fn pid26(&self) -> PID26_R {
        PID26_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Peripheral 27 SleepWalking Status"]
    #[inline(always)]
    pub fn pid27(&self) -> PID27_R {
        PID27_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Peripheral 28 SleepWalking Status"]
    #[inline(always)]
    pub fn pid28(&self) -> PID28_R {
        PID28_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Peripheral 29 SleepWalking Status"]
    #[inline(always)]
    pub fn pid29(&self) -> PID29_R {
        PID29_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Peripheral 30 SleepWalking Status"]
    #[inline(always)]
    pub fn pid30(&self) -> PID30_R {
        PID30_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Peripheral 31 SleepWalking Status"]
    #[inline(always)]
    pub fn pid31(&self) -> PID31_R {
        PID31_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
