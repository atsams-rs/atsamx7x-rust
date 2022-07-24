#[doc = "Register `IR` reader"]
pub struct R(crate::R<IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IR` writer"]
pub struct W(crate::W<IR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IR_SPEC>;
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
impl From<crate::W<IR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RF0N` reader - Receive FIFO 0 New Message"]
pub type RF0N_R = crate::BitReader<bool>;
#[doc = "Field `RF0N` writer - Receive FIFO 0 New Message"]
pub type RF0N_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF0W` reader - Receive FIFO 0 Watermark Reached"]
pub type RF0W_R = crate::BitReader<bool>;
#[doc = "Field `RF0W` writer - Receive FIFO 0 Watermark Reached"]
pub type RF0W_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF0F` reader - Receive FIFO 0 Full"]
pub type RF0F_R = crate::BitReader<bool>;
#[doc = "Field `RF0F` writer - Receive FIFO 0 Full"]
pub type RF0F_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF0L` reader - Receive FIFO 0 Message Lost"]
pub type RF0L_R = crate::BitReader<bool>;
#[doc = "Field `RF0L` writer - Receive FIFO 0 Message Lost"]
pub type RF0L_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF1N` reader - Receive FIFO 1 New Message"]
pub type RF1N_R = crate::BitReader<bool>;
#[doc = "Field `RF1N` writer - Receive FIFO 1 New Message"]
pub type RF1N_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF1W` reader - Receive FIFO 1 Watermark Reached"]
pub type RF1W_R = crate::BitReader<bool>;
#[doc = "Field `RF1W` writer - Receive FIFO 1 Watermark Reached"]
pub type RF1W_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF1F` reader - Receive FIFO 1 Full"]
pub type RF1F_R = crate::BitReader<bool>;
#[doc = "Field `RF1F` writer - Receive FIFO 1 Full"]
pub type RF1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF1L` reader - Receive FIFO 1 Message Lost"]
pub type RF1L_R = crate::BitReader<bool>;
#[doc = "Field `RF1L` writer - Receive FIFO 1 Message Lost"]
pub type RF1L_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `HPM` reader - High Priority Message"]
pub type HPM_R = crate::BitReader<bool>;
#[doc = "Field `HPM` writer - High Priority Message"]
pub type HPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TC` reader - Transmission Completed"]
pub type TC_R = crate::BitReader<bool>;
#[doc = "Field `TC` writer - Transmission Completed"]
pub type TC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TCF` reader - Transmission Cancellation Finished"]
pub type TCF_R = crate::BitReader<bool>;
#[doc = "Field `TCF` writer - Transmission Cancellation Finished"]
pub type TCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TFE` reader - Tx FIFO Empty"]
pub type TFE_R = crate::BitReader<bool>;
#[doc = "Field `TFE` writer - Tx FIFO Empty"]
pub type TFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TEFN` reader - Tx Event FIFO New Entry"]
pub type TEFN_R = crate::BitReader<bool>;
#[doc = "Field `TEFN` writer - Tx Event FIFO New Entry"]
pub type TEFN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TEFW` reader - Tx Event FIFO Watermark Reached"]
pub type TEFW_R = crate::BitReader<bool>;
#[doc = "Field `TEFW` writer - Tx Event FIFO Watermark Reached"]
pub type TEFW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TEFF` reader - Tx Event FIFO Full"]
pub type TEFF_R = crate::BitReader<bool>;
#[doc = "Field `TEFF` writer - Tx Event FIFO Full"]
pub type TEFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TEFL` reader - Tx Event FIFO Element Lost"]
pub type TEFL_R = crate::BitReader<bool>;
#[doc = "Field `TEFL` writer - Tx Event FIFO Element Lost"]
pub type TEFL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TSW` reader - Timestamp Wraparound"]
pub type TSW_R = crate::BitReader<bool>;
#[doc = "Field `TSW` writer - Timestamp Wraparound"]
pub type TSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `MRAF` reader - Message RAM Access Failure"]
pub type MRAF_R = crate::BitReader<bool>;
#[doc = "Field `MRAF` writer - Message RAM Access Failure"]
pub type MRAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TOO` reader - Timeout Occurred"]
pub type TOO_R = crate::BitReader<bool>;
#[doc = "Field `TOO` writer - Timeout Occurred"]
pub type TOO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `DRX` reader - Message stored to Dedicated Receive Buffer"]
pub type DRX_R = crate::BitReader<bool>;
#[doc = "Field `DRX` writer - Message stored to Dedicated Receive Buffer"]
pub type DRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `ELO` reader - Error Logging Overflow"]
pub type ELO_R = crate::BitReader<bool>;
#[doc = "Field `ELO` writer - Error Logging Overflow"]
pub type ELO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `EP` reader - Error Passive"]
pub type EP_R = crate::BitReader<bool>;
#[doc = "Field `EP` writer - Error Passive"]
pub type EP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `EW` reader - Warning Status"]
pub type EW_R = crate::BitReader<bool>;
#[doc = "Field `EW` writer - Warning Status"]
pub type EW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `BO` reader - Bus_Off Status"]
pub type BO_R = crate::BitReader<bool>;
#[doc = "Field `BO` writer - Bus_Off Status"]
pub type BO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `WDI` reader - Watchdog Interrupt"]
pub type WDI_R = crate::BitReader<bool>;
#[doc = "Field `WDI` writer - Watchdog Interrupt"]
pub type WDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `CRCE` reader - CRC Error"]
pub type CRCE_R = crate::BitReader<bool>;
#[doc = "Field `CRCE` writer - CRC Error"]
pub type CRCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `BE` reader - Bit Error"]
pub type BE_R = crate::BitReader<bool>;
#[doc = "Field `BE` writer - Bit Error"]
pub type BE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `ACKE` reader - Acknowledge Error"]
pub type ACKE_R = crate::BitReader<bool>;
#[doc = "Field `ACKE` writer - Acknowledge Error"]
pub type ACKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `FOE` reader - Format Error"]
pub type FOE_R = crate::BitReader<bool>;
#[doc = "Field `FOE` writer - Format Error"]
pub type FOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `STE` reader - Stuff Error"]
pub type STE_R = crate::BitReader<bool>;
#[doc = "Field `STE` writer - Stuff Error"]
pub type STE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Receive FIFO 0 New Message"]
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO 0 Watermark Reached"]
    #[inline(always)]
    pub fn rf0w(&self) -> RF0W_R {
        RF0W_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO 0 Full"]
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO 1 New Message"]
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO 1 Watermark Reached"]
    #[inline(always)]
    pub fn rf1w(&self) -> RF1W_R {
        RF1W_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO 1 Full"]
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Priority Message"]
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Completed"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry"]
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached"]
    #[inline(always)]
    pub fn tefw(&self) -> TEFW_R {
        TEFW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full"]
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost"]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound"]
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure"]
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred"]
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Receive Buffer"]
    #[inline(always)]
    pub fn drx(&self) -> DRX_R {
        DRX_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow"]
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error Passive"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning Status"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt"]
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CRC Error"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Bit Error"]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Acknowledge Error"]
    #[inline(always)]
    pub fn acke(&self) -> ACKE_R {
        ACKE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Format Error"]
    #[inline(always)]
    pub fn foe(&self) -> FOE_R {
        FOE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Stuff Error"]
    #[inline(always)]
    pub fn ste(&self) -> STE_R {
        STE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO 0 New Message"]
    #[inline(always)]
    pub fn rf0n(&mut self) -> RF0N_W<0> {
        RF0N_W::new(self)
    }
    #[doc = "Bit 1 - Receive FIFO 0 Watermark Reached"]
    #[inline(always)]
    pub fn rf0w(&mut self) -> RF0W_W<1> {
        RF0W_W::new(self)
    }
    #[doc = "Bit 2 - Receive FIFO 0 Full"]
    #[inline(always)]
    pub fn rf0f(&mut self) -> RF0F_W<2> {
        RF0F_W::new(self)
    }
    #[doc = "Bit 3 - Receive FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&mut self) -> RF0L_W<3> {
        RF0L_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO 1 New Message"]
    #[inline(always)]
    pub fn rf1n(&mut self) -> RF1N_W<4> {
        RF1N_W::new(self)
    }
    #[doc = "Bit 5 - Receive FIFO 1 Watermark Reached"]
    #[inline(always)]
    pub fn rf1w(&mut self) -> RF1W_W<5> {
        RF1W_W::new(self)
    }
    #[doc = "Bit 6 - Receive FIFO 1 Full"]
    #[inline(always)]
    pub fn rf1f(&mut self) -> RF1F_W<6> {
        RF1F_W::new(self)
    }
    #[doc = "Bit 7 - Receive FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&mut self) -> RF1L_W<7> {
        RF1L_W::new(self)
    }
    #[doc = "Bit 8 - High Priority Message"]
    #[inline(always)]
    pub fn hpm(&mut self) -> HPM_W<8> {
        HPM_W::new(self)
    }
    #[doc = "Bit 9 - Transmission Completed"]
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W<9> {
        TC_W::new(self)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished"]
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W<10> {
        TCF_W::new(self)
    }
    #[doc = "Bit 11 - Tx FIFO Empty"]
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W<11> {
        TFE_W::new(self)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry"]
    #[inline(always)]
    pub fn tefn(&mut self) -> TEFN_W<12> {
        TEFN_W::new(self)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached"]
    #[inline(always)]
    pub fn tefw(&mut self) -> TEFW_W<13> {
        TEFW_W::new(self)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full"]
    #[inline(always)]
    pub fn teff(&mut self) -> TEFF_W<14> {
        TEFF_W::new(self)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost"]
    #[inline(always)]
    pub fn tefl(&mut self) -> TEFL_W<15> {
        TEFL_W::new(self)
    }
    #[doc = "Bit 16 - Timestamp Wraparound"]
    #[inline(always)]
    pub fn tsw(&mut self) -> TSW_W<16> {
        TSW_W::new(self)
    }
    #[doc = "Bit 17 - Message RAM Access Failure"]
    #[inline(always)]
    pub fn mraf(&mut self) -> MRAF_W<17> {
        MRAF_W::new(self)
    }
    #[doc = "Bit 18 - Timeout Occurred"]
    #[inline(always)]
    pub fn too(&mut self) -> TOO_W<18> {
        TOO_W::new(self)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Receive Buffer"]
    #[inline(always)]
    pub fn drx(&mut self) -> DRX_W<19> {
        DRX_W::new(self)
    }
    #[doc = "Bit 22 - Error Logging Overflow"]
    #[inline(always)]
    pub fn elo(&mut self) -> ELO_W<22> {
        ELO_W::new(self)
    }
    #[doc = "Bit 23 - Error Passive"]
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W<23> {
        EP_W::new(self)
    }
    #[doc = "Bit 24 - Warning Status"]
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W<24> {
        EW_W::new(self)
    }
    #[doc = "Bit 25 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&mut self) -> BO_W<25> {
        BO_W::new(self)
    }
    #[doc = "Bit 26 - Watchdog Interrupt"]
    #[inline(always)]
    pub fn wdi(&mut self) -> WDI_W<26> {
        WDI_W::new(self)
    }
    #[doc = "Bit 27 - CRC Error"]
    #[inline(always)]
    pub fn crce(&mut self) -> CRCE_W<27> {
        CRCE_W::new(self)
    }
    #[doc = "Bit 28 - Bit Error"]
    #[inline(always)]
    pub fn be(&mut self) -> BE_W<28> {
        BE_W::new(self)
    }
    #[doc = "Bit 29 - Acknowledge Error"]
    #[inline(always)]
    pub fn acke(&mut self) -> ACKE_W<29> {
        ACKE_W::new(self)
    }
    #[doc = "Bit 30 - Format Error"]
    #[inline(always)]
    pub fn foe(&mut self) -> FOE_W<30> {
        FOE_W::new(self)
    }
    #[doc = "Bit 31 - Stuff Error"]
    #[inline(always)]
    pub fn ste(&mut self) -> STE_W<31> {
        STE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](index.html) module"]
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ir::R](R) reader structure"]
impl crate::Readable for IR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ir::W](W) writer structure"]
impl crate::Writable for IR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
