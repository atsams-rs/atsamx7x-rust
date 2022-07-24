#[doc = "Register `US_IDR` writer"]
pub struct W(crate::W<US_IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_IDR_SPEC>;
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
impl From<crate::W<US_IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Disable"]
pub type RXRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_SPEC, bool, O>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Disable"]
pub type TXRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_SPEC, bool, O>;
#[doc = "Field `RXBRK` writer - Receiver Break Interrupt Disable"]
pub type RXBRK_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_SPEC, bool, O>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OVRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_SPEC, bool, O>;
#[doc = "Field `FRAME` writer - Framing Error Interrupt Disable"]
pub type FRAME_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_SPEC, bool, O>;
#[doc = "Field `PARE` writer - Parity Error Interrupt Disable"]
pub type PARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_SPEC, bool, O>;
#[doc = "Field `TIMEOUT` writer - Time-out Interrupt Disable"]
pub type TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_SPEC, bool, O>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Disable"]
pub type TXEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_SPEC, bool, O>;
#[doc = "Field `ITER` writer - Max Number of Repetitions Reached Interrupt Disable"]
pub type ITER_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_SPEC, bool, O>;
#[doc = "Field `NACK` writer - Non Acknowledge Interrupt Disable"]
pub type NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_SPEC, bool, O>;
#[doc = "Field `RIIC` writer - Ring Indicator Input Change Disable"]
pub type RIIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_SPEC, bool, O>;
#[doc = "Field `DSRIC` writer - Data Set Ready Input Change Disable"]
pub type DSRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_SPEC, bool, O>;
#[doc = "Field `DCDIC` writer - Data Carrier Detect Input Change Interrupt Disable"]
pub type DCDIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_SPEC, bool, O>;
#[doc = "Field `CTSIC` writer - Clear to Send Input Change Interrupt Disable"]
pub type CTSIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_SPEC, bool, O>;
#[doc = "Field `MANE` writer - Manchester Error Interrupt Disable"]
pub type MANE_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Disable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RXRDY_W<0> {
        RXRDY_W::new(self)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Disable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TXRDY_W<1> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 2 - Receiver Break Interrupt Disable"]
    #[inline(always)]
    pub fn rxbrk(&mut self) -> RXBRK_W<2> {
        RXBRK_W::new(self)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OVRE_W<5> {
        OVRE_W::new(self)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Disable"]
    #[inline(always)]
    pub fn frame(&mut self) -> FRAME_W<6> {
        FRAME_W::new(self)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Disable"]
    #[inline(always)]
    pub fn pare(&mut self) -> PARE_W<7> {
        PARE_W::new(self)
    }
    #[doc = "Bit 8 - Time-out Interrupt Disable"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<8> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Disable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TXEMPTY_W<9> {
        TXEMPTY_W::new(self)
    }
    #[doc = "Bit 10 - Max Number of Repetitions Reached Interrupt Disable"]
    #[inline(always)]
    pub fn iter(&mut self) -> ITER_W<10> {
        ITER_W::new(self)
    }
    #[doc = "Bit 13 - Non Acknowledge Interrupt Disable"]
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W<13> {
        NACK_W::new(self)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Disable"]
    #[inline(always)]
    pub fn riic(&mut self) -> RIIC_W<16> {
        RIIC_W::new(self)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Disable"]
    #[inline(always)]
    pub fn dsric(&mut self) -> DSRIC_W<17> {
        DSRIC_W::new(self)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Interrupt Disable"]
    #[inline(always)]
    pub fn dcdic(&mut self) -> DCDIC_W<18> {
        DCDIC_W::new(self)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Interrupt Disable"]
    #[inline(always)]
    pub fn ctsic(&mut self) -> CTSIC_W<19> {
        CTSIC_W::new(self)
    }
    #[doc = "Bit 24 - Manchester Error Interrupt Disable"]
    #[inline(always)]
    pub fn mane(&mut self) -> MANE_W<24> {
        MANE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_idr](index.html) module"]
pub struct US_IDR_SPEC;
impl crate::RegisterSpec for US_IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [us_idr::W](W) writer structure"]
impl crate::Writable for US_IDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_IDR to value 0"]
impl crate::Resettable for US_IDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
