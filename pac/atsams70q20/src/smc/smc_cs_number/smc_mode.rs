#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SMC_MODE {
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
pub struct READ_MODER {
    bits: bool,
}
impl READ_MODER {
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
pub struct WRITE_MODER {
    bits: bool,
}
impl WRITE_MODER {
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
#[doc = "Possible values of the field `EXNW_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXNW_MODER {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Frozen Mode"]
    FROZEN,
    #[doc = "Ready Mode"]
    READY,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXNW_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXNW_MODER::DISABLED => 0,
            EXNW_MODER::FROZEN => 2,
            EXNW_MODER::READY => 3,
            EXNW_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXNW_MODER {
        match value {
            0 => EXNW_MODER::DISABLED,
            2 => EXNW_MODER::FROZEN,
            3 => EXNW_MODER::READY,
            i => EXNW_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EXNW_MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `FROZEN`"]
    #[inline]
    pub fn is_frozen(&self) -> bool {
        *self == EXNW_MODER::FROZEN
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline]
    pub fn is_ready(&self) -> bool {
        *self == EXNW_MODER::READY
    }
}
#[doc = "Possible values of the field `BAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BATR {
    #[doc = "Byte select access type:- Write operation is controlled using NCS, NWE, NBS0, NBS1.- Read operation is controlled using NCS, NRD, NBS0, NBS1."]
    BYTE_SELECT,
    #[doc = "Byte write access type:- Write operation is controlled using NCS, NWR0, NWR1.- Read operation is controlled using NCS and NRD."]
    BYTE_WRITE,
}
impl BATR {
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
            BATR::BYTE_SELECT => false,
            BATR::BYTE_WRITE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BATR {
        match value {
            false => BATR::BYTE_SELECT,
            true => BATR::BYTE_WRITE,
        }
    }
    #[doc = "Checks if the value of the field is `BYTE_SELECT`"]
    #[inline]
    pub fn is_byte_select(&self) -> bool {
        *self == BATR::BYTE_SELECT
    }
    #[doc = "Checks if the value of the field is `BYTE_WRITE`"]
    #[inline]
    pub fn is_byte_write(&self) -> bool {
        *self == BATR::BYTE_WRITE
    }
}
#[doc = "Possible values of the field `DBW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBWR {
    #[doc = "8-bit Data Bus"]
    _8_BIT,
    #[doc = "16-bit Data Bus"]
    _16_BIT,
}
impl DBWR {
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
            DBWR::_8_BIT => false,
            DBWR::_16_BIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBWR {
        match value {
            false => DBWR::_8_BIT,
            true => DBWR::_16_BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline]
    pub fn is_8_bit(&self) -> bool {
        *self == DBWR::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline]
    pub fn is_16_bit(&self) -> bool {
        *self == DBWR::_16_BIT
    }
}
#[doc = r" Value of the field"]
pub struct TDF_CYCLESR {
    bits: u8,
}
impl TDF_CYCLESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TDF_MODER {
    bits: bool,
}
impl TDF_MODER {
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
pub struct PMENR {
    bits: bool,
}
impl PMENR {
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
#[doc = "Possible values of the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSR {
    #[doc = "4-byte page"]
    _4_BYTE,
    #[doc = "8-byte page"]
    _8_BYTE,
    #[doc = "16-byte page"]
    _16_BYTE,
    #[doc = "32-byte page"]
    _32_BYTE,
}
impl PSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSR::_4_BYTE => 0,
            PSR::_8_BYTE => 1,
            PSR::_16_BYTE => 2,
            PSR::_32_BYTE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSR {
        match value {
            0 => PSR::_4_BYTE,
            1 => PSR::_8_BYTE,
            2 => PSR::_16_BYTE,
            3 => PSR::_32_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4_BYTE`"]
    #[inline]
    pub fn is_4_byte(&self) -> bool {
        *self == PSR::_4_BYTE
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline]
    pub fn is_8_byte(&self) -> bool {
        *self == PSR::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline]
    pub fn is_16_byte(&self) -> bool {
        *self == PSR::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline]
    pub fn is_32_byte(&self) -> bool {
        *self == PSR::_32_BYTE
    }
}
#[doc = r" Proxy"]
pub struct _READ_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_MODEW<'a> {
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
#[doc = r" Proxy"]
pub struct _WRITE_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WRITE_MODEW<'a> {
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
#[doc = "Values that can be written to the field `EXNW_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXNW_MODEW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Frozen Mode"]
    FROZEN,
    #[doc = "Ready Mode"]
    READY,
}
impl EXNW_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXNW_MODEW::DISABLED => 0,
            EXNW_MODEW::FROZEN => 2,
            EXNW_MODEW::READY => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXNW_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EXNW_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXNW_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXNW_MODEW::DISABLED)
    }
    #[doc = "Frozen Mode"]
    #[inline]
    pub fn frozen(self) -> &'a mut W {
        self.variant(EXNW_MODEW::FROZEN)
    }
    #[doc = "Ready Mode"]
    #[inline]
    pub fn ready(self) -> &'a mut W {
        self.variant(EXNW_MODEW::READY)
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
#[doc = "Values that can be written to the field `BAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BATW {
    #[doc = "Byte select access type:- Write operation is controlled using NCS, NWE, NBS0, NBS1.- Read operation is controlled using NCS, NRD, NBS0, NBS1."]
    BYTE_SELECT,
    #[doc = "Byte write access type:- Write operation is controlled using NCS, NWR0, NWR1.- Read operation is controlled using NCS and NRD."]
    BYTE_WRITE,
}
impl BATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BATW::BYTE_SELECT => false,
            BATW::BYTE_WRITE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BATW<'a> {
    w: &'a mut W,
}
impl<'a> _BATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Byte select access type:- Write operation is controlled using NCS, NWE, NBS0, NBS1.- Read operation is controlled using NCS, NRD, NBS0, NBS1."]
    #[inline]
    pub fn byte_select(self) -> &'a mut W {
        self.variant(BATW::BYTE_SELECT)
    }
    #[doc = "Byte write access type:- Write operation is controlled using NCS, NWR0, NWR1.- Read operation is controlled using NCS and NRD."]
    #[inline]
    pub fn byte_write(self) -> &'a mut W {
        self.variant(BATW::BYTE_WRITE)
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
#[doc = "Values that can be written to the field `DBW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBWW {
    #[doc = "8-bit Data Bus"]
    _8_BIT,
    #[doc = "16-bit Data Bus"]
    _16_BIT,
}
impl DBWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBWW::_8_BIT => false,
            DBWW::_16_BIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBWW<'a> {
    w: &'a mut W,
}
impl<'a> _DBWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "8-bit Data Bus"]
    #[inline]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(DBWW::_8_BIT)
    }
    #[doc = "16-bit Data Bus"]
    #[inline]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(DBWW::_16_BIT)
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
pub struct _TDF_CYCLESW<'a> {
    w: &'a mut W,
}
impl<'a> _TDF_CYCLESW<'a> {
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
pub struct _TDF_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TDF_MODEW<'a> {
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
pub struct _PMENW<'a> {
    w: &'a mut W,
}
impl<'a> _PMENW<'a> {
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
#[doc = "Values that can be written to the field `PS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSW {
    #[doc = "4-byte page"]
    _4_BYTE,
    #[doc = "8-byte page"]
    _8_BYTE,
    #[doc = "16-byte page"]
    _16_BYTE,
    #[doc = "32-byte page"]
    _32_BYTE,
}
impl PSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSW::_4_BYTE => 0,
            PSW::_8_BYTE => 1,
            PSW::_16_BYTE => 2,
            PSW::_32_BYTE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4-byte page"]
    #[inline]
    pub fn _4_byte(self) -> &'a mut W {
        self.variant(PSW::_4_BYTE)
    }
    #[doc = "8-byte page"]
    #[inline]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(PSW::_8_BYTE)
    }
    #[doc = "16-byte page"]
    #[inline]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(PSW::_16_BYTE)
    }
    #[doc = "32-byte page"]
    #[inline]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(PSW::_32_BYTE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 0 - Read Mode"]
    #[inline]
    pub fn read_mode(&self) -> READ_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        READ_MODER { bits }
    }
    #[doc = "Bit 1 - Write Mode"]
    #[inline]
    pub fn write_mode(&self) -> WRITE_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRITE_MODER { bits }
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline]
    pub fn exnw_mode(&self) -> EXNW_MODER {
        EXNW_MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline]
    pub fn bat(&self) -> BATR {
        BATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline]
    pub fn dbw(&self) -> DBWR {
        DBWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline]
    pub fn tdf_cycles(&self) -> TDF_CYCLESR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TDF_CYCLESR { bits }
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline]
    pub fn tdf_mode(&self) -> TDF_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TDF_MODER { bits }
    }
    #[doc = "Bit 24 - Page Mode Enabled"]
    #[inline]
    pub fn pmen(&self) -> PMENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PMENR { bits }
    }
    #[doc = "Bits 28:29 - Page Size"]
    #[inline]
    pub fn ps(&self) -> PSR {
        PSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
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
    #[doc = "Bit 0 - Read Mode"]
    #[inline]
    pub fn read_mode(&mut self) -> _READ_MODEW {
        _READ_MODEW { w: self }
    }
    #[doc = "Bit 1 - Write Mode"]
    #[inline]
    pub fn write_mode(&mut self) -> _WRITE_MODEW {
        _WRITE_MODEW { w: self }
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline]
    pub fn exnw_mode(&mut self) -> _EXNW_MODEW {
        _EXNW_MODEW { w: self }
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline]
    pub fn bat(&mut self) -> _BATW {
        _BATW { w: self }
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline]
    pub fn dbw(&mut self) -> _DBWW {
        _DBWW { w: self }
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline]
    pub fn tdf_cycles(&mut self) -> _TDF_CYCLESW {
        _TDF_CYCLESW { w: self }
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline]
    pub fn tdf_mode(&mut self) -> _TDF_MODEW {
        _TDF_MODEW { w: self }
    }
    #[doc = "Bit 24 - Page Mode Enabled"]
    #[inline]
    pub fn pmen(&mut self) -> _PMENW {
        _PMENW { w: self }
    }
    #[doc = "Bits 28:29 - Page Size"]
    #[inline]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
}
