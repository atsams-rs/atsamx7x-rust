#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::UART_CMPR {
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
pub struct VAL1R {
    bits: u8,
}
impl VAL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CMPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPMODER {
    #[doc = "Any character is received and comparison function drives CMP flag."]
    FLAG_ONLY,
    #[doc = "Comparison condition must be met to start reception."]
    START_CONDITION,
}
impl CMPMODER {
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
            CMPMODER::FLAG_ONLY => false,
            CMPMODER::START_CONDITION => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMPMODER {
        match value {
            false => CMPMODER::FLAG_ONLY,
            true => CMPMODER::START_CONDITION,
        }
    }
    #[doc = "Checks if the value of the field is `FLAG_ONLY`"]
    #[inline]
    pub fn is_flag_only(&self) -> bool {
        *self == CMPMODER::FLAG_ONLY
    }
    #[doc = "Checks if the value of the field is `START_CONDITION`"]
    #[inline]
    pub fn is_start_condition(&self) -> bool {
        *self == CMPMODER::START_CONDITION
    }
}
#[doc = r" Value of the field"]
pub struct CMPPARR {
    bits: bool,
}
impl CMPPARR {
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
pub struct VAL2R {
    bits: u8,
}
impl VAL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _VAL1W<'a> {
    w: &'a mut W,
}
impl<'a> _VAL1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPMODEW {
    #[doc = "Any character is received and comparison function drives CMP flag."]
    FLAG_ONLY,
    #[doc = "Comparison condition must be met to start reception."]
    START_CONDITION,
}
impl CMPMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMPMODEW::FLAG_ONLY => false,
            CMPMODEW::START_CONDITION => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMPMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Any character is received and comparison function drives CMP flag."]
    #[inline]
    pub fn flag_only(self) -> &'a mut W {
        self.variant(CMPMODEW::FLAG_ONLY)
    }
    #[doc = "Comparison condition must be met to start reception."]
    #[inline]
    pub fn start_condition(self) -> &'a mut W {
        self.variant(CMPMODEW::START_CONDITION)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMPPARW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPPARW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VAL2W<'a> {
    w: &'a mut W,
}
impl<'a> _VAL2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - First Comparison Value for Received Character"]
    #[inline]
    pub fn val1(&self) -> VAL1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VAL1R { bits }
    }
    #[doc = "Bit 12 - Comparison Mode"]
    #[inline]
    pub fn cmpmode(&self) -> CMPMODER {
        CMPMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Compare Parity"]
    #[inline]
    pub fn cmppar(&self) -> CMPPARR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CMPPARR { bits }
    }
    #[doc = "Bits 16:23 - Second Comparison Value for Received Character"]
    #[inline]
    pub fn val2(&self) -> VAL2R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VAL2R { bits }
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
    #[doc = "Bits 0:7 - First Comparison Value for Received Character"]
    #[inline]
    pub fn val1(&mut self) -> _VAL1W {
        _VAL1W { w: self }
    }
    #[doc = "Bit 12 - Comparison Mode"]
    #[inline]
    pub fn cmpmode(&mut self) -> _CMPMODEW {
        _CMPMODEW { w: self }
    }
    #[doc = "Bit 14 - Compare Parity"]
    #[inline]
    pub fn cmppar(&mut self) -> _CMPPARW {
        _CMPPARW { w: self }
    }
    #[doc = "Bits 16:23 - Second Comparison Value for Received Character"]
    #[inline]
    pub fn val2(&mut self) -> _VAL2W {
        _VAL2W { w: self }
    }
}
