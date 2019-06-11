#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDRAMC_CR {
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
#[doc = "Possible values of the field `NC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCR {
    #[doc = "8 column bits"]
    COL8,
    #[doc = "9 column bits"]
    COL9,
    #[doc = "10 column bits"]
    COL10,
    #[doc = "11 column bits"]
    COL11,
}
impl NCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NCR::COL8 => 0,
            NCR::COL9 => 1,
            NCR::COL10 => 2,
            NCR::COL11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NCR {
        match value {
            0 => NCR::COL8,
            1 => NCR::COL9,
            2 => NCR::COL10,
            3 => NCR::COL11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COL8`"]
    #[inline]
    pub fn is_col8(&self) -> bool {
        *self == NCR::COL8
    }
    #[doc = "Checks if the value of the field is `COL9`"]
    #[inline]
    pub fn is_col9(&self) -> bool {
        *self == NCR::COL9
    }
    #[doc = "Checks if the value of the field is `COL10`"]
    #[inline]
    pub fn is_col10(&self) -> bool {
        *self == NCR::COL10
    }
    #[doc = "Checks if the value of the field is `COL11`"]
    #[inline]
    pub fn is_col11(&self) -> bool {
        *self == NCR::COL11
    }
}
#[doc = "Possible values of the field `NR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRR {
    #[doc = "11 row bits"]
    ROW11,
    #[doc = "12 row bits"]
    ROW12,
    #[doc = "13 row bits"]
    ROW13,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NRR::ROW11 => 0,
            NRR::ROW12 => 1,
            NRR::ROW13 => 2,
            NRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NRR {
        match value {
            0 => NRR::ROW11,
            1 => NRR::ROW12,
            2 => NRR::ROW13,
            i => NRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ROW11`"]
    #[inline]
    pub fn is_row11(&self) -> bool {
        *self == NRR::ROW11
    }
    #[doc = "Checks if the value of the field is `ROW12`"]
    #[inline]
    pub fn is_row12(&self) -> bool {
        *self == NRR::ROW12
    }
    #[doc = "Checks if the value of the field is `ROW13`"]
    #[inline]
    pub fn is_row13(&self) -> bool {
        *self == NRR::ROW13
    }
}
#[doc = "Possible values of the field `NB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBR {
    #[doc = "2 banks"]
    BANK2,
    #[doc = "4 banks"]
    BANK4,
}
impl NBR {
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
            NBR::BANK2 => false,
            NBR::BANK4 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NBR {
        match value {
            false => NBR::BANK2,
            true => NBR::BANK4,
        }
    }
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline]
    pub fn is_bank2(&self) -> bool {
        *self == NBR::BANK2
    }
    #[doc = "Checks if the value of the field is `BANK4`"]
    #[inline]
    pub fn is_bank4(&self) -> bool {
        *self == NBR::BANK4
    }
}
#[doc = "Possible values of the field `CAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASR {
    #[doc = "1 cycle CAS latency"]
    LATENCY1,
    #[doc = "2 cycle CAS latency"]
    LATENCY2,
    #[doc = "3 cycle CAS latency"]
    LATENCY3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CASR::LATENCY1 => 1,
            CASR::LATENCY2 => 2,
            CASR::LATENCY3 => 3,
            CASR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CASR {
        match value {
            1 => CASR::LATENCY1,
            2 => CASR::LATENCY2,
            3 => CASR::LATENCY3,
            i => CASR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LATENCY1`"]
    #[inline]
    pub fn is_latency1(&self) -> bool {
        *self == CASR::LATENCY1
    }
    #[doc = "Checks if the value of the field is `LATENCY2`"]
    #[inline]
    pub fn is_latency2(&self) -> bool {
        *self == CASR::LATENCY2
    }
    #[doc = "Checks if the value of the field is `LATENCY3`"]
    #[inline]
    pub fn is_latency3(&self) -> bool {
        *self == CASR::LATENCY3
    }
}
#[doc = r" Value of the field"]
pub struct DBWR {
    bits: bool,
}
impl DBWR {
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
pub struct TWRR {
    bits: u8,
}
impl TWRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRC_TRFCR {
    bits: u8,
}
impl TRC_TRFCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRPR {
    bits: u8,
}
impl TRPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRCDR {
    bits: u8,
}
impl TRCDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRASR {
    bits: u8,
}
impl TRASR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXSRR {
    bits: u8,
}
impl TXSRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `NC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCW {
    #[doc = "8 column bits"]
    COL8,
    #[doc = "9 column bits"]
    COL9,
    #[doc = "10 column bits"]
    COL10,
    #[doc = "11 column bits"]
    COL11,
}
impl NCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NCW::COL8 => 0,
            NCW::COL9 => 1,
            NCW::COL10 => 2,
            NCW::COL11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NCW<'a> {
    w: &'a mut W,
}
impl<'a> _NCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "8 column bits"]
    #[inline]
    pub fn col8(self) -> &'a mut W {
        self.variant(NCW::COL8)
    }
    #[doc = "9 column bits"]
    #[inline]
    pub fn col9(self) -> &'a mut W {
        self.variant(NCW::COL9)
    }
    #[doc = "10 column bits"]
    #[inline]
    pub fn col10(self) -> &'a mut W {
        self.variant(NCW::COL10)
    }
    #[doc = "11 column bits"]
    #[inline]
    pub fn col11(self) -> &'a mut W {
        self.variant(NCW::COL11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRW {
    #[doc = "11 row bits"]
    ROW11,
    #[doc = "12 row bits"]
    ROW12,
    #[doc = "13 row bits"]
    ROW13,
}
impl NRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NRW::ROW11 => 0,
            NRW::ROW12 => 1,
            NRW::ROW13 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NRW<'a> {
    w: &'a mut W,
}
impl<'a> _NRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "11 row bits"]
    #[inline]
    pub fn row11(self) -> &'a mut W {
        self.variant(NRW::ROW11)
    }
    #[doc = "12 row bits"]
    #[inline]
    pub fn row12(self) -> &'a mut W {
        self.variant(NRW::ROW12)
    }
    #[doc = "13 row bits"]
    #[inline]
    pub fn row13(self) -> &'a mut W {
        self.variant(NRW::ROW13)
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
#[doc = "Values that can be written to the field `NB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBW {
    #[doc = "2 banks"]
    BANK2,
    #[doc = "4 banks"]
    BANK4,
}
impl NBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NBW::BANK2 => false,
            NBW::BANK4 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NBW<'a> {
    w: &'a mut W,
}
impl<'a> _NBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "2 banks"]
    #[inline]
    pub fn bank2(self) -> &'a mut W {
        self.variant(NBW::BANK2)
    }
    #[doc = "4 banks"]
    #[inline]
    pub fn bank4(self) -> &'a mut W {
        self.variant(NBW::BANK4)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASW {
    #[doc = "1 cycle CAS latency"]
    LATENCY1,
    #[doc = "2 cycle CAS latency"]
    LATENCY2,
    #[doc = "3 cycle CAS latency"]
    LATENCY3,
}
impl CASW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CASW::LATENCY1 => 1,
            CASW::LATENCY2 => 2,
            CASW::LATENCY3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CASW<'a> {
    w: &'a mut W,
}
impl<'a> _CASW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CASW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 cycle CAS latency"]
    #[inline]
    pub fn latency1(self) -> &'a mut W {
        self.variant(CASW::LATENCY1)
    }
    #[doc = "2 cycle CAS latency"]
    #[inline]
    pub fn latency2(self) -> &'a mut W {
        self.variant(CASW::LATENCY2)
    }
    #[doc = "3 cycle CAS latency"]
    #[inline]
    pub fn latency3(self) -> &'a mut W {
        self.variant(CASW::LATENCY3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DBWW<'a> {
    w: &'a mut W,
}
impl<'a> _DBWW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TWRW<'a> {
    w: &'a mut W,
}
impl<'a> _TWRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRC_TRFCW<'a> {
    w: &'a mut W,
}
impl<'a> _TRC_TRFCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRPW<'a> {
    w: &'a mut W,
}
impl<'a> _TRPW<'a> {
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
pub struct _TRCDW<'a> {
    w: &'a mut W,
}
impl<'a> _TRCDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRASW<'a> {
    w: &'a mut W,
}
impl<'a> _TRASW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXSRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:1 - Number of Column Bits"]
    #[inline]
    pub fn nc(&self) -> NCR {
        NCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Number of Row Bits"]
    #[inline]
    pub fn nr(&self) -> NRR {
        NRR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Number of Banks"]
    #[inline]
    pub fn nb(&self) -> NBR {
        NBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - CAS Latency"]
    #[inline]
    pub fn cas(&self) -> CASR {
        CASR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Data Bus Width"]
    #[inline]
    pub fn dbw(&self) -> DBWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DBWR { bits }
    }
    #[doc = "Bits 8:11 - Write Recovery Delay"]
    #[inline]
    pub fn twr(&self) -> TWRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TWRR { bits }
    }
    #[doc = "Bits 12:15 - Row Cycle Delay and Row Refresh Cycle"]
    #[inline]
    pub fn trc_trfc(&self) -> TRC_TRFCR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRC_TRFCR { bits }
    }
    #[doc = "Bits 16:19 - Row Precharge Delay"]
    #[inline]
    pub fn trp(&self) -> TRPR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRPR { bits }
    }
    #[doc = "Bits 20:23 - Row to Column Delay"]
    #[inline]
    pub fn trcd(&self) -> TRCDR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRCDR { bits }
    }
    #[doc = "Bits 24:27 - Active to Precharge Delay"]
    #[inline]
    pub fn tras(&self) -> TRASR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRASR { bits }
    }
    #[doc = "Bits 28:31 - Exit Self-Refresh to Active Delay"]
    #[inline]
    pub fn txsr(&self) -> TXSRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXSRR { bits }
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
    #[doc = "Bits 0:1 - Number of Column Bits"]
    #[inline]
    pub fn nc(&mut self) -> _NCW {
        _NCW { w: self }
    }
    #[doc = "Bits 2:3 - Number of Row Bits"]
    #[inline]
    pub fn nr(&mut self) -> _NRW {
        _NRW { w: self }
    }
    #[doc = "Bit 4 - Number of Banks"]
    #[inline]
    pub fn nb(&mut self) -> _NBW {
        _NBW { w: self }
    }
    #[doc = "Bits 5:6 - CAS Latency"]
    #[inline]
    pub fn cas(&mut self) -> _CASW {
        _CASW { w: self }
    }
    #[doc = "Bit 7 - Data Bus Width"]
    #[inline]
    pub fn dbw(&mut self) -> _DBWW {
        _DBWW { w: self }
    }
    #[doc = "Bits 8:11 - Write Recovery Delay"]
    #[inline]
    pub fn twr(&mut self) -> _TWRW {
        _TWRW { w: self }
    }
    #[doc = "Bits 12:15 - Row Cycle Delay and Row Refresh Cycle"]
    #[inline]
    pub fn trc_trfc(&mut self) -> _TRC_TRFCW {
        _TRC_TRFCW { w: self }
    }
    #[doc = "Bits 16:19 - Row Precharge Delay"]
    #[inline]
    pub fn trp(&mut self) -> _TRPW {
        _TRPW { w: self }
    }
    #[doc = "Bits 20:23 - Row to Column Delay"]
    #[inline]
    pub fn trcd(&mut self) -> _TRCDW {
        _TRCDW { w: self }
    }
    #[doc = "Bits 24:27 - Active to Precharge Delay"]
    #[inline]
    pub fn tras(&mut self) -> _TRASW {
        _TRASW { w: self }
    }
    #[doc = "Bits 28:31 - Exit Self-Refresh to Active Delay"]
    #[inline]
    pub fn txsr(&mut self) -> _TXSRW {
        _TXSRW { w: self }
    }
}
