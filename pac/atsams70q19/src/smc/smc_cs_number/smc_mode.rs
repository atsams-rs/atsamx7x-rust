#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SMC_MODE {
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
pub type READ_MODE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _READ_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _READ_MODEW<'a> {
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
#[doc = r"Reader of the field"]
pub type WRITE_MODE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WRITE_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WRITE_MODEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
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
}
impl crate::ToBits<u8> for EXNW_MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            EXNW_MODER::DISABLED => 0,
            EXNW_MODER::FROZEN => 2,
            EXNW_MODER::READY => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type EXNW_MODE_R = crate::FR<u8, EXNW_MODER>;
impl EXNW_MODE_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXNW_MODER::DISABLED
    }
    #[doc = "Checks if the value of the field is `FROZEN`"]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == EXNW_MODER::FROZEN
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == EXNW_MODER::READY
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXNW_MODEW::DISABLED => 0,
            EXNW_MODEW::FROZEN => 2,
            EXNW_MODEW::READY => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EXNW_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EXNW_MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXNW_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXNW_MODEW::DISABLED)
    }
    #[doc = "Frozen Mode"]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut W {
        self.variant(EXNW_MODEW::FROZEN)
    }
    #[doc = "Ready Mode"]
    #[inline(always)]
    pub fn ready(self) -> &'a mut W {
        self.variant(EXNW_MODEW::READY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
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
impl crate::ToBits<bool> for BATR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BATR::BYTE_SELECT => false,
            BATR::BYTE_WRITE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BAT_R = crate::FR<bool, BATR>;
impl BAT_R {
    #[doc = "Checks if the value of the field is `BYTE_SELECT`"]
    #[inline(always)]
    pub fn is_byte_select(&self) -> bool {
        *self == BATR::BYTE_SELECT
    }
    #[doc = "Checks if the value of the field is `BYTE_WRITE`"]
    #[inline(always)]
    pub fn is_byte_write(&self) -> bool {
        *self == BATR::BYTE_WRITE
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
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BATW::BYTE_SELECT => false,
            BATW::BYTE_WRITE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BATW<'a> {
    w: &'a mut W,
}
impl<'a> _BATW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Byte select access type:- Write operation is controlled using NCS, NWE, NBS0, NBS1.- Read operation is controlled using NCS, NRD, NBS0, NBS1."]
    #[inline(always)]
    pub fn byte_select(self) -> &'a mut W {
        self.variant(BATW::BYTE_SELECT)
    }
    #[doc = "Byte write access type:- Write operation is controlled using NCS, NWR0, NWR1.- Read operation is controlled using NCS and NRD."]
    #[inline(always)]
    pub fn byte_write(self) -> &'a mut W {
        self.variant(BATW::BYTE_WRITE)
    }
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
#[doc = "Possible values of the field `DBW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBWR {
    #[doc = "8-bit Data Bus"]
    _8_BIT,
    #[doc = "16-bit Data Bus"]
    _16_BIT,
}
impl crate::ToBits<bool> for DBWR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DBWR::_8_BIT => false,
            DBWR::_16_BIT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DBW_R = crate::FR<bool, DBWR>;
impl DBW_R {
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == DBWR::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == DBWR::_16_BIT
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
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DBWW::_8_BIT => false,
            DBWW::_16_BIT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DBWW<'a> {
    w: &'a mut W,
}
impl<'a> _DBWW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "8-bit Data Bus"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(DBWW::_8_BIT)
    }
    #[doc = "16-bit Data Bus"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(DBWW::_16_BIT)
    }
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
pub type TDF_CYCLES_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TDF_CYCLESW<'a> {
    w: &'a mut W,
}
impl<'a> _TDF_CYCLESW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TDF_MODE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TDF_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TDF_MODEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PMEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PMENW<'a> {
    w: &'a mut W,
}
impl<'a> _PMENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
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
impl crate::ToBits<u8> for PSR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PSR::_4_BYTE => 0,
            PSR::_8_BYTE => 1,
            PSR::_16_BYTE => 2,
            PSR::_32_BYTE => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PS_R = crate::FR<u8, PSR>;
impl PS_R {
    #[doc = "Checks if the value of the field is `_4_BYTE`"]
    #[inline(always)]
    pub fn is_4_byte(&self) -> bool {
        *self == PSR::_4_BYTE
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == PSR::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == PSR::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == PSR::_32_BYTE
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSW::_4_BYTE => 0,
            PSW::_8_BYTE => 1,
            PSW::_16_BYTE => 2,
            PSW::_32_BYTE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PSW<'a> {
    w: &'a mut W,
}
impl<'a> _PSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "4-byte page"]
    #[inline(always)]
    pub fn _4_byte(self) -> &'a mut W {
        self.variant(PSW::_4_BYTE)
    }
    #[doc = "8-byte page"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(PSW::_8_BYTE)
    }
    #[doc = "16-byte page"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(PSW::_16_BYTE)
    }
    #[doc = "32-byte page"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(PSW::_32_BYTE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Read Mode"]
    #[inline(always)]
    pub fn read_mode(&self) -> READ_MODE_R {
        READ_MODE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write Mode"]
    #[inline(always)]
    pub fn write_mode(&self) -> WRITE_MODE_R {
        WRITE_MODE_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&self) -> EXNW_MODE_R {
        EXNW_MODE_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline(always)]
    pub fn bat(&self) -> BAT_R {
        BAT_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    pub fn tdf_cycles(&self) -> TDF_CYCLES_R {
        TDF_CYCLES_R::new(((self.bits() >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    pub fn tdf_mode(&self) -> TDF_MODE_R {
        TDF_MODE_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Page Mode Enabled"]
    #[inline(always)]
    pub fn pmen(&self) -> PMEN_R {
        PMEN_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Page Size"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits() >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Read Mode"]
    #[inline(always)]
    pub fn read_mode(&mut self) -> _READ_MODEW {
        _READ_MODEW { w: self }
    }
    #[doc = "Bit 1 - Write Mode"]
    #[inline(always)]
    pub fn write_mode(&mut self) -> _WRITE_MODEW {
        _WRITE_MODEW { w: self }
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&mut self) -> _EXNW_MODEW {
        _EXNW_MODEW { w: self }
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline(always)]
    pub fn bat(&mut self) -> _BATW {
        _BATW { w: self }
    }
    #[doc = "Bit 12 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&mut self) -> _DBWW {
        _DBWW { w: self }
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    pub fn tdf_cycles(&mut self) -> _TDF_CYCLESW {
        _TDF_CYCLESW { w: self }
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    pub fn tdf_mode(&mut self) -> _TDF_MODEW {
        _TDF_MODEW { w: self }
    }
    #[doc = "Bit 24 - Page Mode Enabled"]
    #[inline(always)]
    pub fn pmen(&mut self) -> _PMENW {
        _PMENW { w: self }
    }
    #[doc = "Bits 28:29 - Page Size"]
    #[inline(always)]
    pub fn ps(&mut self) -> _PSW {
        _PSW { w: self }
    }
}
