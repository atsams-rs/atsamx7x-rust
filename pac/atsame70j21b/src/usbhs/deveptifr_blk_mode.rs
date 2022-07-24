#[doc = "Register `DEVEPTIFR_BLK_MODE[%s]` writer"]
pub struct W(crate::W<DEVEPTIFR_BLK_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVEPTIFR_BLK_MODE_SPEC>;
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
impl From<crate::W<DEVEPTIFR_BLK_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVEPTIFR_BLK_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXINIS` writer - Transmitted IN Data Interrupt Set"]
pub type TXINIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIFR_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `RXOUTIS` writer - Received OUT Data Interrupt Set"]
pub type RXOUTIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIFR_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `RXSTPIS` writer - Received SETUP Interrupt Set"]
pub type RXSTPIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIFR_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `NAKOUTIS` writer - NAKed OUT Interrupt Set"]
pub type NAKOUTIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIFR_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `NAKINIS` writer - NAKed IN Interrupt Set"]
pub type NAKINIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIFR_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `OVERFIS` writer - Overflow Interrupt Set"]
pub type OVERFIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIFR_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `STALLEDIS` writer - STALLed Interrupt Set"]
pub type STALLEDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIFR_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `SHORTPACKETS` writer - Short Packet Interrupt Set"]
pub type SHORTPACKETS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DEVEPTIFR_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `NBUSYBKS` writer - Number of Busy Banks Interrupt Set"]
pub type NBUSYBKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVEPTIFR_BLK_MODE_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt Set"]
    #[inline(always)]
    pub fn txinis(&mut self) -> TXINIS_W<0> {
        TXINIS_W::new(self)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt Set"]
    #[inline(always)]
    pub fn rxoutis(&mut self) -> RXOUTIS_W<1> {
        RXOUTIS_W::new(self)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt Set"]
    #[inline(always)]
    pub fn rxstpis(&mut self) -> RXSTPIS_W<2> {
        RXSTPIS_W::new(self)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt Set"]
    #[inline(always)]
    pub fn nakoutis(&mut self) -> NAKOUTIS_W<3> {
        NAKOUTIS_W::new(self)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt Set"]
    #[inline(always)]
    pub fn nakinis(&mut self) -> NAKINIS_W<4> {
        NAKINIS_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Set"]
    #[inline(always)]
    pub fn overfis(&mut self) -> OVERFIS_W<5> {
        OVERFIS_W::new(self)
    }
    #[doc = "Bit 6 - STALLed Interrupt Set"]
    #[inline(always)]
    pub fn stalledis(&mut self) -> STALLEDIS_W<6> {
        STALLEDIS_W::new(self)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Set"]
    #[inline(always)]
    pub fn shortpackets(&mut self) -> SHORTPACKETS_W<7> {
        SHORTPACKETS_W::new(self)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Set"]
    #[inline(always)]
    pub fn nbusybks(&mut self) -> NBUSYBKS_W<12> {
        NBUSYBKS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Endpoint Interrupt Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deveptifr_blk_mode](index.html) module"]
pub struct DEVEPTIFR_BLK_MODE_SPEC;
impl crate::RegisterSpec for DEVEPTIFR_BLK_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [deveptifr_blk_mode::W](W) writer structure"]
impl crate::Writable for DEVEPTIFR_BLK_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVEPTIFR_BLK_MODE[%s]
to value 0"]
impl crate::Resettable for DEVEPTIFR_BLK_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
