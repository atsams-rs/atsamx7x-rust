#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFEC_TEMPMR {
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
#[doc = r"Reader of the field"]
pub type RTCT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RTCTW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCTW<'a> {
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
impl crate::ToBits<u8> for TEMPCMPMODR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TEMPCMPMODR::LOW => 0,
            TEMPCMPMODR::HIGH => 1,
            TEMPCMPMODR::IN => 2,
            TEMPCMPMODR::OUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TEMPCMPMOD_R = crate::FR<u8, TEMPCMPMODR>;
impl TEMPCMPMOD_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TEMPCMPMODR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TEMPCMPMODR::HIGH
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == TEMPCMPMODR::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == TEMPCMPMODR::OUT
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TEMPCMPMODW::LOW => 0,
            TEMPCMPMODW::HIGH => 1,
            TEMPCMPMODW::IN => 2,
            TEMPCMPMODW::OUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TEMPCMPMODW<'a> {
    w: &'a mut W,
}
impl<'a> _TEMPCMPMODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TEMPCMPMODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(TEMPCMPMODW::LOW)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(TEMPCMPMODW::HIGH)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(TEMPCMPMODW::IN)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(TEMPCMPMODW::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Temperature Sensor RTC Trigger Mode"]
    #[inline(always)]
    pub fn rtct(&self) -> RTCT_R {
        RTCT_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Temperature Comparison Mode"]
    #[inline(always)]
    pub fn tempcmpmod(&self) -> TEMPCMPMOD_R {
        TEMPCMPMOD_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Temperature Sensor RTC Trigger Mode"]
    #[inline(always)]
    pub fn rtct(&mut self) -> _RTCTW {
        _RTCTW { w: self }
    }
    #[doc = "Bits 4:5 - Temperature Comparison Mode"]
    #[inline(always)]
    pub fn tempcmpmod(&mut self) -> _TEMPCMPMODW {
        _TEMPCMPMODW { w: self }
    }
}
