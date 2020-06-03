#[doc = "Reader of register MCAN_TXBCF"]
pub type R = crate::R<u32, super::MCAN_TXBCF>;
#[doc = "Reader of field `CF0`"]
pub type CF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF1`"]
pub type CF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF2`"]
pub type CF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF3`"]
pub type CF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF4`"]
pub type CF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF5`"]
pub type CF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF6`"]
pub type CF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF7`"]
pub type CF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF8`"]
pub type CF8_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF9`"]
pub type CF9_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF10`"]
pub type CF10_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF11`"]
pub type CF11_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF12`"]
pub type CF12_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF13`"]
pub type CF13_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF14`"]
pub type CF14_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF15`"]
pub type CF15_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF16`"]
pub type CF16_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF17`"]
pub type CF17_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF18`"]
pub type CF18_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF19`"]
pub type CF19_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF20`"]
pub type CF20_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF21`"]
pub type CF21_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF22`"]
pub type CF22_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF23`"]
pub type CF23_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF24`"]
pub type CF24_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF25`"]
pub type CF25_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF26`"]
pub type CF26_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF27`"]
pub type CF27_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF28`"]
pub type CF28_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF29`"]
pub type CF29_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF30`"]
pub type CF30_R = crate::R<bool, bool>;
#[doc = "Reader of field `CF31`"]
pub type CF31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Cancellation Finished for Transmit Buffer 0"]
    #[inline(always)]
    pub fn cf0(&self) -> CF0_R {
        CF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Cancellation Finished for Transmit Buffer 1"]
    #[inline(always)]
    pub fn cf1(&self) -> CF1_R {
        CF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Cancellation Finished for Transmit Buffer 2"]
    #[inline(always)]
    pub fn cf2(&self) -> CF2_R {
        CF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Cancellation Finished for Transmit Buffer 3"]
    #[inline(always)]
    pub fn cf3(&self) -> CF3_R {
        CF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Cancellation Finished for Transmit Buffer 4"]
    #[inline(always)]
    pub fn cf4(&self) -> CF4_R {
        CF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Cancellation Finished for Transmit Buffer 5"]
    #[inline(always)]
    pub fn cf5(&self) -> CF5_R {
        CF5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Cancellation Finished for Transmit Buffer 6"]
    #[inline(always)]
    pub fn cf6(&self) -> CF6_R {
        CF6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Cancellation Finished for Transmit Buffer 7"]
    #[inline(always)]
    pub fn cf7(&self) -> CF7_R {
        CF7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Cancellation Finished for Transmit Buffer 8"]
    #[inline(always)]
    pub fn cf8(&self) -> CF8_R {
        CF8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Cancellation Finished for Transmit Buffer 9"]
    #[inline(always)]
    pub fn cf9(&self) -> CF9_R {
        CF9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Cancellation Finished for Transmit Buffer 10"]
    #[inline(always)]
    pub fn cf10(&self) -> CF10_R {
        CF10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Cancellation Finished for Transmit Buffer 11"]
    #[inline(always)]
    pub fn cf11(&self) -> CF11_R {
        CF11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Cancellation Finished for Transmit Buffer 12"]
    #[inline(always)]
    pub fn cf12(&self) -> CF12_R {
        CF12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Cancellation Finished for Transmit Buffer 13"]
    #[inline(always)]
    pub fn cf13(&self) -> CF13_R {
        CF13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Cancellation Finished for Transmit Buffer 14"]
    #[inline(always)]
    pub fn cf14(&self) -> CF14_R {
        CF14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Cancellation Finished for Transmit Buffer 15"]
    #[inline(always)]
    pub fn cf15(&self) -> CF15_R {
        CF15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Cancellation Finished for Transmit Buffer 16"]
    #[inline(always)]
    pub fn cf16(&self) -> CF16_R {
        CF16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Cancellation Finished for Transmit Buffer 17"]
    #[inline(always)]
    pub fn cf17(&self) -> CF17_R {
        CF17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Cancellation Finished for Transmit Buffer 18"]
    #[inline(always)]
    pub fn cf18(&self) -> CF18_R {
        CF18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Cancellation Finished for Transmit Buffer 19"]
    #[inline(always)]
    pub fn cf19(&self) -> CF19_R {
        CF19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Cancellation Finished for Transmit Buffer 20"]
    #[inline(always)]
    pub fn cf20(&self) -> CF20_R {
        CF20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Cancellation Finished for Transmit Buffer 21"]
    #[inline(always)]
    pub fn cf21(&self) -> CF21_R {
        CF21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Cancellation Finished for Transmit Buffer 22"]
    #[inline(always)]
    pub fn cf22(&self) -> CF22_R {
        CF22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Cancellation Finished for Transmit Buffer 23"]
    #[inline(always)]
    pub fn cf23(&self) -> CF23_R {
        CF23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Cancellation Finished for Transmit Buffer 24"]
    #[inline(always)]
    pub fn cf24(&self) -> CF24_R {
        CF24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Cancellation Finished for Transmit Buffer 25"]
    #[inline(always)]
    pub fn cf25(&self) -> CF25_R {
        CF25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Cancellation Finished for Transmit Buffer 26"]
    #[inline(always)]
    pub fn cf26(&self) -> CF26_R {
        CF26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Cancellation Finished for Transmit Buffer 27"]
    #[inline(always)]
    pub fn cf27(&self) -> CF27_R {
        CF27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Cancellation Finished for Transmit Buffer 28"]
    #[inline(always)]
    pub fn cf28(&self) -> CF28_R {
        CF28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Cancellation Finished for Transmit Buffer 29"]
    #[inline(always)]
    pub fn cf29(&self) -> CF29_R {
        CF29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Cancellation Finished for Transmit Buffer 30"]
    #[inline(always)]
    pub fn cf30(&self) -> CF30_R {
        CF30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Cancellation Finished for Transmit Buffer 31"]
    #[inline(always)]
    pub fn cf31(&self) -> CF31_R {
        CF31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
