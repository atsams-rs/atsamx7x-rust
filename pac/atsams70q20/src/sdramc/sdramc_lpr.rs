#[doc = "Reader of register SDRAMC_LPR"]
pub type R = crate::R<u32, super::SDRAMC_LPR>;
#[doc = "Writer for register SDRAMC_LPR"]
pub type W = crate::W<u32, super::SDRAMC_LPR>;
#[doc = "Register SDRAMC_LPR `reset()`'s with value 0"]
impl crate::ResetValue for super::SDRAMC_LPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Low-power Configuration Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPCB_A {
    #[doc = "0: Low Power Feature is inhibited: no Power-down, Self-refresh or Deep Power-down command is issued to the SDRAM device."]
    DISABLED = 0,
    #[doc = "1: The SDRAMC issues a Self-refresh command to the SDRAM device, the SDCK clock is deactivated and the SDCKE signal is set low. The SDRAM device leaves the Self Refresh Mode when accessed and enters it after the access."]
    SELF_REFRESH = 1,
    #[doc = "2: The SDRAMC issues a Power-down Command to the SDRAM device after each access, the SDCKE signal is set to low. The SDRAM device leaves the Power-down Mode when accessed and enters it after the access."]
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
#[doc = "Reader of field `LPCB`"]
pub type LPCB_R = crate::R<u8, LPCB_A>;
impl LPCB_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `LPCB`"]
pub struct LPCB_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPCB_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low Power Feature is inhibited: no Power-down, Self-refresh or Deep Power-down command is issued to the SDRAM device."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPCB_A::DISABLED)
    }
    #[doc = "The SDRAMC issues a Self-refresh command to the SDRAM device, the SDCK clock is deactivated and the SDCKE signal is set low. The SDRAM device leaves the Self Refresh Mode when accessed and enters it after the access."]
    #[inline(always)]
    pub fn self_refresh(self) -> &'a mut W {
        self.variant(LPCB_A::SELF_REFRESH)
    }
    #[doc = "The SDRAMC issues a Power-down Command to the SDRAM device after each access, the SDCKE signal is set to low. The SDRAM device leaves the Power-down Mode when accessed and enters it after the access."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(LPCB_A::POWER_DOWN)
    }
    #[doc = "The SDRAMC issues a Deep Power-down command to the SDRAM device. This mode is unique to low-power SDRAM."]
    #[inline(always)]
    pub fn deep_power_down(self) -> &'a mut W {
        self.variant(LPCB_A::DEEP_POWER_DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PASR`"]
pub type PASR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PASR`"]
pub struct PASR_W<'a> {
    w: &'a mut W,
}
impl<'a> PASR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `TCSR`"]
pub type TCSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCSR`"]
pub struct TCSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TCSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `DS`"]
pub type DS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DS`"]
pub struct DS_W<'a> {
    w: &'a mut W,
}
impl<'a> DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Time to Define When Low-power Mode Is Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMEOUT_A {
    #[doc = "0: The SDRAMC activates the SDRAM low-power mode immediately after the end of the last transfer."]
    LP_LAST_XFER = 0,
    #[doc = "1: The SDRAMC activates the SDRAM low-power mode 64 clock cycles after the end of the last transfer."]
    LP_LAST_XFER_64 = 1,
    #[doc = "2: The SDRAMC activates the SDRAM low-power mode 128 clock cycles after the end of the last transfer."]
    LP_LAST_XFER_128 = 2,
}
impl From<TIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TIMEOUT`"]
pub type TIMEOUT_R = crate::R<u8, TIMEOUT_A>;
impl TIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMEOUT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIMEOUT_A::LP_LAST_XFER),
            1 => Val(TIMEOUT_A::LP_LAST_XFER_64),
            2 => Val(TIMEOUT_A::LP_LAST_XFER_128),
            i => Res(i),
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
#[doc = "Write proxy for field `TIMEOUT`"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The SDRAMC activates the SDRAM low-power mode immediately after the end of the last transfer."]
    #[inline(always)]
    pub fn lp_last_xfer(self) -> &'a mut W {
        self.variant(TIMEOUT_A::LP_LAST_XFER)
    }
    #[doc = "The SDRAMC activates the SDRAM low-power mode 64 clock cycles after the end of the last transfer."]
    #[inline(always)]
    pub fn lp_last_xfer_64(self) -> &'a mut W {
        self.variant(TIMEOUT_A::LP_LAST_XFER_64)
    }
    #[doc = "The SDRAMC activates the SDRAM low-power mode 128 clock cycles after the end of the last transfer."]
    #[inline(always)]
    pub fn lp_last_xfer_128(self) -> &'a mut W {
        self.variant(TIMEOUT_A::LP_LAST_XFER_128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Low-power Configuration Bits"]
    #[inline(always)]
    pub fn lpcb(&self) -> LPCB_R {
        LPCB_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Partial Array Self-refresh (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn pasr(&self) -> PASR_R {
        PASR_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn tcsr(&self) -> TCSR_R {
        TCSR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Drive Strength (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Time to Define When Low-power Mode Is Enabled"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low-power Configuration Bits"]
    #[inline(always)]
    pub fn lpcb(&mut self) -> LPCB_W {
        LPCB_W { w: self }
    }
    #[doc = "Bits 4:6 - Partial Array Self-refresh (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn pasr(&mut self) -> PASR_W {
        PASR_W { w: self }
    }
    #[doc = "Bits 8:9 - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn tcsr(&mut self) -> TCSR_W {
        TCSR_W { w: self }
    }
    #[doc = "Bits 10:11 - Drive Strength (only for low-power SDRAM)"]
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W {
        DS_W { w: self }
    }
    #[doc = "Bits 12:13 - Time to Define When Low-power Mode Is Enabled"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
}
