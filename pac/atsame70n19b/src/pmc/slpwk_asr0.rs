#[doc = "Register `SLPWK_ASR0` reader"]
pub struct R(crate::R<SLPWK_ASR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLPWK_ASR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLPWK_ASR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLPWK_ASR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PID7` reader - Peripheral 7 Activity Status"]
pub type PID7_R = crate::BitReader<bool>;
#[doc = "Field `PID8` reader - Peripheral 8 Activity Status"]
pub type PID8_R = crate::BitReader<bool>;
#[doc = "Field `PID10` reader - Peripheral 10 Activity Status"]
pub type PID10_R = crate::BitReader<bool>;
#[doc = "Field `PID11` reader - Peripheral 11 Activity Status"]
pub type PID11_R = crate::BitReader<bool>;
#[doc = "Field `PID13` reader - Peripheral 13 Activity Status"]
pub type PID13_R = crate::BitReader<bool>;
#[doc = "Field `PID14` reader - Peripheral 14 Activity Status"]
pub type PID14_R = crate::BitReader<bool>;
#[doc = "Field `PID15` reader - Peripheral 15 Activity Status"]
pub type PID15_R = crate::BitReader<bool>;
#[doc = "Field `PID16` reader - Peripheral 16 Activity Status"]
pub type PID16_R = crate::BitReader<bool>;
#[doc = "Field `PID18` reader - Peripheral 18 Activity Status"]
pub type PID18_R = crate::BitReader<bool>;
#[doc = "Field `PID19` reader - Peripheral 19 Activity Status"]
pub type PID19_R = crate::BitReader<bool>;
#[doc = "Field `PID20` reader - Peripheral 20 Activity Status"]
pub type PID20_R = crate::BitReader<bool>;
#[doc = "Field `PID21` reader - Peripheral 21 Activity Status"]
pub type PID21_R = crate::BitReader<bool>;
#[doc = "Field `PID22` reader - Peripheral 22 Activity Status"]
pub type PID22_R = crate::BitReader<bool>;
#[doc = "Field `PID23` reader - Peripheral 23 Activity Status"]
pub type PID23_R = crate::BitReader<bool>;
#[doc = "Field `PID24` reader - Peripheral 24 Activity Status"]
pub type PID24_R = crate::BitReader<bool>;
#[doc = "Field `PID25` reader - Peripheral 25 Activity Status"]
pub type PID25_R = crate::BitReader<bool>;
#[doc = "Field `PID26` reader - Peripheral 26 Activity Status"]
pub type PID26_R = crate::BitReader<bool>;
#[doc = "Field `PID27` reader - Peripheral 27 Activity Status"]
pub type PID27_R = crate::BitReader<bool>;
#[doc = "Field `PID28` reader - Peripheral 28 Activity Status"]
pub type PID28_R = crate::BitReader<bool>;
#[doc = "Field `PID29` reader - Peripheral 29 Activity Status"]
pub type PID29_R = crate::BitReader<bool>;
#[doc = "Field `PID30` reader - Peripheral 30 Activity Status"]
pub type PID30_R = crate::BitReader<bool>;
#[doc = "Field `PID31` reader - Peripheral 31 Activity Status"]
pub type PID31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 7 - Peripheral 7 Activity Status"]
    #[inline(always)]
    pub fn pid7(&self) -> PID7_R {
        PID7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Peripheral 8 Activity Status"]
    #[inline(always)]
    pub fn pid8(&self) -> PID8_R {
        PID8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Peripheral 10 Activity Status"]
    #[inline(always)]
    pub fn pid10(&self) -> PID10_R {
        PID10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Peripheral 11 Activity Status"]
    #[inline(always)]
    pub fn pid11(&self) -> PID11_R {
        PID11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Peripheral 13 Activity Status"]
    #[inline(always)]
    pub fn pid13(&self) -> PID13_R {
        PID13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Peripheral 14 Activity Status"]
    #[inline(always)]
    pub fn pid14(&self) -> PID14_R {
        PID14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Peripheral 15 Activity Status"]
    #[inline(always)]
    pub fn pid15(&self) -> PID15_R {
        PID15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Peripheral 16 Activity Status"]
    #[inline(always)]
    pub fn pid16(&self) -> PID16_R {
        PID16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Peripheral 18 Activity Status"]
    #[inline(always)]
    pub fn pid18(&self) -> PID18_R {
        PID18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Peripheral 19 Activity Status"]
    #[inline(always)]
    pub fn pid19(&self) -> PID19_R {
        PID19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Peripheral 20 Activity Status"]
    #[inline(always)]
    pub fn pid20(&self) -> PID20_R {
        PID20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Peripheral 21 Activity Status"]
    #[inline(always)]
    pub fn pid21(&self) -> PID21_R {
        PID21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Peripheral 22 Activity Status"]
    #[inline(always)]
    pub fn pid22(&self) -> PID22_R {
        PID22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Peripheral 23 Activity Status"]
    #[inline(always)]
    pub fn pid23(&self) -> PID23_R {
        PID23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Peripheral 24 Activity Status"]
    #[inline(always)]
    pub fn pid24(&self) -> PID24_R {
        PID24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Peripheral 25 Activity Status"]
    #[inline(always)]
    pub fn pid25(&self) -> PID25_R {
        PID25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Peripheral 26 Activity Status"]
    #[inline(always)]
    pub fn pid26(&self) -> PID26_R {
        PID26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Peripheral 27 Activity Status"]
    #[inline(always)]
    pub fn pid27(&self) -> PID27_R {
        PID27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Peripheral 28 Activity Status"]
    #[inline(always)]
    pub fn pid28(&self) -> PID28_R {
        PID28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Peripheral 29 Activity Status"]
    #[inline(always)]
    pub fn pid29(&self) -> PID29_R {
        PID29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Peripheral 30 Activity Status"]
    #[inline(always)]
    pub fn pid30(&self) -> PID30_R {
        PID30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Peripheral 31 Activity Status"]
    #[inline(always)]
    pub fn pid31(&self) -> PID31_R {
        PID31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SleepWalking Activity Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slpwk_asr0](index.html) module"]
pub struct SLPWK_ASR0_SPEC;
impl crate::RegisterSpec for SLPWK_ASR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slpwk_asr0::R](R) reader structure"]
impl crate::Readable for SLPWK_ASR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SLPWK_ASR0 to value 0"]
impl crate::Resettable for SLPWK_ASR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
