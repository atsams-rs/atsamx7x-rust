#[doc = "Register `CMUPD3` writer"]
pub struct W(crate::W<CMUPD3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMUPD3_SPEC>;
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
impl From<crate::W<CMUPD3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMUPD3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPOLUP` writer - Channel Polarity Update"]
pub type CPOLUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMUPD3_SPEC, bool, O>;
#[doc = "Field `CPOLINVUP` writer - Channel Polarity Inversion Update"]
pub type CPOLINVUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMUPD3_SPEC, bool, O>;
impl W {
    #[doc = "Bit 9 - Channel Polarity Update"]
    #[inline(always)]
    pub fn cpolup(&mut self) -> CPOLUP_W<9> {
        CPOLUP_W::new(self)
    }
    #[doc = "Bit 13 - Channel Polarity Inversion Update"]
    #[inline(always)]
    pub fn cpolinvup(&mut self) -> CPOLINVUP_W<13> {
        CPOLINVUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Mode Update Register (ch_num = 3)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmupd3](index.html) module"]
pub struct CMUPD3_SPEC;
impl crate::RegisterSpec for CMUPD3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmupd3::W](W) writer structure"]
impl crate::Writable for CMUPD3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMUPD3 to value 0"]
impl crate::Resettable for CMUPD3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
