#[doc = "Register `GWS` reader"]
pub struct R(crate::R<GWS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GWS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GWS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GWS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GWS` writer"]
pub struct W(crate::W<GWS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GWS_SPEC>;
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
impl From<crate::W<GWS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GWS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WS0` reader - XDMAC Channel 0 Write Suspend Bit"]
pub type WS0_R = crate::BitReader<bool>;
#[doc = "Field `WS0` writer - XDMAC Channel 0 Write Suspend Bit"]
pub type WS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS1` reader - XDMAC Channel 1 Write Suspend Bit"]
pub type WS1_R = crate::BitReader<bool>;
#[doc = "Field `WS1` writer - XDMAC Channel 1 Write Suspend Bit"]
pub type WS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS2` reader - XDMAC Channel 2 Write Suspend Bit"]
pub type WS2_R = crate::BitReader<bool>;
#[doc = "Field `WS2` writer - XDMAC Channel 2 Write Suspend Bit"]
pub type WS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS3` reader - XDMAC Channel 3 Write Suspend Bit"]
pub type WS3_R = crate::BitReader<bool>;
#[doc = "Field `WS3` writer - XDMAC Channel 3 Write Suspend Bit"]
pub type WS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS4` reader - XDMAC Channel 4 Write Suspend Bit"]
pub type WS4_R = crate::BitReader<bool>;
#[doc = "Field `WS4` writer - XDMAC Channel 4 Write Suspend Bit"]
pub type WS4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS5` reader - XDMAC Channel 5 Write Suspend Bit"]
pub type WS5_R = crate::BitReader<bool>;
#[doc = "Field `WS5` writer - XDMAC Channel 5 Write Suspend Bit"]
pub type WS5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS6` reader - XDMAC Channel 6 Write Suspend Bit"]
pub type WS6_R = crate::BitReader<bool>;
#[doc = "Field `WS6` writer - XDMAC Channel 6 Write Suspend Bit"]
pub type WS6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS7` reader - XDMAC Channel 7 Write Suspend Bit"]
pub type WS7_R = crate::BitReader<bool>;
#[doc = "Field `WS7` writer - XDMAC Channel 7 Write Suspend Bit"]
pub type WS7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS8` reader - XDMAC Channel 8 Write Suspend Bit"]
pub type WS8_R = crate::BitReader<bool>;
#[doc = "Field `WS8` writer - XDMAC Channel 8 Write Suspend Bit"]
pub type WS8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS9` reader - XDMAC Channel 9 Write Suspend Bit"]
pub type WS9_R = crate::BitReader<bool>;
#[doc = "Field `WS9` writer - XDMAC Channel 9 Write Suspend Bit"]
pub type WS9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS10` reader - XDMAC Channel 10 Write Suspend Bit"]
pub type WS10_R = crate::BitReader<bool>;
#[doc = "Field `WS10` writer - XDMAC Channel 10 Write Suspend Bit"]
pub type WS10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS11` reader - XDMAC Channel 11 Write Suspend Bit"]
pub type WS11_R = crate::BitReader<bool>;
#[doc = "Field `WS11` writer - XDMAC Channel 11 Write Suspend Bit"]
pub type WS11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS12` reader - XDMAC Channel 12 Write Suspend Bit"]
pub type WS12_R = crate::BitReader<bool>;
#[doc = "Field `WS12` writer - XDMAC Channel 12 Write Suspend Bit"]
pub type WS12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS13` reader - XDMAC Channel 13 Write Suspend Bit"]
pub type WS13_R = crate::BitReader<bool>;
#[doc = "Field `WS13` writer - XDMAC Channel 13 Write Suspend Bit"]
pub type WS13_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS14` reader - XDMAC Channel 14 Write Suspend Bit"]
pub type WS14_R = crate::BitReader<bool>;
#[doc = "Field `WS14` writer - XDMAC Channel 14 Write Suspend Bit"]
pub type WS14_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS15` reader - XDMAC Channel 15 Write Suspend Bit"]
pub type WS15_R = crate::BitReader<bool>;
#[doc = "Field `WS15` writer - XDMAC Channel 15 Write Suspend Bit"]
pub type WS15_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS16` reader - XDMAC Channel 16 Write Suspend Bit"]
pub type WS16_R = crate::BitReader<bool>;
#[doc = "Field `WS16` writer - XDMAC Channel 16 Write Suspend Bit"]
pub type WS16_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS17` reader - XDMAC Channel 17 Write Suspend Bit"]
pub type WS17_R = crate::BitReader<bool>;
#[doc = "Field `WS17` writer - XDMAC Channel 17 Write Suspend Bit"]
pub type WS17_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS18` reader - XDMAC Channel 18 Write Suspend Bit"]
pub type WS18_R = crate::BitReader<bool>;
#[doc = "Field `WS18` writer - XDMAC Channel 18 Write Suspend Bit"]
pub type WS18_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS19` reader - XDMAC Channel 19 Write Suspend Bit"]
pub type WS19_R = crate::BitReader<bool>;
#[doc = "Field `WS19` writer - XDMAC Channel 19 Write Suspend Bit"]
pub type WS19_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS20` reader - XDMAC Channel 20 Write Suspend Bit"]
pub type WS20_R = crate::BitReader<bool>;
#[doc = "Field `WS20` writer - XDMAC Channel 20 Write Suspend Bit"]
pub type WS20_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS21` reader - XDMAC Channel 21 Write Suspend Bit"]
pub type WS21_R = crate::BitReader<bool>;
#[doc = "Field `WS21` writer - XDMAC Channel 21 Write Suspend Bit"]
pub type WS21_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS22` reader - XDMAC Channel 22 Write Suspend Bit"]
pub type WS22_R = crate::BitReader<bool>;
#[doc = "Field `WS22` writer - XDMAC Channel 22 Write Suspend Bit"]
pub type WS22_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
#[doc = "Field `WS23` reader - XDMAC Channel 23 Write Suspend Bit"]
pub type WS23_R = crate::BitReader<bool>;
#[doc = "Field `WS23` writer - XDMAC Channel 23 Write Suspend Bit"]
pub type WS23_W<'a, const O: u8> = crate::BitWriter<'a, u32, GWS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - XDMAC Channel 0 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws0(&self) -> WS0_R {
        WS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws1(&self) -> WS1_R {
        WS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws2(&self) -> WS2_R {
        WS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws3(&self) -> WS3_R {
        WS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws4(&self) -> WS4_R {
        WS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws5(&self) -> WS5_R {
        WS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws6(&self) -> WS6_R {
        WS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws7(&self) -> WS7_R {
        WS7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws8(&self) -> WS8_R {
        WS8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws9(&self) -> WS9_R {
        WS9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws10(&self) -> WS10_R {
        WS10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws11(&self) -> WS11_R {
        WS11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws12(&self) -> WS12_R {
        WS12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws13(&self) -> WS13_R {
        WS13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws14(&self) -> WS14_R {
        WS14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws15(&self) -> WS15_R {
        WS15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws16(&self) -> WS16_R {
        WS16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws17(&self) -> WS17_R {
        WS17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws18(&self) -> WS18_R {
        WS18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws19(&self) -> WS19_R {
        WS19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws20(&self) -> WS20_R {
        WS20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws21(&self) -> WS21_R {
        WS21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws22(&self) -> WS22_R {
        WS22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws23(&self) -> WS23_R {
        WS23_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XDMAC Channel 0 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws0(&mut self) -> WS0_W<0> {
        WS0_W::new(self)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws1(&mut self) -> WS1_W<1> {
        WS1_W::new(self)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws2(&mut self) -> WS2_W<2> {
        WS2_W::new(self)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws3(&mut self) -> WS3_W<3> {
        WS3_W::new(self)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws4(&mut self) -> WS4_W<4> {
        WS4_W::new(self)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws5(&mut self) -> WS5_W<5> {
        WS5_W::new(self)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws6(&mut self) -> WS6_W<6> {
        WS6_W::new(self)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws7(&mut self) -> WS7_W<7> {
        WS7_W::new(self)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws8(&mut self) -> WS8_W<8> {
        WS8_W::new(self)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws9(&mut self) -> WS9_W<9> {
        WS9_W::new(self)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws10(&mut self) -> WS10_W<10> {
        WS10_W::new(self)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws11(&mut self) -> WS11_W<11> {
        WS11_W::new(self)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws12(&mut self) -> WS12_W<12> {
        WS12_W::new(self)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws13(&mut self) -> WS13_W<13> {
        WS13_W::new(self)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws14(&mut self) -> WS14_W<14> {
        WS14_W::new(self)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws15(&mut self) -> WS15_W<15> {
        WS15_W::new(self)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws16(&mut self) -> WS16_W<16> {
        WS16_W::new(self)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws17(&mut self) -> WS17_W<17> {
        WS17_W::new(self)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws18(&mut self) -> WS18_W<18> {
        WS18_W::new(self)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws19(&mut self) -> WS19_W<19> {
        WS19_W::new(self)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws20(&mut self) -> WS20_W<20> {
        WS20_W::new(self)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws21(&mut self) -> WS21_W<21> {
        WS21_W::new(self)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws22(&mut self) -> WS22_W<22> {
        WS22_W::new(self)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws23(&mut self) -> WS23_W<23> {
        WS23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Channel Write Suspend Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gws](index.html) module"]
pub struct GWS_SPEC;
impl crate::RegisterSpec for GWS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gws::R](R) reader structure"]
impl crate::Readable for GWS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gws::W](W) writer structure"]
impl crate::Writable for GWS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GWS to value 0"]
impl crate::Resettable for GWS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
