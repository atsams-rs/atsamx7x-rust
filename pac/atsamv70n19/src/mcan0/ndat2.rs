#[doc = "Register `NDAT2` reader"]
pub struct R(crate::R<NDAT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NDAT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NDAT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NDAT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NDAT2` writer"]
pub struct W(crate::W<NDAT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NDAT2_SPEC>;
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
impl From<crate::W<NDAT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NDAT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ND32` reader - New Data"]
pub type ND32_R = crate::BitReader<bool>;
#[doc = "Field `ND32` writer - New Data"]
pub type ND32_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND33` reader - New Data"]
pub type ND33_R = crate::BitReader<bool>;
#[doc = "Field `ND33` writer - New Data"]
pub type ND33_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND34` reader - New Data"]
pub type ND34_R = crate::BitReader<bool>;
#[doc = "Field `ND34` writer - New Data"]
pub type ND34_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND35` reader - New Data"]
pub type ND35_R = crate::BitReader<bool>;
#[doc = "Field `ND35` writer - New Data"]
pub type ND35_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND36` reader - New Data"]
pub type ND36_R = crate::BitReader<bool>;
#[doc = "Field `ND36` writer - New Data"]
pub type ND36_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND37` reader - New Data"]
pub type ND37_R = crate::BitReader<bool>;
#[doc = "Field `ND37` writer - New Data"]
pub type ND37_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND38` reader - New Data"]
pub type ND38_R = crate::BitReader<bool>;
#[doc = "Field `ND38` writer - New Data"]
pub type ND38_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND39` reader - New Data"]
pub type ND39_R = crate::BitReader<bool>;
#[doc = "Field `ND39` writer - New Data"]
pub type ND39_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND40` reader - New Data"]
pub type ND40_R = crate::BitReader<bool>;
#[doc = "Field `ND40` writer - New Data"]
pub type ND40_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND41` reader - New Data"]
pub type ND41_R = crate::BitReader<bool>;
#[doc = "Field `ND41` writer - New Data"]
pub type ND41_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND42` reader - New Data"]
pub type ND42_R = crate::BitReader<bool>;
#[doc = "Field `ND42` writer - New Data"]
pub type ND42_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND43` reader - New Data"]
pub type ND43_R = crate::BitReader<bool>;
#[doc = "Field `ND43` writer - New Data"]
pub type ND43_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND44` reader - New Data"]
pub type ND44_R = crate::BitReader<bool>;
#[doc = "Field `ND44` writer - New Data"]
pub type ND44_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND45` reader - New Data"]
pub type ND45_R = crate::BitReader<bool>;
#[doc = "Field `ND45` writer - New Data"]
pub type ND45_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND46` reader - New Data"]
pub type ND46_R = crate::BitReader<bool>;
#[doc = "Field `ND46` writer - New Data"]
pub type ND46_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND47` reader - New Data"]
pub type ND47_R = crate::BitReader<bool>;
#[doc = "Field `ND47` writer - New Data"]
pub type ND47_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND48` reader - New Data"]
pub type ND48_R = crate::BitReader<bool>;
#[doc = "Field `ND48` writer - New Data"]
pub type ND48_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND49` reader - New Data"]
pub type ND49_R = crate::BitReader<bool>;
#[doc = "Field `ND49` writer - New Data"]
pub type ND49_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND50` reader - New Data"]
pub type ND50_R = crate::BitReader<bool>;
#[doc = "Field `ND50` writer - New Data"]
pub type ND50_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND51` reader - New Data"]
pub type ND51_R = crate::BitReader<bool>;
#[doc = "Field `ND51` writer - New Data"]
pub type ND51_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND52` reader - New Data"]
pub type ND52_R = crate::BitReader<bool>;
#[doc = "Field `ND52` writer - New Data"]
pub type ND52_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND53` reader - New Data"]
pub type ND53_R = crate::BitReader<bool>;
#[doc = "Field `ND53` writer - New Data"]
pub type ND53_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND54` reader - New Data"]
pub type ND54_R = crate::BitReader<bool>;
#[doc = "Field `ND54` writer - New Data"]
pub type ND54_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND55` reader - New Data"]
pub type ND55_R = crate::BitReader<bool>;
#[doc = "Field `ND55` writer - New Data"]
pub type ND55_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND56` reader - New Data"]
pub type ND56_R = crate::BitReader<bool>;
#[doc = "Field `ND56` writer - New Data"]
pub type ND56_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND57` reader - New Data"]
pub type ND57_R = crate::BitReader<bool>;
#[doc = "Field `ND57` writer - New Data"]
pub type ND57_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND58` reader - New Data"]
pub type ND58_R = crate::BitReader<bool>;
#[doc = "Field `ND58` writer - New Data"]
pub type ND58_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND59` reader - New Data"]
pub type ND59_R = crate::BitReader<bool>;
#[doc = "Field `ND59` writer - New Data"]
pub type ND59_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND60` reader - New Data"]
pub type ND60_R = crate::BitReader<bool>;
#[doc = "Field `ND60` writer - New Data"]
pub type ND60_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND61` reader - New Data"]
pub type ND61_R = crate::BitReader<bool>;
#[doc = "Field `ND61` writer - New Data"]
pub type ND61_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND62` reader - New Data"]
pub type ND62_R = crate::BitReader<bool>;
#[doc = "Field `ND62` writer - New Data"]
pub type ND62_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
#[doc = "Field `ND63` reader - New Data"]
pub type ND63_R = crate::BitReader<bool>;
#[doc = "Field `ND63` writer - New Data"]
pub type ND63_W<'a, const O: u8> = crate::BitWriter<'a, u32, NDAT2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - New Data"]
    #[inline(always)]
    pub fn nd32(&self) -> ND32_R {
        ND32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - New Data"]
    #[inline(always)]
    pub fn nd33(&self) -> ND33_R {
        ND33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - New Data"]
    #[inline(always)]
    pub fn nd34(&self) -> ND34_R {
        ND34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - New Data"]
    #[inline(always)]
    pub fn nd35(&self) -> ND35_R {
        ND35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - New Data"]
    #[inline(always)]
    pub fn nd36(&self) -> ND36_R {
        ND36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - New Data"]
    #[inline(always)]
    pub fn nd37(&self) -> ND37_R {
        ND37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - New Data"]
    #[inline(always)]
    pub fn nd38(&self) -> ND38_R {
        ND38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - New Data"]
    #[inline(always)]
    pub fn nd39(&self) -> ND39_R {
        ND39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - New Data"]
    #[inline(always)]
    pub fn nd40(&self) -> ND40_R {
        ND40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - New Data"]
    #[inline(always)]
    pub fn nd41(&self) -> ND41_R {
        ND41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - New Data"]
    #[inline(always)]
    pub fn nd42(&self) -> ND42_R {
        ND42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - New Data"]
    #[inline(always)]
    pub fn nd43(&self) -> ND43_R {
        ND43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - New Data"]
    #[inline(always)]
    pub fn nd44(&self) -> ND44_R {
        ND44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - New Data"]
    #[inline(always)]
    pub fn nd45(&self) -> ND45_R {
        ND45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - New Data"]
    #[inline(always)]
    pub fn nd46(&self) -> ND46_R {
        ND46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - New Data"]
    #[inline(always)]
    pub fn nd47(&self) -> ND47_R {
        ND47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - New Data"]
    #[inline(always)]
    pub fn nd48(&self) -> ND48_R {
        ND48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - New Data"]
    #[inline(always)]
    pub fn nd49(&self) -> ND49_R {
        ND49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - New Data"]
    #[inline(always)]
    pub fn nd50(&self) -> ND50_R {
        ND50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - New Data"]
    #[inline(always)]
    pub fn nd51(&self) -> ND51_R {
        ND51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - New Data"]
    #[inline(always)]
    pub fn nd52(&self) -> ND52_R {
        ND52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - New Data"]
    #[inline(always)]
    pub fn nd53(&self) -> ND53_R {
        ND53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - New Data"]
    #[inline(always)]
    pub fn nd54(&self) -> ND54_R {
        ND54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - New Data"]
    #[inline(always)]
    pub fn nd55(&self) -> ND55_R {
        ND55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - New Data"]
    #[inline(always)]
    pub fn nd56(&self) -> ND56_R {
        ND56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - New Data"]
    #[inline(always)]
    pub fn nd57(&self) -> ND57_R {
        ND57_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - New Data"]
    #[inline(always)]
    pub fn nd58(&self) -> ND58_R {
        ND58_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - New Data"]
    #[inline(always)]
    pub fn nd59(&self) -> ND59_R {
        ND59_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - New Data"]
    #[inline(always)]
    pub fn nd60(&self) -> ND60_R {
        ND60_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - New Data"]
    #[inline(always)]
    pub fn nd61(&self) -> ND61_R {
        ND61_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - New Data"]
    #[inline(always)]
    pub fn nd62(&self) -> ND62_R {
        ND62_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - New Data"]
    #[inline(always)]
    pub fn nd63(&self) -> ND63_R {
        ND63_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - New Data"]
    #[inline(always)]
    pub fn nd32(&mut self) -> ND32_W<0> {
        ND32_W::new(self)
    }
    #[doc = "Bit 1 - New Data"]
    #[inline(always)]
    pub fn nd33(&mut self) -> ND33_W<1> {
        ND33_W::new(self)
    }
    #[doc = "Bit 2 - New Data"]
    #[inline(always)]
    pub fn nd34(&mut self) -> ND34_W<2> {
        ND34_W::new(self)
    }
    #[doc = "Bit 3 - New Data"]
    #[inline(always)]
    pub fn nd35(&mut self) -> ND35_W<3> {
        ND35_W::new(self)
    }
    #[doc = "Bit 4 - New Data"]
    #[inline(always)]
    pub fn nd36(&mut self) -> ND36_W<4> {
        ND36_W::new(self)
    }
    #[doc = "Bit 5 - New Data"]
    #[inline(always)]
    pub fn nd37(&mut self) -> ND37_W<5> {
        ND37_W::new(self)
    }
    #[doc = "Bit 6 - New Data"]
    #[inline(always)]
    pub fn nd38(&mut self) -> ND38_W<6> {
        ND38_W::new(self)
    }
    #[doc = "Bit 7 - New Data"]
    #[inline(always)]
    pub fn nd39(&mut self) -> ND39_W<7> {
        ND39_W::new(self)
    }
    #[doc = "Bit 8 - New Data"]
    #[inline(always)]
    pub fn nd40(&mut self) -> ND40_W<8> {
        ND40_W::new(self)
    }
    #[doc = "Bit 9 - New Data"]
    #[inline(always)]
    pub fn nd41(&mut self) -> ND41_W<9> {
        ND41_W::new(self)
    }
    #[doc = "Bit 10 - New Data"]
    #[inline(always)]
    pub fn nd42(&mut self) -> ND42_W<10> {
        ND42_W::new(self)
    }
    #[doc = "Bit 11 - New Data"]
    #[inline(always)]
    pub fn nd43(&mut self) -> ND43_W<11> {
        ND43_W::new(self)
    }
    #[doc = "Bit 12 - New Data"]
    #[inline(always)]
    pub fn nd44(&mut self) -> ND44_W<12> {
        ND44_W::new(self)
    }
    #[doc = "Bit 13 - New Data"]
    #[inline(always)]
    pub fn nd45(&mut self) -> ND45_W<13> {
        ND45_W::new(self)
    }
    #[doc = "Bit 14 - New Data"]
    #[inline(always)]
    pub fn nd46(&mut self) -> ND46_W<14> {
        ND46_W::new(self)
    }
    #[doc = "Bit 15 - New Data"]
    #[inline(always)]
    pub fn nd47(&mut self) -> ND47_W<15> {
        ND47_W::new(self)
    }
    #[doc = "Bit 16 - New Data"]
    #[inline(always)]
    pub fn nd48(&mut self) -> ND48_W<16> {
        ND48_W::new(self)
    }
    #[doc = "Bit 17 - New Data"]
    #[inline(always)]
    pub fn nd49(&mut self) -> ND49_W<17> {
        ND49_W::new(self)
    }
    #[doc = "Bit 18 - New Data"]
    #[inline(always)]
    pub fn nd50(&mut self) -> ND50_W<18> {
        ND50_W::new(self)
    }
    #[doc = "Bit 19 - New Data"]
    #[inline(always)]
    pub fn nd51(&mut self) -> ND51_W<19> {
        ND51_W::new(self)
    }
    #[doc = "Bit 20 - New Data"]
    #[inline(always)]
    pub fn nd52(&mut self) -> ND52_W<20> {
        ND52_W::new(self)
    }
    #[doc = "Bit 21 - New Data"]
    #[inline(always)]
    pub fn nd53(&mut self) -> ND53_W<21> {
        ND53_W::new(self)
    }
    #[doc = "Bit 22 - New Data"]
    #[inline(always)]
    pub fn nd54(&mut self) -> ND54_W<22> {
        ND54_W::new(self)
    }
    #[doc = "Bit 23 - New Data"]
    #[inline(always)]
    pub fn nd55(&mut self) -> ND55_W<23> {
        ND55_W::new(self)
    }
    #[doc = "Bit 24 - New Data"]
    #[inline(always)]
    pub fn nd56(&mut self) -> ND56_W<24> {
        ND56_W::new(self)
    }
    #[doc = "Bit 25 - New Data"]
    #[inline(always)]
    pub fn nd57(&mut self) -> ND57_W<25> {
        ND57_W::new(self)
    }
    #[doc = "Bit 26 - New Data"]
    #[inline(always)]
    pub fn nd58(&mut self) -> ND58_W<26> {
        ND58_W::new(self)
    }
    #[doc = "Bit 27 - New Data"]
    #[inline(always)]
    pub fn nd59(&mut self) -> ND59_W<27> {
        ND59_W::new(self)
    }
    #[doc = "Bit 28 - New Data"]
    #[inline(always)]
    pub fn nd60(&mut self) -> ND60_W<28> {
        ND60_W::new(self)
    }
    #[doc = "Bit 29 - New Data"]
    #[inline(always)]
    pub fn nd61(&mut self) -> ND61_W<29> {
        ND61_W::new(self)
    }
    #[doc = "Bit 30 - New Data"]
    #[inline(always)]
    pub fn nd62(&mut self) -> ND62_W<30> {
        ND62_W::new(self)
    }
    #[doc = "Bit 31 - New Data"]
    #[inline(always)]
    pub fn nd63(&mut self) -> ND63_W<31> {
        ND63_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "New Data 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ndat2](index.html) module"]
pub struct NDAT2_SPEC;
impl crate::RegisterSpec for NDAT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ndat2::R](R) reader structure"]
impl crate::Readable for NDAT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ndat2::W](W) writer structure"]
impl crate::Writable for NDAT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NDAT2 to value 0"]
impl crate::Resettable for NDAT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
