#[doc = "Register `PCER1` writer"]
pub struct W(crate::W<PCER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCER1_SPEC>;
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
impl From<crate::W<PCER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID32` writer - Peripheral Clock 32 Enable"]
pub type PID32_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID33` writer - Peripheral Clock 33 Enable"]
pub type PID33_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID34` writer - Peripheral Clock 34 Enable"]
pub type PID34_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID35` writer - Peripheral Clock 35 Enable"]
pub type PID35_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID37` writer - Peripheral Clock 37 Enable"]
pub type PID37_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID39` writer - Peripheral Clock 39 Enable"]
pub type PID39_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID40` writer - Peripheral Clock 40 Enable"]
pub type PID40_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID41` writer - Peripheral Clock 41 Enable"]
pub type PID41_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID42` writer - Peripheral Clock 42 Enable"]
pub type PID42_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID43` writer - Peripheral Clock 43 Enable"]
pub type PID43_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID44` writer - Peripheral Clock 44 Enable"]
pub type PID44_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID45` writer - Peripheral Clock 45 Enable"]
pub type PID45_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID46` writer - Peripheral Clock 46 Enable"]
pub type PID46_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID47` writer - Peripheral Clock 47 Enable"]
pub type PID47_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID48` writer - Peripheral Clock 48 Enable"]
pub type PID48_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID49` writer - Peripheral Clock 49 Enable"]
pub type PID49_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID50` writer - Peripheral Clock 50 Enable"]
pub type PID50_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID51` writer - Peripheral Clock 51 Enable"]
pub type PID51_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID52` writer - Peripheral Clock 52 Enable"]
pub type PID52_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID56` writer - Peripheral Clock 56 Enable"]
pub type PID56_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID57` writer - Peripheral Clock 57 Enable"]
pub type PID57_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID58` writer - Peripheral Clock 58 Enable"]
pub type PID58_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID59` writer - Peripheral Clock 59 Enable"]
pub type PID59_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
#[doc = "Field `PID60` writer - Peripheral Clock 60 Enable"]
pub type PID60_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCER1_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Peripheral Clock 32 Enable"]
    #[inline(always)]
    pub fn pid32(&mut self) -> PID32_W<0> {
        PID32_W::new(self)
    }
    #[doc = "Bit 1 - Peripheral Clock 33 Enable"]
    #[inline(always)]
    pub fn pid33(&mut self) -> PID33_W<1> {
        PID33_W::new(self)
    }
    #[doc = "Bit 2 - Peripheral Clock 34 Enable"]
    #[inline(always)]
    pub fn pid34(&mut self) -> PID34_W<2> {
        PID34_W::new(self)
    }
    #[doc = "Bit 3 - Peripheral Clock 35 Enable"]
    #[inline(always)]
    pub fn pid35(&mut self) -> PID35_W<3> {
        PID35_W::new(self)
    }
    #[doc = "Bit 5 - Peripheral Clock 37 Enable"]
    #[inline(always)]
    pub fn pid37(&mut self) -> PID37_W<5> {
        PID37_W::new(self)
    }
    #[doc = "Bit 7 - Peripheral Clock 39 Enable"]
    #[inline(always)]
    pub fn pid39(&mut self) -> PID39_W<7> {
        PID39_W::new(self)
    }
    #[doc = "Bit 8 - Peripheral Clock 40 Enable"]
    #[inline(always)]
    pub fn pid40(&mut self) -> PID40_W<8> {
        PID40_W::new(self)
    }
    #[doc = "Bit 9 - Peripheral Clock 41 Enable"]
    #[inline(always)]
    pub fn pid41(&mut self) -> PID41_W<9> {
        PID41_W::new(self)
    }
    #[doc = "Bit 10 - Peripheral Clock 42 Enable"]
    #[inline(always)]
    pub fn pid42(&mut self) -> PID42_W<10> {
        PID42_W::new(self)
    }
    #[doc = "Bit 11 - Peripheral Clock 43 Enable"]
    #[inline(always)]
    pub fn pid43(&mut self) -> PID43_W<11> {
        PID43_W::new(self)
    }
    #[doc = "Bit 12 - Peripheral Clock 44 Enable"]
    #[inline(always)]
    pub fn pid44(&mut self) -> PID44_W<12> {
        PID44_W::new(self)
    }
    #[doc = "Bit 13 - Peripheral Clock 45 Enable"]
    #[inline(always)]
    pub fn pid45(&mut self) -> PID45_W<13> {
        PID45_W::new(self)
    }
    #[doc = "Bit 14 - Peripheral Clock 46 Enable"]
    #[inline(always)]
    pub fn pid46(&mut self) -> PID46_W<14> {
        PID46_W::new(self)
    }
    #[doc = "Bit 15 - Peripheral Clock 47 Enable"]
    #[inline(always)]
    pub fn pid47(&mut self) -> PID47_W<15> {
        PID47_W::new(self)
    }
    #[doc = "Bit 16 - Peripheral Clock 48 Enable"]
    #[inline(always)]
    pub fn pid48(&mut self) -> PID48_W<16> {
        PID48_W::new(self)
    }
    #[doc = "Bit 17 - Peripheral Clock 49 Enable"]
    #[inline(always)]
    pub fn pid49(&mut self) -> PID49_W<17> {
        PID49_W::new(self)
    }
    #[doc = "Bit 18 - Peripheral Clock 50 Enable"]
    #[inline(always)]
    pub fn pid50(&mut self) -> PID50_W<18> {
        PID50_W::new(self)
    }
    #[doc = "Bit 19 - Peripheral Clock 51 Enable"]
    #[inline(always)]
    pub fn pid51(&mut self) -> PID51_W<19> {
        PID51_W::new(self)
    }
    #[doc = "Bit 20 - Peripheral Clock 52 Enable"]
    #[inline(always)]
    pub fn pid52(&mut self) -> PID52_W<20> {
        PID52_W::new(self)
    }
    #[doc = "Bit 24 - Peripheral Clock 56 Enable"]
    #[inline(always)]
    pub fn pid56(&mut self) -> PID56_W<24> {
        PID56_W::new(self)
    }
    #[doc = "Bit 25 - Peripheral Clock 57 Enable"]
    #[inline(always)]
    pub fn pid57(&mut self) -> PID57_W<25> {
        PID57_W::new(self)
    }
    #[doc = "Bit 26 - Peripheral Clock 58 Enable"]
    #[inline(always)]
    pub fn pid58(&mut self) -> PID58_W<26> {
        PID58_W::new(self)
    }
    #[doc = "Bit 27 - Peripheral Clock 59 Enable"]
    #[inline(always)]
    pub fn pid59(&mut self) -> PID59_W<27> {
        PID59_W::new(self)
    }
    #[doc = "Bit 28 - Peripheral Clock 60 Enable"]
    #[inline(always)]
    pub fn pid60(&mut self) -> PID60_W<28> {
        PID60_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Enable Register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcer1](index.html) module"]
pub struct PCER1_SPEC;
impl crate::RegisterSpec for PCER1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pcer1::W](W) writer structure"]
impl crate::Writable for PCER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCER1 to value 0"]
impl crate::Resettable for PCER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
