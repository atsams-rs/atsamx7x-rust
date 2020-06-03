#[doc = "Reader of register TC_BMR"]
pub type R = crate::R<u32, super::TC_BMR>;
#[doc = "Writer for register TC_BMR"]
pub type W = crate::W<u32, super::TC_BMR>;
#[doc = "Register TC_BMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TC_BMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "External Clock Signal 0 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TC0XC0S_A {
    #[doc = "0: Signal connected to XC0: TCLK0"]
    TCLK0 = 0,
    #[doc = "2: Signal connected to XC0: TIOA1"]
    TIOA1 = 2,
    #[doc = "3: Signal connected to XC0: TIOA2"]
    TIOA2 = 3,
}
impl From<TC0XC0S_A> for u8 {
    #[inline(always)]
    fn from(variant: TC0XC0S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TC0XC0S`"]
pub type TC0XC0S_R = crate::R<u8, TC0XC0S_A>;
impl TC0XC0S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TC0XC0S_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TC0XC0S_A::TCLK0),
            2 => Val(TC0XC0S_A::TIOA1),
            3 => Val(TC0XC0S_A::TIOA2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TCLK0`"]
    #[inline(always)]
    pub fn is_tclk0(&self) -> bool {
        *self == TC0XC0S_A::TCLK0
    }
    #[doc = "Checks if the value of the field is `TIOA1`"]
    #[inline(always)]
    pub fn is_tioa1(&self) -> bool {
        *self == TC0XC0S_A::TIOA1
    }
    #[doc = "Checks if the value of the field is `TIOA2`"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        *self == TC0XC0S_A::TIOA2
    }
}
#[doc = "Write proxy for field `TC0XC0S`"]
pub struct TC0XC0S_W<'a> {
    w: &'a mut W,
}
impl<'a> TC0XC0S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC0XC0S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Signal connected to XC0: TCLK0"]
    #[inline(always)]
    pub fn tclk0(self) -> &'a mut W {
        self.variant(TC0XC0S_A::TCLK0)
    }
    #[doc = "Signal connected to XC0: TIOA1"]
    #[inline(always)]
    pub fn tioa1(self) -> &'a mut W {
        self.variant(TC0XC0S_A::TIOA1)
    }
    #[doc = "Signal connected to XC0: TIOA2"]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC0XC0S_A::TIOA2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "External Clock Signal 1 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TC1XC1S_A {
    #[doc = "0: Signal connected to XC1: TCLK1"]
    TCLK1 = 0,
    #[doc = "2: Signal connected to XC1: TIOA0"]
    TIOA0 = 2,
    #[doc = "3: Signal connected to XC1: TIOA2"]
    TIOA2 = 3,
}
impl From<TC1XC1S_A> for u8 {
    #[inline(always)]
    fn from(variant: TC1XC1S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TC1XC1S`"]
pub type TC1XC1S_R = crate::R<u8, TC1XC1S_A>;
impl TC1XC1S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TC1XC1S_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TC1XC1S_A::TCLK1),
            2 => Val(TC1XC1S_A::TIOA0),
            3 => Val(TC1XC1S_A::TIOA2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TCLK1`"]
    #[inline(always)]
    pub fn is_tclk1(&self) -> bool {
        *self == TC1XC1S_A::TCLK1
    }
    #[doc = "Checks if the value of the field is `TIOA0`"]
    #[inline(always)]
    pub fn is_tioa0(&self) -> bool {
        *self == TC1XC1S_A::TIOA0
    }
    #[doc = "Checks if the value of the field is `TIOA2`"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        *self == TC1XC1S_A::TIOA2
    }
}
#[doc = "Write proxy for field `TC1XC1S`"]
pub struct TC1XC1S_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1XC1S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC1XC1S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Signal connected to XC1: TCLK1"]
    #[inline(always)]
    pub fn tclk1(self) -> &'a mut W {
        self.variant(TC1XC1S_A::TCLK1)
    }
    #[doc = "Signal connected to XC1: TIOA0"]
    #[inline(always)]
    pub fn tioa0(self) -> &'a mut W {
        self.variant(TC1XC1S_A::TIOA0)
    }
    #[doc = "Signal connected to XC1: TIOA2"]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC1XC1S_A::TIOA2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "External Clock Signal 2 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TC2XC2S_A {
    #[doc = "0: Signal connected to XC2: TCLK2"]
    TCLK2 = 0,
    #[doc = "2: Signal connected to XC2: TIOA0"]
    TIOA0 = 2,
    #[doc = "3: Signal connected to XC2: TIOA1"]
    TIOA1 = 3,
}
impl From<TC2XC2S_A> for u8 {
    #[inline(always)]
    fn from(variant: TC2XC2S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TC2XC2S`"]
pub type TC2XC2S_R = crate::R<u8, TC2XC2S_A>;
impl TC2XC2S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TC2XC2S_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TC2XC2S_A::TCLK2),
            2 => Val(TC2XC2S_A::TIOA0),
            3 => Val(TC2XC2S_A::TIOA1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TCLK2`"]
    #[inline(always)]
    pub fn is_tclk2(&self) -> bool {
        *self == TC2XC2S_A::TCLK2
    }
    #[doc = "Checks if the value of the field is `TIOA0`"]
    #[inline(always)]
    pub fn is_tioa0(&self) -> bool {
        *self == TC2XC2S_A::TIOA0
    }
    #[doc = "Checks if the value of the field is `TIOA1`"]
    #[inline(always)]
    pub fn is_tioa1(&self) -> bool {
        *self == TC2XC2S_A::TIOA1
    }
}
#[doc = "Write proxy for field `TC2XC2S`"]
pub struct TC2XC2S_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2XC2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC2XC2S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Signal connected to XC2: TCLK2"]
    #[inline(always)]
    pub fn tclk2(self) -> &'a mut W {
        self.variant(TC2XC2S_A::TCLK2)
    }
    #[doc = "Signal connected to XC2: TIOA0"]
    #[inline(always)]
    pub fn tioa0(self) -> &'a mut W {
        self.variant(TC2XC2S_A::TIOA0)
    }
    #[doc = "Signal connected to XC2: TIOA1"]
    #[inline(always)]
    pub fn tioa1(self) -> &'a mut W {
        self.variant(TC2XC2S_A::TIOA1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `QDEN`"]
pub type QDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QDEN`"]
pub struct QDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QDEN_W<'a> {
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
#[doc = "Reader of field `POSEN`"]
pub type POSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POSEN`"]
pub struct POSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> POSEN_W<'a> {
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
#[doc = "Reader of field `SPEEDEN`"]
pub type SPEEDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPEEDEN`"]
pub struct SPEEDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEEDEN_W<'a> {
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
#[doc = "Reader of field `QDTRANS`"]
pub type QDTRANS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QDTRANS`"]
pub struct QDTRANS_W<'a> {
    w: &'a mut W,
}
impl<'a> QDTRANS_W<'a> {
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
#[doc = "Reader of field `EDGPHA`"]
pub type EDGPHA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGPHA`"]
pub struct EDGPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGPHA_W<'a> {
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
#[doc = "Reader of field `INVA`"]
pub type INVA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVA`"]
pub struct INVA_W<'a> {
    w: &'a mut W,
}
impl<'a> INVA_W<'a> {
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
#[doc = "Reader of field `INVB`"]
pub type INVB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVB`"]
pub struct INVB_W<'a> {
    w: &'a mut W,
}
impl<'a> INVB_W<'a> {
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
#[doc = "Reader of field `INVIDX`"]
pub type INVIDX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVIDX`"]
pub struct INVIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> INVIDX_W<'a> {
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
#[doc = "Reader of field `SWAP`"]
pub type SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWAP`"]
pub struct SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP_W<'a> {
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
#[doc = "Reader of field `IDXPHB`"]
pub type IDXPHB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDXPHB`"]
pub struct IDXPHB_W<'a> {
    w: &'a mut W,
}
impl<'a> IDXPHB_W<'a> {
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
#[doc = "Reader of field `AUTOC`"]
pub type AUTOC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUTOC`"]
pub struct AUTOC_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `MAXFILT`"]
pub type MAXFILT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAXFILT`"]
pub struct MAXFILT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXFILT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | (((value as u32) & 0x3f) << 20);
        self.w
    }
}
#[doc = "Reader of field `MAXCMP`"]
pub type MAXCMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAXCMP`"]
pub struct MAXCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXCMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 26)) | (((value as u32) & 0x0f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline(always)]
    pub fn tc0xc0s(&self) -> TC0XC0S_R {
        TC0XC0S_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline(always)]
    pub fn tc1xc1s(&self) -> TC1XC1S_R {
        TC1XC1S_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline(always)]
    pub fn tc2xc2s(&self) -> TC2XC2S_R {
        TC2XC2S_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Quadrature Decoder Enabled"]
    #[inline(always)]
    pub fn qden(&self) -> QDEN_R {
        QDEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Position Enabled"]
    #[inline(always)]
    pub fn posen(&self) -> POSEN_R {
        POSEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Speed Enabled"]
    #[inline(always)]
    pub fn speeden(&self) -> SPEEDEN_R {
        SPEEDEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Quadrature Decoding Transparent"]
    #[inline(always)]
    pub fn qdtrans(&self) -> QDTRANS_R {
        QDTRANS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Edge on PHA Count Mode"]
    #[inline(always)]
    pub fn edgpha(&self) -> EDGPHA_R {
        EDGPHA_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Inverted PHA"]
    #[inline(always)]
    pub fn inva(&self) -> INVA_R {
        INVA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Inverted PHB"]
    #[inline(always)]
    pub fn invb(&self) -> INVB_R {
        INVB_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Inverted Index"]
    #[inline(always)]
    pub fn invidx(&self) -> INVIDX_R {
        INVIDX_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Swap PHA and PHB"]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Index Pin is PHB Pin"]
    #[inline(always)]
    pub fn idxphb(&self) -> IDXPHB_R {
        IDXPHB_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - AutoCorrection of missing pulses"]
    #[inline(always)]
    pub fn autoc(&self) -> AUTOC_R {
        AUTOC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 20:25 - Maximum Filter"]
    #[inline(always)]
    pub fn maxfilt(&self) -> MAXFILT_R {
        MAXFILT_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:29 - Maximum Consecutive Missing Pulses"]
    #[inline(always)]
    pub fn maxcmp(&self) -> MAXCMP_R {
        MAXCMP_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline(always)]
    pub fn tc0xc0s(&mut self) -> TC0XC0S_W {
        TC0XC0S_W { w: self }
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline(always)]
    pub fn tc1xc1s(&mut self) -> TC1XC1S_W {
        TC1XC1S_W { w: self }
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline(always)]
    pub fn tc2xc2s(&mut self) -> TC2XC2S_W {
        TC2XC2S_W { w: self }
    }
    #[doc = "Bit 8 - Quadrature Decoder Enabled"]
    #[inline(always)]
    pub fn qden(&mut self) -> QDEN_W {
        QDEN_W { w: self }
    }
    #[doc = "Bit 9 - Position Enabled"]
    #[inline(always)]
    pub fn posen(&mut self) -> POSEN_W {
        POSEN_W { w: self }
    }
    #[doc = "Bit 10 - Speed Enabled"]
    #[inline(always)]
    pub fn speeden(&mut self) -> SPEEDEN_W {
        SPEEDEN_W { w: self }
    }
    #[doc = "Bit 11 - Quadrature Decoding Transparent"]
    #[inline(always)]
    pub fn qdtrans(&mut self) -> QDTRANS_W {
        QDTRANS_W { w: self }
    }
    #[doc = "Bit 12 - Edge on PHA Count Mode"]
    #[inline(always)]
    pub fn edgpha(&mut self) -> EDGPHA_W {
        EDGPHA_W { w: self }
    }
    #[doc = "Bit 13 - Inverted PHA"]
    #[inline(always)]
    pub fn inva(&mut self) -> INVA_W {
        INVA_W { w: self }
    }
    #[doc = "Bit 14 - Inverted PHB"]
    #[inline(always)]
    pub fn invb(&mut self) -> INVB_W {
        INVB_W { w: self }
    }
    #[doc = "Bit 15 - Inverted Index"]
    #[inline(always)]
    pub fn invidx(&mut self) -> INVIDX_W {
        INVIDX_W { w: self }
    }
    #[doc = "Bit 16 - Swap PHA and PHB"]
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W {
        SWAP_W { w: self }
    }
    #[doc = "Bit 17 - Index Pin is PHB Pin"]
    #[inline(always)]
    pub fn idxphb(&mut self) -> IDXPHB_W {
        IDXPHB_W { w: self }
    }
    #[doc = "Bit 18 - AutoCorrection of missing pulses"]
    #[inline(always)]
    pub fn autoc(&mut self) -> AUTOC_W {
        AUTOC_W { w: self }
    }
    #[doc = "Bits 20:25 - Maximum Filter"]
    #[inline(always)]
    pub fn maxfilt(&mut self) -> MAXFILT_W {
        MAXFILT_W { w: self }
    }
    #[doc = "Bits 26:29 - Maximum Consecutive Missing Pulses"]
    #[inline(always)]
    pub fn maxcmp(&mut self) -> MAXCMP_W {
        MAXCMP_W { w: self }
    }
}
