#[doc = "Register `US_IDR_LON_MODE` writer"]
pub struct W(crate::W<US_IDR_LON_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<US_IDR_LON_MODE_SPEC>;
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
impl From<crate::W<US_IDR_LON_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<US_IDR_LON_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Disable"]
pub type RXRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_LON_MODE_SPEC, bool, O>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Disable"]
pub type TXRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_LON_MODE_SPEC, bool, O>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OVRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_LON_MODE_SPEC, bool, O>;
#[doc = "Field `LSFE` writer - LON Short Frame Error Interrupt Disable"]
pub type LSFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_LON_MODE_SPEC, bool, O>;
#[doc = "Field `LCRCE` writer - LON CRC Error Interrupt Disable"]
pub type LCRCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_LON_MODE_SPEC, bool, O>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Disable"]
pub type TXEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_LON_MODE_SPEC, bool, O>;
#[doc = "Field `UNRE` writer - SPI Underrun Error Interrupt Disable"]
pub type UNRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_LON_MODE_SPEC, bool, O>;
#[doc = "Field `LTXD` writer - LON Transmission Done Interrupt Disable"]
pub type LTXD_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_LON_MODE_SPEC, bool, O>;
#[doc = "Field `LCOL` writer - LON Collision Interrupt Disable"]
pub type LCOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_LON_MODE_SPEC, bool, O>;
#[doc = "Field `LFET` writer - LON Frame Early Termination Interrupt Disable"]
pub type LFET_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_LON_MODE_SPEC, bool, O>;
#[doc = "Field `LRXD` writer - LON Reception Done Interrupt Disable"]
pub type LRXD_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_LON_MODE_SPEC, bool, O>;
#[doc = "Field `LBLOVFE` writer - LON Backlog Overflow Error Interrupt Disable"]
pub type LBLOVFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, US_IDR_LON_MODE_SPEC, bool, O>;
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
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OVRE_W<5> {
        OVRE_W::new(self)
    }
    #[doc = "Bit 6 - LON Short Frame Error Interrupt Disable"]
    #[inline(always)]
    pub fn lsfe(&mut self) -> LSFE_W<6> {
        LSFE_W::new(self)
    }
    #[doc = "Bit 7 - LON CRC Error Interrupt Disable"]
    #[inline(always)]
    pub fn lcrce(&mut self) -> LCRCE_W<7> {
        LCRCE_W::new(self)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Disable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TXEMPTY_W<9> {
        TXEMPTY_W::new(self)
    }
    #[doc = "Bit 10 - SPI Underrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn unre(&mut self) -> UNRE_W<10> {
        UNRE_W::new(self)
    }
    #[doc = "Bit 24 - LON Transmission Done Interrupt Disable"]
    #[inline(always)]
    pub fn ltxd(&mut self) -> LTXD_W<24> {
        LTXD_W::new(self)
    }
    #[doc = "Bit 25 - LON Collision Interrupt Disable"]
    #[inline(always)]
    pub fn lcol(&mut self) -> LCOL_W<25> {
        LCOL_W::new(self)
    }
    #[doc = "Bit 26 - LON Frame Early Termination Interrupt Disable"]
    #[inline(always)]
    pub fn lfet(&mut self) -> LFET_W<26> {
        LFET_W::new(self)
    }
    #[doc = "Bit 27 - LON Reception Done Interrupt Disable"]
    #[inline(always)]
    pub fn lrxd(&mut self) -> LRXD_W<27> {
        LRXD_W::new(self)
    }
    #[doc = "Bit 28 - LON Backlog Overflow Error Interrupt Disable"]
    #[inline(always)]
    pub fn lblovfe(&mut self) -> LBLOVFE_W<28> {
        LBLOVFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_idr_lon_mode](index.html) module"]
pub struct US_IDR_LON_MODE_SPEC;
impl crate::RegisterSpec for US_IDR_LON_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [us_idr_lon_mode::W](W) writer structure"]
impl crate::Writable for US_IDR_LON_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets US_IDR_LON_MODE to value 0"]
impl crate::Resettable for US_IDR_LON_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
