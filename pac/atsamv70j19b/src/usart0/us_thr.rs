#[doc = "Register `US_THR` writer"]
pub struct W(crate::W<US_THR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_THR_SPEC>;
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
impl From<crate::W<US_THR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_THR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXCHR` writer - Character to be Transmitted"]
pub type TXCHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, US_THR_SPEC, u16, u16, 9, O>;
#[doc = "Field `TXSYNH` writer - Sync Field to be Transmitted"]
pub type TXSYNH_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_THR_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:8 - Character to be Transmitted"]
    #[inline(always)]
    pub fn txchr(&mut self) -> TXCHR_W<0> {
        TXCHR_W::new(self)
    }
    #[doc = "Bit 15 - Sync Field to be Transmitted"]
    #[inline(always)]
    pub fn txsynh(&mut self) -> TXSYNH_W<15> {
        TXSYNH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Holding Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_thr](index.html) module"]
pub struct US_THR_SPEC;
impl crate::RegisterSpec for US_THR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [us_thr::W](W) writer structure"]
impl crate::Writable for US_THR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_THR to value 0"]
impl crate::Resettable for US_THR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
