#[doc = "Register `OSC` writer"]
pub struct W(crate::W<OSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC_SPEC>;
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
impl From<crate::W<OSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSCH0` writer - Output Selection Clear for PWMH output of the channel 0"]
pub type OSCH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_SPEC, bool, O>;
#[doc = "Field `OSCH1` writer - Output Selection Clear for PWMH output of the channel 1"]
pub type OSCH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_SPEC, bool, O>;
#[doc = "Field `OSCH2` writer - Output Selection Clear for PWMH output of the channel 2"]
pub type OSCH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_SPEC, bool, O>;
#[doc = "Field `OSCH3` writer - Output Selection Clear for PWMH output of the channel 3"]
pub type OSCH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_SPEC, bool, O>;
#[doc = "Field `OSCL0` writer - Output Selection Clear for PWML output of the channel 0"]
pub type OSCL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_SPEC, bool, O>;
#[doc = "Field `OSCL1` writer - Output Selection Clear for PWML output of the channel 1"]
pub type OSCL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_SPEC, bool, O>;
#[doc = "Field `OSCL2` writer - Output Selection Clear for PWML output of the channel 2"]
pub type OSCL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_SPEC, bool, O>;
#[doc = "Field `OSCL3` writer - Output Selection Clear for PWML output of the channel 3"]
pub type OSCL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Output Selection Clear for PWMH output of the channel 0"]
    #[inline(always)]
    pub fn osch0(&mut self) -> OSCH0_W<0> {
        OSCH0_W::new(self)
    }
    #[doc = "Bit 1 - Output Selection Clear for PWMH output of the channel 1"]
    #[inline(always)]
    pub fn osch1(&mut self) -> OSCH1_W<1> {
        OSCH1_W::new(self)
    }
    #[doc = "Bit 2 - Output Selection Clear for PWMH output of the channel 2"]
    #[inline(always)]
    pub fn osch2(&mut self) -> OSCH2_W<2> {
        OSCH2_W::new(self)
    }
    #[doc = "Bit 3 - Output Selection Clear for PWMH output of the channel 3"]
    #[inline(always)]
    pub fn osch3(&mut self) -> OSCH3_W<3> {
        OSCH3_W::new(self)
    }
    #[doc = "Bit 16 - Output Selection Clear for PWML output of the channel 0"]
    #[inline(always)]
    pub fn oscl0(&mut self) -> OSCL0_W<16> {
        OSCL0_W::new(self)
    }
    #[doc = "Bit 17 - Output Selection Clear for PWML output of the channel 1"]
    #[inline(always)]
    pub fn oscl1(&mut self) -> OSCL1_W<17> {
        OSCL1_W::new(self)
    }
    #[doc = "Bit 18 - Output Selection Clear for PWML output of the channel 2"]
    #[inline(always)]
    pub fn oscl2(&mut self) -> OSCL2_W<18> {
        OSCL2_W::new(self)
    }
    #[doc = "Bit 19 - Output Selection Clear for PWML output of the channel 3"]
    #[inline(always)]
    pub fn oscl3(&mut self) -> OSCL3_W<19> {
        OSCL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Output Selection Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc](index.html) module"]
pub struct OSC_SPEC;
impl crate::RegisterSpec for OSC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [osc::W](W) writer structure"]
impl crate::Writable for OSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSC to value 0"]
impl crate::Resettable for OSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
