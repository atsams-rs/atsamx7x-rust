#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDRAMC_LPR {
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
#[doc = "Possible values of the field `LPCB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCBR {
    #[doc = "Low Power Feature is inhibited: no Power-down, Self-refresh or Deep Power-down command is issued to the SDRAM device."]
    DISABLED,
    #[doc = "The SDRAMC issues a Self-refresh command to the SDRAM device, the SDCK clock is deactivated and the SDCKE signal is set low. The SDRAM device leaves the Self Refresh Mode when accessed and enters it after the access."]
    SELF_REFRESH,
    #[doc = "The SDRAMC issues a Power-down Command to the SDRAM device after each access, the SDCKE signal is set to low. The SDRAM device leaves the Power-down Mode when accessed and enters it after the access."]
    POWER_DOWN,
    #[doc = "The SDRAMC issues a Deep Power-down command to the SDRAM device. This mode is unique to low-power SDRAM."]
    DEEP_POWER_DOWN,
}
impl LPCBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPCBR::DISABLED => 0,
            LPCBR::SELF_REFRESH => 1,
            LPCBR::POWER_DOWN => 2,
            LPCBR::DEEP_POWER_DOWN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPCBR {
        match value {
            0 => LPCBR::DISABLED,
            1 => LPCBR::SELF_REFRESH,
            2 => LPCBR::POWER_DOWN,
            3 => LPCBR::DEEP_POWER_DOWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LPCBR::DISABLED
    }
    #[doc = "Checks if the value of the field is `SELF_REFRESH`"]
    #[inline]
    pub fn is_self_refresh(&self) -> bool {
        *self == LPCBR::SELF_REFRESH
    }
    #[doc = "Checks if the value of the field is `POWER_DOWN`"]
    #[inline]
    pub fn is_power_down(&self) -> bool {
        *self == LPCBR::POWER_DOWN
    }
    #[doc = "Checks if the value of the field is `DEEP_POWER_DOWN`"]
    #[inline]
    pub fn is_deep_power_down(&self) -> bool {
        *self == LPCBR::DEEP_POWER_DOWN
    }
}
#[doc = r" Value of the field"]
pub struct PASRR {
    bits: u8,
}
impl PASRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TCSRR {
    bits: u8,
}
impl TCSRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DSR {
    bits: u8,
}
impl DSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUTR {
    #[doc = "The SDRAMC activates the SDRAM low-power mode immediately after the end of the last transfer."]
    LP_LAST_XFER,
    #[doc = "The SDRAMC activates the SDRAM low-power mode 64 clock cycles after the end of the last transfer."]
    LP_LAST_XFER_64,
    #[doc = "The SDRAMC activates the SDRAM low-power mode 128 clock cycles after the end of the last transfer."]
    LP_LAST_XFER_128,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TIMEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMEOUTR::LP_LAST_XFER => 0,
            TIMEOUTR::LP_LAST_XFER_64 => 1,
            TIMEOUTR::LP_LAST_XFER_128 => 2,
            TIMEOUTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMEOUTR {
        match value {
            0 => TIMEOUTR::LP_LAST_XFER,
            1 => TIMEOUTR::LP_LAST_XFER_64,
            2 => TIMEOUTR::LP_LAST_XFER_128,
            i => TIMEOUTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LP_LAST_XFER`"]
    #[inline]
    pub fn is_lp_last_xfer(&self) -> bool {
        *self == TIMEOUTR::LP_LAST_XFER
    }
    #[doc = "Checks if the value of the field is `LP_LAST_XFER_64`"]
    #[inline]
    pub fn is_lp_last_xfer_64(&self) -> bool {
        *self == TIMEOUTR::LP_LAST_XFER_64
    }
    #[doc = "Checks if the value of the field is `LP_LAST_XFER_128`"]
    #[inline]
    pub fn is_lp_last_xfer_128(&self) -> bool {
        *self == TIMEOUTR::LP_LAST_XFER_128
    }
}
#[doc = "Values that can be written to the field `LPCB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCBW {
    #[doc = "Low Power Feature is inhibited: no Power-down, Self-refresh or Deep Power-down command is issued to the SDRAM device."]
    DISABLED,
    #[doc = "The SDRAMC issues a Self-refresh command to the SDRAM device, the SDCK clock is deactivated and the SDCKE signal is set low. The SDRAM device leaves the Self Refresh Mode when accessed and enters it after the access."]
    SELF_REFRESH,
    #[doc = "The SDRAMC issues a Power-down Command to the SDRAM device after each access, the SDCKE signal is set to low. The SDRAM device leaves the Power-down Mode when accessed and enters it after the access."]
    POWER_DOWN,
    #[doc = "The SDRAMC issues a Deep Power-down command to the SDRAM device. This mode is unique to low-power SDRAM."]
    DEEP_POWER_DOWN,
}
impl LPCBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPCBW::DISABLED => 0,
            LPCBW::SELF_REFRESH => 1,
            LPCBW::POWER_DOWN => 2,
            LPCBW::DEEP_POWER_DOWN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPCBW<'a> {
    w: &'a mut W,
}
impl<'a> _LPCBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPCBW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Low Power Feature is inhibited: no Power-down, Self-refresh or Deep Power-down command is issued to the SDRAM device."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPCBW::DISABLED)
    }
    #[doc = "The SDRAMC issues a Self-refresh command to the SDRAM device, the SDCK clock is deactivated and the SDCKE signal is set low. The SDRAM device leaves the Self Refresh Mode when accessed and enters it after the access."]
    #[inline]
    pub fn self_refresh(self) -> &'a mut W {
        self.variant(LPCBW::SELF_REFRESH)
    }
    #[doc = "The SDRAMC issues a Power-down Command to the SDRAM device after each access, the SDCKE signal is set to low. The SDRAM device leaves the Power-down Mode when accessed and enters it after the access."]
    #[inline]
    pub fn power_down(self) -> &'a mut W {
        self.variant(LPCBW::POWER_DOWN)
    }
    #[doc = "The SDRAMC issues a Deep Power-down command to the SDRAM device. This mode is unique to low-power SDRAM."]
    #[inline]
    pub fn deep_power_down(self) -> &'a mut W {
        self.variant(LPCBW::DEEP_POWER_DOWN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PASRW<'a> {
    w: &'a mut W,
}
impl<'a> _PASRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TCSRW<'a> {
    w: &'a mut W,
}
impl<'a> _TCSRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DSW<'a> {
    w: &'a mut W,
}
impl<'a> _DSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUTW {
    #[doc = "The SDRAMC activates the SDRAM low-power mode immediately after the end of the last transfer."]
    LP_LAST_XFER,
    #[doc = "The SDRAMC activates the SDRAM low-power mode 64 clock cycles after the end of the last transfer."]
    LP_LAST_XFER_64,
    #[doc = "The SDRAMC activates the SDRAM low-power mode 128 clock cycles after the end of the last transfer."]
    LP_LAST_XFER_128,
}
impl TIMEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMEOUTW::LP_LAST_XFER => 0,
            TIMEOUTW::LP_LAST_XFER_64 => 1,
            TIMEOUTW::LP_LAST_XFER_128 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMEOUTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The SDRAMC activates the SDRAM low-power mode immediately after the end of the last transfer."]
    #[inline]
    pub fn lp_last_xfer(self) -> &'a mut W {
        self.variant(TIMEOUTW::LP_LAST_XFER)
    }
    #[doc = "The SDRAMC activates the SDRAM low-power mode 64 clock cycles after the end of the last transfer."]
    #[inline]
    pub fn lp_last_xfer_64(self) -> &'a mut W {
        self.variant(TIMEOUTW::LP_LAST_XFER_64)
    }
    #[doc = "The SDRAMC activates the SDRAM low-power mode 128 clock cycles after the end of the last transfer."]
    #[inline]
    pub fn lp_last_xfer_128(self) -> &'a mut W {
        self.variant(TIMEOUTW::LP_LAST_XFER_128)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:1 - Low-power Configuration Bits"]
    #[inline]
    pub fn lpcb(&self) -> LPCBR {
        LPCBR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Partial Array Self-refresh (only for low-power SDRAM)"]
    #[inline]
    pub fn pasr(&self) -> PASRR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PASRR { bits }
    }
    #[doc = "Bits 8:9 - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
    #[inline]
    pub fn tcsr(&self) -> TCSRR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TCSRR { bits }
    }
    #[doc = "Bits 10:11 - Drive Strength (only for low-power SDRAM)"]
    #[inline]
    pub fn ds(&self) -> DSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DSR { bits }
    }
    #[doc = "Bits 12:13 - Time to Define When Low-power Mode Is Enabled"]
    #[inline]
    pub fn timeout(&self) -> TIMEOUTR {
        TIMEOUTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:1 - Low-power Configuration Bits"]
    #[inline]
    pub fn lpcb(&mut self) -> _LPCBW {
        _LPCBW { w: self }
    }
    #[doc = "Bits 4:6 - Partial Array Self-refresh (only for low-power SDRAM)"]
    #[inline]
    pub fn pasr(&mut self) -> _PASRW {
        _PASRW { w: self }
    }
    #[doc = "Bits 8:9 - Temperature Compensated Self-Refresh (only for low-power SDRAM)"]
    #[inline]
    pub fn tcsr(&mut self) -> _TCSRW {
        _TCSRW { w: self }
    }
    #[doc = "Bits 10:11 - Drive Strength (only for low-power SDRAM)"]
    #[inline]
    pub fn ds(&mut self) -> _DSW {
        _DSW { w: self }
    }
    #[doc = "Bits 12:13 - Time to Define When Low-power Mode Is Enabled"]
    #[inline]
    pub fn timeout(&mut self) -> _TIMEOUTW {
        _TIMEOUTW { w: self }
    }
}
