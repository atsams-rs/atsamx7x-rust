#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTRX` writer - Reset Receiver"]
pub type RSTRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RSTTX` writer - Reset Transmitter"]
pub type RSTTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub type RXDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub type TXDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RSTSTA` writer - Reset Status"]
pub type RSTSTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `REQCLR` writer - Request Clear"]
pub type REQCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 2 - Reset Receiver"]
    #[inline(always)]
    pub fn rstrx(&mut self) -> RSTRX_W<2> {
        RSTRX_W::new(self)
    }
    #[doc = "Bit 3 - Reset Transmitter"]
    #[inline(always)]
    pub fn rsttx(&mut self) -> RSTTX_W<3> {
        RSTTX_W::new(self)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W<4> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 5 - Receiver Disable"]
    #[inline(always)]
    pub fn rxdis(&mut self) -> RXDIS_W<5> {
        RXDIS_W::new(self)
    }
    #[doc = "Bit 6 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W<6> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 7 - Transmitter Disable"]
    #[inline(always)]
    pub fn txdis(&mut self) -> TXDIS_W<7> {
        TXDIS_W::new(self)
    }
    #[doc = "Bit 8 - Reset Status"]
    #[inline(always)]
    pub fn rststa(&mut self) -> RSTSTA_W<8> {
        RSTSTA_W::new(self)
    }
    #[doc = "Bit 12 - Request Clear"]
    #[inline(always)]
    pub fn reqclr(&mut self) -> REQCLR_W<12> {
        REQCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
