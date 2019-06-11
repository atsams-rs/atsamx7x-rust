#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SUPC_WUMR {
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
#[doc = "Possible values of the field `SMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMENR {
    #[doc = "The supply monitor detection has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "The supply monitor detection forces the wake-up of the core power supply."]
    ENABLE,
}
impl SMENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SMENR::NOT_ENABLE => false,
            SMENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMENR {
        match value {
            false => SMENR::NOT_ENABLE,
            true => SMENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline]
    pub fn is_not_enable(&self) -> bool {
        *self == SMENR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == SMENR::ENABLE
    }
}
#[doc = "Possible values of the field `RTTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTTENR {
    #[doc = "The RTT alarm signal has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "The RTT alarm signal forces the wake-up of the core power supply."]
    ENABLE,
}
impl RTTENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RTTENR::NOT_ENABLE => false,
            RTTENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTTENR {
        match value {
            false => RTTENR::NOT_ENABLE,
            true => RTTENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline]
    pub fn is_not_enable(&self) -> bool {
        *self == RTTENR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RTTENR::ENABLE
    }
}
#[doc = "Possible values of the field `RTCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCENR {
    #[doc = "The RTC alarm signal has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "The RTC alarm signal forces the wake-up of the core power supply."]
    ENABLE,
}
impl RTCENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RTCENR::NOT_ENABLE => false,
            RTCENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTCENR {
        match value {
            false => RTCENR::NOT_ENABLE,
            true => RTCENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline]
    pub fn is_not_enable(&self) -> bool {
        *self == RTCENR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == RTCENR::ENABLE
    }
}
#[doc = "Possible values of the field `LPDBCEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCEN0R {
    #[doc = "The WKUP0 input pin is not connected to the low-power debouncer."]
    NOT_ENABLE,
    #[doc = "The WKUP0 input pin is connected to the low-power debouncer and forces a system wake-up."]
    ENABLE,
}
impl LPDBCEN0R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LPDBCEN0R::NOT_ENABLE => false,
            LPDBCEN0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPDBCEN0R {
        match value {
            false => LPDBCEN0R::NOT_ENABLE,
            true => LPDBCEN0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline]
    pub fn is_not_enable(&self) -> bool {
        *self == LPDBCEN0R::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == LPDBCEN0R::ENABLE
    }
}
#[doc = "Possible values of the field `LPDBCEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCEN1R {
    #[doc = "The WKUP1 input pin is not connected to the low-power debouncer."]
    NOT_ENABLE,
    #[doc = "The WKUP1 input pin is connected to the low-power debouncer and forces a system wake-up."]
    ENABLE,
}
impl LPDBCEN1R {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LPDBCEN1R::NOT_ENABLE => false,
            LPDBCEN1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPDBCEN1R {
        match value {
            false => LPDBCEN1R::NOT_ENABLE,
            true => LPDBCEN1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline]
    pub fn is_not_enable(&self) -> bool {
        *self == LPDBCEN1R::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == LPDBCEN1R::ENABLE
    }
}
#[doc = "Possible values of the field `LPDBCCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCCLRR {
    #[doc = "A low-power debounce event does not create an immediate clear on the first half of GPBR registers."]
    NOT_ENABLE,
    #[doc = "A low-power debounce event on WKUP0 or WKUP1 generates an immediate clear on the first half of GPBR registers."]
    ENABLE,
}
impl LPDBCCLRR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LPDBCCLRR::NOT_ENABLE => false,
            LPDBCCLRR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPDBCCLRR {
        match value {
            false => LPDBCCLRR::NOT_ENABLE,
            true => LPDBCCLRR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline]
    pub fn is_not_enable(&self) -> bool {
        *self == LPDBCCLRR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == LPDBCCLRR::ENABLE
    }
}
#[doc = "Possible values of the field `WKUPDBC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPDBCR {
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    IMMEDIATE,
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    _3_SLCK,
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    _32_SLCK,
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    _512_SLCK,
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    _4096_SLCK,
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    _32768_SLCK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WKUPDBCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WKUPDBCR::IMMEDIATE => 0,
            WKUPDBCR::_3_SLCK => 1,
            WKUPDBCR::_32_SLCK => 2,
            WKUPDBCR::_512_SLCK => 3,
            WKUPDBCR::_4096_SLCK => 4,
            WKUPDBCR::_32768_SLCK => 5,
            WKUPDBCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WKUPDBCR {
        match value {
            0 => WKUPDBCR::IMMEDIATE,
            1 => WKUPDBCR::_3_SLCK,
            2 => WKUPDBCR::_32_SLCK,
            3 => WKUPDBCR::_512_SLCK,
            4 => WKUPDBCR::_4096_SLCK,
            5 => WKUPDBCR::_32768_SLCK,
            i => WKUPDBCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IMMEDIATE`"]
    #[inline]
    pub fn is_immediate(&self) -> bool {
        *self == WKUPDBCR::IMMEDIATE
    }
    #[doc = "Checks if the value of the field is `_3_SLCK`"]
    #[inline]
    pub fn is_3_slck(&self) -> bool {
        *self == WKUPDBCR::_3_SLCK
    }
    #[doc = "Checks if the value of the field is `_32_SLCK`"]
    #[inline]
    pub fn is_32_slck(&self) -> bool {
        *self == WKUPDBCR::_32_SLCK
    }
    #[doc = "Checks if the value of the field is `_512_SLCK`"]
    #[inline]
    pub fn is_512_slck(&self) -> bool {
        *self == WKUPDBCR::_512_SLCK
    }
    #[doc = "Checks if the value of the field is `_4096_SLCK`"]
    #[inline]
    pub fn is_4096_slck(&self) -> bool {
        *self == WKUPDBCR::_4096_SLCK
    }
    #[doc = "Checks if the value of the field is `_32768_SLCK`"]
    #[inline]
    pub fn is_32768_slck(&self) -> bool {
        *self == WKUPDBCR::_32768_SLCK
    }
}
#[doc = "Possible values of the field `LPDBC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCR {
    #[doc = "Disable the low-power debouncers."]
    DISABLE,
    #[doc = "WKUP0/1 in active state for at least 2 RTCOUTx clock periods"]
    _2_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 3 RTCOUTx clock periods"]
    _3_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 4 RTCOUTx clock periods"]
    _4_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 5 RTCOUTx clock periods"]
    _5_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 6 RTCOUTx clock periods"]
    _6_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 7 RTCOUTx clock periods"]
    _7_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 8 RTCOUTx clock periods"]
    _8_RTCOUT,
}
impl LPDBCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPDBCR::DISABLE => 0,
            LPDBCR::_2_RTCOUT => 1,
            LPDBCR::_3_RTCOUT => 2,
            LPDBCR::_4_RTCOUT => 3,
            LPDBCR::_5_RTCOUT => 4,
            LPDBCR::_6_RTCOUT => 5,
            LPDBCR::_7_RTCOUT => 6,
            LPDBCR::_8_RTCOUT => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPDBCR {
        match value {
            0 => LPDBCR::DISABLE,
            1 => LPDBCR::_2_RTCOUT,
            2 => LPDBCR::_3_RTCOUT,
            3 => LPDBCR::_4_RTCOUT,
            4 => LPDBCR::_5_RTCOUT,
            5 => LPDBCR::_6_RTCOUT,
            6 => LPDBCR::_7_RTCOUT,
            7 => LPDBCR::_8_RTCOUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == LPDBCR::DISABLE
    }
    #[doc = "Checks if the value of the field is `_2_RTCOUT`"]
    #[inline]
    pub fn is_2_rtcout(&self) -> bool {
        *self == LPDBCR::_2_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_3_RTCOUT`"]
    #[inline]
    pub fn is_3_rtcout(&self) -> bool {
        *self == LPDBCR::_3_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_4_RTCOUT`"]
    #[inline]
    pub fn is_4_rtcout(&self) -> bool {
        *self == LPDBCR::_4_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_5_RTCOUT`"]
    #[inline]
    pub fn is_5_rtcout(&self) -> bool {
        *self == LPDBCR::_5_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_6_RTCOUT`"]
    #[inline]
    pub fn is_6_rtcout(&self) -> bool {
        *self == LPDBCR::_6_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_7_RTCOUT`"]
    #[inline]
    pub fn is_7_rtcout(&self) -> bool {
        *self == LPDBCR::_7_RTCOUT
    }
    #[doc = "Checks if the value of the field is `_8_RTCOUT`"]
    #[inline]
    pub fn is_8_rtcout(&self) -> bool {
        *self == LPDBCR::_8_RTCOUT
    }
}
#[doc = "Values that can be written to the field `SMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMENW {
    #[doc = "The supply monitor detection has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "The supply monitor detection forces the wake-up of the core power supply."]
    ENABLE,
}
impl SMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMENW::NOT_ENABLE => false,
            SMENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMENW<'a> {
    w: &'a mut W,
}
impl<'a> _SMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The supply monitor detection has no wake-up effect."]
    #[inline]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(SMENW::NOT_ENABLE)
    }
    #[doc = "The supply monitor detection forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(SMENW::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTTENW {
    #[doc = "The RTT alarm signal has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "The RTT alarm signal forces the wake-up of the core power supply."]
    ENABLE,
}
impl RTTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTTENW::NOT_ENABLE => false,
            RTTENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTTENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The RTT alarm signal has no wake-up effect."]
    #[inline]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(RTTENW::NOT_ENABLE)
    }
    #[doc = "The RTT alarm signal forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTTENW::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCENW {
    #[doc = "The RTC alarm signal has no wake-up effect."]
    NOT_ENABLE,
    #[doc = "The RTC alarm signal forces the wake-up of the core power supply."]
    ENABLE,
}
impl RTCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTCENW::NOT_ENABLE => false,
            RTCENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCENW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The RTC alarm signal has no wake-up effect."]
    #[inline]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(RTCENW::NOT_ENABLE)
    }
    #[doc = "The RTC alarm signal forces the wake-up of the core power supply."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(RTCENW::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPDBCEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCEN0W {
    #[doc = "The WKUP0 input pin is not connected to the low-power debouncer."]
    NOT_ENABLE,
    #[doc = "The WKUP0 input pin is connected to the low-power debouncer and forces a system wake-up."]
    ENABLE,
}
impl LPDBCEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPDBCEN0W::NOT_ENABLE => false,
            LPDBCEN0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPDBCEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _LPDBCEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPDBCEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The WKUP0 input pin is not connected to the low-power debouncer."]
    #[inline]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(LPDBCEN0W::NOT_ENABLE)
    }
    #[doc = "The WKUP0 input pin is connected to the low-power debouncer and forces a system wake-up."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPDBCEN0W::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPDBCEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCEN1W {
    #[doc = "The WKUP1 input pin is not connected to the low-power debouncer."]
    NOT_ENABLE,
    #[doc = "The WKUP1 input pin is connected to the low-power debouncer and forces a system wake-up."]
    ENABLE,
}
impl LPDBCEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPDBCEN1W::NOT_ENABLE => false,
            LPDBCEN1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPDBCEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _LPDBCEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPDBCEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The WKUP1 input pin is not connected to the low-power debouncer."]
    #[inline]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(LPDBCEN1W::NOT_ENABLE)
    }
    #[doc = "The WKUP1 input pin is connected to the low-power debouncer and forces a system wake-up."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPDBCEN1W::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPDBCCLR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCCLRW {
    #[doc = "A low-power debounce event does not create an immediate clear on the first half of GPBR registers."]
    NOT_ENABLE,
    #[doc = "A low-power debounce event on WKUP0 or WKUP1 generates an immediate clear on the first half of GPBR registers."]
    ENABLE,
}
impl LPDBCCLRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPDBCCLRW::NOT_ENABLE => false,
            LPDBCCLRW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPDBCCLRW<'a> {
    w: &'a mut W,
}
impl<'a> _LPDBCCLRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPDBCCLRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A low-power debounce event does not create an immediate clear on the first half of GPBR registers."]
    #[inline]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(LPDBCCLRW::NOT_ENABLE)
    }
    #[doc = "A low-power debounce event on WKUP0 or WKUP1 generates an immediate clear on the first half of GPBR registers."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(LPDBCCLRW::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WKUPDBC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPDBCW {
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    IMMEDIATE,
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    _3_SLCK,
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    _32_SLCK,
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    _512_SLCK,
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    _4096_SLCK,
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    _32768_SLCK,
}
impl WKUPDBCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WKUPDBCW::IMMEDIATE => 0,
            WKUPDBCW::_3_SLCK => 1,
            WKUPDBCW::_32_SLCK => 2,
            WKUPDBCW::_512_SLCK => 3,
            WKUPDBCW::_4096_SLCK => 4,
            WKUPDBCW::_32768_SLCK => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WKUPDBCW<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPDBCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WKUPDBCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate, no debouncing, detected active at least on one Slow Clock edge."]
    #[inline]
    pub fn immediate(self) -> &'a mut W {
        self.variant(WKUPDBCW::IMMEDIATE)
    }
    #[doc = "WKUPx shall be in its active state for at least 3 SLCK periods"]
    #[inline]
    pub fn _3_slck(self) -> &'a mut W {
        self.variant(WKUPDBCW::_3_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 32 SLCK periods"]
    #[inline]
    pub fn _32_slck(self) -> &'a mut W {
        self.variant(WKUPDBCW::_32_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 512 SLCK periods"]
    #[inline]
    pub fn _512_slck(self) -> &'a mut W {
        self.variant(WKUPDBCW::_512_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 4,096 SLCK periods"]
    #[inline]
    pub fn _4096_slck(self) -> &'a mut W {
        self.variant(WKUPDBCW::_4096_SLCK)
    }
    #[doc = "WKUPx shall be in its active state for at least 32,768 SLCK periods"]
    #[inline]
    pub fn _32768_slck(self) -> &'a mut W {
        self.variant(WKUPDBCW::_32768_SLCK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPDBC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCW {
    #[doc = "Disable the low-power debouncers."]
    DISABLE,
    #[doc = "WKUP0/1 in active state for at least 2 RTCOUTx clock periods"]
    _2_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 3 RTCOUTx clock periods"]
    _3_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 4 RTCOUTx clock periods"]
    _4_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 5 RTCOUTx clock periods"]
    _5_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 6 RTCOUTx clock periods"]
    _6_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 7 RTCOUTx clock periods"]
    _7_RTCOUT,
    #[doc = "WKUP0/1 in active state for at least 8 RTCOUTx clock periods"]
    _8_RTCOUT,
}
impl LPDBCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPDBCW::DISABLE => 0,
            LPDBCW::_2_RTCOUT => 1,
            LPDBCW::_3_RTCOUT => 2,
            LPDBCW::_4_RTCOUT => 3,
            LPDBCW::_5_RTCOUT => 4,
            LPDBCW::_6_RTCOUT => 5,
            LPDBCW::_7_RTCOUT => 6,
            LPDBCW::_8_RTCOUT => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPDBCW<'a> {
    w: &'a mut W,
}
impl<'a> _LPDBCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPDBCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disable the low-power debouncers."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(LPDBCW::DISABLE)
    }
    #[doc = "WKUP0/1 in active state for at least 2 RTCOUTx clock periods"]
    #[inline]
    pub fn _2_rtcout(self) -> &'a mut W {
        self.variant(LPDBCW::_2_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 3 RTCOUTx clock periods"]
    #[inline]
    pub fn _3_rtcout(self) -> &'a mut W {
        self.variant(LPDBCW::_3_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 4 RTCOUTx clock periods"]
    #[inline]
    pub fn _4_rtcout(self) -> &'a mut W {
        self.variant(LPDBCW::_4_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 5 RTCOUTx clock periods"]
    #[inline]
    pub fn _5_rtcout(self) -> &'a mut W {
        self.variant(LPDBCW::_5_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 6 RTCOUTx clock periods"]
    #[inline]
    pub fn _6_rtcout(self) -> &'a mut W {
        self.variant(LPDBCW::_6_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 7 RTCOUTx clock periods"]
    #[inline]
    pub fn _7_rtcout(self) -> &'a mut W {
        self.variant(LPDBCW::_7_RTCOUT)
    }
    #[doc = "WKUP0/1 in active state for at least 8 RTCOUTx clock periods"]
    #[inline]
    pub fn _8_rtcout(self) -> &'a mut W {
        self.variant(LPDBCW::_8_RTCOUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline]
    pub fn smen(&self) -> SMENR {
        SMENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Real-time Timer Wake-up Enable"]
    #[inline]
    pub fn rtten(&self) -> RTTENR {
        RTTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Real-time Clock Wake-up Enable"]
    #[inline]
    pub fn rtcen(&self) -> RTCENR {
        RTCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Low-power Debouncer Enable WKUP0"]
    #[inline]
    pub fn lpdbcen0(&self) -> LPDBCEN0R {
        LPDBCEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Low-power Debouncer Enable WKUP1"]
    #[inline]
    pub fn lpdbcen1(&self) -> LPDBCEN1R {
        LPDBCEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Low-power Debouncer Clear"]
    #[inline]
    pub fn lpdbcclr(&self) -> LPDBCCLRR {
        LPDBCCLRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline]
    pub fn wkupdbc(&self) -> WKUPDBCR {
        WKUPDBCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Low-power Debouncer Period"]
    #[inline]
    pub fn lpdbc(&self) -> LPDBCR {
        LPDBCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
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
    #[doc = "Bit 1 - Supply Monitor Wake-up Enable"]
    #[inline]
    pub fn smen(&mut self) -> _SMENW {
        _SMENW { w: self }
    }
    #[doc = "Bit 2 - Real-time Timer Wake-up Enable"]
    #[inline]
    pub fn rtten(&mut self) -> _RTTENW {
        _RTTENW { w: self }
    }
    #[doc = "Bit 3 - Real-time Clock Wake-up Enable"]
    #[inline]
    pub fn rtcen(&mut self) -> _RTCENW {
        _RTCENW { w: self }
    }
    #[doc = "Bit 5 - Low-power Debouncer Enable WKUP0"]
    #[inline]
    pub fn lpdbcen0(&mut self) -> _LPDBCEN0W {
        _LPDBCEN0W { w: self }
    }
    #[doc = "Bit 6 - Low-power Debouncer Enable WKUP1"]
    #[inline]
    pub fn lpdbcen1(&mut self) -> _LPDBCEN1W {
        _LPDBCEN1W { w: self }
    }
    #[doc = "Bit 7 - Low-power Debouncer Clear"]
    #[inline]
    pub fn lpdbcclr(&mut self) -> _LPDBCCLRW {
        _LPDBCCLRW { w: self }
    }
    #[doc = "Bits 12:14 - Wake-up Inputs Debouncer Period"]
    #[inline]
    pub fn wkupdbc(&mut self) -> _WKUPDBCW {
        _WKUPDBCW { w: self }
    }
    #[doc = "Bits 16:18 - Low-power Debouncer Period"]
    #[inline]
    pub fn lpdbc(&mut self) -> _LPDBCW {
        _LPDBCW { w: self }
    }
}
