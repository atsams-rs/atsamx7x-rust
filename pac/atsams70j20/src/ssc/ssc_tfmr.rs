#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSC_TFMR {
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
#[doc = r" Value of the field"]
pub struct DATLENR {
    bits: u8,
}
impl DATLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATDEFR {
    bits: bool,
}
impl DATDEFR {
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
#[doc = r" Value of the field"]
pub struct MSBFR {
    bits: bool,
}
impl MSBFR {
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
#[doc = r" Value of the field"]
pub struct DATNBR {
    bits: u8,
}
impl DATNBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FSLENR {
    bits: u8,
}
impl FSLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `FSOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSOSR {
    #[doc = "None, TF pin is an input"]
    NONE,
    #[doc = "Negative Pulse, TF pin is an output"]
    NEGATIVE,
    #[doc = "Positive Pulse, TF pin is an output"]
    POSITIVE,
    #[doc = "Driven Low during data transfer"]
    LOW,
    #[doc = "Driven High during data transfer"]
    HIGH,
    #[doc = "Toggling at each start of data transfer"]
    TOGGLING,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl FSOSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FSOSR::NONE => 0,
            FSOSR::NEGATIVE => 1,
            FSOSR::POSITIVE => 2,
            FSOSR::LOW => 3,
            FSOSR::HIGH => 4,
            FSOSR::TOGGLING => 5,
            FSOSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FSOSR {
        match value {
            0 => FSOSR::NONE,
            1 => FSOSR::NEGATIVE,
            2 => FSOSR::POSITIVE,
            3 => FSOSR::LOW,
            4 => FSOSR::HIGH,
            5 => FSOSR::TOGGLING,
            i => FSOSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == FSOSR::NONE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline]
    pub fn is_negative(&self) -> bool {
        *self == FSOSR::NEGATIVE
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline]
    pub fn is_positive(&self) -> bool {
        *self == FSOSR::POSITIVE
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == FSOSR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == FSOSR::HIGH
    }
    #[doc = "Checks if the value of the field is `TOGGLING`"]
    #[inline]
    pub fn is_toggling(&self) -> bool {
        *self == FSOSR::TOGGLING
    }
}
#[doc = r" Value of the field"]
pub struct FSDENR {
    bits: bool,
}
impl FSDENR {
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
#[doc = "Possible values of the field `FSEDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSEDGER {
    #[doc = "Positive Edge Detection"]
    POSITIVE,
    #[doc = "Negative Edge Detection"]
    NEGATIVE,
}
impl FSEDGER {
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
            FSEDGER::POSITIVE => false,
            FSEDGER::NEGATIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FSEDGER {
        match value {
            false => FSEDGER::POSITIVE,
            true => FSEDGER::NEGATIVE,
        }
    }
    #[doc = "Checks if the value of the field is `POSITIVE`"]
    #[inline]
    pub fn is_positive(&self) -> bool {
        *self == FSEDGER::POSITIVE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE`"]
    #[inline]
    pub fn is_negative(&self) -> bool {
        *self == FSEDGER::NEGATIVE
    }
}
#[doc = r" Value of the field"]
pub struct FSLEN_EXTR {
    bits: u8,
}
impl FSLEN_EXTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DATLENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATLENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATDEFW<'a> {
    w: &'a mut W,
}
impl<'a> _DATDEFW<'a> {
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
#[doc = r" Proxy"]
pub struct _MSBFW<'a> {
    w: &'a mut W,
}
impl<'a> _MSBFW<'a> {
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
#[doc = r" Proxy"]
pub struct _DATNBW<'a> {
    w: &'a mut W,
}
impl<'a> _DATNBW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSLENW<'a> {
    w: &'a mut W,
}
impl<'a> _FSLENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FSOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSOSW {
    #[doc = "None, TF pin is an input"]
    NONE,
    #[doc = "Negative Pulse, TF pin is an output"]
    NEGATIVE,
    #[doc = "Positive Pulse, TF pin is an output"]
    POSITIVE,
    #[doc = "Driven Low during data transfer"]
    LOW,
    #[doc = "Driven High during data transfer"]
    HIGH,
    #[doc = "Toggling at each start of data transfer"]
    TOGGLING,
}
impl FSOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FSOSW::NONE => 0,
            FSOSW::NEGATIVE => 1,
            FSOSW::POSITIVE => 2,
            FSOSW::LOW => 3,
            FSOSW::HIGH => 4,
            FSOSW::TOGGLING => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSOSW<'a> {
    w: &'a mut W,
}
impl<'a> _FSOSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSOSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "None, TF pin is an input"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(FSOSW::NONE)
    }
    #[doc = "Negative Pulse, TF pin is an output"]
    #[inline]
    pub fn negative(self) -> &'a mut W {
        self.variant(FSOSW::NEGATIVE)
    }
    #[doc = "Positive Pulse, TF pin is an output"]
    #[inline]
    pub fn positive(self) -> &'a mut W {
        self.variant(FSOSW::POSITIVE)
    }
    #[doc = "Driven Low during data transfer"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(FSOSW::LOW)
    }
    #[doc = "Driven High during data transfer"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(FSOSW::HIGH)
    }
    #[doc = "Toggling at each start of data transfer"]
    #[inline]
    pub fn toggling(self) -> &'a mut W {
        self.variant(FSOSW::TOGGLING)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSDENW<'a> {
    w: &'a mut W,
}
impl<'a> _FSDENW<'a> {
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
#[doc = "Values that can be written to the field `FSEDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSEDGEW {
    #[doc = "Positive Edge Detection"]
    POSITIVE,
    #[doc = "Negative Edge Detection"]
    NEGATIVE,
}
impl FSEDGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FSEDGEW::POSITIVE => false,
            FSEDGEW::NEGATIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSEDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _FSEDGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSEDGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Positive Edge Detection"]
    #[inline]
    pub fn positive(self) -> &'a mut W {
        self.variant(FSEDGEW::POSITIVE)
    }
    #[doc = "Negative Edge Detection"]
    #[inline]
    pub fn negative(self) -> &'a mut W {
        self.variant(FSEDGEW::NEGATIVE)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FSLEN_EXTW<'a> {
    w: &'a mut W,
}
impl<'a> _FSLEN_EXTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:4 - Data Length"]
    #[inline]
    pub fn datlen(&self) -> DATLENR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATLENR { bits }
    }
    #[doc = "Bit 5 - Data Default Value"]
    #[inline]
    pub fn datdef(&self) -> DATDEFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATDEFR { bits }
    }
    #[doc = "Bit 7 - Most Significant Bit First"]
    #[inline]
    pub fn msbf(&self) -> MSBFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MSBFR { bits }
    }
    #[doc = "Bits 8:11 - Data Number per Frame"]
    #[inline]
    pub fn datnb(&self) -> DATNBR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATNBR { bits }
    }
    #[doc = "Bits 16:19 - Transmit Frame Sync Length"]
    #[inline]
    pub fn fslen(&self) -> FSLENR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FSLENR { bits }
    }
    #[doc = "Bits 20:22 - Transmit Frame Sync Output Selection"]
    #[inline]
    pub fn fsos(&self) -> FSOSR {
        FSOSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - Frame Sync Data Enable"]
    #[inline]
    pub fn fsden(&self) -> FSDENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FSDENR { bits }
    }
    #[doc = "Bit 24 - Frame Sync Edge Detection"]
    #[inline]
    pub fn fsedge(&self) -> FSEDGER {
        FSEDGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 28:31 - FSLEN Field Extension"]
    #[inline]
    pub fn fslen_ext(&self) -> FSLEN_EXTR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FSLEN_EXTR { bits }
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
    #[doc = "Bits 0:4 - Data Length"]
    #[inline]
    pub fn datlen(&mut self) -> _DATLENW {
        _DATLENW { w: self }
    }
    #[doc = "Bit 5 - Data Default Value"]
    #[inline]
    pub fn datdef(&mut self) -> _DATDEFW {
        _DATDEFW { w: self }
    }
    #[doc = "Bit 7 - Most Significant Bit First"]
    #[inline]
    pub fn msbf(&mut self) -> _MSBFW {
        _MSBFW { w: self }
    }
    #[doc = "Bits 8:11 - Data Number per Frame"]
    #[inline]
    pub fn datnb(&mut self) -> _DATNBW {
        _DATNBW { w: self }
    }
    #[doc = "Bits 16:19 - Transmit Frame Sync Length"]
    #[inline]
    pub fn fslen(&mut self) -> _FSLENW {
        _FSLENW { w: self }
    }
    #[doc = "Bits 20:22 - Transmit Frame Sync Output Selection"]
    #[inline]
    pub fn fsos(&mut self) -> _FSOSW {
        _FSOSW { w: self }
    }
    #[doc = "Bit 23 - Frame Sync Data Enable"]
    #[inline]
    pub fn fsden(&mut self) -> _FSDENW {
        _FSDENW { w: self }
    }
    #[doc = "Bit 24 - Frame Sync Edge Detection"]
    #[inline]
    pub fn fsedge(&mut self) -> _FSEDGEW {
        _FSEDGEW { w: self }
    }
    #[doc = "Bits 28:31 - FSLEN Field Extension"]
    #[inline]
    pub fn fslen_ext(&mut self) -> _FSLEN_EXTW {
        _FSLEN_EXTW { w: self }
    }
}
