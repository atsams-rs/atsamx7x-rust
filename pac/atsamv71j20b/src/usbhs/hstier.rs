#[doc = "Register `HSTIER` writer"]
pub struct W(crate::W<HSTIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTIER_SPEC>;
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
impl From<crate::W<HSTIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCONNIES` writer - Device Connection Interrupt Enable"]
pub type DCONNIES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `DDISCIES` writer - Device Disconnection Interrupt Enable"]
pub type DDISCIES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `RSTIES` writer - USB Reset Sent Interrupt Enable"]
pub type RSTIES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `RSMEDIES` writer - Downstream Resume Sent Interrupt Enable"]
pub type RSMEDIES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `RXRSMIES` writer - Upstream Resume Received Interrupt Enable"]
pub type RXRSMIES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `HSOFIES` writer - Host Start of Frame Interrupt Enable"]
pub type HSOFIES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `HWUPIES` writer - Host Wake-Up Interrupt Enable"]
pub type HWUPIES_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `PEP_0` writer - Pipe 0 Interrupt Enable"]
pub type PEP_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `PEP_1` writer - Pipe 1 Interrupt Enable"]
pub type PEP_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `PEP_2` writer - Pipe 2 Interrupt Enable"]
pub type PEP_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `PEP_3` writer - Pipe 3 Interrupt Enable"]
pub type PEP_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `PEP_4` writer - Pipe 4 Interrupt Enable"]
pub type PEP_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `PEP_5` writer - Pipe 5 Interrupt Enable"]
pub type PEP_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `PEP_6` writer - Pipe 6 Interrupt Enable"]
pub type PEP_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `PEP_7` writer - Pipe 7 Interrupt Enable"]
pub type PEP_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `PEP_8` writer - Pipe 8 Interrupt Enable"]
pub type PEP_8_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `PEP_9` writer - Pipe 9 Interrupt Enable"]
pub type PEP_9_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `DMA_1` writer - DMA Channel 0 Interrupt Enable"]
pub type DMA_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `DMA_2` writer - DMA Channel 1 Interrupt Enable"]
pub type DMA_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `DMA_3` writer - DMA Channel 2 Interrupt Enable"]
pub type DMA_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `DMA_4` writer - DMA Channel 3 Interrupt Enable"]
pub type DMA_4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `DMA_5` writer - DMA Channel 4 Interrupt Enable"]
pub type DMA_5_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `DMA_6` writer - DMA Channel 5 Interrupt Enable"]
pub type DMA_6_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
#[doc = "Field `DMA_7` writer - DMA Channel 6 Interrupt Enable"]
pub type DMA_7_W<'a, const O: u8> = crate::BitWriter<'a, u32, HSTIER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Device Connection Interrupt Enable"]
    #[inline(always)]
    pub fn dconnies(&mut self) -> DCONNIES_W<0> {
        DCONNIES_W::new(self)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt Enable"]
    #[inline(always)]
    pub fn ddiscies(&mut self) -> DDISCIES_W<1> {
        DDISCIES_W::new(self)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt Enable"]
    #[inline(always)]
    pub fn rsties(&mut self) -> RSTIES_W<2> {
        RSTIES_W::new(self)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt Enable"]
    #[inline(always)]
    pub fn rsmedies(&mut self) -> RSMEDIES_W<3> {
        RSMEDIES_W::new(self)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt Enable"]
    #[inline(always)]
    pub fn rxrsmies(&mut self) -> RXRSMIES_W<4> {
        RXRSMIES_W::new(self)
    }
    #[doc = "Bit 5 - Host Start of Frame Interrupt Enable"]
    #[inline(always)]
    pub fn hsofies(&mut self) -> HSOFIES_W<5> {
        HSOFIES_W::new(self)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt Enable"]
    #[inline(always)]
    pub fn hwupies(&mut self) -> HWUPIES_W<6> {
        HWUPIES_W::new(self)
    }
    #[doc = "Bit 8 - Pipe 0 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_0(&mut self) -> PEP_0_W<8> {
        PEP_0_W::new(self)
    }
    #[doc = "Bit 9 - Pipe 1 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_1(&mut self) -> PEP_1_W<9> {
        PEP_1_W::new(self)
    }
    #[doc = "Bit 10 - Pipe 2 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_2(&mut self) -> PEP_2_W<10> {
        PEP_2_W::new(self)
    }
    #[doc = "Bit 11 - Pipe 3 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_3(&mut self) -> PEP_3_W<11> {
        PEP_3_W::new(self)
    }
    #[doc = "Bit 12 - Pipe 4 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_4(&mut self) -> PEP_4_W<12> {
        PEP_4_W::new(self)
    }
    #[doc = "Bit 13 - Pipe 5 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_5(&mut self) -> PEP_5_W<13> {
        PEP_5_W::new(self)
    }
    #[doc = "Bit 14 - Pipe 6 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_6(&mut self) -> PEP_6_W<14> {
        PEP_6_W::new(self)
    }
    #[doc = "Bit 15 - Pipe 7 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_7(&mut self) -> PEP_7_W<15> {
        PEP_7_W::new(self)
    }
    #[doc = "Bit 16 - Pipe 8 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_8(&mut self) -> PEP_8_W<16> {
        PEP_8_W::new(self)
    }
    #[doc = "Bit 17 - Pipe 9 Interrupt Enable"]
    #[inline(always)]
    pub fn pep_9(&mut self) -> PEP_9_W<17> {
        PEP_9_W::new(self)
    }
    #[doc = "Bit 25 - DMA Channel 0 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_1(&mut self) -> DMA_1_W<25> {
        DMA_1_W::new(self)
    }
    #[doc = "Bit 26 - DMA Channel 1 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_2(&mut self) -> DMA_2_W<26> {
        DMA_2_W::new(self)
    }
    #[doc = "Bit 27 - DMA Channel 2 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_3(&mut self) -> DMA_3_W<27> {
        DMA_3_W::new(self)
    }
    #[doc = "Bit 28 - DMA Channel 3 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_4(&mut self) -> DMA_4_W<28> {
        DMA_4_W::new(self)
    }
    #[doc = "Bit 29 - DMA Channel 4 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_5(&mut self) -> DMA_5_W<29> {
        DMA_5_W::new(self)
    }
    #[doc = "Bit 30 - DMA Channel 5 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_6(&mut self) -> DMA_6_W<30> {
        DMA_6_W::new(self)
    }
    #[doc = "Bit 31 - DMA Channel 6 Interrupt Enable"]
    #[inline(always)]
    pub fn dma_7(&mut self) -> DMA_7_W<31> {
        DMA_7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Global Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstier](index.html) module"]
pub struct HSTIER_SPEC;
impl crate::RegisterSpec for HSTIER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hstier::W](W) writer structure"]
impl crate::Writable for HSTIER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSTIER to value 0"]
impl crate::Resettable for HSTIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
