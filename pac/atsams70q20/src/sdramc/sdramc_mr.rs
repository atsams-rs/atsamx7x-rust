#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDRAMC_MR {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Normal mode. Any access to the SDRAM is decoded normally. To activate this mode, command must be followed by a write to the SDRAM."]
    NORMAL,
    #[doc = "The SDRAMC issues a NOP command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    NOP,
    #[doc = "The SDRAMC issues an 'All Banks Precharge' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    ALLBANKS_PRECHARGE,
    #[doc = "The SDRAMC issues a 'Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    LOAD_MODEREG,
    #[doc = "The SDRAMC issues an 'Auto-Refresh' Command when the SDRAM device is accessed regardless of the cycle. Previously, an 'All Banks Precharge' command must be issued. To activate this mode, command must be followed by a write to the SDRAM."]
    AUTO_REFRESH,
    #[doc = "The SDRAMC issues an 'Extended Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the 'Extended Load Mode Register' command must be followed by a write to the SDRAM. The write in the SDRAM must be done in the appropriate bank; most low-power SDRAM devices use the bank 1."]
    EXT_LOAD_MODEREG,
    #[doc = "Deep power-down mode. Enters deep power-down mode."]
    DEEP_POWERDOWN,
}
impl crate::ToBits<u8> for MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MODER::NORMAL => 0,
            MODER::NOP => 1,
            MODER::ALLBANKS_PRECHARGE => 2,
            MODER::LOAD_MODEREG => 3,
            MODER::AUTO_REFRESH => 4,
            MODER::EXT_LOAD_MODEREG => 5,
            MODER::DEEP_POWERDOWN => 6,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MODE_R = crate::FR<u8, MODER>;
impl MODE_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MODER::NORMAL
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == MODER::NOP
    }
    #[doc = "Checks if the value of the field is `ALLBANKS_PRECHARGE`"]
    #[inline(always)]
    pub fn is_allbanks_precharge(&self) -> bool {
        *self == MODER::ALLBANKS_PRECHARGE
    }
    #[doc = "Checks if the value of the field is `LOAD_MODEREG`"]
    #[inline(always)]
    pub fn is_load_modereg(&self) -> bool {
        *self == MODER::LOAD_MODEREG
    }
    #[doc = "Checks if the value of the field is `AUTO_REFRESH`"]
    #[inline(always)]
    pub fn is_auto_refresh(&self) -> bool {
        *self == MODER::AUTO_REFRESH
    }
    #[doc = "Checks if the value of the field is `EXT_LOAD_MODEREG`"]
    #[inline(always)]
    pub fn is_ext_load_modereg(&self) -> bool {
        *self == MODER::EXT_LOAD_MODEREG
    }
    #[doc = "Checks if the value of the field is `DEEP_POWERDOWN`"]
    #[inline(always)]
    pub fn is_deep_powerdown(&self) -> bool {
        *self == MODER::DEEP_POWERDOWN
    }
}
#[doc = "Values that can be written to the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODEW {
    #[doc = "Normal mode. Any access to the SDRAM is decoded normally. To activate this mode, command must be followed by a write to the SDRAM."]
    NORMAL,
    #[doc = "The SDRAMC issues a NOP command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    NOP,
    #[doc = "The SDRAMC issues an 'All Banks Precharge' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    ALLBANKS_PRECHARGE,
    #[doc = "The SDRAMC issues a 'Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    LOAD_MODEREG,
    #[doc = "The SDRAMC issues an 'Auto-Refresh' Command when the SDRAM device is accessed regardless of the cycle. Previously, an 'All Banks Precharge' command must be issued. To activate this mode, command must be followed by a write to the SDRAM."]
    AUTO_REFRESH,
    #[doc = "The SDRAMC issues an 'Extended Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the 'Extended Load Mode Register' command must be followed by a write to the SDRAM. The write in the SDRAM must be done in the appropriate bank; most low-power SDRAM devices use the bank 1."]
    EXT_LOAD_MODEREG,
    #[doc = "Deep power-down mode. Enters deep power-down mode."]
    DEEP_POWERDOWN,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::NORMAL => 0,
            MODEW::NOP => 1,
            MODEW::ALLBANKS_PRECHARGE => 2,
            MODEW::LOAD_MODEREG => 3,
            MODEW::AUTO_REFRESH => 4,
            MODEW::EXT_LOAD_MODEREG => 5,
            MODEW::DEEP_POWERDOWN => 6,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal mode. Any access to the SDRAM is decoded normally. To activate this mode, command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MODEW::NORMAL)
    }
    #[doc = "The SDRAMC issues a NOP command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(MODEW::NOP)
    }
    #[doc = "The SDRAMC issues an 'All Banks Precharge' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn allbanks_precharge(self) -> &'a mut W {
        self.variant(MODEW::ALLBANKS_PRECHARGE)
    }
    #[doc = "The SDRAMC issues a 'Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn load_modereg(self) -> &'a mut W {
        self.variant(MODEW::LOAD_MODEREG)
    }
    #[doc = "The SDRAMC issues an 'Auto-Refresh' Command when the SDRAM device is accessed regardless of the cycle. Previously, an 'All Banks Precharge' command must be issued. To activate this mode, command must be followed by a write to the SDRAM."]
    #[inline(always)]
    pub fn auto_refresh(self) -> &'a mut W {
        self.variant(MODEW::AUTO_REFRESH)
    }
    #[doc = "The SDRAMC issues an 'Extended Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the 'Extended Load Mode Register' command must be followed by a write to the SDRAM. The write in the SDRAM must be done in the appropriate bank; most low-power SDRAM devices use the bank 1."]
    #[inline(always)]
    pub fn ext_load_modereg(self) -> &'a mut W {
        self.variant(MODEW::EXT_LOAD_MODEREG)
    }
    #[doc = "Deep power-down mode. Enters deep power-down mode."]
    #[inline(always)]
    pub fn deep_powerdown(self) -> &'a mut W {
        self.variant(MODEW::DEEP_POWERDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - SDRAMC Command Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits() & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - SDRAMC Command Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
}
