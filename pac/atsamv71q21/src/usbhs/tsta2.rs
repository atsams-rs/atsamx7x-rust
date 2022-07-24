#[doc = "Register `TSTA2` reader"]
pub struct R(crate::R<TSTA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSTA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSTA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSTA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSTA2` writer"]
pub struct W(crate::W<TSTA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSTA2_SPEC>;
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
impl From<crate::W<TSTA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSTA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FullDetachEn` reader - Full Detach Enable"]
pub type FULLDETACHEN_R = crate::BitReader<bool>;
#[doc = "Field `FullDetachEn` writer - Full Detach Enable"]
pub type FULLDETACHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSTA2_SPEC, bool, O>;
#[doc = "Field `HSSerialMode` reader - HS Serial Mode"]
pub type HSSERIALMODE_R = crate::BitReader<bool>;
#[doc = "Field `HSSerialMode` writer - HS Serial Mode"]
pub type HSSERIALMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSTA2_SPEC, bool, O>;
#[doc = "Field `LoopBackMode` reader - Loop-back Mode"]
pub type LOOPBACKMODE_R = crate::BitReader<bool>;
#[doc = "Field `LoopBackMode` writer - Loop-back Mode"]
pub type LOOPBACKMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSTA2_SPEC, bool, O>;
#[doc = "Field `DisableGatedClock` reader - Disable Gated Clock"]
pub type DISABLEGATEDCLOCK_R = crate::BitReader<bool>;
#[doc = "Field `DisableGatedClock` writer - Disable Gated Clock"]
pub type DISABLEGATEDCLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSTA2_SPEC, bool, O>;
#[doc = "Field `ForceSuspendMTo1` reader - Force SuspendM to 1"]
pub type FORCESUSPENDMTO1_R = crate::BitReader<bool>;
#[doc = "Field `ForceSuspendMTo1` writer - Force SuspendM to 1"]
pub type FORCESUSPENDMTO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSTA2_SPEC, bool, O>;
#[doc = "Field `ByPassDpll` reader - Bypass DPLL"]
pub type BYPASSDPLL_R = crate::BitReader<bool>;
#[doc = "Field `ByPassDpll` writer - Bypass DPLL"]
pub type BYPASSDPLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSTA2_SPEC, bool, O>;
#[doc = "Field `HostHSDisconnectDisable` reader - Host HS Disconnect Disable"]
pub type HOSTHSDISCONNECTDISABLE_R = crate::BitReader<bool>;
#[doc = "Field `HostHSDisconnectDisable` writer - Host HS Disconnect Disable"]
pub type HOSTHSDISCONNECTDISABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TSTA2_SPEC, bool, O>;
#[doc = "Field `ForceHSRst_50ms` reader - Force HS Reset to 50 ms"]
pub type FORCEHSRST_50MS_R = crate::BitReader<bool>;
#[doc = "Field `ForceHSRst_50ms` writer - Force HS Reset to 50 ms"]
pub type FORCEHSRST_50MS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSTA2_SPEC, bool, O>;
#[doc = "Field `RemovePUWhenTX` reader - Remove Pull-up When TX"]
pub type REMOVEPUWHENTX_R = crate::BitReader<bool>;
#[doc = "Field `RemovePUWhenTX` writer - Remove Pull-up When TX"]
pub type REMOVEPUWHENTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, TSTA2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Full Detach Enable"]
    #[inline(always)]
    pub fn full_detach_en(&self) -> FULLDETACHEN_R {
        FULLDETACHEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HS Serial Mode"]
    #[inline(always)]
    pub fn hsserial_mode(&self) -> HSSERIALMODE_R {
        HSSERIALMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Loop-back Mode"]
    #[inline(always)]
    pub fn loop_back_mode(&self) -> LOOPBACKMODE_R {
        LOOPBACKMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable Gated Clock"]
    #[inline(always)]
    pub fn disable_gated_clock(&self) -> DISABLEGATEDCLOCK_R {
        DISABLEGATEDCLOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force SuspendM to 1"]
    #[inline(always)]
    pub fn force_suspend_mto1(&self) -> FORCESUSPENDMTO1_R {
        FORCESUSPENDMTO1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass DPLL"]
    #[inline(always)]
    pub fn by_pass_dpll(&self) -> BYPASSDPLL_R {
        BYPASSDPLL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Host HS Disconnect Disable"]
    #[inline(always)]
    pub fn host_hsdisconnect_disable(&self) -> HOSTHSDISCONNECTDISABLE_R {
        HOSTHSDISCONNECTDISABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force HS Reset to 50 ms"]
    #[inline(always)]
    pub fn force_hsrst_50ms(&self) -> FORCEHSRST_50MS_R {
        FORCEHSRST_50MS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Remove Pull-up When TX"]
    #[inline(always)]
    pub fn remove_puwhen_tx(&self) -> REMOVEPUWHENTX_R {
        REMOVEPUWHENTX_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Full Detach Enable"]
    #[inline(always)]
    pub fn full_detach_en(&mut self) -> FULLDETACHEN_W<0> {
        FULLDETACHEN_W::new(self)
    }
    #[doc = "Bit 1 - HS Serial Mode"]
    #[inline(always)]
    pub fn hsserial_mode(&mut self) -> HSSERIALMODE_W<1> {
        HSSERIALMODE_W::new(self)
    }
    #[doc = "Bit 2 - Loop-back Mode"]
    #[inline(always)]
    pub fn loop_back_mode(&mut self) -> LOOPBACKMODE_W<2> {
        LOOPBACKMODE_W::new(self)
    }
    #[doc = "Bit 3 - Disable Gated Clock"]
    #[inline(always)]
    pub fn disable_gated_clock(&mut self) -> DISABLEGATEDCLOCK_W<3> {
        DISABLEGATEDCLOCK_W::new(self)
    }
    #[doc = "Bit 4 - Force SuspendM to 1"]
    #[inline(always)]
    pub fn force_suspend_mto1(&mut self) -> FORCESUSPENDMTO1_W<4> {
        FORCESUSPENDMTO1_W::new(self)
    }
    #[doc = "Bit 5 - Bypass DPLL"]
    #[inline(always)]
    pub fn by_pass_dpll(&mut self) -> BYPASSDPLL_W<5> {
        BYPASSDPLL_W::new(self)
    }
    #[doc = "Bit 6 - Host HS Disconnect Disable"]
    #[inline(always)]
    pub fn host_hsdisconnect_disable(&mut self) -> HOSTHSDISCONNECTDISABLE_W<6> {
        HOSTHSDISCONNECTDISABLE_W::new(self)
    }
    #[doc = "Bit 7 - Force HS Reset to 50 ms"]
    #[inline(always)]
    pub fn force_hsrst_50ms(&mut self) -> FORCEHSRST_50MS_W<7> {
        FORCEHSRST_50MS_W::new(self)
    }
    #[doc = "Bit 9 - Remove Pull-up When TX"]
    #[inline(always)]
    pub fn remove_puwhen_tx(&mut self) -> REMOVEPUWHENTX_W<9> {
        REMOVEPUWHENTX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Test A2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsta2](index.html) module"]
pub struct TSTA2_SPEC;
impl crate::RegisterSpec for TSTA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsta2::R](R) reader structure"]
impl crate::Readable for TSTA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsta2::W](W) writer structure"]
impl crate::Writable for TSTA2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSTA2 to value 0"]
impl crate::Resettable for TSTA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
