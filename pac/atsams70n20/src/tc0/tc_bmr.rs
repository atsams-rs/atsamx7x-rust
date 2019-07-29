#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TC_BMR {
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
#[doc = "Possible values of the field `TC0XC0S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0XC0SR {
    #[doc = "Signal connected to XC0: TCLK0"]
    TCLK0,
    #[doc = "Signal connected to XC0: TIOA1"]
    TIOA1,
    #[doc = "Signal connected to XC0: TIOA2"]
    TIOA2,
}
impl crate::ToBits<u8> for TC0XC0SR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TC0XC0SR::TCLK0 => 0,
            TC0XC0SR::TIOA1 => 2,
            TC0XC0SR::TIOA2 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC0XC0S_R = crate::FR<u8, TC0XC0SR>;
impl TC0XC0S_R {
    #[doc = "Checks if the value of the field is `TCLK0`"]
    #[inline(always)]
    pub fn is_tclk0(&self) -> bool {
        *self == TC0XC0SR::TCLK0
    }
    #[doc = "Checks if the value of the field is `TIOA1`"]
    #[inline(always)]
    pub fn is_tioa1(&self) -> bool {
        *self == TC0XC0SR::TIOA1
    }
    #[doc = "Checks if the value of the field is `TIOA2`"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        *self == TC0XC0SR::TIOA2
    }
}
#[doc = "Values that can be written to the field `TC0XC0S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC0XC0SW {
    #[doc = "Signal connected to XC0: TCLK0"]
    TCLK0,
    #[doc = "Signal connected to XC0: TIOA1"]
    TIOA1,
    #[doc = "Signal connected to XC0: TIOA2"]
    TIOA2,
}
impl TC0XC0SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TC0XC0SW::TCLK0 => 0,
            TC0XC0SW::TIOA1 => 2,
            TC0XC0SW::TIOA2 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TC0XC0SW<'a> {
    w: &'a mut W,
}
impl<'a> _TC0XC0SW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC0XC0SW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Signal connected to XC0: TCLK0"]
    #[inline(always)]
    pub fn tclk0(self) -> &'a mut W {
        self.variant(TC0XC0SW::TCLK0)
    }
    #[doc = "Signal connected to XC0: TIOA1"]
    #[inline(always)]
    pub fn tioa1(self) -> &'a mut W {
        self.variant(TC0XC0SW::TIOA1)
    }
    #[doc = "Signal connected to XC0: TIOA2"]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC0XC0SW::TIOA2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `TC1XC1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1XC1SR {
    #[doc = "Signal connected to XC1: TCLK1"]
    TCLK1,
    #[doc = "Signal connected to XC1: TIOA0"]
    TIOA0,
    #[doc = "Signal connected to XC1: TIOA2"]
    TIOA2,
}
impl crate::ToBits<u8> for TC1XC1SR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TC1XC1SR::TCLK1 => 0,
            TC1XC1SR::TIOA0 => 2,
            TC1XC1SR::TIOA2 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC1XC1S_R = crate::FR<u8, TC1XC1SR>;
impl TC1XC1S_R {
    #[doc = "Checks if the value of the field is `TCLK1`"]
    #[inline(always)]
    pub fn is_tclk1(&self) -> bool {
        *self == TC1XC1SR::TCLK1
    }
    #[doc = "Checks if the value of the field is `TIOA0`"]
    #[inline(always)]
    pub fn is_tioa0(&self) -> bool {
        *self == TC1XC1SR::TIOA0
    }
    #[doc = "Checks if the value of the field is `TIOA2`"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        *self == TC1XC1SR::TIOA2
    }
}
#[doc = "Values that can be written to the field `TC1XC1S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC1XC1SW {
    #[doc = "Signal connected to XC1: TCLK1"]
    TCLK1,
    #[doc = "Signal connected to XC1: TIOA0"]
    TIOA0,
    #[doc = "Signal connected to XC1: TIOA2"]
    TIOA2,
}
impl TC1XC1SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TC1XC1SW::TCLK1 => 0,
            TC1XC1SW::TIOA0 => 2,
            TC1XC1SW::TIOA2 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TC1XC1SW<'a> {
    w: &'a mut W,
}
impl<'a> _TC1XC1SW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC1XC1SW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Signal connected to XC1: TCLK1"]
    #[inline(always)]
    pub fn tclk1(self) -> &'a mut W {
        self.variant(TC1XC1SW::TCLK1)
    }
    #[doc = "Signal connected to XC1: TIOA0"]
    #[inline(always)]
    pub fn tioa0(self) -> &'a mut W {
        self.variant(TC1XC1SW::TIOA0)
    }
    #[doc = "Signal connected to XC1: TIOA2"]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC1XC1SW::TIOA2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `TC2XC2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2XC2SR {
    #[doc = "Signal connected to XC2: TCLK2"]
    TCLK2,
    #[doc = "Signal connected to XC2: TIOA0"]
    TIOA0,
    #[doc = "Signal connected to XC2: TIOA1"]
    TIOA1,
}
impl crate::ToBits<u8> for TC2XC2SR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TC2XC2SR::TCLK2 => 0,
            TC2XC2SR::TIOA0 => 2,
            TC2XC2SR::TIOA1 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TC2XC2S_R = crate::FR<u8, TC2XC2SR>;
impl TC2XC2S_R {
    #[doc = "Checks if the value of the field is `TCLK2`"]
    #[inline(always)]
    pub fn is_tclk2(&self) -> bool {
        *self == TC2XC2SR::TCLK2
    }
    #[doc = "Checks if the value of the field is `TIOA0`"]
    #[inline(always)]
    pub fn is_tioa0(&self) -> bool {
        *self == TC2XC2SR::TIOA0
    }
    #[doc = "Checks if the value of the field is `TIOA1`"]
    #[inline(always)]
    pub fn is_tioa1(&self) -> bool {
        *self == TC2XC2SR::TIOA1
    }
}
#[doc = "Values that can be written to the field `TC2XC2S`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TC2XC2SW {
    #[doc = "Signal connected to XC2: TCLK2"]
    TCLK2,
    #[doc = "Signal connected to XC2: TIOA0"]
    TIOA0,
    #[doc = "Signal connected to XC2: TIOA1"]
    TIOA1,
}
impl TC2XC2SW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TC2XC2SW::TCLK2 => 0,
            TC2XC2SW::TIOA0 => 2,
            TC2XC2SW::TIOA1 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TC2XC2SW<'a> {
    w: &'a mut W,
}
impl<'a> _TC2XC2SW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC2XC2SW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Signal connected to XC2: TCLK2"]
    #[inline(always)]
    pub fn tclk2(self) -> &'a mut W {
        self.variant(TC2XC2SW::TCLK2)
    }
    #[doc = "Signal connected to XC2: TIOA0"]
    #[inline(always)]
    pub fn tioa0(self) -> &'a mut W {
        self.variant(TC2XC2SW::TIOA0)
    }
    #[doc = "Signal connected to XC2: TIOA1"]
    #[inline(always)]
    pub fn tioa1(self) -> &'a mut W {
        self.variant(TC2XC2SW::TIOA1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type QDEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _QDENW<'a> {
    w: &'a mut W,
}
impl<'a> _QDENW<'a> {
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
#[doc = r"Reader of the field"]
pub type POSEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _POSENW<'a> {
    w: &'a mut W,
}
impl<'a> _POSENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SPEEDEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SPEEDENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPEEDENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type QDTRANS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _QDTRANSW<'a> {
    w: &'a mut W,
}
impl<'a> _QDTRANSW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EDGPHA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EDGPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _EDGPHAW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type INVA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _INVAW<'a> {
    w: &'a mut W,
}
impl<'a> _INVAW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type INVB_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _INVBW<'a> {
    w: &'a mut W,
}
impl<'a> _INVBW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type INVIDX_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _INVIDXW<'a> {
    w: &'a mut W,
}
impl<'a> _INVIDXW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SWAP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SWAPW<'a> {
    w: &'a mut W,
}
impl<'a> _SWAPW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type IDXPHB_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _IDXPHBW<'a> {
    w: &'a mut W,
}
impl<'a> _IDXPHBW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type MAXFILT_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _MAXFILTW<'a> {
    w: &'a mut W,
}
impl<'a> _MAXFILTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | (((value as u32) & 0x3f) << 20);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline(always)]
    pub fn tc0xc0s(&self) -> TC0XC0S_R {
        TC0XC0S_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline(always)]
    pub fn tc1xc1s(&self) -> TC1XC1S_R {
        TC1XC1S_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline(always)]
    pub fn tc2xc2s(&self) -> TC2XC2S_R {
        TC2XC2S_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Quadrature Decoder Enabled"]
    #[inline(always)]
    pub fn qden(&self) -> QDEN_R {
        QDEN_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Position Enabled"]
    #[inline(always)]
    pub fn posen(&self) -> POSEN_R {
        POSEN_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Speed Enabled"]
    #[inline(always)]
    pub fn speeden(&self) -> SPEEDEN_R {
        SPEEDEN_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Quadrature Decoding Transparent"]
    #[inline(always)]
    pub fn qdtrans(&self) -> QDTRANS_R {
        QDTRANS_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Edge on PHA Count Mode"]
    #[inline(always)]
    pub fn edgpha(&self) -> EDGPHA_R {
        EDGPHA_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Inverted PHA"]
    #[inline(always)]
    pub fn inva(&self) -> INVA_R {
        INVA_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Inverted PHB"]
    #[inline(always)]
    pub fn invb(&self) -> INVB_R {
        INVB_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Inverted Index"]
    #[inline(always)]
    pub fn invidx(&self) -> INVIDX_R {
        INVIDX_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Swap PHA and PHB"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Index Pin is PHB Pin"]
    #[inline(always)]
    pub fn idxphb(&self) -> IDXPHB_R {
        IDXPHB_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 20:25 - Maximum Filter"]
    #[inline(always)]
    pub fn maxfilt(&self) -> MAXFILT_R {
        MAXFILT_R::new(((self.bits() >> 20) & 0x3f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline(always)]
    pub fn tc0xc0s(&mut self) -> _TC0XC0SW {
        _TC0XC0SW { w: self }
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline(always)]
    pub fn tc1xc1s(&mut self) -> _TC1XC1SW {
        _TC1XC1SW { w: self }
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline(always)]
    pub fn tc2xc2s(&mut self) -> _TC2XC2SW {
        _TC2XC2SW { w: self }
    }
    #[doc = "Bit 8 - Quadrature Decoder Enabled"]
    #[inline(always)]
    pub fn qden(&mut self) -> _QDENW {
        _QDENW { w: self }
    }
    #[doc = "Bit 9 - Position Enabled"]
    #[inline(always)]
    pub fn posen(&mut self) -> _POSENW {
        _POSENW { w: self }
    }
    #[doc = "Bit 10 - Speed Enabled"]
    #[inline(always)]
    pub fn speeden(&mut self) -> _SPEEDENW {
        _SPEEDENW { w: self }
    }
    #[doc = "Bit 11 - Quadrature Decoding Transparent"]
    #[inline(always)]
    pub fn qdtrans(&mut self) -> _QDTRANSW {
        _QDTRANSW { w: self }
    }
    #[doc = "Bit 12 - Edge on PHA Count Mode"]
    #[inline(always)]
    pub fn edgpha(&mut self) -> _EDGPHAW {
        _EDGPHAW { w: self }
    }
    #[doc = "Bit 13 - Inverted PHA"]
    #[inline(always)]
    pub fn inva(&mut self) -> _INVAW {
        _INVAW { w: self }
    }
    #[doc = "Bit 14 - Inverted PHB"]
    #[inline(always)]
    pub fn invb(&mut self) -> _INVBW {
        _INVBW { w: self }
    }
    #[doc = "Bit 15 - Inverted Index"]
    #[inline(always)]
    pub fn invidx(&mut self) -> _INVIDXW {
        _INVIDXW { w: self }
    }
    #[doc = "Bit 16 - Swap PHA and PHB"]
    #[inline(always)]
    pub fn swap(&mut self) -> _SWAPW {
        _SWAPW { w: self }
    }
    #[doc = "Bit 17 - Index Pin is PHB Pin"]
    #[inline(always)]
    pub fn idxphb(&mut self) -> _IDXPHBW {
        _IDXPHBW { w: self }
    }
    #[doc = "Bits 20:25 - Maximum Filter"]
    #[inline(always)]
    pub fn maxfilt(&mut self) -> _MAXFILTW {
        _MAXFILTW { w: self }
    }
}
