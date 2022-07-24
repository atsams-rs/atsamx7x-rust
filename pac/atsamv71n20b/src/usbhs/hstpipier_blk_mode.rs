#[doc = "Register `HSTPIPIER_BLK_MODE[%s]` writer"]
pub struct W(crate::W<HSTPIPIER_BLK_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTPIPIER_BLK_MODE_SPEC>;
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
impl From<crate::W<HSTPIPIER_BLK_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTPIPIER_BLK_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXINES` writer - Received IN Data Interrupt Enable"]
pub type RXINES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIPIER_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `TXOUTES` writer - Transmitted OUT Data Interrupt Enable"]
pub type TXOUTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIPIER_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `TXSTPES` writer - Transmitted SETUP Interrupt Enable"]
pub type TXSTPES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIPIER_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `PERRES` writer - Pipe Error Interrupt Enable"]
pub type PERRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIPIER_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `NAKEDES` writer - NAKed Interrupt Enable"]
pub type NAKEDES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIPIER_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `OVERFIES` writer - Overflow Interrupt Enable"]
pub type OVERFIES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIPIER_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `RXSTALLDES` writer - Received STALLed Interrupt Enable"]
pub type RXSTALLDES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HSTPIPIER_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `SHORTPACKETIES` writer - Short Packet Interrupt Enable"]
pub type SHORTPACKETIES_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HSTPIPIER_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `NBUSYBKES` writer - Number of Busy Banks Enable"]
pub type NBUSYBKES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIPIER_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `PDISHDMAS` writer - Pipe Interrupts Disable HDMA Request Enable"]
pub type PDISHDMAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIPIER_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `PFREEZES` writer - Pipe Freeze Enable"]
pub type PFREEZES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIPIER_BLK_MODE_SPEC, bool, O>;
#[doc = "Field `RSTDTS` writer - Reset Data Toggle Enable"]
pub type RSTDTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTPIPIER_BLK_MODE_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Received IN Data Interrupt Enable"]
    #[inline(always)]
    pub fn rxines(&mut self) -> RXINES_W<0> {
        RXINES_W::new(self)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt Enable"]
    #[inline(always)]
    pub fn txoutes(&mut self) -> TXOUTES_W<1> {
        TXOUTES_W::new(self)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt Enable"]
    #[inline(always)]
    pub fn txstpes(&mut self) -> TXSTPES_W<2> {
        TXSTPES_W::new(self)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt Enable"]
    #[inline(always)]
    pub fn perres(&mut self) -> PERRES_W<3> {
        PERRES_W::new(self)
    }
    #[doc = "Bit 4 - NAKed Interrupt Enable"]
    #[inline(always)]
    pub fn nakedes(&mut self) -> NAKEDES_W<4> {
        NAKEDES_W::new(self)
    }
    #[doc = "Bit 5 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn overfies(&mut self) -> OVERFIES_W<5> {
        OVERFIES_W::new(self)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt Enable"]
    #[inline(always)]
    pub fn rxstalldes(&mut self) -> RXSTALLDES_W<6> {
        RXSTALLDES_W::new(self)
    }
    #[doc = "Bit 7 - Short Packet Interrupt Enable"]
    #[inline(always)]
    pub fn shortpacketies(&mut self) -> SHORTPACKETIES_W<7> {
        SHORTPACKETIES_W::new(self)
    }
    #[doc = "Bit 12 - Number of Busy Banks Enable"]
    #[inline(always)]
    pub fn nbusybkes(&mut self) -> NBUSYBKES_W<12> {
        NBUSYBKES_W::new(self)
    }
    #[doc = "Bit 16 - Pipe Interrupts Disable HDMA Request Enable"]
    #[inline(always)]
    pub fn pdishdmas(&mut self) -> PDISHDMAS_W<16> {
        PDISHDMAS_W::new(self)
    }
    #[doc = "Bit 17 - Pipe Freeze Enable"]
    #[inline(always)]
    pub fn pfreezes(&mut self) -> PFREEZES_W<17> {
        PFREEZES_W::new(self)
    }
    #[doc = "Bit 18 - Reset Data Toggle Enable"]
    #[inline(always)]
    pub fn rstdts(&mut self) -> RSTDTS_W<18> {
        RSTDTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Pipe Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipier_blk_mode](index.html) module"]
pub struct HSTPIPIER_BLK_MODE_SPEC;
impl crate::RegisterSpec for HSTPIPIER_BLK_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hstpipier_blk_mode::W](W) writer structure"]
impl crate::Writable for HSTPIPIER_BLK_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSTPIPIER_BLK_MODE[%s]
to value 0"]
impl crate::Resettable for HSTPIPIER_BLK_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
