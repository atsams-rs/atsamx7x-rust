#[doc = "Register `HSTPIP` reader"]
pub struct R(crate::R<HSTPIP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTPIP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTPIP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTPIP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSTPIP` writer"]
pub struct W(crate::W<HSTPIP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTPIP_SPEC>;
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
impl From<crate::W<HSTPIP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTPIP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEN0` reader - Pipe 0 Enable"]
pub type PEN0_R = crate::BitReader<bool>;
#[doc = "Field `PEN0` writer - Pipe 0 Enable"]
pub type PEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIP_SPEC, bool, O>;
#[doc = "Field `PEN1` reader - Pipe 1 Enable"]
pub type PEN1_R = crate::BitReader<bool>;
#[doc = "Field `PEN1` writer - Pipe 1 Enable"]
pub type PEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIP_SPEC, bool, O>;
#[doc = "Field `PEN2` reader - Pipe 2 Enable"]
pub type PEN2_R = crate::BitReader<bool>;
#[doc = "Field `PEN2` writer - Pipe 2 Enable"]
pub type PEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIP_SPEC, bool, O>;
#[doc = "Field `PEN3` reader - Pipe 3 Enable"]
pub type PEN3_R = crate::BitReader<bool>;
#[doc = "Field `PEN3` writer - Pipe 3 Enable"]
pub type PEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIP_SPEC, bool, O>;
#[doc = "Field `PEN4` reader - Pipe 4 Enable"]
pub type PEN4_R = crate::BitReader<bool>;
#[doc = "Field `PEN4` writer - Pipe 4 Enable"]
pub type PEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIP_SPEC, bool, O>;
#[doc = "Field `PEN5` reader - Pipe 5 Enable"]
pub type PEN5_R = crate::BitReader<bool>;
#[doc = "Field `PEN5` writer - Pipe 5 Enable"]
pub type PEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIP_SPEC, bool, O>;
#[doc = "Field `PEN6` reader - Pipe 6 Enable"]
pub type PEN6_R = crate::BitReader<bool>;
#[doc = "Field `PEN6` writer - Pipe 6 Enable"]
pub type PEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIP_SPEC, bool, O>;
#[doc = "Field `PEN7` reader - Pipe 7 Enable"]
pub type PEN7_R = crate::BitReader<bool>;
#[doc = "Field `PEN7` writer - Pipe 7 Enable"]
pub type PEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIP_SPEC, bool, O>;
#[doc = "Field `PEN8` reader - Pipe 8 Enable"]
pub type PEN8_R = crate::BitReader<bool>;
#[doc = "Field `PEN8` writer - Pipe 8 Enable"]
pub type PEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIP_SPEC, bool, O>;
#[doc = "Field `PRST0` reader - Pipe 0 Reset"]
pub type PRST0_R = crate::BitReader<bool>;
#[doc = "Field `PRST0` writer - Pipe 0 Reset"]
pub type PRST0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIP_SPEC, bool, O>;
#[doc = "Field `PRST1` reader - Pipe 1 Reset"]
pub type PRST1_R = crate::BitReader<bool>;
#[doc = "Field `PRST1` writer - Pipe 1 Reset"]
pub type PRST1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIP_SPEC, bool, O>;
#[doc = "Field `PRST2` reader - Pipe 2 Reset"]
pub type PRST2_R = crate::BitReader<bool>;
#[doc = "Field `PRST2` writer - Pipe 2 Reset"]
pub type PRST2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIP_SPEC, bool, O>;
#[doc = "Field `PRST3` reader - Pipe 3 Reset"]
pub type PRST3_R = crate::BitReader<bool>;
#[doc = "Field `PRST3` writer - Pipe 3 Reset"]
pub type PRST3_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIP_SPEC, bool, O>;
#[doc = "Field `PRST4` reader - Pipe 4 Reset"]
pub type PRST4_R = crate::BitReader<bool>;
#[doc = "Field `PRST4` writer - Pipe 4 Reset"]
pub type PRST4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIP_SPEC, bool, O>;
#[doc = "Field `PRST5` reader - Pipe 5 Reset"]
pub type PRST5_R = crate::BitReader<bool>;
#[doc = "Field `PRST5` writer - Pipe 5 Reset"]
pub type PRST5_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIP_SPEC, bool, O>;
#[doc = "Field `PRST6` reader - Pipe 6 Reset"]
pub type PRST6_R = crate::BitReader<bool>;
#[doc = "Field `PRST6` writer - Pipe 6 Reset"]
pub type PRST6_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIP_SPEC, bool, O>;
#[doc = "Field `PRST7` reader - Pipe 7 Reset"]
pub type PRST7_R = crate::BitReader<bool>;
#[doc = "Field `PRST7` writer - Pipe 7 Reset"]
pub type PRST7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIP_SPEC, bool, O>;
#[doc = "Field `PRST8` reader - Pipe 8 Reset"]
pub type PRST8_R = crate::BitReader<bool>;
#[doc = "Field `PRST8` writer - Pipe 8 Reset"]
pub type PRST8_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pipe 0 Enable"]
    #[inline(always)]
    pub fn pen0(&self) -> PEN0_R {
        PEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pipe 1 Enable"]
    #[inline(always)]
    pub fn pen1(&self) -> PEN1_R {
        PEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pipe 2 Enable"]
    #[inline(always)]
    pub fn pen2(&self) -> PEN2_R {
        PEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pipe 3 Enable"]
    #[inline(always)]
    pub fn pen3(&self) -> PEN3_R {
        PEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pipe 4 Enable"]
    #[inline(always)]
    pub fn pen4(&self) -> PEN4_R {
        PEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pipe 5 Enable"]
    #[inline(always)]
    pub fn pen5(&self) -> PEN5_R {
        PEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pipe 6 Enable"]
    #[inline(always)]
    pub fn pen6(&self) -> PEN6_R {
        PEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pipe 7 Enable"]
    #[inline(always)]
    pub fn pen7(&self) -> PEN7_R {
        PEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pipe 8 Enable"]
    #[inline(always)]
    pub fn pen8(&self) -> PEN8_R {
        PEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Pipe 0 Reset"]
    #[inline(always)]
    pub fn prst0(&self) -> PRST0_R {
        PRST0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pipe 1 Reset"]
    #[inline(always)]
    pub fn prst1(&self) -> PRST1_R {
        PRST1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pipe 2 Reset"]
    #[inline(always)]
    pub fn prst2(&self) -> PRST2_R {
        PRST2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Pipe 3 Reset"]
    #[inline(always)]
    pub fn prst3(&self) -> PRST3_R {
        PRST3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Pipe 4 Reset"]
    #[inline(always)]
    pub fn prst4(&self) -> PRST4_R {
        PRST4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Pipe 5 Reset"]
    #[inline(always)]
    pub fn prst5(&self) -> PRST5_R {
        PRST5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pipe 6 Reset"]
    #[inline(always)]
    pub fn prst6(&self) -> PRST6_R {
        PRST6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Pipe 7 Reset"]
    #[inline(always)]
    pub fn prst7(&self) -> PRST7_R {
        PRST7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Pipe 8 Reset"]
    #[inline(always)]
    pub fn prst8(&self) -> PRST8_R {
        PRST8_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pipe 0 Enable"]
    #[inline(always)]
    pub fn pen0(&mut self) -> PEN0_W<0> {
        PEN0_W::new(self)
    }
    #[doc = "Bit 1 - Pipe 1 Enable"]
    #[inline(always)]
    pub fn pen1(&mut self) -> PEN1_W<1> {
        PEN1_W::new(self)
    }
    #[doc = "Bit 2 - Pipe 2 Enable"]
    #[inline(always)]
    pub fn pen2(&mut self) -> PEN2_W<2> {
        PEN2_W::new(self)
    }
    #[doc = "Bit 3 - Pipe 3 Enable"]
    #[inline(always)]
    pub fn pen3(&mut self) -> PEN3_W<3> {
        PEN3_W::new(self)
    }
    #[doc = "Bit 4 - Pipe 4 Enable"]
    #[inline(always)]
    pub fn pen4(&mut self) -> PEN4_W<4> {
        PEN4_W::new(self)
    }
    #[doc = "Bit 5 - Pipe 5 Enable"]
    #[inline(always)]
    pub fn pen5(&mut self) -> PEN5_W<5> {
        PEN5_W::new(self)
    }
    #[doc = "Bit 6 - Pipe 6 Enable"]
    #[inline(always)]
    pub fn pen6(&mut self) -> PEN6_W<6> {
        PEN6_W::new(self)
    }
    #[doc = "Bit 7 - Pipe 7 Enable"]
    #[inline(always)]
    pub fn pen7(&mut self) -> PEN7_W<7> {
        PEN7_W::new(self)
    }
    #[doc = "Bit 8 - Pipe 8 Enable"]
    #[inline(always)]
    pub fn pen8(&mut self) -> PEN8_W<8> {
        PEN8_W::new(self)
    }
    #[doc = "Bit 16 - Pipe 0 Reset"]
    #[inline(always)]
    pub fn prst0(&mut self) -> PRST0_W<16> {
        PRST0_W::new(self)
    }
    #[doc = "Bit 17 - Pipe 1 Reset"]
    #[inline(always)]
    pub fn prst1(&mut self) -> PRST1_W<17> {
        PRST1_W::new(self)
    }
    #[doc = "Bit 18 - Pipe 2 Reset"]
    #[inline(always)]
    pub fn prst2(&mut self) -> PRST2_W<18> {
        PRST2_W::new(self)
    }
    #[doc = "Bit 19 - Pipe 3 Reset"]
    #[inline(always)]
    pub fn prst3(&mut self) -> PRST3_W<19> {
        PRST3_W::new(self)
    }
    #[doc = "Bit 20 - Pipe 4 Reset"]
    #[inline(always)]
    pub fn prst4(&mut self) -> PRST4_W<20> {
        PRST4_W::new(self)
    }
    #[doc = "Bit 21 - Pipe 5 Reset"]
    #[inline(always)]
    pub fn prst5(&mut self) -> PRST5_W<21> {
        PRST5_W::new(self)
    }
    #[doc = "Bit 22 - Pipe 6 Reset"]
    #[inline(always)]
    pub fn prst6(&mut self) -> PRST6_W<22> {
        PRST6_W::new(self)
    }
    #[doc = "Bit 23 - Pipe 7 Reset"]
    #[inline(always)]
    pub fn prst7(&mut self) -> PRST7_W<23> {
        PRST7_W::new(self)
    }
    #[doc = "Bit 24 - Pipe 8 Reset"]
    #[inline(always)]
    pub fn prst8(&mut self) -> PRST8_W<24> {
        PRST8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Pipe Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpip](index.html) module"]
pub struct HSTPIP_SPEC;
impl crate::RegisterSpec for HSTPIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstpip::R](R) reader structure"]
impl crate::Readable for HSTPIP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hstpip::W](W) writer structure"]
impl crate::Writable for HSTPIP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSTPIP to value 0"]
impl crate::Resettable for HSTPIP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
