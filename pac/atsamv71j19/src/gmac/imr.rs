#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR` writer"]
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MFS` reader - Management Frame Sent"]
pub type MFS_R = crate::BitReader<bool>;
#[doc = "Field `MFS` writer - Management Frame Sent"]
pub type MFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `RCOMP` reader - Receive Complete"]
pub type RCOMP_R = crate::BitReader<bool>;
#[doc = "Field `RCOMP` writer - Receive Complete"]
pub type RCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `RXUBR` reader - RX Used Bit Read"]
pub type RXUBR_R = crate::BitReader<bool>;
#[doc = "Field `RXUBR` writer - RX Used Bit Read"]
pub type RXUBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `TXUBR` reader - TX Used Bit Read"]
pub type TXUBR_R = crate::BitReader<bool>;
#[doc = "Field `TXUBR` writer - TX Used Bit Read"]
pub type TXUBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `TUR` reader - Transmit Underrun"]
pub type TUR_R = crate::BitReader<bool>;
#[doc = "Field `TUR` writer - Transmit Underrun"]
pub type TUR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `RLEX` reader - Retry Limit Exceeded"]
pub type RLEX_R = crate::BitReader<bool>;
#[doc = "Field `RLEX` writer - Retry Limit Exceeded"]
pub type RLEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `TFC` reader - Transmit Frame Corruption Due to AHB Error"]
pub type TFC_R = crate::BitReader<bool>;
#[doc = "Field `TFC` writer - Transmit Frame Corruption Due to AHB Error"]
pub type TFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `TCOMP` reader - Transmit Complete"]
pub type TCOMP_R = crate::BitReader<bool>;
#[doc = "Field `TCOMP` writer - Transmit Complete"]
pub type TCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `ROVR` reader - Receive Overrun"]
pub type ROVR_R = crate::BitReader<bool>;
#[doc = "Field `ROVR` writer - Receive Overrun"]
pub type ROVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `HRESP` reader - HRESP Not OK"]
pub type HRESP_R = crate::BitReader<bool>;
#[doc = "Field `HRESP` writer - HRESP Not OK"]
pub type HRESP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `PFNZ` reader - Pause Frame with Non-zero Pause Quantum Received"]
pub type PFNZ_R = crate::BitReader<bool>;
#[doc = "Field `PFNZ` writer - Pause Frame with Non-zero Pause Quantum Received"]
pub type PFNZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `PTZ` reader - Pause Time Zero"]
pub type PTZ_R = crate::BitReader<bool>;
#[doc = "Field `PTZ` writer - Pause Time Zero"]
pub type PTZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `PFTR` reader - Pause Frame Transmitted"]
pub type PFTR_R = crate::BitReader<bool>;
#[doc = "Field `PFTR` writer - Pause Frame Transmitted"]
pub type PFTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `EXINT` reader - External Interrupt"]
pub type EXINT_R = crate::BitReader<bool>;
#[doc = "Field `EXINT` writer - External Interrupt"]
pub type EXINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `DRQFR` reader - PTP Delay Request Frame Received"]
pub type DRQFR_R = crate::BitReader<bool>;
#[doc = "Field `DRQFR` writer - PTP Delay Request Frame Received"]
pub type DRQFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `SFR` reader - PTP Sync Frame Received"]
pub type SFR_R = crate::BitReader<bool>;
#[doc = "Field `SFR` writer - PTP Sync Frame Received"]
pub type SFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `DRQFT` reader - PTP Delay Request Frame Transmitted"]
pub type DRQFT_R = crate::BitReader<bool>;
#[doc = "Field `DRQFT` writer - PTP Delay Request Frame Transmitted"]
pub type DRQFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `SFT` reader - PTP Sync Frame Transmitted"]
pub type SFT_R = crate::BitReader<bool>;
#[doc = "Field `SFT` writer - PTP Sync Frame Transmitted"]
pub type SFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `PDRQFR` reader - PDelay Request Frame Received"]
pub type PDRQFR_R = crate::BitReader<bool>;
#[doc = "Field `PDRQFR` writer - PDelay Request Frame Received"]
pub type PDRQFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `PDRSFR` reader - PDelay Response Frame Received"]
pub type PDRSFR_R = crate::BitReader<bool>;
#[doc = "Field `PDRSFR` writer - PDelay Response Frame Received"]
pub type PDRSFR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `PDRQFT` reader - PDelay Request Frame Transmitted"]
pub type PDRQFT_R = crate::BitReader<bool>;
#[doc = "Field `PDRQFT` writer - PDelay Request Frame Transmitted"]
pub type PDRQFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `PDRSFT` reader - PDelay Response Frame Transmitted"]
pub type PDRSFT_R = crate::BitReader<bool>;
#[doc = "Field `PDRSFT` writer - PDelay Response Frame Transmitted"]
pub type PDRSFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `SRI` reader - TSU Seconds Register Increment"]
pub type SRI_R = crate::BitReader<bool>;
#[doc = "Field `SRI` writer - TSU Seconds Register Increment"]
pub type SRI_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `RXLPISBC` reader - Enable RX LPI Indication"]
pub type RXLPISBC_R = crate::BitReader<bool>;
#[doc = "Field `RXLPISBC` writer - Enable RX LPI Indication"]
pub type RXLPISBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `WOL` reader - Wake On LAN"]
pub type WOL_R = crate::BitReader<bool>;
#[doc = "Field `WOL` writer - Wake On LAN"]
pub type WOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
#[doc = "Field `TSUTIMCOMP` reader - TSU Timer Comparison"]
pub type TSUTIMCOMP_R = crate::BitReader<bool>;
#[doc = "Field `TSUTIMCOMP` writer - TSU Timer Comparison"]
pub type TSUTIMCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Management Frame Sent"]
    #[inline(always)]
    pub fn mfs(&self) -> MFS_R {
        MFS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&self) -> RCOMP_R {
        RCOMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&self) -> RXUBR_R {
        RXUBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX Used Bit Read"]
    #[inline(always)]
    pub fn txubr(&self) -> TXUBR_R {
        TXUBR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Underrun"]
    #[inline(always)]
    pub fn tur(&self) -> TUR_R {
        TUR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded"]
    #[inline(always)]
    pub fn rlex(&self) -> RLEX_R {
        RLEX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    pub fn tfc(&self) -> TFC_R {
        TFC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&self) -> TCOMP_R {
        TCOMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&self) -> ROVR_R {
        ROVR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    pub fn hresp(&self) -> HRESP_R {
        HRESP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pause Frame with Non-zero Pause Quantum Received"]
    #[inline(always)]
    pub fn pfnz(&self) -> PFNZ_R {
        PFNZ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pause Time Zero"]
    #[inline(always)]
    pub fn ptz(&self) -> PTZ_R {
        PTZ_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pause Frame Transmitted"]
    #[inline(always)]
    pub fn pftr(&self) -> PFTR_R {
        PFTR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External Interrupt"]
    #[inline(always)]
    pub fn exint(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - PTP Delay Request Frame Received"]
    #[inline(always)]
    pub fn drqfr(&self) -> DRQFR_R {
        DRQFR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PTP Sync Frame Received"]
    #[inline(always)]
    pub fn sfr(&self) -> SFR_R {
        SFR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PTP Delay Request Frame Transmitted"]
    #[inline(always)]
    pub fn drqft(&self) -> DRQFT_R {
        DRQFT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PTP Sync Frame Transmitted"]
    #[inline(always)]
    pub fn sft(&self) -> SFT_R {
        SFT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PDelay Request Frame Received"]
    #[inline(always)]
    pub fn pdrqfr(&self) -> PDRQFR_R {
        PDRQFR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PDelay Response Frame Received"]
    #[inline(always)]
    pub fn pdrsfr(&self) -> PDRSFR_R {
        PDRSFR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PDelay Request Frame Transmitted"]
    #[inline(always)]
    pub fn pdrqft(&self) -> PDRQFT_R {
        PDRQFT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PDelay Response Frame Transmitted"]
    #[inline(always)]
    pub fn pdrsft(&self) -> PDRSFT_R {
        PDRSFT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TSU Seconds Register Increment"]
    #[inline(always)]
    pub fn sri(&self) -> SRI_R {
        SRI_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable RX LPI Indication"]
    #[inline(always)]
    pub fn rxlpisbc(&self) -> RXLPISBC_R {
        RXLPISBC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wake On LAN"]
    #[inline(always)]
    pub fn wol(&self) -> WOL_R {
        WOL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TSU Timer Comparison"]
    #[inline(always)]
    pub fn tsutimcomp(&self) -> TSUTIMCOMP_R {
        TSUTIMCOMP_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Management Frame Sent"]
    #[inline(always)]
    pub fn mfs(&mut self) -> MFS_W<0> {
        MFS_W::new(self)
    }
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&mut self) -> RCOMP_W<1> {
        RCOMP_W::new(self)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&mut self) -> RXUBR_W<2> {
        RXUBR_W::new(self)
    }
    #[doc = "Bit 3 - TX Used Bit Read"]
    #[inline(always)]
    pub fn txubr(&mut self) -> TXUBR_W<3> {
        TXUBR_W::new(self)
    }
    #[doc = "Bit 4 - Transmit Underrun"]
    #[inline(always)]
    pub fn tur(&mut self) -> TUR_W<4> {
        TUR_W::new(self)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded"]
    #[inline(always)]
    pub fn rlex(&mut self) -> RLEX_W<5> {
        RLEX_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    pub fn tfc(&mut self) -> TFC_W<6> {
        TFC_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&mut self) -> TCOMP_W<7> {
        TCOMP_W::new(self)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&mut self) -> ROVR_W<10> {
        ROVR_W::new(self)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    pub fn hresp(&mut self) -> HRESP_W<11> {
        HRESP_W::new(self)
    }
    #[doc = "Bit 12 - Pause Frame with Non-zero Pause Quantum Received"]
    #[inline(always)]
    pub fn pfnz(&mut self) -> PFNZ_W<12> {
        PFNZ_W::new(self)
    }
    #[doc = "Bit 13 - Pause Time Zero"]
    #[inline(always)]
    pub fn ptz(&mut self) -> PTZ_W<13> {
        PTZ_W::new(self)
    }
    #[doc = "Bit 14 - Pause Frame Transmitted"]
    #[inline(always)]
    pub fn pftr(&mut self) -> PFTR_W<14> {
        PFTR_W::new(self)
    }
    #[doc = "Bit 15 - External Interrupt"]
    #[inline(always)]
    pub fn exint(&mut self) -> EXINT_W<15> {
        EXINT_W::new(self)
    }
    #[doc = "Bit 18 - PTP Delay Request Frame Received"]
    #[inline(always)]
    pub fn drqfr(&mut self) -> DRQFR_W<18> {
        DRQFR_W::new(self)
    }
    #[doc = "Bit 19 - PTP Sync Frame Received"]
    #[inline(always)]
    pub fn sfr(&mut self) -> SFR_W<19> {
        SFR_W::new(self)
    }
    #[doc = "Bit 20 - PTP Delay Request Frame Transmitted"]
    #[inline(always)]
    pub fn drqft(&mut self) -> DRQFT_W<20> {
        DRQFT_W::new(self)
    }
    #[doc = "Bit 21 - PTP Sync Frame Transmitted"]
    #[inline(always)]
    pub fn sft(&mut self) -> SFT_W<21> {
        SFT_W::new(self)
    }
    #[doc = "Bit 22 - PDelay Request Frame Received"]
    #[inline(always)]
    pub fn pdrqfr(&mut self) -> PDRQFR_W<22> {
        PDRQFR_W::new(self)
    }
    #[doc = "Bit 23 - PDelay Response Frame Received"]
    #[inline(always)]
    pub fn pdrsfr(&mut self) -> PDRSFR_W<23> {
        PDRSFR_W::new(self)
    }
    #[doc = "Bit 24 - PDelay Request Frame Transmitted"]
    #[inline(always)]
    pub fn pdrqft(&mut self) -> PDRQFT_W<24> {
        PDRQFT_W::new(self)
    }
    #[doc = "Bit 25 - PDelay Response Frame Transmitted"]
    #[inline(always)]
    pub fn pdrsft(&mut self) -> PDRSFT_W<25> {
        PDRSFT_W::new(self)
    }
    #[doc = "Bit 26 - TSU Seconds Register Increment"]
    #[inline(always)]
    pub fn sri(&mut self) -> SRI_W<26> {
        SRI_W::new(self)
    }
    #[doc = "Bit 27 - Enable RX LPI Indication"]
    #[inline(always)]
    pub fn rxlpisbc(&mut self) -> RXLPISBC_W<27> {
        RXLPISBC_W::new(self)
    }
    #[doc = "Bit 28 - Wake On LAN"]
    #[inline(always)]
    pub fn wol(&mut self) -> WOL_W<28> {
        WOL_W::new(self)
    }
    #[doc = "Bit 29 - TSU Timer Comparison"]
    #[inline(always)]
    pub fn tsutimcomp(&mut self) -> TSUTIMCOMP_W<29> {
        TSUTIMCOMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr::W](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
