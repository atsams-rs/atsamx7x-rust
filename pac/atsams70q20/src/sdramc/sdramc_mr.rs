#[doc = "Reader of register SDRAMC_MR"]
pub type R = crate::R<u32, super::SDRAMC_MR>;
#[doc = "Writer for register SDRAMC_MR"]
pub type W = crate::W<u32, super::SDRAMC_MR>;
#[doc = "Register SDRAMC_MR `reset()`'s with value 0"]
impl crate::ResetValue for super::SDRAMC_MR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SDRAMC Command Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Normal mode. Any access to the SDRAM is decoded normally. To activate this mode, command must be followed by a write to the SDRAM."]
    NORMAL = 0,
    #[doc = "1: The SDRAMC issues a NOP command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    NOP = 1,
    #[doc = "2: The SDRAMC issues an 'All Banks Precharge' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    ALLBANKS_PRECHARGE = 2,
    #[doc = "3: The SDRAMC issues a 'Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    LOAD_MODEREG = 3,
    #[doc = "4: The SDRAMC issues an 'Auto-Refresh' Command when the SDRAM device is accessed regardless of the cycle. Previously, an 'All Banks Precharge' command must be issued. To activate this mode, command must be followed by a write to the SDRAM."]
    AUTO_REFRESH = 4,
    #[doc = "5: The SDRAMC issues an 'Extended Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the 'Extended Load Mode Register' command must be followed by a write to the SDRAM. The write in the SDRAM must be done in the appropriate bank; most low-power SDRAM devices use the bank 1."]
    EXT_LOAD_MODEREG = 5,
    #[doc = "6: Deep power-down mode. Enters deep power-down mode."]
    DEEP_POWERDOWN = 6,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::NORMAL),
            1 => Val(MODE_A::NOP),
            2 => Val(MODE_A::ALLBANKS_PRECHARGE),
            3 => Val(MODE_A::LOAD_MODEREG),
            4 => Val(MODE_A::AUTO_REFRESH),
            5 => Val(MODE_A::EXT_LOAD_MODEREG),
            6 => Val(MODE_A::DEEP_POWERDOWN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == MODE_A::NOP
    }
    #[doc = "Checks if the value of the field is `ALLBANKS_PRECHARGE`"]
    #[inline(always)]
    pub fn is_allbanks_precharge(&self) -> bool {
        *self == MODE_A::ALLBANKS_PRECHARGE
    }
    #[doc = "Checks if the value of the field is `LOAD_MODEREG`"]
    #[inline(always)]
    pub fn is_load_modereg(&self) -> bool {
        *self == MODE_A::LOAD_MODEREG
    }
    #[doc = "Checks if the value of the field is `AUTO_REFRESH`"]
    #[inline(always)]
    pub fn is_auto_refresh(&self) -> bool {
        *self == MODE_A::AUTO_REFRESH
    }
    #[doc = "Checks if the value of the field is `EXT_LOAD_MODEREG`"]
    #[inline(always)]
    pub fn is_ext_load_modereg(&self) -> bool {
        *self == MODE_A::EXT_LOAD_MODEREG
    }
    #[doc = "Checks if the value of the field is `DEEP_POWERDOWN`"]
    #[inline(always)]
    pub fn is_deep_powerdown(&self) -> bool {
        *self == MODE_A::DEEP_POWERDOWN
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal mode. Any access to the SDRAM is decoded normally. To activate this mode, command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MODE_A::NORMAL)
    }
    #[doc = "The SDRAMC issues a NOP command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(MODE_A::NOP)
    }
    #[doc = "The SDRAMC issues an 'All Banks Precharge' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn allbanks_precharge(self) -> &'a mut W {
        self.variant(MODE_A::ALLBANKS_PRECHARGE)
    }
    #[doc = "The SDRAMC issues a 'Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn load_modereg(self) -> &'a mut W {
        self.variant(MODE_A::LOAD_MODEREG)
    }
    #[doc = "The SDRAMC issues an 'Auto-Refresh' Command when the SDRAM device is accessed regardless of the cycle. Previously, an 'All Banks Precharge' command must be issued. To activate this mode, command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn auto_refresh(self) -> &'a mut W {
        self.variant(MODE_A::AUTO_REFRESH)
    }
    #[doc = "The SDRAMC issues an 'Extended Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the 'Extended Load Mode Register' command must be followed by a write to the SDRAM. The write in the SDRAM must be done in the appropriate bank; most low-power SDRAM devices use the bank 1."]
    #[inline(always)]
    pub fn ext_load_modereg(self) -> &'a mut W {
        self.variant(MODE_A::EXT_LOAD_MODEREG)
    }
    #[doc = "Deep power-down mode. Enters deep power-down mode."]
    #[inline(always)]
    pub fn deep_powerdown(self) -> &'a mut W {
        self.variant(MODE_A::DEEP_POWERDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SDRAMC Command Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SDRAMC Command Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
