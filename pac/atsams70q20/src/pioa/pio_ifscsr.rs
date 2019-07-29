#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PIO_IFSCSR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type P0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P3_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P4_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P5_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P6_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P7_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P8_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P9_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P10_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P11_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P12_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P13_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P14_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P15_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P16_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P17_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P18_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P19_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P20_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P21_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P22_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P23_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P24_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P25_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P26_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P27_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P28_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P29_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P30_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type P31_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p0(&self) -> P0_R {
        P0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p1(&self) -> P1_R {
        P1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p2(&self) -> P2_R {
        P2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p3(&self) -> P3_R {
        P3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p4(&self) -> P4_R {
        P4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p5(&self) -> P5_R {
        P5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p6(&self) -> P6_R {
        P6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p7(&self) -> P7_R {
        P7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p8(&self) -> P8_R {
        P8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p9(&self) -> P9_R {
        P9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p10(&self) -> P10_R {
        P10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p11(&self) -> P11_R {
        P11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p12(&self) -> P12_R {
        P12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p13(&self) -> P13_R {
        P13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p14(&self) -> P14_R {
        P14_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p15(&self) -> P15_R {
        P15_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p16(&self) -> P16_R {
        P16_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p17(&self) -> P17_R {
        P17_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p18(&self) -> P18_R {
        P18_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p19(&self) -> P19_R {
        P19_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p20(&self) -> P20_R {
        P20_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p21(&self) -> P21_R {
        P21_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p22(&self) -> P22_R {
        P22_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p23(&self) -> P23_R {
        P23_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p24(&self) -> P24_R {
        P24_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p25(&self) -> P25_R {
        P25_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p26(&self) -> P26_R {
        P26_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p27(&self) -> P27_R {
        P27_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p28(&self) -> P28_R {
        P28_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p29(&self) -> P29_R {
        P29_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p30(&self) -> P30_R {
        P30_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Glitch or Debouncing Filter Selection Status"]
    #[inline(always)]
    pub fn p31(&self) -> P31_R {
        P31_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
