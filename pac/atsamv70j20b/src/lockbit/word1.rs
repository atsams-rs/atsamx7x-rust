#[doc = "Register `WORD1` reader"]
pub struct R(crate::R<WORD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WORD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WORD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WORD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WORD1` writer"]
pub struct W(crate::W<WORD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WORD1_SPEC>;
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
impl From<crate::W<WORD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WORD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK_REGION_32` reader - Lock Region 32"]
pub type LOCK_REGION_32_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_32` writer - Lock Region 32"]
pub type LOCK_REGION_32_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_33` reader - Lock Region 33"]
pub type LOCK_REGION_33_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_33` writer - Lock Region 33"]
pub type LOCK_REGION_33_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_34` reader - Lock Region 34"]
pub type LOCK_REGION_34_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_34` writer - Lock Region 34"]
pub type LOCK_REGION_34_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_35` reader - Lock Region 35"]
pub type LOCK_REGION_35_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_35` writer - Lock Region 35"]
pub type LOCK_REGION_35_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_36` reader - Lock Region 36"]
pub type LOCK_REGION_36_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_36` writer - Lock Region 36"]
pub type LOCK_REGION_36_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_37` reader - Lock Region 37"]
pub type LOCK_REGION_37_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_37` writer - Lock Region 37"]
pub type LOCK_REGION_37_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_38` reader - Lock Region 38"]
pub type LOCK_REGION_38_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_38` writer - Lock Region 38"]
pub type LOCK_REGION_38_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_39` reader - Lock Region 39"]
pub type LOCK_REGION_39_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_39` writer - Lock Region 39"]
pub type LOCK_REGION_39_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_40` reader - Lock Region 40"]
pub type LOCK_REGION_40_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_40` writer - Lock Region 40"]
pub type LOCK_REGION_40_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_41` reader - Lock Region 41"]
pub type LOCK_REGION_41_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_41` writer - Lock Region 41"]
pub type LOCK_REGION_41_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_42` reader - Lock Region 42"]
pub type LOCK_REGION_42_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_42` writer - Lock Region 42"]
pub type LOCK_REGION_42_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_43` reader - Lock Region 43"]
pub type LOCK_REGION_43_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_43` writer - Lock Region 43"]
pub type LOCK_REGION_43_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_44` reader - Lock Region 44"]
pub type LOCK_REGION_44_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_44` writer - Lock Region 44"]
pub type LOCK_REGION_44_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_45` reader - Lock Region 45"]
pub type LOCK_REGION_45_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_45` writer - Lock Region 45"]
pub type LOCK_REGION_45_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_46` reader - Lock Region 46"]
pub type LOCK_REGION_46_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_46` writer - Lock Region 46"]
pub type LOCK_REGION_46_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_47` reader - Lock Region 47"]
pub type LOCK_REGION_47_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_47` writer - Lock Region 47"]
pub type LOCK_REGION_47_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_48` reader - Lock Region 48"]
pub type LOCK_REGION_48_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_48` writer - Lock Region 48"]
pub type LOCK_REGION_48_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_49` reader - Lock Region 49"]
pub type LOCK_REGION_49_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_49` writer - Lock Region 49"]
pub type LOCK_REGION_49_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_50` reader - Lock Region 50"]
pub type LOCK_REGION_50_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_50` writer - Lock Region 50"]
pub type LOCK_REGION_50_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_51` reader - Lock Region 51"]
pub type LOCK_REGION_51_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_51` writer - Lock Region 51"]
pub type LOCK_REGION_51_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_52` reader - Lock Region 52"]
pub type LOCK_REGION_52_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_52` writer - Lock Region 52"]
pub type LOCK_REGION_52_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_53` reader - Lock Region 53"]
pub type LOCK_REGION_53_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_53` writer - Lock Region 53"]
pub type LOCK_REGION_53_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_54` reader - Lock Region 54"]
pub type LOCK_REGION_54_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_54` writer - Lock Region 54"]
pub type LOCK_REGION_54_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_55` reader - Lock Region 55"]
pub type LOCK_REGION_55_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_55` writer - Lock Region 55"]
pub type LOCK_REGION_55_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_56` reader - Lock Region 56"]
pub type LOCK_REGION_56_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_56` writer - Lock Region 56"]
pub type LOCK_REGION_56_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_57` reader - Lock Region 57"]
pub type LOCK_REGION_57_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_57` writer - Lock Region 57"]
pub type LOCK_REGION_57_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_58` reader - Lock Region 58"]
pub type LOCK_REGION_58_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_58` writer - Lock Region 58"]
pub type LOCK_REGION_58_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_59` reader - Lock Region 59"]
pub type LOCK_REGION_59_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_59` writer - Lock Region 59"]
pub type LOCK_REGION_59_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_60` reader - Lock Region 60"]
pub type LOCK_REGION_60_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_60` writer - Lock Region 60"]
pub type LOCK_REGION_60_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_61` reader - Lock Region 61"]
pub type LOCK_REGION_61_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_61` writer - Lock Region 61"]
pub type LOCK_REGION_61_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_62` reader - Lock Region 62"]
pub type LOCK_REGION_62_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_62` writer - Lock Region 62"]
pub type LOCK_REGION_62_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
#[doc = "Field `LOCK_REGION_63` reader - Lock Region 63"]
pub type LOCK_REGION_63_R = crate::BitReader<bool>;
#[doc = "Field `LOCK_REGION_63` writer - Lock Region 63"]
pub type LOCK_REGION_63_W<'a, const O: u8> = crate::BitWriter<'a, u32, WORD1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Lock Region 32"]
    #[inline(always)]
    pub fn lock_region_32(&self) -> LOCK_REGION_32_R {
        LOCK_REGION_32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Region 33"]
    #[inline(always)]
    pub fn lock_region_33(&self) -> LOCK_REGION_33_R {
        LOCK_REGION_33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lock Region 34"]
    #[inline(always)]
    pub fn lock_region_34(&self) -> LOCK_REGION_34_R {
        LOCK_REGION_34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock Region 35"]
    #[inline(always)]
    pub fn lock_region_35(&self) -> LOCK_REGION_35_R {
        LOCK_REGION_35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lock Region 36"]
    #[inline(always)]
    pub fn lock_region_36(&self) -> LOCK_REGION_36_R {
        LOCK_REGION_36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lock Region 37"]
    #[inline(always)]
    pub fn lock_region_37(&self) -> LOCK_REGION_37_R {
        LOCK_REGION_37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lock Region 38"]
    #[inline(always)]
    pub fn lock_region_38(&self) -> LOCK_REGION_38_R {
        LOCK_REGION_38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Lock Region 39"]
    #[inline(always)]
    pub fn lock_region_39(&self) -> LOCK_REGION_39_R {
        LOCK_REGION_39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Lock Region 40"]
    #[inline(always)]
    pub fn lock_region_40(&self) -> LOCK_REGION_40_R {
        LOCK_REGION_40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Lock Region 41"]
    #[inline(always)]
    pub fn lock_region_41(&self) -> LOCK_REGION_41_R {
        LOCK_REGION_41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Lock Region 42"]
    #[inline(always)]
    pub fn lock_region_42(&self) -> LOCK_REGION_42_R {
        LOCK_REGION_42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Lock Region 43"]
    #[inline(always)]
    pub fn lock_region_43(&self) -> LOCK_REGION_43_R {
        LOCK_REGION_43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Lock Region 44"]
    #[inline(always)]
    pub fn lock_region_44(&self) -> LOCK_REGION_44_R {
        LOCK_REGION_44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Lock Region 45"]
    #[inline(always)]
    pub fn lock_region_45(&self) -> LOCK_REGION_45_R {
        LOCK_REGION_45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Lock Region 46"]
    #[inline(always)]
    pub fn lock_region_46(&self) -> LOCK_REGION_46_R {
        LOCK_REGION_46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Lock Region 47"]
    #[inline(always)]
    pub fn lock_region_47(&self) -> LOCK_REGION_47_R {
        LOCK_REGION_47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Lock Region 48"]
    #[inline(always)]
    pub fn lock_region_48(&self) -> LOCK_REGION_48_R {
        LOCK_REGION_48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Lock Region 49"]
    #[inline(always)]
    pub fn lock_region_49(&self) -> LOCK_REGION_49_R {
        LOCK_REGION_49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Lock Region 50"]
    #[inline(always)]
    pub fn lock_region_50(&self) -> LOCK_REGION_50_R {
        LOCK_REGION_50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Lock Region 51"]
    #[inline(always)]
    pub fn lock_region_51(&self) -> LOCK_REGION_51_R {
        LOCK_REGION_51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Lock Region 52"]
    #[inline(always)]
    pub fn lock_region_52(&self) -> LOCK_REGION_52_R {
        LOCK_REGION_52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Lock Region 53"]
    #[inline(always)]
    pub fn lock_region_53(&self) -> LOCK_REGION_53_R {
        LOCK_REGION_53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Lock Region 54"]
    #[inline(always)]
    pub fn lock_region_54(&self) -> LOCK_REGION_54_R {
        LOCK_REGION_54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Lock Region 55"]
    #[inline(always)]
    pub fn lock_region_55(&self) -> LOCK_REGION_55_R {
        LOCK_REGION_55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Lock Region 56"]
    #[inline(always)]
    pub fn lock_region_56(&self) -> LOCK_REGION_56_R {
        LOCK_REGION_56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Lock Region 57"]
    #[inline(always)]
    pub fn lock_region_57(&self) -> LOCK_REGION_57_R {
        LOCK_REGION_57_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Lock Region 58"]
    #[inline(always)]
    pub fn lock_region_58(&self) -> LOCK_REGION_58_R {
        LOCK_REGION_58_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Lock Region 59"]
    #[inline(always)]
    pub fn lock_region_59(&self) -> LOCK_REGION_59_R {
        LOCK_REGION_59_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Lock Region 60"]
    #[inline(always)]
    pub fn lock_region_60(&self) -> LOCK_REGION_60_R {
        LOCK_REGION_60_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Lock Region 61"]
    #[inline(always)]
    pub fn lock_region_61(&self) -> LOCK_REGION_61_R {
        LOCK_REGION_61_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Lock Region 62"]
    #[inline(always)]
    pub fn lock_region_62(&self) -> LOCK_REGION_62_R {
        LOCK_REGION_62_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Lock Region 63"]
    #[inline(always)]
    pub fn lock_region_63(&self) -> LOCK_REGION_63_R {
        LOCK_REGION_63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock Region 32"]
    #[inline(always)]
    pub fn lock_region_32(&mut self) -> LOCK_REGION_32_W<0> {
        LOCK_REGION_32_W::new(self)
    }
    #[doc = "Bit 1 - Lock Region 33"]
    #[inline(always)]
    pub fn lock_region_33(&mut self) -> LOCK_REGION_33_W<1> {
        LOCK_REGION_33_W::new(self)
    }
    #[doc = "Bit 2 - Lock Region 34"]
    #[inline(always)]
    pub fn lock_region_34(&mut self) -> LOCK_REGION_34_W<2> {
        LOCK_REGION_34_W::new(self)
    }
    #[doc = "Bit 3 - Lock Region 35"]
    #[inline(always)]
    pub fn lock_region_35(&mut self) -> LOCK_REGION_35_W<3> {
        LOCK_REGION_35_W::new(self)
    }
    #[doc = "Bit 4 - Lock Region 36"]
    #[inline(always)]
    pub fn lock_region_36(&mut self) -> LOCK_REGION_36_W<4> {
        LOCK_REGION_36_W::new(self)
    }
    #[doc = "Bit 5 - Lock Region 37"]
    #[inline(always)]
    pub fn lock_region_37(&mut self) -> LOCK_REGION_37_W<5> {
        LOCK_REGION_37_W::new(self)
    }
    #[doc = "Bit 6 - Lock Region 38"]
    #[inline(always)]
    pub fn lock_region_38(&mut self) -> LOCK_REGION_38_W<6> {
        LOCK_REGION_38_W::new(self)
    }
    #[doc = "Bit 7 - Lock Region 39"]
    #[inline(always)]
    pub fn lock_region_39(&mut self) -> LOCK_REGION_39_W<7> {
        LOCK_REGION_39_W::new(self)
    }
    #[doc = "Bit 8 - Lock Region 40"]
    #[inline(always)]
    pub fn lock_region_40(&mut self) -> LOCK_REGION_40_W<8> {
        LOCK_REGION_40_W::new(self)
    }
    #[doc = "Bit 9 - Lock Region 41"]
    #[inline(always)]
    pub fn lock_region_41(&mut self) -> LOCK_REGION_41_W<9> {
        LOCK_REGION_41_W::new(self)
    }
    #[doc = "Bit 10 - Lock Region 42"]
    #[inline(always)]
    pub fn lock_region_42(&mut self) -> LOCK_REGION_42_W<10> {
        LOCK_REGION_42_W::new(self)
    }
    #[doc = "Bit 11 - Lock Region 43"]
    #[inline(always)]
    pub fn lock_region_43(&mut self) -> LOCK_REGION_43_W<11> {
        LOCK_REGION_43_W::new(self)
    }
    #[doc = "Bit 12 - Lock Region 44"]
    #[inline(always)]
    pub fn lock_region_44(&mut self) -> LOCK_REGION_44_W<12> {
        LOCK_REGION_44_W::new(self)
    }
    #[doc = "Bit 13 - Lock Region 45"]
    #[inline(always)]
    pub fn lock_region_45(&mut self) -> LOCK_REGION_45_W<13> {
        LOCK_REGION_45_W::new(self)
    }
    #[doc = "Bit 14 - Lock Region 46"]
    #[inline(always)]
    pub fn lock_region_46(&mut self) -> LOCK_REGION_46_W<14> {
        LOCK_REGION_46_W::new(self)
    }
    #[doc = "Bit 15 - Lock Region 47"]
    #[inline(always)]
    pub fn lock_region_47(&mut self) -> LOCK_REGION_47_W<15> {
        LOCK_REGION_47_W::new(self)
    }
    #[doc = "Bit 16 - Lock Region 48"]
    #[inline(always)]
    pub fn lock_region_48(&mut self) -> LOCK_REGION_48_W<16> {
        LOCK_REGION_48_W::new(self)
    }
    #[doc = "Bit 17 - Lock Region 49"]
    #[inline(always)]
    pub fn lock_region_49(&mut self) -> LOCK_REGION_49_W<17> {
        LOCK_REGION_49_W::new(self)
    }
    #[doc = "Bit 18 - Lock Region 50"]
    #[inline(always)]
    pub fn lock_region_50(&mut self) -> LOCK_REGION_50_W<18> {
        LOCK_REGION_50_W::new(self)
    }
    #[doc = "Bit 19 - Lock Region 51"]
    #[inline(always)]
    pub fn lock_region_51(&mut self) -> LOCK_REGION_51_W<19> {
        LOCK_REGION_51_W::new(self)
    }
    #[doc = "Bit 20 - Lock Region 52"]
    #[inline(always)]
    pub fn lock_region_52(&mut self) -> LOCK_REGION_52_W<20> {
        LOCK_REGION_52_W::new(self)
    }
    #[doc = "Bit 21 - Lock Region 53"]
    #[inline(always)]
    pub fn lock_region_53(&mut self) -> LOCK_REGION_53_W<21> {
        LOCK_REGION_53_W::new(self)
    }
    #[doc = "Bit 22 - Lock Region 54"]
    #[inline(always)]
    pub fn lock_region_54(&mut self) -> LOCK_REGION_54_W<22> {
        LOCK_REGION_54_W::new(self)
    }
    #[doc = "Bit 23 - Lock Region 55"]
    #[inline(always)]
    pub fn lock_region_55(&mut self) -> LOCK_REGION_55_W<23> {
        LOCK_REGION_55_W::new(self)
    }
    #[doc = "Bit 24 - Lock Region 56"]
    #[inline(always)]
    pub fn lock_region_56(&mut self) -> LOCK_REGION_56_W<24> {
        LOCK_REGION_56_W::new(self)
    }
    #[doc = "Bit 25 - Lock Region 57"]
    #[inline(always)]
    pub fn lock_region_57(&mut self) -> LOCK_REGION_57_W<25> {
        LOCK_REGION_57_W::new(self)
    }
    #[doc = "Bit 26 - Lock Region 58"]
    #[inline(always)]
    pub fn lock_region_58(&mut self) -> LOCK_REGION_58_W<26> {
        LOCK_REGION_58_W::new(self)
    }
    #[doc = "Bit 27 - Lock Region 59"]
    #[inline(always)]
    pub fn lock_region_59(&mut self) -> LOCK_REGION_59_W<27> {
        LOCK_REGION_59_W::new(self)
    }
    #[doc = "Bit 28 - Lock Region 60"]
    #[inline(always)]
    pub fn lock_region_60(&mut self) -> LOCK_REGION_60_W<28> {
        LOCK_REGION_60_W::new(self)
    }
    #[doc = "Bit 29 - Lock Region 61"]
    #[inline(always)]
    pub fn lock_region_61(&mut self) -> LOCK_REGION_61_W<29> {
        LOCK_REGION_61_W::new(self)
    }
    #[doc = "Bit 30 - Lock Region 62"]
    #[inline(always)]
    pub fn lock_region_62(&mut self) -> LOCK_REGION_62_W<30> {
        LOCK_REGION_62_W::new(self)
    }
    #[doc = "Bit 31 - Lock Region 63"]
    #[inline(always)]
    pub fn lock_region_63(&mut self) -> LOCK_REGION_63_W<31> {
        LOCK_REGION_63_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lock Bits Word 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [word1](index.html) module"]
pub struct WORD1_SPEC;
impl crate::RegisterSpec for WORD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [word1::R](R) reader structure"]
impl crate::Readable for WORD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [word1::W](W) writer structure"]
impl crate::Writable for WORD1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WORD1 to value 0"]
impl crate::Resettable for WORD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
