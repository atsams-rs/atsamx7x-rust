#[doc = "Register `PER` writer"]
pub struct W(crate::W<PER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PER_SPEC>;
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
impl From<crate::W<PER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P0` writer - PIO Enable"]
pub type P0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P1` writer - PIO Enable"]
pub type P1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P2` writer - PIO Enable"]
pub type P2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P3` writer - PIO Enable"]
pub type P3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P4` writer - PIO Enable"]
pub type P4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P5` writer - PIO Enable"]
pub type P5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P6` writer - PIO Enable"]
pub type P6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P7` writer - PIO Enable"]
pub type P7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P8` writer - PIO Enable"]
pub type P8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P9` writer - PIO Enable"]
pub type P9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P10` writer - PIO Enable"]
pub type P10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P11` writer - PIO Enable"]
pub type P11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P12` writer - PIO Enable"]
pub type P12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P13` writer - PIO Enable"]
pub type P13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P14` writer - PIO Enable"]
pub type P14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P15` writer - PIO Enable"]
pub type P15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P16` writer - PIO Enable"]
pub type P16_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P17` writer - PIO Enable"]
pub type P17_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P18` writer - PIO Enable"]
pub type P18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P19` writer - PIO Enable"]
pub type P19_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P20` writer - PIO Enable"]
pub type P20_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P21` writer - PIO Enable"]
pub type P21_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P22` writer - PIO Enable"]
pub type P22_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P23` writer - PIO Enable"]
pub type P23_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P24` writer - PIO Enable"]
pub type P24_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P25` writer - PIO Enable"]
pub type P25_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P26` writer - PIO Enable"]
pub type P26_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P27` writer - PIO Enable"]
pub type P27_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P28` writer - PIO Enable"]
pub type P28_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P29` writer - PIO Enable"]
pub type P29_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P30` writer - PIO Enable"]
pub type P30_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
#[doc = "Field `P31` writer - PIO Enable"]
pub type P31_W<'a, const O: u8> = crate::BitWriter<'a, u32, PER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - PIO Enable"]
    #[inline(always)]
    pub fn p0(&mut self) -> P0_W<0> {
        P0_W::new(self)
    }
    #[doc = "Bit 1 - PIO Enable"]
    #[inline(always)]
    pub fn p1(&mut self) -> P1_W<1> {
        P1_W::new(self)
    }
    #[doc = "Bit 2 - PIO Enable"]
    #[inline(always)]
    pub fn p2(&mut self) -> P2_W<2> {
        P2_W::new(self)
    }
    #[doc = "Bit 3 - PIO Enable"]
    #[inline(always)]
    pub fn p3(&mut self) -> P3_W<3> {
        P3_W::new(self)
    }
    #[doc = "Bit 4 - PIO Enable"]
    #[inline(always)]
    pub fn p4(&mut self) -> P4_W<4> {
        P4_W::new(self)
    }
    #[doc = "Bit 5 - PIO Enable"]
    #[inline(always)]
    pub fn p5(&mut self) -> P5_W<5> {
        P5_W::new(self)
    }
    #[doc = "Bit 6 - PIO Enable"]
    #[inline(always)]
    pub fn p6(&mut self) -> P6_W<6> {
        P6_W::new(self)
    }
    #[doc = "Bit 7 - PIO Enable"]
    #[inline(always)]
    pub fn p7(&mut self) -> P7_W<7> {
        P7_W::new(self)
    }
    #[doc = "Bit 8 - PIO Enable"]
    #[inline(always)]
    pub fn p8(&mut self) -> P8_W<8> {
        P8_W::new(self)
    }
    #[doc = "Bit 9 - PIO Enable"]
    #[inline(always)]
    pub fn p9(&mut self) -> P9_W<9> {
        P9_W::new(self)
    }
    #[doc = "Bit 10 - PIO Enable"]
    #[inline(always)]
    pub fn p10(&mut self) -> P10_W<10> {
        P10_W::new(self)
    }
    #[doc = "Bit 11 - PIO Enable"]
    #[inline(always)]
    pub fn p11(&mut self) -> P11_W<11> {
        P11_W::new(self)
    }
    #[doc = "Bit 12 - PIO Enable"]
    #[inline(always)]
    pub fn p12(&mut self) -> P12_W<12> {
        P12_W::new(self)
    }
    #[doc = "Bit 13 - PIO Enable"]
    #[inline(always)]
    pub fn p13(&mut self) -> P13_W<13> {
        P13_W::new(self)
    }
    #[doc = "Bit 14 - PIO Enable"]
    #[inline(always)]
    pub fn p14(&mut self) -> P14_W<14> {
        P14_W::new(self)
    }
    #[doc = "Bit 15 - PIO Enable"]
    #[inline(always)]
    pub fn p15(&mut self) -> P15_W<15> {
        P15_W::new(self)
    }
    #[doc = "Bit 16 - PIO Enable"]
    #[inline(always)]
    pub fn p16(&mut self) -> P16_W<16> {
        P16_W::new(self)
    }
    #[doc = "Bit 17 - PIO Enable"]
    #[inline(always)]
    pub fn p17(&mut self) -> P17_W<17> {
        P17_W::new(self)
    }
    #[doc = "Bit 18 - PIO Enable"]
    #[inline(always)]
    pub fn p18(&mut self) -> P18_W<18> {
        P18_W::new(self)
    }
    #[doc = "Bit 19 - PIO Enable"]
    #[inline(always)]
    pub fn p19(&mut self) -> P19_W<19> {
        P19_W::new(self)
    }
    #[doc = "Bit 20 - PIO Enable"]
    #[inline(always)]
    pub fn p20(&mut self) -> P20_W<20> {
        P20_W::new(self)
    }
    #[doc = "Bit 21 - PIO Enable"]
    #[inline(always)]
    pub fn p21(&mut self) -> P21_W<21> {
        P21_W::new(self)
    }
    #[doc = "Bit 22 - PIO Enable"]
    #[inline(always)]
    pub fn p22(&mut self) -> P22_W<22> {
        P22_W::new(self)
    }
    #[doc = "Bit 23 - PIO Enable"]
    #[inline(always)]
    pub fn p23(&mut self) -> P23_W<23> {
        P23_W::new(self)
    }
    #[doc = "Bit 24 - PIO Enable"]
    #[inline(always)]
    pub fn p24(&mut self) -> P24_W<24> {
        P24_W::new(self)
    }
    #[doc = "Bit 25 - PIO Enable"]
    #[inline(always)]
    pub fn p25(&mut self) -> P25_W<25> {
        P25_W::new(self)
    }
    #[doc = "Bit 26 - PIO Enable"]
    #[inline(always)]
    pub fn p26(&mut self) -> P26_W<26> {
        P26_W::new(self)
    }
    #[doc = "Bit 27 - PIO Enable"]
    #[inline(always)]
    pub fn p27(&mut self) -> P27_W<27> {
        P27_W::new(self)
    }
    #[doc = "Bit 28 - PIO Enable"]
    #[inline(always)]
    pub fn p28(&mut self) -> P28_W<28> {
        P28_W::new(self)
    }
    #[doc = "Bit 29 - PIO Enable"]
    #[inline(always)]
    pub fn p29(&mut self) -> P29_W<29> {
        P29_W::new(self)
    }
    #[doc = "Bit 30 - PIO Enable"]
    #[inline(always)]
    pub fn p30(&mut self) -> P30_W<30> {
        P30_W::new(self)
    }
    #[doc = "Bit 31 - PIO Enable"]
    #[inline(always)]
    pub fn p31(&mut self) -> P31_W<31> {
        P31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PIO Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per](index.html) module"]
pub struct PER_SPEC;
impl crate::RegisterSpec for PER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [per::W](W) writer structure"]
impl crate::Writable for PER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PER to value 0"]
impl crate::Resettable for PER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
