#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TC_EMR {
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
#[doc = "Possible values of the field `TRIGSRCA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGSRCAR {
    #[doc = "The trigger/capture input A is driven by external pin TIOAx"]
    EXTERNAL_TIOAX,
    #[doc = "The trigger/capture input A is driven internally by PWMx"]
    PWMX,
}
impl crate::ToBits<u8> for TRIGSRCAR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TRIGSRCAR::EXTERNAL_TIOAX => 0,
            TRIGSRCAR::PWMX => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TRIGSRCA_R = crate::FR<u8, TRIGSRCAR>;
impl TRIGSRCA_R {
    #[doc = "Checks if the value of the field is `EXTERNAL_TIOAX`"]
    #[inline(always)]
    pub fn is_external_tioax(&self) -> bool {
        *self == TRIGSRCAR::EXTERNAL_TIOAX
    }
    #[doc = "Checks if the value of the field is `PWMX`"]
    #[inline(always)]
    pub fn is_pwmx(&self) -> bool {
        *self == TRIGSRCAR::PWMX
    }
}
#[doc = "Values that can be written to the field `TRIGSRCA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGSRCAW {
    #[doc = "The trigger/capture input A is driven by external pin TIOAx"]
    EXTERNAL_TIOAX,
    #[doc = "The trigger/capture input A is driven internally by PWMx"]
    PWMX,
}
impl TRIGSRCAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRIGSRCAW::EXTERNAL_TIOAX => 0,
            TRIGSRCAW::PWMX => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TRIGSRCAW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGSRCAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGSRCAW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The trigger/capture input A is driven by external pin TIOAx"]
    #[inline(always)]
    pub fn external_tioax(self) -> &'a mut W {
        self.variant(TRIGSRCAW::EXTERNAL_TIOAX)
    }
    #[doc = "The trigger/capture input A is driven internally by PWMx"]
    #[inline(always)]
    pub fn pwmx(self) -> &'a mut W {
        self.variant(TRIGSRCAW::PWMX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `TRIGSRCB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGSRCBR {
    #[doc = "The trigger/capture input B is driven by external pin TIOBx"]
    EXTERNAL_TIOBX,
    #[doc = "For TC0 to TC10: The trigger/capture input B is driven internally by the comparator output (see Figure 7-16) of the PWMx.For TC11: The trigger/capture input B is driven internally by the GTSUCOMP signal of the Ethernet MAC (GMAC)."]
    PWMX,
}
impl crate::ToBits<u8> for TRIGSRCBR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TRIGSRCBR::EXTERNAL_TIOBX => 0,
            TRIGSRCBR::PWMX => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TRIGSRCB_R = crate::FR<u8, TRIGSRCBR>;
impl TRIGSRCB_R {
    #[doc = "Checks if the value of the field is `EXTERNAL_TIOBX`"]
    #[inline(always)]
    pub fn is_external_tiobx(&self) -> bool {
        *self == TRIGSRCBR::EXTERNAL_TIOBX
    }
    #[doc = "Checks if the value of the field is `PWMX`"]
    #[inline(always)]
    pub fn is_pwmx(&self) -> bool {
        *self == TRIGSRCBR::PWMX
    }
}
#[doc = "Values that can be written to the field `TRIGSRCB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGSRCBW {
    #[doc = "The trigger/capture input B is driven by external pin TIOBx"]
    EXTERNAL_TIOBX,
    #[doc = "For TC0 to TC10: The trigger/capture input B is driven internally by the comparator output (see Figure 7-16) of the PWMx.For TC11: The trigger/capture input B is driven internally by the GTSUCOMP signal of the Ethernet MAC (GMAC)."]
    PWMX,
}
impl TRIGSRCBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRIGSRCBW::EXTERNAL_TIOBX => 0,
            TRIGSRCBW::PWMX => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TRIGSRCBW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGSRCBW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGSRCBW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The trigger/capture input B is driven by external pin TIOBx"]
    #[inline(always)]
    pub fn external_tiobx(self) -> &'a mut W {
        self.variant(TRIGSRCBW::EXTERNAL_TIOBX)
    }
    #[doc = "For TC0 to TC10: The trigger/capture input B is driven internally by the comparator output (see Figure 7-16) of the PWMx.For TC11: The trigger/capture input B is driven internally by the GTSUCOMP signal of the Ethernet MAC (GMAC)."]
    #[inline(always)]
    pub fn pwmx(self) -> &'a mut W {
        self.variant(TRIGSRCBW::PWMX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type NODIVCLK_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _NODIVCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _NODIVCLKW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Trigger Source for Input A"]
    #[inline(always)]
    pub fn trigsrca(&self) -> TRIGSRCA_R {
        TRIGSRCA_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Trigger Source for Input B"]
    #[inline(always)]
    pub fn trigsrcb(&self) -> TRIGSRCB_R {
        TRIGSRCB_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - No Divided Clock"]
    #[inline(always)]
    pub fn nodivclk(&self) -> NODIVCLK_R {
        NODIVCLK_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Trigger Source for Input A"]
    #[inline(always)]
    pub fn trigsrca(&mut self) -> _TRIGSRCAW {
        _TRIGSRCAW { w: self }
    }
    #[doc = "Bits 4:5 - Trigger Source for Input B"]
    #[inline(always)]
    pub fn trigsrcb(&mut self) -> _TRIGSRCBW {
        _TRIGSRCBW { w: self }
    }
    #[doc = "Bit 8 - No Divided Clock"]
    #[inline(always)]
    pub fn nodivclk(&mut self) -> _NODIVCLKW {
        _NODIVCLKW { w: self }
    }
}
