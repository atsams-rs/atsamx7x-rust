#[doc = "Register `SCER` writer"]
pub struct W(crate::W<SCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCER_SPEC>;
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
impl From<crate::W<SCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBCLK` writer - Enable USB FS Clock"]
pub type USBCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCER_SPEC, bool, O>;
#[doc = "Field `PCK0` writer - Programmable Clock 0 Output Enable"]
pub type PCK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCER_SPEC, bool, O>;
#[doc = "Field `PCK1` writer - Programmable Clock 1 Output Enable"]
pub type PCK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCER_SPEC, bool, O>;
#[doc = "Field `PCK2` writer - Programmable Clock 2 Output Enable"]
pub type PCK2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCER_SPEC, bool, O>;
#[doc = "Field `PCK3` writer - Programmable Clock 3 Output Enable"]
pub type PCK3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCER_SPEC, bool, O>;
#[doc = "Field `PCK4` writer - Programmable Clock 4 Output Enable"]
pub type PCK4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCER_SPEC, bool, O>;
#[doc = "Field `PCK5` writer - Programmable Clock 5 Output Enable"]
pub type PCK5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCER_SPEC, bool, O>;
#[doc = "Field `PCK6` writer - Programmable Clock 6 Output Enable"]
pub type PCK6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 5 - Enable USB FS Clock"]
    #[inline(always)]
    pub fn usbclk(&mut self) -> USBCLK_W<5> {
        USBCLK_W::new(self)
    }
    #[doc = "Bit 8 - Programmable Clock 0 Output Enable"]
    #[inline(always)]
    pub fn pck0(&mut self) -> PCK0_W<8> {
        PCK0_W::new(self)
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Enable"]
    #[inline(always)]
    pub fn pck1(&mut self) -> PCK1_W<9> {
        PCK1_W::new(self)
    }
    #[doc = "Bit 10 - Programmable Clock 2 Output Enable"]
    #[inline(always)]
    pub fn pck2(&mut self) -> PCK2_W<10> {
        PCK2_W::new(self)
    }
    #[doc = "Bit 11 - Programmable Clock 3 Output Enable"]
    #[inline(always)]
    pub fn pck3(&mut self) -> PCK3_W<11> {
        PCK3_W::new(self)
    }
    #[doc = "Bit 12 - Programmable Clock 4 Output Enable"]
    #[inline(always)]
    pub fn pck4(&mut self) -> PCK4_W<12> {
        PCK4_W::new(self)
    }
    #[doc = "Bit 13 - Programmable Clock 5 Output Enable"]
    #[inline(always)]
    pub fn pck5(&mut self) -> PCK5_W<13> {
        PCK5_W::new(self)
    }
    #[doc = "Bit 14 - Programmable Clock 6 Output Enable"]
    #[inline(always)]
    pub fn pck6(&mut self) -> PCK6_W<14> {
        PCK6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scer](index.html) module"]
pub struct SCER_SPEC;
impl crate::RegisterSpec for SCER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [scer::W](W) writer structure"]
impl crate::Writable for SCER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCER to value 0"]
impl crate::Resettable for SCER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
