#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TC_EMR {
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
#[doc = "Possible values of the field `TRIGSRCA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGSRCAR {
    #[doc = "The trigger/capture input A is driven by external pin TIOAx"]
    EXTERNAL_TIOAX,
    #[doc = "The trigger/capture input A is driven internally by PWMx"]
    PWMX,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRIGSRCAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRIGSRCAR::EXTERNAL_TIOAX => 0,
            TRIGSRCAR::PWMX => 1,
            TRIGSRCAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRIGSRCAR {
        match value {
            0 => TRIGSRCAR::EXTERNAL_TIOAX,
            1 => TRIGSRCAR::PWMX,
            i => TRIGSRCAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_TIOAX`"]
    #[inline]
    pub fn is_external_tioax(&self) -> bool {
        *self == TRIGSRCAR::EXTERNAL_TIOAX
    }
    #[doc = "Checks if the value of the field is `PWMX`"]
    #[inline]
    pub fn is_pwmx(&self) -> bool {
        *self == TRIGSRCAR::PWMX
    }
}
#[doc = "Possible values of the field `TRIGSRCB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGSRCBR {
    #[doc = "The trigger/capture input B is driven by external pin TIOBx"]
    EXTERNAL_TIOBX,
    #[doc = "For TC0 to TC10: The trigger/capture input B is driven internally by the comparator output (see Figure 7-16) of the PWMx.For TC11: The trigger/capture input B is driven internally by the GTSUCOMP signal of the Ethernet MAC (GMAC)."]
    PWMX,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRIGSRCBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRIGSRCBR::EXTERNAL_TIOBX => 0,
            TRIGSRCBR::PWMX => 1,
            TRIGSRCBR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRIGSRCBR {
        match value {
            0 => TRIGSRCBR::EXTERNAL_TIOBX,
            1 => TRIGSRCBR::PWMX,
            i => TRIGSRCBR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_TIOBX`"]
    #[inline]
    pub fn is_external_tiobx(&self) -> bool {
        *self == TRIGSRCBR::EXTERNAL_TIOBX
    }
    #[doc = "Checks if the value of the field is `PWMX`"]
    #[inline]
    pub fn is_pwmx(&self) -> bool {
        *self == TRIGSRCBR::PWMX
    }
}
#[doc = r" Value of the field"]
pub struct NODIVCLKR {
    bits: bool,
}
impl NODIVCLKR {
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRIGSRCAW::EXTERNAL_TIOAX => 0,
            TRIGSRCAW::PWMX => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGSRCAW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGSRCAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGSRCAW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The trigger/capture input A is driven by external pin TIOAx"]
    #[inline]
    pub fn external_tioax(self) -> &'a mut W {
        self.variant(TRIGSRCAW::EXTERNAL_TIOAX)
    }
    #[doc = "The trigger/capture input A is driven internally by PWMx"]
    #[inline]
    pub fn pwmx(self) -> &'a mut W {
        self.variant(TRIGSRCAW::PWMX)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRIGSRCBW::EXTERNAL_TIOBX => 0,
            TRIGSRCBW::PWMX => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGSRCBW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGSRCBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGSRCBW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The trigger/capture input B is driven by external pin TIOBx"]
    #[inline]
    pub fn external_tiobx(self) -> &'a mut W {
        self.variant(TRIGSRCBW::EXTERNAL_TIOBX)
    }
    #[doc = "For TC0 to TC10: The trigger/capture input B is driven internally by the comparator output (see Figure 7-16) of the PWMx.For TC11: The trigger/capture input B is driven internally by the GTSUCOMP signal of the Ethernet MAC (GMAC)."]
    #[inline]
    pub fn pwmx(self) -> &'a mut W {
        self.variant(TRIGSRCBW::PWMX)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NODIVCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _NODIVCLKW<'a> {
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
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:1 - Trigger Source for Input A"]
    #[inline]
    pub fn trigsrca(&self) -> TRIGSRCAR {
        TRIGSRCAR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Trigger Source for Input B"]
    #[inline]
    pub fn trigsrcb(&self) -> TRIGSRCBR {
        TRIGSRCBR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - No Divided Clock"]
    #[inline]
    pub fn nodivclk(&self) -> NODIVCLKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NODIVCLKR { bits }
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
    #[doc = "Bits 0:1 - Trigger Source for Input A"]
    #[inline]
    pub fn trigsrca(&mut self) -> _TRIGSRCAW {
        _TRIGSRCAW { w: self }
    }
    #[doc = "Bits 4:5 - Trigger Source for Input B"]
    #[inline]
    pub fn trigsrcb(&mut self) -> _TRIGSRCBW {
        _TRIGSRCBW { w: self }
    }
    #[doc = "Bit 8 - No Divided Clock"]
    #[inline]
    pub fn nodivclk(&mut self) -> _NODIVCLKW {
        _NODIVCLKW { w: self }
    }
}
