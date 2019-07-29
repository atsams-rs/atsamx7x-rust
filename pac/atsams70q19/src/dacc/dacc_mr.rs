#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DACC_MR {
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
#[doc = "Possible values of the field `MAXS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXS0R {
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    TRIG_EVENT,
    #[doc = "Max speed mode enabled."]
    MAXIMUM,
}
impl crate::ToBits<bool> for MAXS0R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MAXS0R::TRIG_EVENT => false,
            MAXS0R::MAXIMUM => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MAXS0_R = crate::FR<bool, MAXS0R>;
impl MAXS0_R {
    #[doc = "Checks if the value of the field is `TRIG_EVENT`"]
    #[inline(always)]
    pub fn is_trig_event(&self) -> bool {
        *self == MAXS0R::TRIG_EVENT
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == MAXS0R::MAXIMUM
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
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MAXS0W::TRIG_EVENT => false,
            MAXS0W::MAXIMUM => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MAXS0W<'a> {
    w: &'a mut W,
}
impl<'a> _MAXS0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAXS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    #[inline(always)]
    pub fn trig_event(self) -> &'a mut W {
        self.variant(MAXS0W::TRIG_EVENT)
    }
    #[doc = "Max speed mode enabled."]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(MAXS0W::MAXIMUM)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
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
impl crate::ToBits<bool> for MAXS1R {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MAXS1R::TRIG_EVENT => false,
            MAXS1R::MAXIMUM => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MAXS1_R = crate::FR<bool, MAXS1R>;
impl MAXS1_R {
    #[doc = "Checks if the value of the field is `TRIG_EVENT`"]
    #[inline(always)]
    pub fn is_trig_event(&self) -> bool {
        *self == MAXS1R::TRIG_EVENT
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == MAXS1R::MAXIMUM
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
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MAXS1W::TRIG_EVENT => false,
            MAXS1W::MAXIMUM => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MAXS1W<'a> {
    w: &'a mut W,
}
impl<'a> _MAXS1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAXS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    #[inline(always)]
    pub fn trig_event(self) -> &'a mut W {
        self.variant(MAXS1W::TRIG_EVENT)
    }
    #[doc = "Max speed mode enabled."]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(MAXS1W::MAXIMUM)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
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
impl crate::ToBits<bool> for WORDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WORDR::DISABLED => false,
            WORDR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WORD_R = crate::FR<bool, WORDR>;
impl WORD_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WORDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WORDR::ENABLED
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
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WORDW::DISABLED => false,
            WORDW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WORDW<'a> {
    w: &'a mut W,
}
impl<'a> _WORDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WORDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "One data to convert is written to the FIFO per access to DACC."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WORDW::DISABLED)
    }
    #[doc = "Two data to convert are written to the FIFO per access to DACC (reduces the number of requests to DMA and the number of system bus accesses)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WORDW::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ZERO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ZEROW<'a> {
    w: &'a mut W,
}
impl<'a> _ZEROW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
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
impl crate::ToBits<bool> for DIFFR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DIFFR::DISABLED => false,
            DIFFR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DIFF_R = crate::FR<bool, DIFFR>;
impl DIFF_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIFFR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIFFR::ENABLED
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
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DIFFW::DISABLED => false,
            DIFFW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DIFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DIFFW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC0 and DAC1 are single-ended outputs."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIFFW::DISABLED)
    }
    #[doc = "DACP and DACN are differential outputs. The differential level is configured by the channel 0 value."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIFFW::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PRESCALER_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRESCALERW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALERW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Max Speed Mode for Channel 0"]
    #[inline(always)]
    pub fn maxs0(&self) -> MAXS0_R {
        MAXS0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Max Speed Mode for Channel 1"]
    #[inline(always)]
    pub fn maxs1(&self) -> MAXS1_R {
        MAXS1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Word Transfer Mode"]
    #[inline(always)]
    pub fn word(&self) -> WORD_R {
        WORD_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Must always be written to 0."]
    #[inline(always)]
    pub fn zero(&self) -> ZERO_R {
        ZERO_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Peripheral Clock to DAC Clock Ratio"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits() >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Max Speed Mode for Channel 0"]
    #[inline(always)]
    pub fn maxs0(&mut self) -> _MAXS0W {
        _MAXS0W { w: self }
    }
    #[doc = "Bit 1 - Max Speed Mode for Channel 1"]
    #[inline(always)]
    pub fn maxs1(&mut self) -> _MAXS1W {
        _MAXS1W { w: self }
    }
    #[doc = "Bit 4 - Word Transfer Mode"]
    #[inline(always)]
    pub fn word(&mut self) -> _WORDW {
        _WORDW { w: self }
    }
    #[doc = "Bit 5 - Must always be written to 0."]
    #[inline(always)]
    pub fn zero(&mut self) -> _ZEROW {
        _ZEROW { w: self }
    }
    #[doc = "Bit 23 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&mut self) -> _DIFFW {
        _DIFFW { w: self }
    }
    #[doc = "Bits 24:27 - Peripheral Clock to DAC Clock Ratio"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> _PRESCALERW {
        _PRESCALERW { w: self }
    }
}
