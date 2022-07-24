#[doc = "Register `LPR` reader"]
pub struct R(crate::R<LPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPR` writer"]
pub struct W(crate::W<LPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPR_SPEC>;
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
impl From<crate::W<LPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Low-power Configuration Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPCB_A {
    #[doc = "0: The low-power feature is inhibited: no Power-down, Self-refresh or Deep Power-down command is issued to the SDRAM device."]
    DISABLED = 0,
    #[doc = "1: The SDRAMC issues a Self-refresh command to the SDRAM device, the SDCK clock is deactivated and the SDCKE signal is set low. The SDRAM device leaves the Self-refresh mode when accessed and enters it after the access."]
    SELF_REFRESH = 1,
    #[doc = "2: The SDRAMC issues a Power-down Command to the SDRAM device after each access, the SDCKE signal is set to low. The SDRAM device leaves the Power-down mode when accessed and enters it after the access."]
    POWER_DOWN = 2,
    #[doc = "3: The SDRAMC issues a Deep Power-down command to the SDRAM device. This mode is unique to low-power SDRAM."]
    DEEP_POWER_DOWN = 3,
}
impl From<LPCB_A> for u8 {
    #[inline(always)]
    fn from(variant: LPCB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPCB` reader - Low-power Configuration Bits"]
pub type LPCB_R = crate::FieldReader<u8, LPCB_A>;
impl LPCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCB_A {
        match self.bits {
            0 => LPCB_A::DISABLED,
            1 => LPCB_A::SELF_REFRESH,
            2 => LPCB_A::POWER_DOWN,
            3 => LPCB_A::DEEP_POWER_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPCB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `SELF_REFRESH`"]
    #[inline(always)]
    pub fn is_self_refresh(&self) -> bool {
        *self == LPCB_A::SELF_REFRESH
    }
    #[doc = "Checks if the value of the field is `POWER_DOWN`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == LPCB_A::POWER_DOWN
    }
    #[doc = "Checks if the value of the field is `DEEP_POWER_DOWN`"]
    #[inline(always)]
    pub fn is_deep_power_down(&self) -> bool {
        *self == LPCB_A::DEEP_POWER_DOWN
    }
}
#[doc = "Field `LPCB` writer - Low-power Configuration Bits"]
pub type LPCB_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, LPR_SPEC, u8, LPCB_A, 2, O>;
impl<'a, const O: u8> LPCB_W<'a, O> {
    #[doc = "The low-power feature is inhibited: no Power-down, Self-refresh or Deep Power-down command is issued to the SDRAM device."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPCB_A::DISABLED)
    }
    #[doc = "The SDRAMC issues a Self-refresh command to the SDRAM device, the SDCK clock is deactivated and the SDCKE signal is set low. The SDRAM device leaves the Self-refresh mode when accessed and enters it after the access."]
    #[inline(always)]
    pub fn self_refresh(self) -> &'a mut W {
        self.variant(LPCB_A::SELF_REFRESH)
    }
    #[doc = "The SDRAMC issues a Power-down Command to the SDRAM device after each access, the SDCKE signal is set to low. The SDRAM device leaves the Power-down mode when accessed and enters it after the access."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(LPCB_A::POWER_DOWN)
    }
    #[doc = "The SDRAMC issues a Deep Power-down command to the SDRAM device. This mode is unique to low-power SDRAM."]
    #[inline(always)]
    pub fn deep_power_down(self) -> &'a mut W {
        self.variant(LPCB_A::DEEP_POWER_DOWN)
    }
}
#[doc = "Field `PASR` reader - Partial Array Self-refresh (only for low-power SDRAM)"]
pub type PASR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PASR` writer - Partial Array Self-refresh (only for low-power SDRAM)"]
pub type PASR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TCSR` reader - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
pub type TCSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCSR` writer - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
pub type TCSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DS` reader - Drive Strength (only for low-power SDRAM)"]
pub type DS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DS` writer - Drive Strength (only for low-power SDRAM)"]
pub type DS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPR_SPEC, u8, u8, 2, O>;
#[doc = "Time to Define When Low-power Mode Is Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMEOUT_A {
    #[doc = "0: The SDRAMC activates the SDRAM Low-power mode immediately after the end of the last transfer."]
    LP_LAST_XFER = 0,
    #[doc = "1: The SDRAMC activates the SDRAM Low-power mode 64 clock cycles after the end of the last transfer."]
    LP_LAST_XFER_64 = 1,
    #[doc = "2: The SDRAMC activates the SDRAM Low-power mode 128 clock cycles after the end of the last transfer."]
    LP_LAST_XFER_128 = 2,
}
impl From<TIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMEOUT` reader - Time to Define When Low-power Mode Is Enabled"]
pub type TIMEOUT_R = crate::FieldReader<u8, TIMEOUT_A>;
impl TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMEOUT_A> {
        match self.bits {
            0 => Some(TIMEOUT_A::LP_LAST_XFER),
            1 => Some(TIMEOUT_A::LP_LAST_XFER_64),
            2 => Some(TIMEOUT_A::LP_LAST_XFER_128),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LP_LAST_XFER`"]
    #[inline(always)]
    pub fn is_lp_last_xfer(&self) -> bool {
        *self == TIMEOUT_A::LP_LAST_XFER
    }
    #[doc = "Checks if the value of the field is `LP_LAST_XFER_64`"]
    #[inline(always)]
    pub fn is_lp_last_xfer_64(&self) -> bool {
        *self == TIMEOUT_A::LP_LAST_XFER_64
    }
    #[doc = "Checks if the value of the field is `LP_LAST_XFER_128`"]
    #[inline(always)]
    pub fn is_lp_last_xfer_128(&self) -> bool {
        *self == TIMEOUT_A::LP_LAST_XFER_128
    }
}
#[doc = "Field `TIMEOUT` writer - Time to Define When Low-power Mode Is Enabled"]
pub type TIMEOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPR_SPEC, u8, TIMEOUT_A, 2, O>;
impl<'a, const O: u8> TIMEOUT_W<'a, O> {
    #[doc = "The SDRAMC activates the SDRAM Low-power mode immediately after the end of the last transfer."]
    #[inline(always)]
    pub fn lp_last_xfer(self) -> &'a mut W {
        self.variant(TIMEOUT_A::LP_LAST_XFER)
    }
    #[doc = "The SDRAMC activates the SDRAM Low-power mode 64 clock cycles after the end of the last transfer."]
    #[inline(always)]
    pub fn lp_last_xfer_64(self) -> &'a mut W {
        self.variant(TIMEOUT_A::LP_LAST_XFER_64)
    }
    #[doc = "The SDRAMC activates the SDRAM Low-power mode 128 clock cycles after the end of the last transfer."]
    #[inline(always)]
    pub fn lp_last_xfer_128(self) -> &'a mut W {
        self.variant(TIMEOUT_A::LP_LAST_XFER_128)
    }
}
impl R {
    #[doc = "Bits 0:1 - Low-power Configuration Bits"]
    #[inline(always)]
    pub fn lpcb(&self) -> LPCB_R {
        LPCB_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Partial Array Self-refresh (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn pasr(&self) -> PASR_R {
        PASR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn tcsr(&self) -> TCSR_R {
        TCSR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive Strength (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Time to Define When Low-power Mode Is Enabled"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low-power Configuration Bits"]
    #[inline(always)]
    pub fn lpcb(&mut self) -> LPCB_W<0> {
        LPCB_W::new(self)
    }
    #[doc = "Bits 4:6 - Partial Array Self-refresh (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn pasr(&mut self) -> PASR_W<4> {
        PASR_W::new(self)
    }
    #[doc = "Bits 8:9 - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn tcsr(&mut self) -> TCSR_W<8> {
        TCSR_W::new(self)
    }
    #[doc = "Bits 10:11 - Drive Strength (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W<10> {
        DS_W::new(self)
    }
    #[doc = "Bits 12:13 - Time to Define When Low-power Mode Is Enabled"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<12> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAMC Low Power Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpr](index.html) module"]
pub struct LPR_SPEC;
impl crate::RegisterSpec for LPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpr::R](R) reader structure"]
impl crate::Readable for LPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpr::W](W) writer structure"]
impl crate::Writable for LPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPR to value 0"]
impl crate::Resettable for LPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
