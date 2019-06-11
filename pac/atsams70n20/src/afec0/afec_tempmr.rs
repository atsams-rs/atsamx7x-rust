#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFEC_TEMPMR {
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
pub struct RTCTR {
    bits: bool,
}
impl RTCTR {
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
#[doc = "Possible values of the field `TEMPCMPMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMPCMPMODR {
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    LOW,
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    HIGH,
    #[doc = "Generates an event when the converted data is in the comparison window."]
    IN,
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    OUT,
}
impl TEMPCMPMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TEMPCMPMODR::LOW => 0,
            TEMPCMPMODR::HIGH => 1,
            TEMPCMPMODR::IN => 2,
            TEMPCMPMODR::OUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TEMPCMPMODR {
        match value {
            0 => TEMPCMPMODR::LOW,
            1 => TEMPCMPMODR::HIGH,
            2 => TEMPCMPMODR::IN,
            3 => TEMPCMPMODR::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == TEMPCMPMODR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == TEMPCMPMODR::HIGH
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == TEMPCMPMODR::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == TEMPCMPMODR::OUT
    }
}
#[doc = r" Proxy"]
pub struct _RTCTW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCTW<'a> {
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
#[doc = "Values that can be written to the field `TEMPCMPMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEMPCMPMODW {
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    LOW,
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    HIGH,
    #[doc = "Generates an event when the converted data is in the comparison window."]
    IN,
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    OUT,
}
impl TEMPCMPMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TEMPCMPMODW::LOW => 0,
            TEMPCMPMODW::HIGH => 1,
            TEMPCMPMODW::IN => 2,
            TEMPCMPMODW::OUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEMPCMPMODW<'a> {
    w: &'a mut W,
}
impl<'a> _TEMPCMPMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEMPCMPMODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(TEMPCMPMODW::LOW)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(TEMPCMPMODW::HIGH)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline]
    pub fn in_(self) -> &'a mut W {
        self.variant(TEMPCMPMODW::IN)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(TEMPCMPMODW::OUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Temperature Sensor RTC Trigger Mode"]
    #[inline]
    pub fn rtct(&self) -> RTCTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RTCTR { bits }
    }
    #[doc = "Bits 4:5 - Temperature Comparison Mode"]
    #[inline]
    pub fn tempcmpmod(&self) -> TEMPCMPMODR {
        TEMPCMPMODR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Temperature Sensor RTC Trigger Mode"]
    #[inline]
    pub fn rtct(&mut self) -> _RTCTW {
        _RTCTW { w: self }
    }
    #[doc = "Bits 4:5 - Temperature Comparison Mode"]
    #[inline]
    pub fn tempcmpmod(&mut self) -> _TEMPCMPMODW {
        _TEMPCMPMODW { w: self }
    }
}
