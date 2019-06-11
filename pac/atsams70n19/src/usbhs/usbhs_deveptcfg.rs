#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBHS_DEVEPTCFG {
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
pub struct ALLOCR {
    bits: bool,
}
impl ALLOCR {
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
#[doc = "Possible values of the field `EPBK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPBKR {
    #[doc = "Single-bank endpoint"]
    _1_BANK,
    #[doc = "Double-bank endpoint"]
    _2_BANK,
    #[doc = "Triple-bank endpoint"]
    _3_BANK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EPBKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EPBKR::_1_BANK => 0,
            EPBKR::_2_BANK => 1,
            EPBKR::_3_BANK => 2,
            EPBKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EPBKR {
        match value {
            0 => EPBKR::_1_BANK,
            1 => EPBKR::_2_BANK,
            2 => EPBKR::_3_BANK,
            i => EPBKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1_BANK`"]
    #[inline]
    pub fn is_1_bank(&self) -> bool {
        *self == EPBKR::_1_BANK
    }
    #[doc = "Checks if the value of the field is `_2_BANK`"]
    #[inline]
    pub fn is_2_bank(&self) -> bool {
        *self == EPBKR::_2_BANK
    }
    #[doc = "Checks if the value of the field is `_3_BANK`"]
    #[inline]
    pub fn is_3_bank(&self) -> bool {
        *self == EPBKR::_3_BANK
    }
}
#[doc = "Possible values of the field `EPSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPSIZER {
    #[doc = "8 bytes"]
    _8_BYTE,
    #[doc = "16 bytes"]
    _16_BYTE,
    #[doc = "32 bytes"]
    _32_BYTE,
    #[doc = "64 bytes"]
    _64_BYTE,
    #[doc = "128 bytes"]
    _128_BYTE,
    #[doc = "256 bytes"]
    _256_BYTE,
    #[doc = "512 bytes"]
    _512_BYTE,
    #[doc = "1024 bytes"]
    _1024_BYTE,
}
impl EPSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EPSIZER::_8_BYTE => 0,
            EPSIZER::_16_BYTE => 1,
            EPSIZER::_32_BYTE => 2,
            EPSIZER::_64_BYTE => 3,
            EPSIZER::_128_BYTE => 4,
            EPSIZER::_256_BYTE => 5,
            EPSIZER::_512_BYTE => 6,
            EPSIZER::_1024_BYTE => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EPSIZER {
        match value {
            0 => EPSIZER::_8_BYTE,
            1 => EPSIZER::_16_BYTE,
            2 => EPSIZER::_32_BYTE,
            3 => EPSIZER::_64_BYTE,
            4 => EPSIZER::_128_BYTE,
            5 => EPSIZER::_256_BYTE,
            6 => EPSIZER::_512_BYTE,
            7 => EPSIZER::_1024_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline]
    pub fn is_8_byte(&self) -> bool {
        *self == EPSIZER::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline]
    pub fn is_16_byte(&self) -> bool {
        *self == EPSIZER::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline]
    pub fn is_32_byte(&self) -> bool {
        *self == EPSIZER::_32_BYTE
    }
    #[doc = "Checks if the value of the field is `_64_BYTE`"]
    #[inline]
    pub fn is_64_byte(&self) -> bool {
        *self == EPSIZER::_64_BYTE
    }
    #[doc = "Checks if the value of the field is `_128_BYTE`"]
    #[inline]
    pub fn is_128_byte(&self) -> bool {
        *self == EPSIZER::_128_BYTE
    }
    #[doc = "Checks if the value of the field is `_256_BYTE`"]
    #[inline]
    pub fn is_256_byte(&self) -> bool {
        *self == EPSIZER::_256_BYTE
    }
    #[doc = "Checks if the value of the field is `_512_BYTE`"]
    #[inline]
    pub fn is_512_byte(&self) -> bool {
        *self == EPSIZER::_512_BYTE
    }
    #[doc = "Checks if the value of the field is `_1024_BYTE`"]
    #[inline]
    pub fn is_1024_byte(&self) -> bool {
        *self == EPSIZER::_1024_BYTE
    }
}
#[doc = "Possible values of the field `EPDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPDIRR {
    #[doc = "The endpoint direction is OUT."]
    OUT,
    #[doc = "The endpoint direction is IN (nor for control endpoints)."]
    IN,
}
impl EPDIRR {
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
            EPDIRR::OUT => false,
            EPDIRR::IN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPDIRR {
        match value {
            false => EPDIRR::OUT,
            true => EPDIRR::IN,
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == EPDIRR::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == EPDIRR::IN
    }
}
#[doc = r" Value of the field"]
pub struct AUTOSWR {
    bits: bool,
}
impl AUTOSWR {
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
#[doc = "Possible values of the field `EPTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPTYPER {
    #[doc = "Control"]
    CTRL,
    #[doc = "Isochronous"]
    ISO,
    #[doc = "Bulk"]
    BLK,
    #[doc = "Interrupt"]
    INTRPT,
}
impl EPTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EPTYPER::CTRL => 0,
            EPTYPER::ISO => 1,
            EPTYPER::BLK => 2,
            EPTYPER::INTRPT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EPTYPER {
        match value {
            0 => EPTYPER::CTRL,
            1 => EPTYPER::ISO,
            2 => EPTYPER::BLK,
            3 => EPTYPER::INTRPT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CTRL`"]
    #[inline]
    pub fn is_ctrl(&self) -> bool {
        *self == EPTYPER::CTRL
    }
    #[doc = "Checks if the value of the field is `ISO`"]
    #[inline]
    pub fn is_iso(&self) -> bool {
        *self == EPTYPER::ISO
    }
    #[doc = "Checks if the value of the field is `BLK`"]
    #[inline]
    pub fn is_blk(&self) -> bool {
        *self == EPTYPER::BLK
    }
    #[doc = "Checks if the value of the field is `INTRPT`"]
    #[inline]
    pub fn is_intrpt(&self) -> bool {
        *self == EPTYPER::INTRPT
    }
}
#[doc = "Possible values of the field `NBTRANS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBTRANSR {
    #[doc = "Reserved to endpoint that does not have the high-bandwidth isochronous capability."]
    _0_TRANS,
    #[doc = "Default value: one transaction per microframe."]
    _1_TRANS,
    #[doc = "Two transactions per microframe. This endpoint should be configured as double-bank."]
    _2_TRANS,
    #[doc = "Three transactions per microframe. This endpoint should be configured as triple-bank."]
    _3_TRANS,
}
impl NBTRANSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NBTRANSR::_0_TRANS => 0,
            NBTRANSR::_1_TRANS => 1,
            NBTRANSR::_2_TRANS => 2,
            NBTRANSR::_3_TRANS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NBTRANSR {
        match value {
            0 => NBTRANSR::_0_TRANS,
            1 => NBTRANSR::_1_TRANS,
            2 => NBTRANSR::_2_TRANS,
            3 => NBTRANSR::_3_TRANS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0_TRANS`"]
    #[inline]
    pub fn is_0_trans(&self) -> bool {
        *self == NBTRANSR::_0_TRANS
    }
    #[doc = "Checks if the value of the field is `_1_TRANS`"]
    #[inline]
    pub fn is_1_trans(&self) -> bool {
        *self == NBTRANSR::_1_TRANS
    }
    #[doc = "Checks if the value of the field is `_2_TRANS`"]
    #[inline]
    pub fn is_2_trans(&self) -> bool {
        *self == NBTRANSR::_2_TRANS
    }
    #[doc = "Checks if the value of the field is `_3_TRANS`"]
    #[inline]
    pub fn is_3_trans(&self) -> bool {
        *self == NBTRANSR::_3_TRANS
    }
}
#[doc = r" Proxy"]
pub struct _ALLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _ALLOCW<'a> {
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
#[doc = "Values that can be written to the field `EPBK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPBKW {
    #[doc = "Single-bank endpoint"]
    _1_BANK,
    #[doc = "Double-bank endpoint"]
    _2_BANK,
    #[doc = "Triple-bank endpoint"]
    _3_BANK,
}
impl EPBKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPBKW::_1_BANK => 0,
            EPBKW::_2_BANK => 1,
            EPBKW::_3_BANK => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPBKW<'a> {
    w: &'a mut W,
}
impl<'a> _EPBKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPBKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Single-bank endpoint"]
    #[inline]
    pub fn _1_bank(self) -> &'a mut W {
        self.variant(EPBKW::_1_BANK)
    }
    #[doc = "Double-bank endpoint"]
    #[inline]
    pub fn _2_bank(self) -> &'a mut W {
        self.variant(EPBKW::_2_BANK)
    }
    #[doc = "Triple-bank endpoint"]
    #[inline]
    pub fn _3_bank(self) -> &'a mut W {
        self.variant(EPBKW::_3_BANK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EPSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPSIZEW {
    #[doc = "8 bytes"]
    _8_BYTE,
    #[doc = "16 bytes"]
    _16_BYTE,
    #[doc = "32 bytes"]
    _32_BYTE,
    #[doc = "64 bytes"]
    _64_BYTE,
    #[doc = "128 bytes"]
    _128_BYTE,
    #[doc = "256 bytes"]
    _256_BYTE,
    #[doc = "512 bytes"]
    _512_BYTE,
    #[doc = "1024 bytes"]
    _1024_BYTE,
}
impl EPSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPSIZEW::_8_BYTE => 0,
            EPSIZEW::_16_BYTE => 1,
            EPSIZEW::_32_BYTE => 2,
            EPSIZEW::_64_BYTE => 3,
            EPSIZEW::_128_BYTE => 4,
            EPSIZEW::_256_BYTE => 5,
            EPSIZEW::_512_BYTE => 6,
            EPSIZEW::_1024_BYTE => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPSIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "8 bytes"]
    #[inline]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(EPSIZEW::_8_BYTE)
    }
    #[doc = "16 bytes"]
    #[inline]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(EPSIZEW::_16_BYTE)
    }
    #[doc = "32 bytes"]
    #[inline]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(EPSIZEW::_32_BYTE)
    }
    #[doc = "64 bytes"]
    #[inline]
    pub fn _64_byte(self) -> &'a mut W {
        self.variant(EPSIZEW::_64_BYTE)
    }
    #[doc = "128 bytes"]
    #[inline]
    pub fn _128_byte(self) -> &'a mut W {
        self.variant(EPSIZEW::_128_BYTE)
    }
    #[doc = "256 bytes"]
    #[inline]
    pub fn _256_byte(self) -> &'a mut W {
        self.variant(EPSIZEW::_256_BYTE)
    }
    #[doc = "512 bytes"]
    #[inline]
    pub fn _512_byte(self) -> &'a mut W {
        self.variant(EPSIZEW::_512_BYTE)
    }
    #[doc = "1024 bytes"]
    #[inline]
    pub fn _1024_byte(self) -> &'a mut W {
        self.variant(EPSIZEW::_1024_BYTE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EPDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPDIRW {
    #[doc = "The endpoint direction is OUT."]
    OUT,
    #[doc = "The endpoint direction is IN (nor for control endpoints)."]
    IN,
}
impl EPDIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPDIRW::OUT => false,
            EPDIRW::IN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _EPDIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPDIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The endpoint direction is OUT."]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(EPDIRW::OUT)
    }
    #[doc = "The endpoint direction is IN (nor for control endpoints)."]
    #[inline]
    pub fn in_(self) -> &'a mut W {
        self.variant(EPDIRW::IN)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _AUTOSWW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOSWW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EPTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPTYPEW {
    #[doc = "Control"]
    CTRL,
    #[doc = "Isochronous"]
    ISO,
    #[doc = "Bulk"]
    BLK,
    #[doc = "Interrupt"]
    INTRPT,
}
impl EPTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPTYPEW::CTRL => 0,
            EPTYPEW::ISO => 1,
            EPTYPEW::BLK => 2,
            EPTYPEW::INTRPT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPTYPEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Control"]
    #[inline]
    pub fn ctrl(self) -> &'a mut W {
        self.variant(EPTYPEW::CTRL)
    }
    #[doc = "Isochronous"]
    #[inline]
    pub fn iso(self) -> &'a mut W {
        self.variant(EPTYPEW::ISO)
    }
    #[doc = "Bulk"]
    #[inline]
    pub fn blk(self) -> &'a mut W {
        self.variant(EPTYPEW::BLK)
    }
    #[doc = "Interrupt"]
    #[inline]
    pub fn intrpt(self) -> &'a mut W {
        self.variant(EPTYPEW::INTRPT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NBTRANS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBTRANSW {
    #[doc = "Reserved to endpoint that does not have the high-bandwidth isochronous capability."]
    _0_TRANS,
    #[doc = "Default value: one transaction per microframe."]
    _1_TRANS,
    #[doc = "Two transactions per microframe. This endpoint should be configured as double-bank."]
    _2_TRANS,
    #[doc = "Three transactions per microframe. This endpoint should be configured as triple-bank."]
    _3_TRANS,
}
impl NBTRANSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NBTRANSW::_0_TRANS => 0,
            NBTRANSW::_1_TRANS => 1,
            NBTRANSW::_2_TRANS => 2,
            NBTRANSW::_3_TRANS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NBTRANSW<'a> {
    w: &'a mut W,
}
impl<'a> _NBTRANSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NBTRANSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Reserved to endpoint that does not have the high-bandwidth isochronous capability."]
    #[inline]
    pub fn _0_trans(self) -> &'a mut W {
        self.variant(NBTRANSW::_0_TRANS)
    }
    #[doc = "Default value: one transaction per microframe."]
    #[inline]
    pub fn _1_trans(self) -> &'a mut W {
        self.variant(NBTRANSW::_1_TRANS)
    }
    #[doc = "Two transactions per microframe. This endpoint should be configured as double-bank."]
    #[inline]
    pub fn _2_trans(self) -> &'a mut W {
        self.variant(NBTRANSW::_2_TRANS)
    }
    #[doc = "Three transactions per microframe. This endpoint should be configured as triple-bank."]
    #[inline]
    pub fn _3_trans(self) -> &'a mut W {
        self.variant(NBTRANSW::_3_TRANS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 13;
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
    #[doc = "Bit 1 - Endpoint Memory Allocate"]
    #[inline]
    pub fn alloc(&self) -> ALLOCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALLOCR { bits }
    }
    #[doc = "Bits 2:3 - Endpoint Banks"]
    #[inline]
    pub fn epbk(&self) -> EPBKR {
        EPBKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Endpoint Size"]
    #[inline]
    pub fn epsize(&self) -> EPSIZER {
        EPSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Endpoint Direction"]
    #[inline]
    pub fn epdir(&self) -> EPDIRR {
        EPDIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Automatic Switch"]
    #[inline]
    pub fn autosw(&self) -> AUTOSWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOSWR { bits }
    }
    #[doc = "Bits 11:12 - Endpoint Type"]
    #[inline]
    pub fn eptype(&self) -> EPTYPER {
        EPTYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 13:14 - Number of transactions per microframe for isochronous endpoint"]
    #[inline]
    pub fn nbtrans(&self) -> NBTRANSR {
        NBTRANSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
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
    #[doc = "Bit 1 - Endpoint Memory Allocate"]
    #[inline]
    pub fn alloc(&mut self) -> _ALLOCW {
        _ALLOCW { w: self }
    }
    #[doc = "Bits 2:3 - Endpoint Banks"]
    #[inline]
    pub fn epbk(&mut self) -> _EPBKW {
        _EPBKW { w: self }
    }
    #[doc = "Bits 4:6 - Endpoint Size"]
    #[inline]
    pub fn epsize(&mut self) -> _EPSIZEW {
        _EPSIZEW { w: self }
    }
    #[doc = "Bit 8 - Endpoint Direction"]
    #[inline]
    pub fn epdir(&mut self) -> _EPDIRW {
        _EPDIRW { w: self }
    }
    #[doc = "Bit 9 - Automatic Switch"]
    #[inline]
    pub fn autosw(&mut self) -> _AUTOSWW {
        _AUTOSWW { w: self }
    }
    #[doc = "Bits 11:12 - Endpoint Type"]
    #[inline]
    pub fn eptype(&mut self) -> _EPTYPEW {
        _EPTYPEW { w: self }
    }
    #[doc = "Bits 13:14 - Number of transactions per microframe for isochronous endpoint"]
    #[inline]
    pub fn nbtrans(&mut self) -> _NBTRANSW {
        _NBTRANSW { w: self }
    }
}
