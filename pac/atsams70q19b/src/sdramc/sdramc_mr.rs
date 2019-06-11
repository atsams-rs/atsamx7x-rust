#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDRAMC_MR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Normal mode. Any access to the SDRAM is decoded normally. To activate this mode, the command must be followed by a write to the SDRAM."]
    NORMAL,
    #[doc = "The SDRAMC issues a NOP command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    NOP,
    #[doc = "The SDRAMC issues an 'All Banks Precharge' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    ALLBANKS_PRECHARGE,
    #[doc = "The SDRAMC issues a 'Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    LOAD_MODEREG,
    #[doc = "The SDRAMC issues an 'Auto-Refresh' Command when the SDRAM device is accessed regardless of the cycle. Previously, an 'All Banks Precharge' command must be issued. To activate this mode, the command must be followed by a write to the SDRAM."]
    AUTO_REFRESH,
    #[doc = "The SDRAMC issues an 'Extended Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the 'Extended Load Mode Register' command must be followed by a write to the SDRAM. The write in the SDRAM must be done in the appropriate bank; most low-power SDRAM devices use the bank 1."]
    EXT_LOAD_MODEREG,
    #[doc = "Deep Power-down mode. Enters Deep Power-down mode."]
    DEEP_POWERDOWN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::NORMAL => 0,
            MODER::NOP => 1,
            MODER::ALLBANKS_PRECHARGE => 2,
            MODER::LOAD_MODEREG => 3,
            MODER::AUTO_REFRESH => 4,
            MODER::EXT_LOAD_MODEREG => 5,
            MODER::DEEP_POWERDOWN => 6,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::NORMAL,
            1 => MODER::NOP,
            2 => MODER::ALLBANKS_PRECHARGE,
            3 => MODER::LOAD_MODEREG,
            4 => MODER::AUTO_REFRESH,
            5 => MODER::EXT_LOAD_MODEREG,
            6 => MODER::DEEP_POWERDOWN,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == MODER::NORMAL
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline]
    pub fn is_nop(&self) -> bool {
        *self == MODER::NOP
    }
    #[doc = "Checks if the value of the field is `ALLBANKS_PRECHARGE`"]
    #[inline]
    pub fn is_allbanks_precharge(&self) -> bool {
        *self == MODER::ALLBANKS_PRECHARGE
    }
    #[doc = "Checks if the value of the field is `LOAD_MODEREG`"]
    #[inline]
    pub fn is_load_modereg(&self) -> bool {
        *self == MODER::LOAD_MODEREG
    }
    #[doc = "Checks if the value of the field is `AUTO_REFRESH`"]
    #[inline]
    pub fn is_auto_refresh(&self) -> bool {
        *self == MODER::AUTO_REFRESH
    }
    #[doc = "Checks if the value of the field is `EXT_LOAD_MODEREG`"]
    #[inline]
    pub fn is_ext_load_modereg(&self) -> bool {
        *self == MODER::EXT_LOAD_MODEREG
    }
    #[doc = "Checks if the value of the field is `DEEP_POWERDOWN`"]
    #[inline]
    pub fn is_deep_powerdown(&self) -> bool {
        *self == MODER::DEEP_POWERDOWN
    }
}
#[doc = "Values that can be written to the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODEW {
    #[doc = "Normal mode. Any access to the SDRAM is decoded normally. To activate this mode, the command must be followed by a write to the SDRAM."]
    NORMAL,
    #[doc = "The SDRAMC issues a NOP command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    NOP,
    #[doc = "The SDRAMC issues an 'All Banks Precharge' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    ALLBANKS_PRECHARGE,
    #[doc = "The SDRAMC issues a 'Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    LOAD_MODEREG,
    #[doc = "The SDRAMC issues an 'Auto-Refresh' Command when the SDRAM device is accessed regardless of the cycle. Previously, an 'All Banks Precharge' command must be issued. To activate this mode, the command must be followed by a write to the SDRAM."]
    AUTO_REFRESH,
    #[doc = "The SDRAMC issues an 'Extended Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the 'Extended Load Mode Register' command must be followed by a write to the SDRAM. The write in the SDRAM must be done in the appropriate bank; most low-power SDRAM devices use the bank 1."]
    EXT_LOAD_MODEREG,
    #[doc = "Deep Power-down mode. Enters Deep Power-down mode."]
    DEEP_POWERDOWN,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
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
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal mode. Any access to the SDRAM is decoded normally. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(MODEW::NORMAL)
    }
    #[doc = "The SDRAMC issues a NOP command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline]
    pub fn nop(self) -> &'a mut W {
        self.variant(MODEW::NOP)
    }
    #[doc = "The SDRAMC issues an 'All Banks Precharge' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline]
    pub fn allbanks_precharge(self) -> &'a mut W {
        self.variant(MODEW::ALLBANKS_PRECHARGE)
    }
    #[doc = "The SDRAMC issues a 'Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline]
    pub fn load_modereg(self) -> &'a mut W {
        self.variant(MODEW::LOAD_MODEREG)
    }
    #[doc = "The SDRAMC issues an 'Auto-Refresh' Command when the SDRAM device is accessed regardless of the cycle. Previously, an 'All Banks Precharge' command must be issued. To activate this mode, the command must be followed by a write to the SDRAM."]
    #[inline]
    pub fn auto_refresh(self) -> &'a mut W {
        self.variant(MODEW::AUTO_REFRESH)
    }
    #[doc = "The SDRAMC issues an 'Extended Load Mode Register' command when the SDRAM device is accessed regardless of the cycle. To activate this mode, the 'Extended Load Mode Register' command must be followed by a write to the SDRAM. The write in the SDRAM must be done in the appropriate bank; most low-power SDRAM devices use the bank 1."]
    #[inline]
    pub fn ext_load_modereg(self) -> &'a mut W {
        self.variant(MODEW::EXT_LOAD_MODEREG)
    }
    #[doc = "Deep Power-down mode. Enters Deep Power-down mode."]
    #[inline]
    pub fn deep_powerdown(self) -> &'a mut W {
        self.variant(MODEW::DEEP_POWERDOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - SDRAMC Command Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - SDRAMC Command Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
}
