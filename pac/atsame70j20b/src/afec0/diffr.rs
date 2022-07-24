#[doc = "Register `DIFFR` reader"]
pub struct R(crate::R<DIFFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIFFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIFFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIFFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIFFR` writer"]
pub struct W(crate::W<DIFFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIFFR_SPEC>;
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
impl From<crate::W<DIFFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIFFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIFF0` reader - Differential inputs for channel 0"]
pub type DIFF0_R = crate::BitReader<bool>;
#[doc = "Field `DIFF0` writer - Differential inputs for channel 0"]
pub type DIFF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIFFR_SPEC, bool, O>;
#[doc = "Field `DIFF1` reader - Differential inputs for channel 1"]
pub type DIFF1_R = crate::BitReader<bool>;
#[doc = "Field `DIFF1` writer - Differential inputs for channel 1"]
pub type DIFF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIFFR_SPEC, bool, O>;
#[doc = "Field `DIFF2` reader - Differential inputs for channel 2"]
pub type DIFF2_R = crate::BitReader<bool>;
#[doc = "Field `DIFF2` writer - Differential inputs for channel 2"]
pub type DIFF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIFFR_SPEC, bool, O>;
#[doc = "Field `DIFF3` reader - Differential inputs for channel 3"]
pub type DIFF3_R = crate::BitReader<bool>;
#[doc = "Field `DIFF3` writer - Differential inputs for channel 3"]
pub type DIFF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIFFR_SPEC, bool, O>;
#[doc = "Field `DIFF4` reader - Differential inputs for channel 4"]
pub type DIFF4_R = crate::BitReader<bool>;
#[doc = "Field `DIFF4` writer - Differential inputs for channel 4"]
pub type DIFF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIFFR_SPEC, bool, O>;
#[doc = "Field `DIFF5` reader - Differential inputs for channel 5"]
pub type DIFF5_R = crate::BitReader<bool>;
#[doc = "Field `DIFF5` writer - Differential inputs for channel 5"]
pub type DIFF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIFFR_SPEC, bool, O>;
#[doc = "Field `DIFF6` reader - Differential inputs for channel 6"]
pub type DIFF6_R = crate::BitReader<bool>;
#[doc = "Field `DIFF6` writer - Differential inputs for channel 6"]
pub type DIFF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIFFR_SPEC, bool, O>;
#[doc = "Field `DIFF7` reader - Differential inputs for channel 7"]
pub type DIFF7_R = crate::BitReader<bool>;
#[doc = "Field `DIFF7` writer - Differential inputs for channel 7"]
pub type DIFF7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIFFR_SPEC, bool, O>;
#[doc = "Field `DIFF8` reader - Differential inputs for channel 8"]
pub type DIFF8_R = crate::BitReader<bool>;
#[doc = "Field `DIFF8` writer - Differential inputs for channel 8"]
pub type DIFF8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIFFR_SPEC, bool, O>;
#[doc = "Field `DIFF9` reader - Differential inputs for channel 9"]
pub type DIFF9_R = crate::BitReader<bool>;
#[doc = "Field `DIFF9` writer - Differential inputs for channel 9"]
pub type DIFF9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIFFR_SPEC, bool, O>;
#[doc = "Field `DIFF10` reader - Differential inputs for channel 10"]
pub type DIFF10_R = crate::BitReader<bool>;
#[doc = "Field `DIFF10` writer - Differential inputs for channel 10"]
pub type DIFF10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIFFR_SPEC, bool, O>;
#[doc = "Field `DIFF11` reader - Differential inputs for channel 11"]
pub type DIFF11_R = crate::BitReader<bool>;
#[doc = "Field `DIFF11` writer - Differential inputs for channel 11"]
pub type DIFF11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIFFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Differential inputs for channel 0"]
    #[inline(always)]
    pub fn diff0(&self) -> DIFF0_R {
        DIFF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Differential inputs for channel 1"]
    #[inline(always)]
    pub fn diff1(&self) -> DIFF1_R {
        DIFF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Differential inputs for channel 2"]
    #[inline(always)]
    pub fn diff2(&self) -> DIFF2_R {
        DIFF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Differential inputs for channel 3"]
    #[inline(always)]
    pub fn diff3(&self) -> DIFF3_R {
        DIFF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Differential inputs for channel 4"]
    #[inline(always)]
    pub fn diff4(&self) -> DIFF4_R {
        DIFF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Differential inputs for channel 5"]
    #[inline(always)]
    pub fn diff5(&self) -> DIFF5_R {
        DIFF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Differential inputs for channel 6"]
    #[inline(always)]
    pub fn diff6(&self) -> DIFF6_R {
        DIFF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Differential inputs for channel 7"]
    #[inline(always)]
    pub fn diff7(&self) -> DIFF7_R {
        DIFF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Differential inputs for channel 8"]
    #[inline(always)]
    pub fn diff8(&self) -> DIFF8_R {
        DIFF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Differential inputs for channel 9"]
    #[inline(always)]
    pub fn diff9(&self) -> DIFF9_R {
        DIFF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Differential inputs for channel 10"]
    #[inline(always)]
    pub fn diff10(&self) -> DIFF10_R {
        DIFF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Differential inputs for channel 11"]
    #[inline(always)]
    pub fn diff11(&self) -> DIFF11_R {
        DIFF11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Differential inputs for channel 0"]
    #[inline(always)]
    pub fn diff0(&mut self) -> DIFF0_W<0> {
        DIFF0_W::new(self)
    }
    #[doc = "Bit 1 - Differential inputs for channel 1"]
    #[inline(always)]
    pub fn diff1(&mut self) -> DIFF1_W<1> {
        DIFF1_W::new(self)
    }
    #[doc = "Bit 2 - Differential inputs for channel 2"]
    #[inline(always)]
    pub fn diff2(&mut self) -> DIFF2_W<2> {
        DIFF2_W::new(self)
    }
    #[doc = "Bit 3 - Differential inputs for channel 3"]
    #[inline(always)]
    pub fn diff3(&mut self) -> DIFF3_W<3> {
        DIFF3_W::new(self)
    }
    #[doc = "Bit 4 - Differential inputs for channel 4"]
    #[inline(always)]
    pub fn diff4(&mut self) -> DIFF4_W<4> {
        DIFF4_W::new(self)
    }
    #[doc = "Bit 5 - Differential inputs for channel 5"]
    #[inline(always)]
    pub fn diff5(&mut self) -> DIFF5_W<5> {
        DIFF5_W::new(self)
    }
    #[doc = "Bit 6 - Differential inputs for channel 6"]
    #[inline(always)]
    pub fn diff6(&mut self) -> DIFF6_W<6> {
        DIFF6_W::new(self)
    }
    #[doc = "Bit 7 - Differential inputs for channel 7"]
    #[inline(always)]
    pub fn diff7(&mut self) -> DIFF7_W<7> {
        DIFF7_W::new(self)
    }
    #[doc = "Bit 8 - Differential inputs for channel 8"]
    #[inline(always)]
    pub fn diff8(&mut self) -> DIFF8_W<8> {
        DIFF8_W::new(self)
    }
    #[doc = "Bit 9 - Differential inputs for channel 9"]
    #[inline(always)]
    pub fn diff9(&mut self) -> DIFF9_W<9> {
        DIFF9_W::new(self)
    }
    #[doc = "Bit 10 - Differential inputs for channel 10"]
    #[inline(always)]
    pub fn diff10(&mut self) -> DIFF10_W<10> {
        DIFF10_W::new(self)
    }
    #[doc = "Bit 11 - Differential inputs for channel 11"]
    #[inline(always)]
    pub fn diff11(&mut self) -> DIFF11_W<11> {
        DIFF11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Channel Differential Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diffr](index.html) module"]
pub struct DIFFR_SPEC;
impl crate::RegisterSpec for DIFFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diffr::R](R) reader structure"]
impl crate::Readable for DIFFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diffr::W](W) writer structure"]
impl crate::Writable for DIFFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIFFR to value 0"]
impl crate::Resettable for DIFFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
