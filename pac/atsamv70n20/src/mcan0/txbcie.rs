#[doc = "Register `TXBCIE` reader"]
pub struct R(crate::R<TXBCIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBCIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBCIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBCIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBCIE` writer"]
pub struct W(crate::W<TXBCIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBCIE_SPEC>;
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
impl From<crate::W<TXBCIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBCIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFIE0` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 0"]
pub type CFIE0_R = crate::BitReader<bool>;
#[doc = "Field `CFIE0` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 0"]
pub type CFIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE1` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 1"]
pub type CFIE1_R = crate::BitReader<bool>;
#[doc = "Field `CFIE1` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 1"]
pub type CFIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE2` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 2"]
pub type CFIE2_R = crate::BitReader<bool>;
#[doc = "Field `CFIE2` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 2"]
pub type CFIE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE3` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 3"]
pub type CFIE3_R = crate::BitReader<bool>;
#[doc = "Field `CFIE3` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 3"]
pub type CFIE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE4` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 4"]
pub type CFIE4_R = crate::BitReader<bool>;
#[doc = "Field `CFIE4` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 4"]
pub type CFIE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE5` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 5"]
pub type CFIE5_R = crate::BitReader<bool>;
#[doc = "Field `CFIE5` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 5"]
pub type CFIE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE6` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 6"]
pub type CFIE6_R = crate::BitReader<bool>;
#[doc = "Field `CFIE6` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 6"]
pub type CFIE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE7` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 7"]
pub type CFIE7_R = crate::BitReader<bool>;
#[doc = "Field `CFIE7` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 7"]
pub type CFIE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE8` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 8"]
pub type CFIE8_R = crate::BitReader<bool>;
#[doc = "Field `CFIE8` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 8"]
pub type CFIE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE9` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 9"]
pub type CFIE9_R = crate::BitReader<bool>;
#[doc = "Field `CFIE9` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 9"]
pub type CFIE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE10` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 10"]
pub type CFIE10_R = crate::BitReader<bool>;
#[doc = "Field `CFIE10` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 10"]
pub type CFIE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE11` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 11"]
pub type CFIE11_R = crate::BitReader<bool>;
#[doc = "Field `CFIE11` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 11"]
pub type CFIE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE12` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 12"]
pub type CFIE12_R = crate::BitReader<bool>;
#[doc = "Field `CFIE12` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 12"]
pub type CFIE12_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE13` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 13"]
pub type CFIE13_R = crate::BitReader<bool>;
#[doc = "Field `CFIE13` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 13"]
pub type CFIE13_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE14` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 14"]
pub type CFIE14_R = crate::BitReader<bool>;
#[doc = "Field `CFIE14` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 14"]
pub type CFIE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE15` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 15"]
pub type CFIE15_R = crate::BitReader<bool>;
#[doc = "Field `CFIE15` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 15"]
pub type CFIE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE16` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 16"]
pub type CFIE16_R = crate::BitReader<bool>;
#[doc = "Field `CFIE16` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 16"]
pub type CFIE16_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE17` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 17"]
pub type CFIE17_R = crate::BitReader<bool>;
#[doc = "Field `CFIE17` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 17"]
pub type CFIE17_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE18` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 18"]
pub type CFIE18_R = crate::BitReader<bool>;
#[doc = "Field `CFIE18` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 18"]
pub type CFIE18_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE19` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 19"]
pub type CFIE19_R = crate::BitReader<bool>;
#[doc = "Field `CFIE19` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 19"]
pub type CFIE19_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE20` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 20"]
pub type CFIE20_R = crate::BitReader<bool>;
#[doc = "Field `CFIE20` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 20"]
pub type CFIE20_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE21` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 21"]
pub type CFIE21_R = crate::BitReader<bool>;
#[doc = "Field `CFIE21` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 21"]
pub type CFIE21_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE22` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 22"]
pub type CFIE22_R = crate::BitReader<bool>;
#[doc = "Field `CFIE22` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 22"]
pub type CFIE22_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE23` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 23"]
pub type CFIE23_R = crate::BitReader<bool>;
#[doc = "Field `CFIE23` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 23"]
pub type CFIE23_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE24` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 24"]
pub type CFIE24_R = crate::BitReader<bool>;
#[doc = "Field `CFIE24` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 24"]
pub type CFIE24_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE25` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 25"]
pub type CFIE25_R = crate::BitReader<bool>;
#[doc = "Field `CFIE25` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 25"]
pub type CFIE25_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE26` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 26"]
pub type CFIE26_R = crate::BitReader<bool>;
#[doc = "Field `CFIE26` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 26"]
pub type CFIE26_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE27` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 27"]
pub type CFIE27_R = crate::BitReader<bool>;
#[doc = "Field `CFIE27` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 27"]
pub type CFIE27_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE28` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 28"]
pub type CFIE28_R = crate::BitReader<bool>;
#[doc = "Field `CFIE28` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 28"]
pub type CFIE28_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE29` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 29"]
pub type CFIE29_R = crate::BitReader<bool>;
#[doc = "Field `CFIE29` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 29"]
pub type CFIE29_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE30` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 30"]
pub type CFIE30_R = crate::BitReader<bool>;
#[doc = "Field `CFIE30` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 30"]
pub type CFIE30_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
#[doc = "Field `CFIE31` reader - Cancellation Finished Interrupt Enable for Transmit Buffer 31"]
pub type CFIE31_R = crate::BitReader<bool>;
#[doc = "Field `CFIE31` writer - Cancellation Finished Interrupt Enable for Transmit Buffer 31"]
pub type CFIE31_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBCIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Cancellation Finished Interrupt Enable for Transmit Buffer 0"]
    #[inline(always)]
    pub fn cfie0(&self) -> CFIE0_R {
        CFIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cancellation Finished Interrupt Enable for Transmit Buffer 1"]
    #[inline(always)]
    pub fn cfie1(&self) -> CFIE1_R {
        CFIE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cancellation Finished Interrupt Enable for Transmit Buffer 2"]
    #[inline(always)]
    pub fn cfie2(&self) -> CFIE2_R {
        CFIE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Cancellation Finished Interrupt Enable for Transmit Buffer 3"]
    #[inline(always)]
    pub fn cfie3(&self) -> CFIE3_R {
        CFIE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Cancellation Finished Interrupt Enable for Transmit Buffer 4"]
    #[inline(always)]
    pub fn cfie4(&self) -> CFIE4_R {
        CFIE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Cancellation Finished Interrupt Enable for Transmit Buffer 5"]
    #[inline(always)]
    pub fn cfie5(&self) -> CFIE5_R {
        CFIE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Cancellation Finished Interrupt Enable for Transmit Buffer 6"]
    #[inline(always)]
    pub fn cfie6(&self) -> CFIE6_R {
        CFIE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Cancellation Finished Interrupt Enable for Transmit Buffer 7"]
    #[inline(always)]
    pub fn cfie7(&self) -> CFIE7_R {
        CFIE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Cancellation Finished Interrupt Enable for Transmit Buffer 8"]
    #[inline(always)]
    pub fn cfie8(&self) -> CFIE8_R {
        CFIE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Cancellation Finished Interrupt Enable for Transmit Buffer 9"]
    #[inline(always)]
    pub fn cfie9(&self) -> CFIE9_R {
        CFIE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Cancellation Finished Interrupt Enable for Transmit Buffer 10"]
    #[inline(always)]
    pub fn cfie10(&self) -> CFIE10_R {
        CFIE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Cancellation Finished Interrupt Enable for Transmit Buffer 11"]
    #[inline(always)]
    pub fn cfie11(&self) -> CFIE11_R {
        CFIE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Cancellation Finished Interrupt Enable for Transmit Buffer 12"]
    #[inline(always)]
    pub fn cfie12(&self) -> CFIE12_R {
        CFIE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Cancellation Finished Interrupt Enable for Transmit Buffer 13"]
    #[inline(always)]
    pub fn cfie13(&self) -> CFIE13_R {
        CFIE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Cancellation Finished Interrupt Enable for Transmit Buffer 14"]
    #[inline(always)]
    pub fn cfie14(&self) -> CFIE14_R {
        CFIE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Cancellation Finished Interrupt Enable for Transmit Buffer 15"]
    #[inline(always)]
    pub fn cfie15(&self) -> CFIE15_R {
        CFIE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Cancellation Finished Interrupt Enable for Transmit Buffer 16"]
    #[inline(always)]
    pub fn cfie16(&self) -> CFIE16_R {
        CFIE16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Cancellation Finished Interrupt Enable for Transmit Buffer 17"]
    #[inline(always)]
    pub fn cfie17(&self) -> CFIE17_R {
        CFIE17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Cancellation Finished Interrupt Enable for Transmit Buffer 18"]
    #[inline(always)]
    pub fn cfie18(&self) -> CFIE18_R {
        CFIE18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Cancellation Finished Interrupt Enable for Transmit Buffer 19"]
    #[inline(always)]
    pub fn cfie19(&self) -> CFIE19_R {
        CFIE19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Cancellation Finished Interrupt Enable for Transmit Buffer 20"]
    #[inline(always)]
    pub fn cfie20(&self) -> CFIE20_R {
        CFIE20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Cancellation Finished Interrupt Enable for Transmit Buffer 21"]
    #[inline(always)]
    pub fn cfie21(&self) -> CFIE21_R {
        CFIE21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Cancellation Finished Interrupt Enable for Transmit Buffer 22"]
    #[inline(always)]
    pub fn cfie22(&self) -> CFIE22_R {
        CFIE22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Cancellation Finished Interrupt Enable for Transmit Buffer 23"]
    #[inline(always)]
    pub fn cfie23(&self) -> CFIE23_R {
        CFIE23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Cancellation Finished Interrupt Enable for Transmit Buffer 24"]
    #[inline(always)]
    pub fn cfie24(&self) -> CFIE24_R {
        CFIE24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Cancellation Finished Interrupt Enable for Transmit Buffer 25"]
    #[inline(always)]
    pub fn cfie25(&self) -> CFIE25_R {
        CFIE25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Cancellation Finished Interrupt Enable for Transmit Buffer 26"]
    #[inline(always)]
    pub fn cfie26(&self) -> CFIE26_R {
        CFIE26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Cancellation Finished Interrupt Enable for Transmit Buffer 27"]
    #[inline(always)]
    pub fn cfie27(&self) -> CFIE27_R {
        CFIE27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Cancellation Finished Interrupt Enable for Transmit Buffer 28"]
    #[inline(always)]
    pub fn cfie28(&self) -> CFIE28_R {
        CFIE28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Cancellation Finished Interrupt Enable for Transmit Buffer 29"]
    #[inline(always)]
    pub fn cfie29(&self) -> CFIE29_R {
        CFIE29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Cancellation Finished Interrupt Enable for Transmit Buffer 30"]
    #[inline(always)]
    pub fn cfie30(&self) -> CFIE30_R {
        CFIE30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Cancellation Finished Interrupt Enable for Transmit Buffer 31"]
    #[inline(always)]
    pub fn cfie31(&self) -> CFIE31_R {
        CFIE31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cancellation Finished Interrupt Enable for Transmit Buffer 0"]
    #[inline(always)]
    pub fn cfie0(&mut self) -> CFIE0_W<0> {
        CFIE0_W::new(self)
    }
    #[doc = "Bit 1 - Cancellation Finished Interrupt Enable for Transmit Buffer 1"]
    #[inline(always)]
    pub fn cfie1(&mut self) -> CFIE1_W<1> {
        CFIE1_W::new(self)
    }
    #[doc = "Bit 2 - Cancellation Finished Interrupt Enable for Transmit Buffer 2"]
    #[inline(always)]
    pub fn cfie2(&mut self) -> CFIE2_W<2> {
        CFIE2_W::new(self)
    }
    #[doc = "Bit 3 - Cancellation Finished Interrupt Enable for Transmit Buffer 3"]
    #[inline(always)]
    pub fn cfie3(&mut self) -> CFIE3_W<3> {
        CFIE3_W::new(self)
    }
    #[doc = "Bit 4 - Cancellation Finished Interrupt Enable for Transmit Buffer 4"]
    #[inline(always)]
    pub fn cfie4(&mut self) -> CFIE4_W<4> {
        CFIE4_W::new(self)
    }
    #[doc = "Bit 5 - Cancellation Finished Interrupt Enable for Transmit Buffer 5"]
    #[inline(always)]
    pub fn cfie5(&mut self) -> CFIE5_W<5> {
        CFIE5_W::new(self)
    }
    #[doc = "Bit 6 - Cancellation Finished Interrupt Enable for Transmit Buffer 6"]
    #[inline(always)]
    pub fn cfie6(&mut self) -> CFIE6_W<6> {
        CFIE6_W::new(self)
    }
    #[doc = "Bit 7 - Cancellation Finished Interrupt Enable for Transmit Buffer 7"]
    #[inline(always)]
    pub fn cfie7(&mut self) -> CFIE7_W<7> {
        CFIE7_W::new(self)
    }
    #[doc = "Bit 8 - Cancellation Finished Interrupt Enable for Transmit Buffer 8"]
    #[inline(always)]
    pub fn cfie8(&mut self) -> CFIE8_W<8> {
        CFIE8_W::new(self)
    }
    #[doc = "Bit 9 - Cancellation Finished Interrupt Enable for Transmit Buffer 9"]
    #[inline(always)]
    pub fn cfie9(&mut self) -> CFIE9_W<9> {
        CFIE9_W::new(self)
    }
    #[doc = "Bit 10 - Cancellation Finished Interrupt Enable for Transmit Buffer 10"]
    #[inline(always)]
    pub fn cfie10(&mut self) -> CFIE10_W<10> {
        CFIE10_W::new(self)
    }
    #[doc = "Bit 11 - Cancellation Finished Interrupt Enable for Transmit Buffer 11"]
    #[inline(always)]
    pub fn cfie11(&mut self) -> CFIE11_W<11> {
        CFIE11_W::new(self)
    }
    #[doc = "Bit 12 - Cancellation Finished Interrupt Enable for Transmit Buffer 12"]
    #[inline(always)]
    pub fn cfie12(&mut self) -> CFIE12_W<12> {
        CFIE12_W::new(self)
    }
    #[doc = "Bit 13 - Cancellation Finished Interrupt Enable for Transmit Buffer 13"]
    #[inline(always)]
    pub fn cfie13(&mut self) -> CFIE13_W<13> {
        CFIE13_W::new(self)
    }
    #[doc = "Bit 14 - Cancellation Finished Interrupt Enable for Transmit Buffer 14"]
    #[inline(always)]
    pub fn cfie14(&mut self) -> CFIE14_W<14> {
        CFIE14_W::new(self)
    }
    #[doc = "Bit 15 - Cancellation Finished Interrupt Enable for Transmit Buffer 15"]
    #[inline(always)]
    pub fn cfie15(&mut self) -> CFIE15_W<15> {
        CFIE15_W::new(self)
    }
    #[doc = "Bit 16 - Cancellation Finished Interrupt Enable for Transmit Buffer 16"]
    #[inline(always)]
    pub fn cfie16(&mut self) -> CFIE16_W<16> {
        CFIE16_W::new(self)
    }
    #[doc = "Bit 17 - Cancellation Finished Interrupt Enable for Transmit Buffer 17"]
    #[inline(always)]
    pub fn cfie17(&mut self) -> CFIE17_W<17> {
        CFIE17_W::new(self)
    }
    #[doc = "Bit 18 - Cancellation Finished Interrupt Enable for Transmit Buffer 18"]
    #[inline(always)]
    pub fn cfie18(&mut self) -> CFIE18_W<18> {
        CFIE18_W::new(self)
    }
    #[doc = "Bit 19 - Cancellation Finished Interrupt Enable for Transmit Buffer 19"]
    #[inline(always)]
    pub fn cfie19(&mut self) -> CFIE19_W<19> {
        CFIE19_W::new(self)
    }
    #[doc = "Bit 20 - Cancellation Finished Interrupt Enable for Transmit Buffer 20"]
    #[inline(always)]
    pub fn cfie20(&mut self) -> CFIE20_W<20> {
        CFIE20_W::new(self)
    }
    #[doc = "Bit 21 - Cancellation Finished Interrupt Enable for Transmit Buffer 21"]
    #[inline(always)]
    pub fn cfie21(&mut self) -> CFIE21_W<21> {
        CFIE21_W::new(self)
    }
    #[doc = "Bit 22 - Cancellation Finished Interrupt Enable for Transmit Buffer 22"]
    #[inline(always)]
    pub fn cfie22(&mut self) -> CFIE22_W<22> {
        CFIE22_W::new(self)
    }
    #[doc = "Bit 23 - Cancellation Finished Interrupt Enable for Transmit Buffer 23"]
    #[inline(always)]
    pub fn cfie23(&mut self) -> CFIE23_W<23> {
        CFIE23_W::new(self)
    }
    #[doc = "Bit 24 - Cancellation Finished Interrupt Enable for Transmit Buffer 24"]
    #[inline(always)]
    pub fn cfie24(&mut self) -> CFIE24_W<24> {
        CFIE24_W::new(self)
    }
    #[doc = "Bit 25 - Cancellation Finished Interrupt Enable for Transmit Buffer 25"]
    #[inline(always)]
    pub fn cfie25(&mut self) -> CFIE25_W<25> {
        CFIE25_W::new(self)
    }
    #[doc = "Bit 26 - Cancellation Finished Interrupt Enable for Transmit Buffer 26"]
    #[inline(always)]
    pub fn cfie26(&mut self) -> CFIE26_W<26> {
        CFIE26_W::new(self)
    }
    #[doc = "Bit 27 - Cancellation Finished Interrupt Enable for Transmit Buffer 27"]
    #[inline(always)]
    pub fn cfie27(&mut self) -> CFIE27_W<27> {
        CFIE27_W::new(self)
    }
    #[doc = "Bit 28 - Cancellation Finished Interrupt Enable for Transmit Buffer 28"]
    #[inline(always)]
    pub fn cfie28(&mut self) -> CFIE28_W<28> {
        CFIE28_W::new(self)
    }
    #[doc = "Bit 29 - Cancellation Finished Interrupt Enable for Transmit Buffer 29"]
    #[inline(always)]
    pub fn cfie29(&mut self) -> CFIE29_W<29> {
        CFIE29_W::new(self)
    }
    #[doc = "Bit 30 - Cancellation Finished Interrupt Enable for Transmit Buffer 30"]
    #[inline(always)]
    pub fn cfie30(&mut self) -> CFIE30_W<30> {
        CFIE30_W::new(self)
    }
    #[doc = "Bit 31 - Cancellation Finished Interrupt Enable for Transmit Buffer 31"]
    #[inline(always)]
    pub fn cfie31(&mut self) -> CFIE31_W<31> {
        CFIE31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Buffer Cancellation Finished Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbcie](index.html) module"]
pub struct TXBCIE_SPEC;
impl crate::RegisterSpec for TXBCIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbcie::R](R) reader structure"]
impl crate::Readable for TXBCIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbcie::W](W) writer structure"]
impl crate::Writable for TXBCIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXBCIE to value 0"]
impl crate::Resettable for TXBCIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
