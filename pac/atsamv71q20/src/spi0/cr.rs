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
#[doc = "Field `SPIEN` writer - SPI Enable"]
pub type SPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SPIDIS` writer - SPI Disable"]
pub type SPIDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SWRST` writer - SPI Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `REQCLR` writer - Request to Clear the Comparison Trigger"]
pub type REQCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TXFCLR` writer - Transmit FIFO Clear"]
pub type TXFCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `RXFCLR` writer - Receive FIFO Clear"]
pub type RXFCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `LASTXFER` writer - Last Transfer"]
pub type LASTXFER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FIFOEN` writer - FIFO Enable"]
pub type FIFOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FIFODIS` writer - FIFO Disable"]
pub type FIFODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - SPI Enable"]
    #[inline(always)]
    pub fn spien(&mut self) -> SPIEN_W<0> {
        SPIEN_W::new(self)
    }
    #[doc = "Bit 1 - SPI Disable"]
    #[inline(always)]
    pub fn spidis(&mut self) -> SPIDIS_W<1> {
        SPIDIS_W::new(self)
    }
    #[doc = "Bit 7 - SPI Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W<7> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 12 - Request to Clear the Comparison Trigger"]
    #[inline(always)]
    pub fn reqclr(&mut self) -> REQCLR_W<12> {
        REQCLR_W::new(self)
    }
    #[doc = "Bit 16 - Transmit FIFO Clear"]
    #[inline(always)]
    pub fn txfclr(&mut self) -> TXFCLR_W<16> {
        TXFCLR_W::new(self)
    }
    #[doc = "Bit 17 - Receive FIFO Clear"]
    #[inline(always)]
    pub fn rxfclr(&mut self) -> RXFCLR_W<17> {
        RXFCLR_W::new(self)
    }
    #[doc = "Bit 24 - Last Transfer"]
    #[inline(always)]
    pub fn lastxfer(&mut self) -> LASTXFER_W<24> {
        LASTXFER_W::new(self)
    }
    #[doc = "Bit 30 - FIFO Enable"]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FIFOEN_W<30> {
        FIFOEN_W::new(self)
    }
    #[doc = "Bit 31 - FIFO Disable"]
    #[inline(always)]
    pub fn fifodis(&mut self) -> FIFODIS_W<31> {
        FIFODIS_W::new(self)
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
