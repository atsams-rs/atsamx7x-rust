#[doc = "Register `GIE` writer"]
pub struct W(crate::W<GIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GIE_SPEC>;
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
impl From<crate::W<GIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IE0` writer - XDMAC Channel 0 Interrupt Enable Bit"]
pub type IE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE1` writer - XDMAC Channel 1 Interrupt Enable Bit"]
pub type IE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE2` writer - XDMAC Channel 2 Interrupt Enable Bit"]
pub type IE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE3` writer - XDMAC Channel 3 Interrupt Enable Bit"]
pub type IE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE4` writer - XDMAC Channel 4 Interrupt Enable Bit"]
pub type IE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE5` writer - XDMAC Channel 5 Interrupt Enable Bit"]
pub type IE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE6` writer - XDMAC Channel 6 Interrupt Enable Bit"]
pub type IE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE7` writer - XDMAC Channel 7 Interrupt Enable Bit"]
pub type IE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE8` writer - XDMAC Channel 8 Interrupt Enable Bit"]
pub type IE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE9` writer - XDMAC Channel 9 Interrupt Enable Bit"]
pub type IE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE10` writer - XDMAC Channel 10 Interrupt Enable Bit"]
pub type IE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE11` writer - XDMAC Channel 11 Interrupt Enable Bit"]
pub type IE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE12` writer - XDMAC Channel 12 Interrupt Enable Bit"]
pub type IE12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE13` writer - XDMAC Channel 13 Interrupt Enable Bit"]
pub type IE13_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE14` writer - XDMAC Channel 14 Interrupt Enable Bit"]
pub type IE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE15` writer - XDMAC Channel 15 Interrupt Enable Bit"]
pub type IE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE16` writer - XDMAC Channel 16 Interrupt Enable Bit"]
pub type IE16_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE17` writer - XDMAC Channel 17 Interrupt Enable Bit"]
pub type IE17_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE18` writer - XDMAC Channel 18 Interrupt Enable Bit"]
pub type IE18_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE19` writer - XDMAC Channel 19 Interrupt Enable Bit"]
pub type IE19_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE20` writer - XDMAC Channel 20 Interrupt Enable Bit"]
pub type IE20_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE21` writer - XDMAC Channel 21 Interrupt Enable Bit"]
pub type IE21_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE22` writer - XDMAC Channel 22 Interrupt Enable Bit"]
pub type IE22_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
#[doc = "Field `IE23` writer - XDMAC Channel 23 Interrupt Enable Bit"]
pub type IE23_W<'a, const O: u8> = crate::BitWriter<'a, u32, GIE_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie0(&mut self) -> IE0_W<0> {
        IE0_W::new(self)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie1(&mut self) -> IE1_W<1> {
        IE1_W::new(self)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie2(&mut self) -> IE2_W<2> {
        IE2_W::new(self)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie3(&mut self) -> IE3_W<3> {
        IE3_W::new(self)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie4(&mut self) -> IE4_W<4> {
        IE4_W::new(self)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie5(&mut self) -> IE5_W<5> {
        IE5_W::new(self)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie6(&mut self) -> IE6_W<6> {
        IE6_W::new(self)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie7(&mut self) -> IE7_W<7> {
        IE7_W::new(self)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie8(&mut self) -> IE8_W<8> {
        IE8_W::new(self)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie9(&mut self) -> IE9_W<9> {
        IE9_W::new(self)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie10(&mut self) -> IE10_W<10> {
        IE10_W::new(self)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie11(&mut self) -> IE11_W<11> {
        IE11_W::new(self)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie12(&mut self) -> IE12_W<12> {
        IE12_W::new(self)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie13(&mut self) -> IE13_W<13> {
        IE13_W::new(self)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie14(&mut self) -> IE14_W<14> {
        IE14_W::new(self)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie15(&mut self) -> IE15_W<15> {
        IE15_W::new(self)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie16(&mut self) -> IE16_W<16> {
        IE16_W::new(self)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie17(&mut self) -> IE17_W<17> {
        IE17_W::new(self)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie18(&mut self) -> IE18_W<18> {
        IE18_W::new(self)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie19(&mut self) -> IE19_W<19> {
        IE19_W::new(self)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie20(&mut self) -> IE20_W<20> {
        IE20_W::new(self)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie21(&mut self) -> IE21_W<21> {
        IE21_W::new(self)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie22(&mut self) -> IE22_W<22> {
        IE22_W::new(self)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Interrupt Enable Bit"]
    #[inline(always)]
    pub fn ie23(&mut self) -> IE23_W<23> {
        IE23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gie](index.html) module"]
pub struct GIE_SPEC;
impl crate::RegisterSpec for GIE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gie::W](W) writer structure"]
impl crate::Writable for GIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GIE to value 0"]
impl crate::Resettable for GIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
