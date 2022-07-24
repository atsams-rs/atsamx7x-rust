#[doc = "Register `FSMR` reader"]
pub struct R(crate::R<FSMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSMR` writer"]
pub struct W(crate::W<FSMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSMR_SPEC>;
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
impl From<crate::W<FSMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSTT0` reader - Fast Startup Input Enable 0"]
pub type FSTT0_R = crate::BitReader<bool>;
#[doc = "Field `FSTT0` writer - Fast Startup Input Enable 0"]
pub type FSTT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `FSTT1` reader - Fast Startup Input Enable 1"]
pub type FSTT1_R = crate::BitReader<bool>;
#[doc = "Field `FSTT1` writer - Fast Startup Input Enable 1"]
pub type FSTT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `FSTT2` reader - Fast Startup Input Enable 2"]
pub type FSTT2_R = crate::BitReader<bool>;
#[doc = "Field `FSTT2` writer - Fast Startup Input Enable 2"]
pub type FSTT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `FSTT3` reader - Fast Startup Input Enable 3"]
pub type FSTT3_R = crate::BitReader<bool>;
#[doc = "Field `FSTT3` writer - Fast Startup Input Enable 3"]
pub type FSTT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `FSTT4` reader - Fast Startup Input Enable 4"]
pub type FSTT4_R = crate::BitReader<bool>;
#[doc = "Field `FSTT4` writer - Fast Startup Input Enable 4"]
pub type FSTT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `FSTT5` reader - Fast Startup Input Enable 5"]
pub type FSTT5_R = crate::BitReader<bool>;
#[doc = "Field `FSTT5` writer - Fast Startup Input Enable 5"]
pub type FSTT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `FSTT6` reader - Fast Startup Input Enable 6"]
pub type FSTT6_R = crate::BitReader<bool>;
#[doc = "Field `FSTT6` writer - Fast Startup Input Enable 6"]
pub type FSTT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `FSTT7` reader - Fast Startup Input Enable 7"]
pub type FSTT7_R = crate::BitReader<bool>;
#[doc = "Field `FSTT7` writer - Fast Startup Input Enable 7"]
pub type FSTT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `FSTT8` reader - Fast Startup Input Enable 8"]
pub type FSTT8_R = crate::BitReader<bool>;
#[doc = "Field `FSTT8` writer - Fast Startup Input Enable 8"]
pub type FSTT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `FSTT9` reader - Fast Startup Input Enable 9"]
pub type FSTT9_R = crate::BitReader<bool>;
#[doc = "Field `FSTT9` writer - Fast Startup Input Enable 9"]
pub type FSTT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `FSTT10` reader - Fast Startup Input Enable 10"]
pub type FSTT10_R = crate::BitReader<bool>;
#[doc = "Field `FSTT10` writer - Fast Startup Input Enable 10"]
pub type FSTT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `FSTT11` reader - Fast Startup Input Enable 11"]
pub type FSTT11_R = crate::BitReader<bool>;
#[doc = "Field `FSTT11` writer - Fast Startup Input Enable 11"]
pub type FSTT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `FSTT12` reader - Fast Startup Input Enable 12"]
pub type FSTT12_R = crate::BitReader<bool>;
#[doc = "Field `FSTT12` writer - Fast Startup Input Enable 12"]
pub type FSTT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `FSTT13` reader - Fast Startup Input Enable 13"]
pub type FSTT13_R = crate::BitReader<bool>;
#[doc = "Field `FSTT13` writer - Fast Startup Input Enable 13"]
pub type FSTT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `FSTT14` reader - Fast Startup Input Enable 14"]
pub type FSTT14_R = crate::BitReader<bool>;
#[doc = "Field `FSTT14` writer - Fast Startup Input Enable 14"]
pub type FSTT14_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `FSTT15` reader - Fast Startup Input Enable 15"]
pub type FSTT15_R = crate::BitReader<bool>;
#[doc = "Field `FSTT15` writer - Fast Startup Input Enable 15"]
pub type FSTT15_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `RTTAL` reader - RTT Alarm Enable"]
pub type RTTAL_R = crate::BitReader<bool>;
#[doc = "Field `RTTAL` writer - RTT Alarm Enable"]
pub type RTTAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `RTCAL` reader - RTC Alarm Enable"]
pub type RTCAL_R = crate::BitReader<bool>;
#[doc = "Field `RTCAL` writer - RTC Alarm Enable"]
pub type RTCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `USBAL` reader - USB Alarm Enable"]
pub type USBAL_R = crate::BitReader<bool>;
#[doc = "Field `USBAL` writer - USB Alarm Enable"]
pub type USBAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Field `LPM` reader - Low-power Mode"]
pub type LPM_R = crate::BitReader<bool>;
#[doc = "Field `LPM` writer - Low-power Mode"]
pub type LPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
#[doc = "Flash Low-power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLPM_A {
    #[doc = "0: Flash is in Standby Mode when system enters Wait Mode"]
    FLASH_STANDBY = 0,
    #[doc = "1: Flash is in Deep-power-down mode when system enters Wait Mode"]
    FLASH_DEEP_POWERDOWN = 1,
    #[doc = "2: Idle mode"]
    FLASH_IDLE = 2,
}
impl From<FLPM_A> for u8 {
    #[inline(always)]
    fn from(variant: FLPM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLPM` reader - Flash Low-power Mode"]
pub type FLPM_R = crate::FieldReader<u8, FLPM_A>;
impl FLPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLPM_A> {
        match self.bits {
            0 => Some(FLPM_A::FLASH_STANDBY),
            1 => Some(FLPM_A::FLASH_DEEP_POWERDOWN),
            2 => Some(FLPM_A::FLASH_IDLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_STANDBY`"]
    #[inline(always)]
    pub fn is_flash_standby(&self) -> bool {
        *self == FLPM_A::FLASH_STANDBY
    }
    #[doc = "Checks if the value of the field is `FLASH_DEEP_POWERDOWN`"]
    #[inline(always)]
    pub fn is_flash_deep_powerdown(&self) -> bool {
        *self == FLPM_A::FLASH_DEEP_POWERDOWN
    }
    #[doc = "Checks if the value of the field is `FLASH_IDLE`"]
    #[inline(always)]
    pub fn is_flash_idle(&self) -> bool {
        *self == FLPM_A::FLASH_IDLE
    }
}
#[doc = "Field `FLPM` writer - Flash Low-power Mode"]
pub type FLPM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSMR_SPEC, u8, FLPM_A, 2, O>;
impl<'a, const O: u8> FLPM_W<'a, O> {
    #[doc = "Flash is in Standby Mode when system enters Wait Mode"]
    #[inline(always)]
    pub fn flash_standby(self) -> &'a mut W {
        self.variant(FLPM_A::FLASH_STANDBY)
    }
    #[doc = "Flash is in Deep-power-down mode when system enters Wait Mode"]
    #[inline(always)]
    pub fn flash_deep_powerdown(self) -> &'a mut W {
        self.variant(FLPM_A::FLASH_DEEP_POWERDOWN)
    }
    #[doc = "Idle mode"]
    #[inline(always)]
    pub fn flash_idle(self) -> &'a mut W {
        self.variant(FLPM_A::FLASH_IDLE)
    }
}
#[doc = "Field `FFLPM` reader - Force Flash Low-power Mode"]
pub type FFLPM_R = crate::BitReader<bool>;
#[doc = "Field `FFLPM` writer - Force Flash Low-power Mode"]
pub type FFLPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Fast Startup Input Enable 0"]
    #[inline(always)]
    pub fn fstt0(&self) -> FSTT0_R {
        FSTT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fast Startup Input Enable 1"]
    #[inline(always)]
    pub fn fstt1(&self) -> FSTT1_R {
        FSTT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fast Startup Input Enable 2"]
    #[inline(always)]
    pub fn fstt2(&self) -> FSTT2_R {
        FSTT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fast Startup Input Enable 3"]
    #[inline(always)]
    pub fn fstt3(&self) -> FSTT3_R {
        FSTT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fast Startup Input Enable 4"]
    #[inline(always)]
    pub fn fstt4(&self) -> FSTT4_R {
        FSTT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fast Startup Input Enable 5"]
    #[inline(always)]
    pub fn fstt5(&self) -> FSTT5_R {
        FSTT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fast Startup Input Enable 6"]
    #[inline(always)]
    pub fn fstt6(&self) -> FSTT6_R {
        FSTT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fast Startup Input Enable 7"]
    #[inline(always)]
    pub fn fstt7(&self) -> FSTT7_R {
        FSTT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fast Startup Input Enable 8"]
    #[inline(always)]
    pub fn fstt8(&self) -> FSTT8_R {
        FSTT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast Startup Input Enable 9"]
    #[inline(always)]
    pub fn fstt9(&self) -> FSTT9_R {
        FSTT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Fast Startup Input Enable 10"]
    #[inline(always)]
    pub fn fstt10(&self) -> FSTT10_R {
        FSTT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Fast Startup Input Enable 11"]
    #[inline(always)]
    pub fn fstt11(&self) -> FSTT11_R {
        FSTT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fast Startup Input Enable 12"]
    #[inline(always)]
    pub fn fstt12(&self) -> FSTT12_R {
        FSTT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Fast Startup Input Enable 13"]
    #[inline(always)]
    pub fn fstt13(&self) -> FSTT13_R {
        FSTT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast Startup Input Enable 14"]
    #[inline(always)]
    pub fn fstt14(&self) -> FSTT14_R {
        FSTT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Fast Startup Input Enable 15"]
    #[inline(always)]
    pub fn fstt15(&self) -> FSTT15_R {
        FSTT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTT Alarm Enable"]
    #[inline(always)]
    pub fn rttal(&self) -> RTTAL_R {
        RTTAL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RTC Alarm Enable"]
    #[inline(always)]
    pub fn rtcal(&self) -> RTCAL_R {
        RTCAL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USB Alarm Enable"]
    #[inline(always)]
    pub fn usbal(&self) -> USBAL_R {
        USBAL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Low-power Mode"]
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Flash Low-power Mode"]
    #[inline(always)]
    pub fn flpm(&self) -> FLPM_R {
        FLPM_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Force Flash Low-power Mode"]
    #[inline(always)]
    pub fn fflpm(&self) -> FFLPM_R {
        FFLPM_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast Startup Input Enable 0"]
    #[inline(always)]
    pub fn fstt0(&mut self) -> FSTT0_W<0> {
        FSTT0_W::new(self)
    }
    #[doc = "Bit 1 - Fast Startup Input Enable 1"]
    #[inline(always)]
    pub fn fstt1(&mut self) -> FSTT1_W<1> {
        FSTT1_W::new(self)
    }
    #[doc = "Bit 2 - Fast Startup Input Enable 2"]
    #[inline(always)]
    pub fn fstt2(&mut self) -> FSTT2_W<2> {
        FSTT2_W::new(self)
    }
    #[doc = "Bit 3 - Fast Startup Input Enable 3"]
    #[inline(always)]
    pub fn fstt3(&mut self) -> FSTT3_W<3> {
        FSTT3_W::new(self)
    }
    #[doc = "Bit 4 - Fast Startup Input Enable 4"]
    #[inline(always)]
    pub fn fstt4(&mut self) -> FSTT4_W<4> {
        FSTT4_W::new(self)
    }
    #[doc = "Bit 5 - Fast Startup Input Enable 5"]
    #[inline(always)]
    pub fn fstt5(&mut self) -> FSTT5_W<5> {
        FSTT5_W::new(self)
    }
    #[doc = "Bit 6 - Fast Startup Input Enable 6"]
    #[inline(always)]
    pub fn fstt6(&mut self) -> FSTT6_W<6> {
        FSTT6_W::new(self)
    }
    #[doc = "Bit 7 - Fast Startup Input Enable 7"]
    #[inline(always)]
    pub fn fstt7(&mut self) -> FSTT7_W<7> {
        FSTT7_W::new(self)
    }
    #[doc = "Bit 8 - Fast Startup Input Enable 8"]
    #[inline(always)]
    pub fn fstt8(&mut self) -> FSTT8_W<8> {
        FSTT8_W::new(self)
    }
    #[doc = "Bit 9 - Fast Startup Input Enable 9"]
    #[inline(always)]
    pub fn fstt9(&mut self) -> FSTT9_W<9> {
        FSTT9_W::new(self)
    }
    #[doc = "Bit 10 - Fast Startup Input Enable 10"]
    #[inline(always)]
    pub fn fstt10(&mut self) -> FSTT10_W<10> {
        FSTT10_W::new(self)
    }
    #[doc = "Bit 11 - Fast Startup Input Enable 11"]
    #[inline(always)]
    pub fn fstt11(&mut self) -> FSTT11_W<11> {
        FSTT11_W::new(self)
    }
    #[doc = "Bit 12 - Fast Startup Input Enable 12"]
    #[inline(always)]
    pub fn fstt12(&mut self) -> FSTT12_W<12> {
        FSTT12_W::new(self)
    }
    #[doc = "Bit 13 - Fast Startup Input Enable 13"]
    #[inline(always)]
    pub fn fstt13(&mut self) -> FSTT13_W<13> {
        FSTT13_W::new(self)
    }
    #[doc = "Bit 14 - Fast Startup Input Enable 14"]
    #[inline(always)]
    pub fn fstt14(&mut self) -> FSTT14_W<14> {
        FSTT14_W::new(self)
    }
    #[doc = "Bit 15 - Fast Startup Input Enable 15"]
    #[inline(always)]
    pub fn fstt15(&mut self) -> FSTT15_W<15> {
        FSTT15_W::new(self)
    }
    #[doc = "Bit 16 - RTT Alarm Enable"]
    #[inline(always)]
    pub fn rttal(&mut self) -> RTTAL_W<16> {
        RTTAL_W::new(self)
    }
    #[doc = "Bit 17 - RTC Alarm Enable"]
    #[inline(always)]
    pub fn rtcal(&mut self) -> RTCAL_W<17> {
        RTCAL_W::new(self)
    }
    #[doc = "Bit 18 - USB Alarm Enable"]
    #[inline(always)]
    pub fn usbal(&mut self) -> USBAL_W<18> {
        USBAL_W::new(self)
    }
    #[doc = "Bit 20 - Low-power Mode"]
    #[inline(always)]
    pub fn lpm(&mut self) -> LPM_W<20> {
        LPM_W::new(self)
    }
    #[doc = "Bits 21:22 - Flash Low-power Mode"]
    #[inline(always)]
    pub fn flpm(&mut self) -> FLPM_W<21> {
        FLPM_W::new(self)
    }
    #[doc = "Bit 23 - Force Flash Low-power Mode"]
    #[inline(always)]
    pub fn fflpm(&mut self) -> FFLPM_W<23> {
        FFLPM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fast Startup Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsmr](index.html) module"]
pub struct FSMR_SPEC;
impl crate::RegisterSpec for FSMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsmr::R](R) reader structure"]
impl crate::Readable for FSMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsmr::W](W) writer structure"]
impl crate::Writable for FSMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSMR to value 0"]
impl crate::Resettable for FSMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
