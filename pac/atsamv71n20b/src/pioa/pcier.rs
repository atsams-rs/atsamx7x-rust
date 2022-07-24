#[doc = "Register `PCIER` writer"]
pub struct W(crate::W<PCIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCIER_SPEC>;
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
impl From<crate::W<PCIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRDY` writer - Parallel Capture Mode Data Ready Interrupt Enable"]
pub type DRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCIER_SPEC, bool, O>;
#[doc = "Field `OVRE` writer - Parallel Capture Mode Overrun Error Interrupt Enable"]
pub type OVRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCIER_SPEC, bool, O>;
#[doc = "Field `ENDRX` writer - End of Reception Transfer Interrupt Enable"]
pub type ENDRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCIER_SPEC, bool, O>;
#[doc = "Field `RXBUFF` writer - Reception Buffer Full Interrupt Enable"]
pub type RXBUFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCIER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Parallel Capture Mode Data Ready Interrupt Enable"]
    #[inline(always)]
    pub fn drdy(&mut self) -> DRDY_W<0> {
        DRDY_W::new(self)
    }
    #[doc = "Bit 1 - Parallel Capture Mode Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OVRE_W<1> {
        OVRE_W::new(self)
    }
    #[doc = "Bit 2 - End of Reception Transfer Interrupt Enable"]
    #[inline(always)]
    pub fn endrx(&mut self) -> ENDRX_W<2> {
        ENDRX_W::new(self)
    }
    #[doc = "Bit 3 - Reception Buffer Full Interrupt Enable"]
    #[inline(always)]
    pub fn rxbuff(&mut self) -> RXBUFF_W<3> {
        RXBUFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parallel Capture Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcier](index.html) module"]
pub struct PCIER_SPEC;
impl crate::RegisterSpec for PCIER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pcier::W](W) writer structure"]
impl crate::Writable for PCIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCIER to value 0"]
impl crate::Resettable for PCIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
