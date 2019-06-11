#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DACC_MR {
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
#[doc = "Possible values of the field `MAXS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXS0R {
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    TRIG_EVENT,
    #[doc = "Max speed mode enabled."]
    MAXIMUM,
}
impl MAXS0R {
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
            MAXS0R::TRIG_EVENT => false,
            MAXS0R::MAXIMUM => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAXS0R {
        match value {
            false => MAXS0R::TRIG_EVENT,
            true => MAXS0R::MAXIMUM,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG_EVENT`"]
    #[inline]
    pub fn is_trig_event(&self) -> bool {
        *self == MAXS0R::TRIG_EVENT
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline]
    pub fn is_maximum(&self) -> bool {
        *self == MAXS0R::MAXIMUM
    }
}
#[doc = "Possible values of the field `MAXS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXS1R {
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    TRIG_EVENT,
    #[doc = "Max speed mode enabled."]
    MAXIMUM,
}
impl MAXS1R {
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
            MAXS1R::TRIG_EVENT => false,
            MAXS1R::MAXIMUM => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAXS1R {
        match value {
            false => MAXS1R::TRIG_EVENT,
            true => MAXS1R::MAXIMUM,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG_EVENT`"]
    #[inline]
    pub fn is_trig_event(&self) -> bool {
        *self == MAXS1R::TRIG_EVENT
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline]
    pub fn is_maximum(&self) -> bool {
        *self == MAXS1R::MAXIMUM
    }
}
#[doc = "Possible values of the field `WORD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WORDR {
    #[doc = "One data to convert is written to the FIFO per access to DACC."]
    DISABLED,
    #[doc = "Two data to convert are written to the FIFO per access to DACC (reduces the number of requests to DMA and the number of system bus accesses)."]
    ENABLED,
}
impl WORDR {
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
            WORDR::DISABLED => false,
            WORDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WORDR {
        match value {
            false => WORDR::DISABLED,
            true => WORDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WORDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WORDR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct ZEROR {
    bits: bool,
}
impl ZEROR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `DIFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFFR {
    #[doc = "DAC0 and DAC1 are single-ended outputs."]
    DISABLED,
    #[doc = "DACP and DACN are differential outputs. The differential level is configured by the channel 0 value."]
    ENABLED,
}
impl DIFFR {
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
            DIFFR::DISABLED => false,
            DIFFR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIFFR {
        match value {
            false => DIFFR::DISABLED,
            true => DIFFR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DIFFR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DIFFR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct PRESCALERR {
    bits: u8,
}
impl PRESCALERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `MAXS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXS0W {
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    TRIG_EVENT,
    #[doc = "Max speed mode enabled."]
    MAXIMUM,
}
impl MAXS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAXS0W::TRIG_EVENT => false,
            MAXS0W::MAXIMUM => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAXS0W<'a> {
    w: &'a mut W,
}
impl<'a> _MAXS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAXS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    #[inline]
    pub fn trig_event(self) -> &'a mut W {
        self.variant(MAXS0W::TRIG_EVENT)
    }
    #[doc = "Max speed mode enabled."]
    #[inline]
    pub fn maximum(self) -> &'a mut W {
        self.variant(MAXS0W::MAXIMUM)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MAXS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXS1W {
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    TRIG_EVENT,
    #[doc = "Max speed mode enabled."]
    MAXIMUM,
}
impl MAXS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAXS1W::TRIG_EVENT => false,
            MAXS1W::MAXIMUM => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAXS1W<'a> {
    w: &'a mut W,
}
impl<'a> _MAXS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAXS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    #[inline]
    pub fn trig_event(self) -> &'a mut W {
        self.variant(MAXS1W::TRIG_EVENT)
    }
    #[doc = "Max speed mode enabled."]
    #[inline]
    pub fn maximum(self) -> &'a mut W {
        self.variant(MAXS1W::MAXIMUM)
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
#[doc = "Values that can be written to the field `WORD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WORDW {
    #[doc = "One data to convert is written to the FIFO per access to DACC."]
    DISABLED,
    #[doc = "Two data to convert are written to the FIFO per access to DACC (reduces the number of requests to DMA and the number of system bus accesses)."]
    ENABLED,
}
impl WORDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WORDW::DISABLED => false,
            WORDW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WORDW<'a> {
    w: &'a mut W,
}
impl<'a> _WORDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WORDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "One data to convert is written to the FIFO per access to DACC."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WORDW::DISABLED)
    }
    #[doc = "Two data to convert are written to the FIFO per access to DACC (reduces the number of requests to DMA and the number of system bus accesses)."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WORDW::ENABLED)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ZEROW<'a> {
    w: &'a mut W,
}
impl<'a> _ZEROW<'a> {
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
#[doc = "Values that can be written to the field `DIFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFFW {
    #[doc = "DAC0 and DAC1 are single-ended outputs."]
    DISABLED,
    #[doc = "DACP and DACN are differential outputs. The differential level is configured by the channel 0 value."]
    ENABLED,
}
impl DIFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIFFW::DISABLED => false,
            DIFFW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DIFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC0 and DAC1 are single-ended outputs."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIFFW::DISABLED)
    }
    #[doc = "DACP and DACN are differential outputs. The differential level is configured by the channel 0 value."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIFFW::ENABLED)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRESCALERW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Max Speed Mode for Channel 0"]
    #[inline]
    pub fn maxs0(&self) -> MAXS0R {
        MAXS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Max Speed Mode for Channel 1"]
    #[inline]
    pub fn maxs1(&self) -> MAXS1R {
        MAXS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Word Transfer Mode"]
    #[inline]
    pub fn word(&self) -> WORDR {
        WORDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Must always be written to 0."]
    #[inline]
    pub fn zero(&self) -> ZEROR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ZEROR { bits }
    }
    #[doc = "Bit 23 - Differential Mode"]
    #[inline]
    pub fn diff(&self) -> DIFFR {
        DIFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:27 - Peripheral Clock to DAC Clock Ratio"]
    #[inline]
    pub fn prescaler(&self) -> PRESCALERR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRESCALERR { bits }
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
    #[doc = "Bit 0 - Max Speed Mode for Channel 0"]
    #[inline]
    pub fn maxs0(&mut self) -> _MAXS0W {
        _MAXS0W { w: self }
    }
    #[doc = "Bit 1 - Max Speed Mode for Channel 1"]
    #[inline]
    pub fn maxs1(&mut self) -> _MAXS1W {
        _MAXS1W { w: self }
    }
    #[doc = "Bit 4 - Word Transfer Mode"]
    #[inline]
    pub fn word(&mut self) -> _WORDW {
        _WORDW { w: self }
    }
    #[doc = "Bit 5 - Must always be written to 0."]
    #[inline]
    pub fn zero(&mut self) -> _ZEROW {
        _ZEROW { w: self }
    }
    #[doc = "Bit 23 - Differential Mode"]
    #[inline]
    pub fn diff(&mut self) -> _DIFFW {
        _DIFFW { w: self }
    }
    #[doc = "Bits 24:27 - Peripheral Clock to DAC Clock Ratio"]
    #[inline]
    pub fn prescaler(&mut self) -> _PRESCALERW {
        _PRESCALERW { w: self }
    }
}
