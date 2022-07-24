#[doc = "Register `TSTA1` reader"]
pub struct R(crate::R<TSTA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSTA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSTA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSTA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSTA1` writer"]
pub struct W(crate::W<TSTA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSTA1_SPEC>;
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
impl From<crate::W<TSTA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSTA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CounterA` reader - Counter A"]
pub type COUNTERA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CounterA` writer - Counter A"]
pub type COUNTERA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSTA1_SPEC, u16, u16, 15, O>;
#[doc = "Field `LoadCntA` reader - Load CounterA"]
pub type LOADCNTA_R = crate::BitReader<bool>;
#[doc = "Field `LoadCntA` writer - Load CounterA"]
pub type LOADCNTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSTA1_SPEC, bool, O>;
#[doc = "Field `CounterB` reader - Counter B"]
pub type COUNTERB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CounterB` writer - Counter B"]
pub type COUNTERB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSTA1_SPEC, u8, u8, 6, O>;
#[doc = "Field `LoadCntB` reader - Load CounterB"]
pub type LOADCNTB_R = crate::BitReader<bool>;
#[doc = "Field `LoadCntB` writer - Load CounterB"]
pub type LOADCNTB_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSTA1_SPEC, bool, O>;
#[doc = "Field `SOFCntMa1` reader - SOF Counter Max"]
pub type SOFCNTMA1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SOFCntMa1` writer - SOF Counter Max"]
pub type SOFCNTMA1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSTA1_SPEC, u8, u8, 7, O>;
#[doc = "Field `LoadSOFCnt` reader - Load SOF Counter"]
pub type LOADSOFCNT_R = crate::BitReader<bool>;
#[doc = "Field `LoadSOFCnt` writer - Load SOF Counter"]
pub type LOADSOFCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSTA1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:14 - Counter A"]
    #[inline(always)]
    pub fn counter_a(&self) -> COUNTERA_R {
        COUNTERA_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Load CounterA"]
    #[inline(always)]
    pub fn load_cnt_a(&self) -> LOADCNTA_R {
        LOADCNTA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Counter B"]
    #[inline(always)]
    pub fn counter_b(&self) -> COUNTERB_R {
        COUNTERB_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Load CounterB"]
    #[inline(always)]
    pub fn load_cnt_b(&self) -> LOADCNTB_R {
        LOADCNTB_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - SOF Counter Max"]
    #[inline(always)]
    pub fn sofcnt_ma1(&self) -> SOFCNTMA1_R {
        SOFCNTMA1_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Load SOF Counter"]
    #[inline(always)]
    pub fn load_sofcnt(&self) -> LOADSOFCNT_R {
        LOADSOFCNT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Counter A"]
    #[inline(always)]
    pub fn counter_a(&mut self) -> COUNTERA_W<0> {
        COUNTERA_W::new(self)
    }
    #[doc = "Bit 15 - Load CounterA"]
    #[inline(always)]
    pub fn load_cnt_a(&mut self) -> LOADCNTA_W<15> {
        LOADCNTA_W::new(self)
    }
    #[doc = "Bits 16:21 - Counter B"]
    #[inline(always)]
    pub fn counter_b(&mut self) -> COUNTERB_W<16> {
        COUNTERB_W::new(self)
    }
    #[doc = "Bit 23 - Load CounterB"]
    #[inline(always)]
    pub fn load_cnt_b(&mut self) -> LOADCNTB_W<23> {
        LOADCNTB_W::new(self)
    }
    #[doc = "Bits 24:30 - SOF Counter Max"]
    #[inline(always)]
    pub fn sofcnt_ma1(&mut self) -> SOFCNTMA1_W<24> {
        SOFCNTMA1_W::new(self)
    }
    #[doc = "Bit 31 - Load SOF Counter"]
    #[inline(always)]
    pub fn load_sofcnt(&mut self) -> LOADSOFCNT_W<31> {
        LOADSOFCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Test A1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsta1](index.html) module"]
pub struct TSTA1_SPEC;
impl crate::RegisterSpec for TSTA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsta1::R](R) reader structure"]
impl crate::Readable for TSTA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsta1::W](W) writer structure"]
impl crate::Writable for TSTA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSTA1 to value 0"]
impl crate::Resettable for TSTA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
