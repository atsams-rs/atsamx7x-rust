#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PMC_SLPWK_SR1 {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type PID32_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID33_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID34_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID40_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID41_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID42_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID43_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID44_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID45_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID46_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID47_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID48_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID49_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID50_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID51_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID52_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID56_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID57_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID58_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID59_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PID60_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Peripheral 32 SleepWalking Status"]
    #[inline(always)]
    pub fn pid32(&self) -> PID32_R {
        PID32_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Peripheral 33 SleepWalking Status"]
    #[inline(always)]
    pub fn pid33(&self) -> PID33_R {
        PID33_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Peripheral 34 SleepWalking Status"]
    #[inline(always)]
    pub fn pid34(&self) -> PID34_R {
        PID34_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Peripheral 40 SleepWalking Status"]
    #[inline(always)]
    pub fn pid40(&self) -> PID40_R {
        PID40_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Peripheral 41 SleepWalking Status"]
    #[inline(always)]
    pub fn pid41(&self) -> PID41_R {
        PID41_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Peripheral 42 SleepWalking Status"]
    #[inline(always)]
    pub fn pid42(&self) -> PID42_R {
        PID42_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Peripheral 43 SleepWalking Status"]
    #[inline(always)]
    pub fn pid43(&self) -> PID43_R {
        PID43_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Peripheral 44 SleepWalking Status"]
    #[inline(always)]
    pub fn pid44(&self) -> PID44_R {
        PID44_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Peripheral 45 SleepWalking Status"]
    #[inline(always)]
    pub fn pid45(&self) -> PID45_R {
        PID45_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Peripheral 46 SleepWalking Status"]
    #[inline(always)]
    pub fn pid46(&self) -> PID46_R {
        PID46_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Peripheral 47 SleepWalking Status"]
    #[inline(always)]
    pub fn pid47(&self) -> PID47_R {
        PID47_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Peripheral 48 SleepWalking Status"]
    #[inline(always)]
    pub fn pid48(&self) -> PID48_R {
        PID48_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Peripheral 49 SleepWalking Status"]
    #[inline(always)]
    pub fn pid49(&self) -> PID49_R {
        PID49_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Peripheral 50 SleepWalking Status"]
    #[inline(always)]
    pub fn pid50(&self) -> PID50_R {
        PID50_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Peripheral 51 SleepWalking Status"]
    #[inline(always)]
    pub fn pid51(&self) -> PID51_R {
        PID51_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Peripheral 52 SleepWalking Status"]
    #[inline(always)]
    pub fn pid52(&self) -> PID52_R {
        PID52_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Peripheral 56 SleepWalking Status"]
    #[inline(always)]
    pub fn pid56(&self) -> PID56_R {
        PID56_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Peripheral 57 SleepWalking Status"]
    #[inline(always)]
    pub fn pid57(&self) -> PID57_R {
        PID57_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Peripheral 58 SleepWalking Status"]
    #[inline(always)]
    pub fn pid58(&self) -> PID58_R {
        PID58_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Peripheral 59 SleepWalking Status"]
    #[inline(always)]
    pub fn pid59(&self) -> PID59_R {
        PID59_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Peripheral 60 SleepWalking Status"]
    #[inline(always)]
    pub fn pid60(&self) -> PID60_R {
        PID60_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
}
