#[doc = "Register `QIER` writer"]
pub struct W(crate::W<QIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QIER_SPEC>;
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
impl From<crate::W<QIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDX` writer - Index"]
pub type IDX_W<'a, const O: u8> = crate::BitWriter<'a, u32, QIER_SPEC, bool, O>;
#[doc = "Field `DIRCHG` writer - Direction Change"]
pub type DIRCHG_W<'a, const O: u8> = crate::BitWriter<'a, u32, QIER_SPEC, bool, O>;
#[doc = "Field `QERR` writer - Quadrature Error"]
pub type QERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, QIER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Index"]
    #[inline(always)]
    pub fn idx(&mut self) -> IDX_W<0> {
        IDX_W::new(self)
    }
    #[doc = "Bit 1 - Direction Change"]
    #[inline(always)]
    pub fn dirchg(&mut self) -> DIRCHG_W<1> {
        DIRCHG_W::new(self)
    }
    #[doc = "Bit 2 - Quadrature Error"]
    #[inline(always)]
    pub fn qerr(&mut self) -> QERR_W<2> {
        QERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QDEC Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qier](index.html) module"]
pub struct QIER_SPEC;
impl crate::RegisterSpec for QIER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [qier::W](W) writer structure"]
impl crate::Writable for QIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QIER to value 0"]
impl crate::Resettable for QIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
