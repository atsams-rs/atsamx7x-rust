#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBHS_HSTPIPCFG_CTRL_BULK_MODE {
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
#[doc = "Possible values of the field `PBK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBKR {
    #[doc = "Single-bank pipe"]
    _1_BANK,
    #[doc = "Double-bank pipe"]
    _2_BANK,
    #[doc = "Triple-bank pipe"]
    _3_BANK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PBKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PBKR::_1_BANK => 0,
            PBKR::_2_BANK => 1,
            PBKR::_3_BANK => 2,
            PBKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PBKR {
        match value {
            0 => PBKR::_1_BANK,
            1 => PBKR::_2_BANK,
            2 => PBKR::_3_BANK,
            i => PBKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1_BANK`"]
    #[inline]
    pub fn is_1_bank(&self) -> bool {
        *self == PBKR::_1_BANK
    }
    #[doc = "Checks if the value of the field is `_2_BANK`"]
    #[inline]
    pub fn is_2_bank(&self) -> bool {
        *self == PBKR::_2_BANK
    }
    #[doc = "Checks if the value of the field is `_3_BANK`"]
    #[inline]
    pub fn is_3_bank(&self) -> bool {
        *self == PBKR::_3_BANK
    }
}
#[doc = "Possible values of the field `PSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSIZER {
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
impl PSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSIZER::_8_BYTE => 0,
            PSIZER::_16_BYTE => 1,
            PSIZER::_32_BYTE => 2,
            PSIZER::_64_BYTE => 3,
            PSIZER::_128_BYTE => 4,
            PSIZER::_256_BYTE => 5,
            PSIZER::_512_BYTE => 6,
            PSIZER::_1024_BYTE => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSIZER {
        match value {
            0 => PSIZER::_8_BYTE,
            1 => PSIZER::_16_BYTE,
            2 => PSIZER::_32_BYTE,
            3 => PSIZER::_64_BYTE,
            4 => PSIZER::_128_BYTE,
            5 => PSIZER::_256_BYTE,
            6 => PSIZER::_512_BYTE,
            7 => PSIZER::_1024_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline]
    pub fn is_8_byte(&self) -> bool {
        *self == PSIZER::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline]
    pub fn is_16_byte(&self) -> bool {
        *self == PSIZER::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline]
    pub fn is_32_byte(&self) -> bool {
        *self == PSIZER::_32_BYTE
    }
    #[doc = "Checks if the value of the field is `_64_BYTE`"]
    #[inline]
    pub fn is_64_byte(&self) -> bool {
        *self == PSIZER::_64_BYTE
    }
    #[doc = "Checks if the value of the field is `_128_BYTE`"]
    #[inline]
    pub fn is_128_byte(&self) -> bool {
        *self == PSIZER::_128_BYTE
    }
    #[doc = "Checks if the value of the field is `_256_BYTE`"]
    #[inline]
    pub fn is_256_byte(&self) -> bool {
        *self == PSIZER::_256_BYTE
    }
    #[doc = "Checks if the value of the field is `_512_BYTE`"]
    #[inline]
    pub fn is_512_byte(&self) -> bool {
        *self == PSIZER::_512_BYTE
    }
    #[doc = "Checks if the value of the field is `_1024_BYTE`"]
    #[inline]
    pub fn is_1024_byte(&self) -> bool {
        *self == PSIZER::_1024_BYTE
    }
}
#[doc = "Possible values of the field `PTOKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTOKENR {
    #[doc = "SETUP"]
    SETUP,
    #[doc = "IN"]
    IN,
    #[doc = "OUT"]
    OUT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PTOKENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PTOKENR::SETUP => 0,
            PTOKENR::IN => 1,
            PTOKENR::OUT => 2,
            PTOKENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PTOKENR {
        match value {
            0 => PTOKENR::SETUP,
            1 => PTOKENR::IN,
            2 => PTOKENR::OUT,
            i => PTOKENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SETUP`"]
    #[inline]
    pub fn is_setup(&self) -> bool {
        *self == PTOKENR::SETUP
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == PTOKENR::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == PTOKENR::OUT
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
#[doc = "Possible values of the field `PTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTYPER {
    #[doc = "Control"]
    CTRL,
    #[doc = "Isochronous"]
    ISO,
    #[doc = "Bulk"]
    BLK,
    #[doc = "Interrupt"]
    INTRPT,
}
impl PTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PTYPER::CTRL => 0,
            PTYPER::ISO => 1,
            PTYPER::BLK => 2,
            PTYPER::INTRPT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PTYPER {
        match value {
            0 => PTYPER::CTRL,
            1 => PTYPER::ISO,
            2 => PTYPER::BLK,
            3 => PTYPER::INTRPT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CTRL`"]
    #[inline]
    pub fn is_ctrl(&self) -> bool {
        *self == PTYPER::CTRL
    }
    #[doc = "Checks if the value of the field is `ISO`"]
    #[inline]
    pub fn is_iso(&self) -> bool {
        *self == PTYPER::ISO
    }
    #[doc = "Checks if the value of the field is `BLK`"]
    #[inline]
    pub fn is_blk(&self) -> bool {
        *self == PTYPER::BLK
    }
    #[doc = "Checks if the value of the field is `INTRPT`"]
    #[inline]
    pub fn is_intrpt(&self) -> bool {
        *self == PTYPER::INTRPT
    }
}
#[doc = r" Value of the field"]
pub struct PEPNUMR {
    bits: u8,
}
impl PEPNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PINGENR {
    bits: bool,
}
impl PINGENR {
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
pub struct BINTERVALR {
    bits: u8,
}
impl BINTERVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
#[doc = "Values that can be written to the field `PBK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBKW {
    #[doc = "Single-bank pipe"]
    _1_BANK,
    #[doc = "Double-bank pipe"]
    _2_BANK,
    #[doc = "Triple-bank pipe"]
    _3_BANK,
}
impl PBKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PBKW::_1_BANK => 0,
            PBKW::_2_BANK => 1,
            PBKW::_3_BANK => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PBKW<'a> {
    w: &'a mut W,
}
impl<'a> _PBKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PBKW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Single-bank pipe"]
    #[inline]
    pub fn _1_bank(self) -> &'a mut W {
        self.variant(PBKW::_1_BANK)
    }
    #[doc = "Double-bank pipe"]
    #[inline]
    pub fn _2_bank(self) -> &'a mut W {
        self.variant(PBKW::_2_BANK)
    }
    #[doc = "Triple-bank pipe"]
    #[inline]
    pub fn _3_bank(self) -> &'a mut W {
        self.variant(PBKW::_3_BANK)
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
#[doc = "Values that can be written to the field `PSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSIZEW {
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
impl PSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSIZEW::_8_BYTE => 0,
            PSIZEW::_16_BYTE => 1,
            PSIZEW::_32_BYTE => 2,
            PSIZEW::_64_BYTE => 3,
            PSIZEW::_128_BYTE => 4,
            PSIZEW::_256_BYTE => 5,
            PSIZEW::_512_BYTE => 6,
            PSIZEW::_1024_BYTE => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _PSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "8 bytes"]
    #[inline]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(PSIZEW::_8_BYTE)
    }
    #[doc = "16 bytes"]
    #[inline]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(PSIZEW::_16_BYTE)
    }
    #[doc = "32 bytes"]
    #[inline]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(PSIZEW::_32_BYTE)
    }
    #[doc = "64 bytes"]
    #[inline]
    pub fn _64_byte(self) -> &'a mut W {
        self.variant(PSIZEW::_64_BYTE)
    }
    #[doc = "128 bytes"]
    #[inline]
    pub fn _128_byte(self) -> &'a mut W {
        self.variant(PSIZEW::_128_BYTE)
    }
    #[doc = "256 bytes"]
    #[inline]
    pub fn _256_byte(self) -> &'a mut W {
        self.variant(PSIZEW::_256_BYTE)
    }
    #[doc = "512 bytes"]
    #[inline]
    pub fn _512_byte(self) -> &'a mut W {
        self.variant(PSIZEW::_512_BYTE)
    }
    #[doc = "1024 bytes"]
    #[inline]
    pub fn _1024_byte(self) -> &'a mut W {
        self.variant(PSIZEW::_1024_BYTE)
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
#[doc = "Values that can be written to the field `PTOKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTOKENW {
    #[doc = "SETUP"]
    SETUP,
    #[doc = "IN"]
    IN,
    #[doc = "OUT"]
    OUT,
}
impl PTOKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PTOKENW::SETUP => 0,
            PTOKENW::IN => 1,
            PTOKENW::OUT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTOKENW<'a> {
    w: &'a mut W,
}
impl<'a> _PTOKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTOKENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SETUP"]
    #[inline]
    pub fn setup(self) -> &'a mut W {
        self.variant(PTOKENW::SETUP)
    }
    #[doc = "IN"]
    #[inline]
    pub fn in_(self) -> &'a mut W {
        self.variant(PTOKENW::IN)
    }
    #[doc = "OUT"]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(PTOKENW::OUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTYPEW {
    #[doc = "Control"]
    CTRL,
    #[doc = "Isochronous"]
    ISO,
    #[doc = "Bulk"]
    BLK,
    #[doc = "Interrupt"]
    INTRPT,
}
impl PTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PTYPEW::CTRL => 0,
            PTYPEW::ISO => 1,
            PTYPEW::BLK => 2,
            PTYPEW::INTRPT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _PTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTYPEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Control"]
    #[inline]
    pub fn ctrl(self) -> &'a mut W {
        self.variant(PTYPEW::CTRL)
    }
    #[doc = "Isochronous"]
    #[inline]
    pub fn iso(self) -> &'a mut W {
        self.variant(PTYPEW::ISO)
    }
    #[doc = "Bulk"]
    #[inline]
    pub fn blk(self) -> &'a mut W {
        self.variant(PTYPEW::BLK)
    }
    #[doc = "Interrupt"]
    #[inline]
    pub fn intrpt(self) -> &'a mut W {
        self.variant(PTYPEW::INTRPT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PEPNUMW<'a> {
    w: &'a mut W,
}
impl<'a> _PEPNUMW<'a> {
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
#[doc = r" Proxy"]
pub struct _PINGENW<'a> {
    w: &'a mut W,
}
impl<'a> _PINGENW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BINTERVALW<'a> {
    w: &'a mut W,
}
impl<'a> _BINTERVALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 1 - Pipe Memory Allocate"]
    #[inline]
    pub fn alloc(&self) -> ALLOCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ALLOCR { bits }
    }
    #[doc = "Bits 2:3 - Pipe Banks"]
    #[inline]
    pub fn pbk(&self) -> PBKR {
        PBKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline]
    pub fn psize(&self) -> PSIZER {
        PSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline]
    pub fn ptoken(&self) -> PTOKENR {
        PTOKENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Automatic Switch"]
    #[inline]
    pub fn autosw(&self) -> AUTOSWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        AUTOSWR { bits }
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline]
    pub fn ptype(&self) -> PTYPER {
        PTYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Pipe Endpoint Number"]
    #[inline]
    pub fn pepnum(&self) -> PEPNUMR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PEPNUMR { bits }
    }
    #[doc = "Bit 20 - Ping Enable"]
    #[inline]
    pub fn pingen(&self) -> PINGENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PINGENR { bits }
    }
    #[doc = "Bits 24:31 - bInterval Parameter for the Bulk-Out/Ping Transaction"]
    #[inline]
    pub fn binterval(&self) -> BINTERVALR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BINTERVALR { bits }
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
    #[doc = "Bit 1 - Pipe Memory Allocate"]
    #[inline]
    pub fn alloc(&mut self) -> _ALLOCW {
        _ALLOCW { w: self }
    }
    #[doc = "Bits 2:3 - Pipe Banks"]
    #[inline]
    pub fn pbk(&mut self) -> _PBKW {
        _PBKW { w: self }
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline]
    pub fn psize(&mut self) -> _PSIZEW {
        _PSIZEW { w: self }
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline]
    pub fn ptoken(&mut self) -> _PTOKENW {
        _PTOKENW { w: self }
    }
    #[doc = "Bit 10 - Automatic Switch"]
    #[inline]
    pub fn autosw(&mut self) -> _AUTOSWW {
        _AUTOSWW { w: self }
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline]
    pub fn ptype(&mut self) -> _PTYPEW {
        _PTYPEW { w: self }
    }
    #[doc = "Bits 16:19 - Pipe Endpoint Number"]
    #[inline]
    pub fn pepnum(&mut self) -> _PEPNUMW {
        _PEPNUMW { w: self }
    }
    #[doc = "Bit 20 - Ping Enable"]
    #[inline]
    pub fn pingen(&mut self) -> _PINGENW {
        _PINGENW { w: self }
    }
    #[doc = "Bits 24:31 - bInterval Parameter for the Bulk-Out/Ping Transaction"]
    #[inline]
    pub fn binterval(&mut self) -> _BINTERVALW {
        _BINTERVALW { w: self }
    }
}
