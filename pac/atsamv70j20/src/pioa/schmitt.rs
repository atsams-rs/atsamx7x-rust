#[doc = "Register `SCHMITT` reader"]
pub struct R(crate::R<SCHMITT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCHMITT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCHMITT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCHMITT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCHMITT` writer"]
pub struct W(crate::W<SCHMITT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCHMITT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SCHMITT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCHMITT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCHMITT0` reader - Schmitt Trigger Control"]
pub type SCHMITT0_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT0` writer - Schmitt Trigger Control"]
pub type SCHMITT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT1` reader - Schmitt Trigger Control"]
pub type SCHMITT1_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT1` writer - Schmitt Trigger Control"]
pub type SCHMITT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT2` reader - Schmitt Trigger Control"]
pub type SCHMITT2_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT2` writer - Schmitt Trigger Control"]
pub type SCHMITT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT3` reader - Schmitt Trigger Control"]
pub type SCHMITT3_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT3` writer - Schmitt Trigger Control"]
pub type SCHMITT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT4` reader - Schmitt Trigger Control"]
pub type SCHMITT4_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT4` writer - Schmitt Trigger Control"]
pub type SCHMITT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT5` reader - Schmitt Trigger Control"]
pub type SCHMITT5_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT5` writer - Schmitt Trigger Control"]
pub type SCHMITT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT6` reader - Schmitt Trigger Control"]
pub type SCHMITT6_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT6` writer - Schmitt Trigger Control"]
pub type SCHMITT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT7` reader - Schmitt Trigger Control"]
pub type SCHMITT7_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT7` writer - Schmitt Trigger Control"]
pub type SCHMITT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT8` reader - Schmitt Trigger Control"]
pub type SCHMITT8_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT8` writer - Schmitt Trigger Control"]
pub type SCHMITT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT9` reader - Schmitt Trigger Control"]
pub type SCHMITT9_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT9` writer - Schmitt Trigger Control"]
pub type SCHMITT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT10` reader - Schmitt Trigger Control"]
pub type SCHMITT10_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT10` writer - Schmitt Trigger Control"]
pub type SCHMITT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT11` reader - Schmitt Trigger Control"]
pub type SCHMITT11_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT11` writer - Schmitt Trigger Control"]
pub type SCHMITT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT12` reader - Schmitt Trigger Control"]
pub type SCHMITT12_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT12` writer - Schmitt Trigger Control"]
pub type SCHMITT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT13` reader - Schmitt Trigger Control"]
pub type SCHMITT13_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT13` writer - Schmitt Trigger Control"]
pub type SCHMITT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT14` reader - Schmitt Trigger Control"]
pub type SCHMITT14_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT14` writer - Schmitt Trigger Control"]
pub type SCHMITT14_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT15` reader - Schmitt Trigger Control"]
pub type SCHMITT15_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT15` writer - Schmitt Trigger Control"]
pub type SCHMITT15_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT16` reader - Schmitt Trigger Control"]
pub type SCHMITT16_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT16` writer - Schmitt Trigger Control"]
pub type SCHMITT16_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT17` reader - Schmitt Trigger Control"]
pub type SCHMITT17_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT17` writer - Schmitt Trigger Control"]
pub type SCHMITT17_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT18` reader - Schmitt Trigger Control"]
pub type SCHMITT18_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT18` writer - Schmitt Trigger Control"]
pub type SCHMITT18_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT19` reader - Schmitt Trigger Control"]
pub type SCHMITT19_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT19` writer - Schmitt Trigger Control"]
pub type SCHMITT19_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT20` reader - Schmitt Trigger Control"]
pub type SCHMITT20_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT20` writer - Schmitt Trigger Control"]
pub type SCHMITT20_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT21` reader - Schmitt Trigger Control"]
pub type SCHMITT21_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT21` writer - Schmitt Trigger Control"]
pub type SCHMITT21_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT22` reader - Schmitt Trigger Control"]
pub type SCHMITT22_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT22` writer - Schmitt Trigger Control"]
pub type SCHMITT22_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT23` reader - Schmitt Trigger Control"]
pub type SCHMITT23_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT23` writer - Schmitt Trigger Control"]
pub type SCHMITT23_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT24` reader - Schmitt Trigger Control"]
pub type SCHMITT24_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT24` writer - Schmitt Trigger Control"]
pub type SCHMITT24_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT25` reader - Schmitt Trigger Control"]
pub type SCHMITT25_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT25` writer - Schmitt Trigger Control"]
pub type SCHMITT25_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT26` reader - Schmitt Trigger Control"]
pub type SCHMITT26_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT26` writer - Schmitt Trigger Control"]
pub type SCHMITT26_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT27` reader - Schmitt Trigger Control"]
pub type SCHMITT27_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT27` writer - Schmitt Trigger Control"]
pub type SCHMITT27_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT28` reader - Schmitt Trigger Control"]
pub type SCHMITT28_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT28` writer - Schmitt Trigger Control"]
pub type SCHMITT28_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT29` reader - Schmitt Trigger Control"]
pub type SCHMITT29_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT29` writer - Schmitt Trigger Control"]
pub type SCHMITT29_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT30` reader - Schmitt Trigger Control"]
pub type SCHMITT30_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT30` writer - Schmitt Trigger Control"]
pub type SCHMITT30_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
#[doc = "Field `SCHMITT31` reader - Schmitt Trigger Control"]
pub type SCHMITT31_R = crate::BitReader<bool>;
#[doc = "Field `SCHMITT31` writer - Schmitt Trigger Control"]
pub type SCHMITT31_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCHMITT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt0(&self) -> SCHMITT0_R {
        SCHMITT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt1(&self) -> SCHMITT1_R {
        SCHMITT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt2(&self) -> SCHMITT2_R {
        SCHMITT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt3(&self) -> SCHMITT3_R {
        SCHMITT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt4(&self) -> SCHMITT4_R {
        SCHMITT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt5(&self) -> SCHMITT5_R {
        SCHMITT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt6(&self) -> SCHMITT6_R {
        SCHMITT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt7(&self) -> SCHMITT7_R {
        SCHMITT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt8(&self) -> SCHMITT8_R {
        SCHMITT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt9(&self) -> SCHMITT9_R {
        SCHMITT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt10(&self) -> SCHMITT10_R {
        SCHMITT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt11(&self) -> SCHMITT11_R {
        SCHMITT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt12(&self) -> SCHMITT12_R {
        SCHMITT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt13(&self) -> SCHMITT13_R {
        SCHMITT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt14(&self) -> SCHMITT14_R {
        SCHMITT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt15(&self) -> SCHMITT15_R {
        SCHMITT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt16(&self) -> SCHMITT16_R {
        SCHMITT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt17(&self) -> SCHMITT17_R {
        SCHMITT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt18(&self) -> SCHMITT18_R {
        SCHMITT18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt19(&self) -> SCHMITT19_R {
        SCHMITT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt20(&self) -> SCHMITT20_R {
        SCHMITT20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt21(&self) -> SCHMITT21_R {
        SCHMITT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt22(&self) -> SCHMITT22_R {
        SCHMITT22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt23(&self) -> SCHMITT23_R {
        SCHMITT23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt24(&self) -> SCHMITT24_R {
        SCHMITT24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt25(&self) -> SCHMITT25_R {
        SCHMITT25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt26(&self) -> SCHMITT26_R {
        SCHMITT26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt27(&self) -> SCHMITT27_R {
        SCHMITT27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt28(&self) -> SCHMITT28_R {
        SCHMITT28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt29(&self) -> SCHMITT29_R {
        SCHMITT29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt30(&self) -> SCHMITT30_R {
        SCHMITT30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt31(&self) -> SCHMITT31_R {
        SCHMITT31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt0(&mut self) -> SCHMITT0_W<0> {
        SCHMITT0_W::new(self)
    }
    #[doc = "Bit 1 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt1(&mut self) -> SCHMITT1_W<1> {
        SCHMITT1_W::new(self)
    }
    #[doc = "Bit 2 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt2(&mut self) -> SCHMITT2_W<2> {
        SCHMITT2_W::new(self)
    }
    #[doc = "Bit 3 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt3(&mut self) -> SCHMITT3_W<3> {
        SCHMITT3_W::new(self)
    }
    #[doc = "Bit 4 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt4(&mut self) -> SCHMITT4_W<4> {
        SCHMITT4_W::new(self)
    }
    #[doc = "Bit 5 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt5(&mut self) -> SCHMITT5_W<5> {
        SCHMITT5_W::new(self)
    }
    #[doc = "Bit 6 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt6(&mut self) -> SCHMITT6_W<6> {
        SCHMITT6_W::new(self)
    }
    #[doc = "Bit 7 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt7(&mut self) -> SCHMITT7_W<7> {
        SCHMITT7_W::new(self)
    }
    #[doc = "Bit 8 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt8(&mut self) -> SCHMITT8_W<8> {
        SCHMITT8_W::new(self)
    }
    #[doc = "Bit 9 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt9(&mut self) -> SCHMITT9_W<9> {
        SCHMITT9_W::new(self)
    }
    #[doc = "Bit 10 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt10(&mut self) -> SCHMITT10_W<10> {
        SCHMITT10_W::new(self)
    }
    #[doc = "Bit 11 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt11(&mut self) -> SCHMITT11_W<11> {
        SCHMITT11_W::new(self)
    }
    #[doc = "Bit 12 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt12(&mut self) -> SCHMITT12_W<12> {
        SCHMITT12_W::new(self)
    }
    #[doc = "Bit 13 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt13(&mut self) -> SCHMITT13_W<13> {
        SCHMITT13_W::new(self)
    }
    #[doc = "Bit 14 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt14(&mut self) -> SCHMITT14_W<14> {
        SCHMITT14_W::new(self)
    }
    #[doc = "Bit 15 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt15(&mut self) -> SCHMITT15_W<15> {
        SCHMITT15_W::new(self)
    }
    #[doc = "Bit 16 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt16(&mut self) -> SCHMITT16_W<16> {
        SCHMITT16_W::new(self)
    }
    #[doc = "Bit 17 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt17(&mut self) -> SCHMITT17_W<17> {
        SCHMITT17_W::new(self)
    }
    #[doc = "Bit 18 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt18(&mut self) -> SCHMITT18_W<18> {
        SCHMITT18_W::new(self)
    }
    #[doc = "Bit 19 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt19(&mut self) -> SCHMITT19_W<19> {
        SCHMITT19_W::new(self)
    }
    #[doc = "Bit 20 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt20(&mut self) -> SCHMITT20_W<20> {
        SCHMITT20_W::new(self)
    }
    #[doc = "Bit 21 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt21(&mut self) -> SCHMITT21_W<21> {
        SCHMITT21_W::new(self)
    }
    #[doc = "Bit 22 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt22(&mut self) -> SCHMITT22_W<22> {
        SCHMITT22_W::new(self)
    }
    #[doc = "Bit 23 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt23(&mut self) -> SCHMITT23_W<23> {
        SCHMITT23_W::new(self)
    }
    #[doc = "Bit 24 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt24(&mut self) -> SCHMITT24_W<24> {
        SCHMITT24_W::new(self)
    }
    #[doc = "Bit 25 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt25(&mut self) -> SCHMITT25_W<25> {
        SCHMITT25_W::new(self)
    }
    #[doc = "Bit 26 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt26(&mut self) -> SCHMITT26_W<26> {
        SCHMITT26_W::new(self)
    }
    #[doc = "Bit 27 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt27(&mut self) -> SCHMITT27_W<27> {
        SCHMITT27_W::new(self)
    }
    #[doc = "Bit 28 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt28(&mut self) -> SCHMITT28_W<28> {
        SCHMITT28_W::new(self)
    }
    #[doc = "Bit 29 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt29(&mut self) -> SCHMITT29_W<29> {
        SCHMITT29_W::new(self)
    }
    #[doc = "Bit 30 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt30(&mut self) -> SCHMITT30_W<30> {
        SCHMITT30_W::new(self)
    }
    #[doc = "Bit 31 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt31(&mut self) -> SCHMITT31_W<31> {
        SCHMITT31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Schmitt Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [schmitt](index.html) module"]
pub struct SCHMITT_SPEC;
impl crate::RegisterSpec for SCHMITT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [schmitt::R](R) reader structure"]
impl crate::Readable for SCHMITT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [schmitt::W](W) writer structure"]
impl crate::Writable for SCHMITT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCHMITT to value 0"]
impl crate::Resettable for SCHMITT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
