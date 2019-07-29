#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SDRAMC_CR {
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
impl crate::ToBits<u8> for NCR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            NCR::COL8 => 0,
            NCR::COL9 => 1,
            NCR::COL10 => 2,
            NCR::COL11 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NC_R = crate::FR<u8, NCR>;
impl NC_R {
    #[doc = "Checks if the value of the field is `COL8`"]
    #[inline(always)]
    pub fn is_col8(&self) -> bool {
        *self == NCR::COL8
    }
    #[doc = "Checks if the value of the field is `COL9`"]
    #[inline(always)]
    pub fn is_col9(&self) -> bool {
        *self == NCR::COL9
    }
    #[doc = "Checks if the value of the field is `COL10`"]
    #[inline(always)]
    pub fn is_col10(&self) -> bool {
        *self == NCR::COL10
    }
    #[doc = "Checks if the value of the field is `COL11`"]
    #[inline(always)]
    pub fn is_col11(&self) -> bool {
        *self == NCR::COL11
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            NCW::COL8 => 0,
            NCW::COL9 => 1,
            NCW::COL10 => 2,
            NCW::COL11 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _NCW<'a> {
    w: &'a mut W,
}
impl<'a> _NCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "8 column bits"]
    #[inline(always)]
    pub fn col8(self) -> &'a mut W {
        self.variant(NCW::COL8)
    }
    #[doc = "9 column bits"]
    #[inline(always)]
    pub fn col9(self) -> &'a mut W {
        self.variant(NCW::COL9)
    }
    #[doc = "10 column bits"]
    #[inline(always)]
    pub fn col10(self) -> &'a mut W {
        self.variant(NCW::COL10)
    }
    #[doc = "11 column bits"]
    #[inline(always)]
    pub fn col11(self) -> &'a mut W {
        self.variant(NCW::COL11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
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
}
impl crate::ToBits<u8> for NRR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            NRR::ROW11 => 0,
            NRR::ROW12 => 1,
            NRR::ROW13 => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NR_R = crate::FR<u8, NRR>;
impl NR_R {
    #[doc = "Checks if the value of the field is `ROW11`"]
    #[inline(always)]
    pub fn is_row11(&self) -> bool {
        *self == NRR::ROW11
    }
    #[doc = "Checks if the value of the field is `ROW12`"]
    #[inline(always)]
    pub fn is_row12(&self) -> bool {
        *self == NRR::ROW12
    }
    #[doc = "Checks if the value of the field is `ROW13`"]
    #[inline(always)]
    pub fn is_row13(&self) -> bool {
        *self == NRR::ROW13
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            NRW::ROW11 => 0,
            NRW::ROW12 => 1,
            NRW::ROW13 => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _NRW<'a> {
    w: &'a mut W,
}
impl<'a> _NRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "11 row bits"]
    #[inline(always)]
    pub fn row11(self) -> &'a mut W {
        self.variant(NRW::ROW11)
    }
    #[doc = "12 row bits"]
    #[inline(always)]
    pub fn row12(self) -> &'a mut W {
        self.variant(NRW::ROW12)
    }
    #[doc = "13 row bits"]
    #[inline(always)]
    pub fn row13(self) -> &'a mut W {
        self.variant(NRW::ROW13)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
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
impl crate::ToBits<bool> for NBR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            NBR::BANK2 => false,
            NBR::BANK4 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NB_R = crate::FR<bool, NBR>;
impl NB_R {
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == NBR::BANK2
    }
    #[doc = "Checks if the value of the field is `BANK4`"]
    #[inline(always)]
    pub fn is_bank4(&self) -> bool {
        *self == NBR::BANK4
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
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            NBW::BANK2 => false,
            NBW::BANK4 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _NBW<'a> {
    w: &'a mut W,
}
impl<'a> _NBW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "2 banks"]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut W {
        self.variant(NBW::BANK2)
    }
    #[doc = "4 banks"]
    #[inline(always)]
    pub fn bank4(self) -> &'a mut W {
        self.variant(NBW::BANK4)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
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
}
impl crate::ToBits<u8> for CASR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CASR::LATENCY1 => 1,
            CASR::LATENCY2 => 2,
            CASR::LATENCY3 => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CAS_R = crate::FR<u8, CASR>;
impl CAS_R {
    #[doc = "Checks if the value of the field is `LATENCY1`"]
    #[inline(always)]
    pub fn is_latency1(&self) -> bool {
        *self == CASR::LATENCY1
    }
    #[doc = "Checks if the value of the field is `LATENCY2`"]
    #[inline(always)]
    pub fn is_latency2(&self) -> bool {
        *self == CASR::LATENCY2
    }
    #[doc = "Checks if the value of the field is `LATENCY3`"]
    #[inline(always)]
    pub fn is_latency3(&self) -> bool {
        *self == CASR::LATENCY3
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CASW::LATENCY1 => 1,
            CASW::LATENCY2 => 2,
            CASW::LATENCY3 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CASW<'a> {
    w: &'a mut W,
}
impl<'a> _CASW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CASW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 cycle CAS latency"]
    #[inline(always)]
    pub fn latency1(self) -> &'a mut W {
        self.variant(CASW::LATENCY1)
    }
    #[doc = "2 cycle CAS latency"]
    #[inline(always)]
    pub fn latency2(self) -> &'a mut W {
        self.variant(CASW::LATENCY2)
    }
    #[doc = "3 cycle CAS latency"]
    #[inline(always)]
    pub fn latency3(self) -> &'a mut W {
        self.variant(CASW::LATENCY3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DBW_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DBWW<'a> {
    w: &'a mut W,
}
impl<'a> _DBWW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TWR_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TWRW<'a> {
    w: &'a mut W,
}
impl<'a> _TWRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TRC_TRFC_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TRC_TRFCW<'a> {
    w: &'a mut W,
}
impl<'a> _TRC_TRFCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TRP_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TRPW<'a> {
    w: &'a mut W,
}
impl<'a> _TRPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TRCD_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TRCDW<'a> {
    w: &'a mut W,
}
impl<'a> _TRCDW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TRAS_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TRASW<'a> {
    w: &'a mut W,
}
impl<'a> _TRASW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TXSR_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TXSRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Number of Column Bits"]
    #[inline(always)]
    pub fn nc(&self) -> NC_R {
        NC_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Number of Row Bits"]
    #[inline(always)]
    pub fn nr(&self) -> NR_R {
        NR_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Number of Banks"]
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - CAS Latency"]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits() >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Write Recovery Delay"]
    #[inline(always)]
    pub fn twr(&self) -> TWR_R {
        TWR_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Row Cycle Delay and Row Refresh Cycle"]
    #[inline(always)]
    pub fn trc_trfc(&self) -> TRC_TRFC_R {
        TRC_TRFC_R::new(((self.bits() >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Row Precharge Delay"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits() >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Row to Column Delay"]
    #[inline(always)]
    pub fn trcd(&self) -> TRCD_R {
        TRCD_R::new(((self.bits() >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Active to Precharge Delay"]
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new(((self.bits() >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Exit Self-Refresh to Active Delay"]
    #[inline(always)]
    pub fn txsr(&self) -> TXSR_R {
        TXSR_R::new(((self.bits() >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Number of Column Bits"]
    #[inline(always)]
    pub fn nc(&mut self) -> _NCW {
        _NCW { w: self }
    }
    #[doc = "Bits 2:3 - Number of Row Bits"]
    #[inline(always)]
    pub fn nr(&mut self) -> _NRW {
        _NRW { w: self }
    }
    #[doc = "Bit 4 - Number of Banks"]
    #[inline(always)]
    pub fn nb(&mut self) -> _NBW {
        _NBW { w: self }
    }
    #[doc = "Bits 5:6 - CAS Latency"]
    #[inline(always)]
    pub fn cas(&mut self) -> _CASW {
        _CASW { w: self }
    }
    #[doc = "Bit 7 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&mut self) -> _DBWW {
        _DBWW { w: self }
    }
    #[doc = "Bits 8:11 - Write Recovery Delay"]
    #[inline(always)]
    pub fn twr(&mut self) -> _TWRW {
        _TWRW { w: self }
    }
    #[doc = "Bits 12:15 - Row Cycle Delay and Row Refresh Cycle"]
    #[inline(always)]
    pub fn trc_trfc(&mut self) -> _TRC_TRFCW {
        _TRC_TRFCW { w: self }
    }
    #[doc = "Bits 16:19 - Row Precharge Delay"]
    #[inline(always)]
    pub fn trp(&mut self) -> _TRPW {
        _TRPW { w: self }
    }
    #[doc = "Bits 20:23 - Row to Column Delay"]
    #[inline(always)]
    pub fn trcd(&mut self) -> _TRCDW {
        _TRCDW { w: self }
    }
    #[doc = "Bits 24:27 - Active to Precharge Delay"]
    #[inline(always)]
    pub fn tras(&mut self) -> _TRASW {
        _TRASW { w: self }
    }
    #[doc = "Bits 28:31 - Exit Self-Refresh to Active Delay"]
    #[inline(always)]
    pub fn txsr(&mut self) -> _TXSRW {
        _TXSRW { w: self }
    }
}
