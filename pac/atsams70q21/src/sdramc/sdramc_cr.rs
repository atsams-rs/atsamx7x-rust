#[doc = "Reader of register SDRAMC_CR"]
pub type R = crate::R<u32, super::SDRAMC_CR>;
#[doc = "Writer for register SDRAMC_CR"]
pub type W = crate::W<u32, super::SDRAMC_CR>;
#[doc = "Register SDRAMC_CR `reset()`'s with value 0"]
impl crate::ResetValue for super::SDRAMC_CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Number of Column Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NC_A {
    #[doc = "0: 8 column bits"]
    COL8 = 0,
    #[doc = "1: 9 column bits"]
    COL9 = 1,
    #[doc = "2: 10 column bits"]
    COL10 = 2,
    #[doc = "3: 11 column bits"]
    COL11 = 3,
}
impl From<NC_A> for u8 {
    #[inline(always)]
    fn from(variant: NC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NC`"]
pub type NC_R = crate::R<u8, NC_A>;
impl NC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NC_A {
        match self.bits {
            0 => NC_A::COL8,
            1 => NC_A::COL9,
            2 => NC_A::COL10,
            3 => NC_A::COL11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `COL8`"]
    #[inline(always)]
    pub fn is_col8(&self) -> bool {
        *self == NC_A::COL8
    }
    #[doc = "Checks if the value of the field is `COL9`"]
    #[inline(always)]
    pub fn is_col9(&self) -> bool {
        *self == NC_A::COL9
    }
    #[doc = "Checks if the value of the field is `COL10`"]
    #[inline(always)]
    pub fn is_col10(&self) -> bool {
        *self == NC_A::COL10
    }
    #[doc = "Checks if the value of the field is `COL11`"]
    #[inline(always)]
    pub fn is_col11(&self) -> bool {
        *self == NC_A::COL11
    }
}
#[doc = "Write proxy for field `NC`"]
pub struct NC_W<'a> {
    w: &'a mut W,
}
impl<'a> NC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "8 column bits"]
    #[inline(always)]
    pub fn col8(self) -> &'a mut W {
        self.variant(NC_A::COL8)
    }
    #[doc = "9 column bits"]
    #[inline(always)]
    pub fn col9(self) -> &'a mut W {
        self.variant(NC_A::COL9)
    }
    #[doc = "10 column bits"]
    #[inline(always)]
    pub fn col10(self) -> &'a mut W {
        self.variant(NC_A::COL10)
    }
    #[doc = "11 column bits"]
    #[inline(always)]
    pub fn col11(self) -> &'a mut W {
        self.variant(NC_A::COL11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Number of Row Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NR_A {
    #[doc = "0: 11 row bits"]
    ROW11 = 0,
    #[doc = "1: 12 row bits"]
    ROW12 = 1,
    #[doc = "2: 13 row bits"]
    ROW13 = 2,
}
impl From<NR_A> for u8 {
    #[inline(always)]
    fn from(variant: NR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NR`"]
pub type NR_R = crate::R<u8, NR_A>;
impl NR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NR_A::ROW11),
            1 => Val(NR_A::ROW12),
            2 => Val(NR_A::ROW13),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ROW11`"]
    #[inline(always)]
    pub fn is_row11(&self) -> bool {
        *self == NR_A::ROW11
    }
    #[doc = "Checks if the value of the field is `ROW12`"]
    #[inline(always)]
    pub fn is_row12(&self) -> bool {
        *self == NR_A::ROW12
    }
    #[doc = "Checks if the value of the field is `ROW13`"]
    #[inline(always)]
    pub fn is_row13(&self) -> bool {
        *self == NR_A::ROW13
    }
}
#[doc = "Write proxy for field `NR`"]
pub struct NR_W<'a> {
    w: &'a mut W,
}
impl<'a> NR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "11 row bits"]
    #[inline(always)]
    pub fn row11(self) -> &'a mut W {
        self.variant(NR_A::ROW11)
    }
    #[doc = "12 row bits"]
    #[inline(always)]
    pub fn row12(self) -> &'a mut W {
        self.variant(NR_A::ROW12)
    }
    #[doc = "13 row bits"]
    #[inline(always)]
    pub fn row13(self) -> &'a mut W {
        self.variant(NR_A::ROW13)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Number of Banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NB_A {
    #[doc = "0: 2 banks"]
    BANK2 = 0,
    #[doc = "1: 4 banks"]
    BANK4 = 1,
}
impl From<NB_A> for bool {
    #[inline(always)]
    fn from(variant: NB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NB`"]
pub type NB_R = crate::R<bool, NB_A>;
impl NB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NB_A {
        match self.bits {
            false => NB_A::BANK2,
            true => NB_A::BANK4,
        }
    }
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == NB_A::BANK2
    }
    #[doc = "Checks if the value of the field is `BANK4`"]
    #[inline(always)]
    pub fn is_bank4(&self) -> bool {
        *self == NB_A::BANK4
    }
}
#[doc = "Write proxy for field `NB`"]
pub struct NB_W<'a> {
    w: &'a mut W,
}
impl<'a> NB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NB_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "2 banks"]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut W {
        self.variant(NB_A::BANK2)
    }
    #[doc = "4 banks"]
    #[inline(always)]
    pub fn bank4(self) -> &'a mut W {
        self.variant(NB_A::BANK4)
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
#[doc = "CAS Latency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAS_A {
    #[doc = "1: 1 cycle CAS latency"]
    LATENCY1 = 1,
    #[doc = "2: 2 cycle CAS latency"]
    LATENCY2 = 2,
    #[doc = "3: 3 cycle CAS latency"]
    LATENCY3 = 3,
}
impl From<CAS_A> for u8 {
    #[inline(always)]
    fn from(variant: CAS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CAS`"]
pub type CAS_R = crate::R<u8, CAS_A>;
impl CAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CAS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CAS_A::LATENCY1),
            2 => Val(CAS_A::LATENCY2),
            3 => Val(CAS_A::LATENCY3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LATENCY1`"]
    #[inline(always)]
    pub fn is_latency1(&self) -> bool {
        *self == CAS_A::LATENCY1
    }
    #[doc = "Checks if the value of the field is `LATENCY2`"]
    #[inline(always)]
    pub fn is_latency2(&self) -> bool {
        *self == CAS_A::LATENCY2
    }
    #[doc = "Checks if the value of the field is `LATENCY3`"]
    #[inline(always)]
    pub fn is_latency3(&self) -> bool {
        *self == CAS_A::LATENCY3
    }
}
#[doc = "Write proxy for field `CAS`"]
pub struct CAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 cycle CAS latency"]
    #[inline(always)]
    pub fn latency1(self) -> &'a mut W {
        self.variant(CAS_A::LATENCY1)
    }
    #[doc = "2 cycle CAS latency"]
    #[inline(always)]
    pub fn latency2(self) -> &'a mut W {
        self.variant(CAS_A::LATENCY2)
    }
    #[doc = "3 cycle CAS latency"]
    #[inline(always)]
    pub fn latency3(self) -> &'a mut W {
        self.variant(CAS_A::LATENCY3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `DBW`"]
pub type DBW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBW`"]
pub struct DBW_W<'a> {
    w: &'a mut W,
}
impl<'a> DBW_W<'a> {
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
#[doc = "Reader of field `TWR`"]
pub type TWR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TWR`"]
pub struct TWR_W<'a> {
    w: &'a mut W,
}
impl<'a> TWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TRC_TRFC`"]
pub type TRC_TRFC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRC_TRFC`"]
pub struct TRC_TRFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRC_TRFC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `TRP`"]
pub type TRP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRP`"]
pub struct TRP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `TRCD`"]
pub type TRCD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRCD`"]
pub struct TRCD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRCD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `TRAS`"]
pub type TRAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRAS`"]
pub struct TRAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `TXSR`"]
pub type TXSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXSR`"]
pub struct TXSR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Number of Column Bits"]
    #[inline(always)]
    pub fn nc(&self) -> NC_R {
        NC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Number of Row Bits"]
    #[inline(always)]
    pub fn nr(&self) -> NR_R {
        NR_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Number of Banks"]
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - CAS Latency"]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Write Recovery Delay"]
    #[inline(always)]
    pub fn twr(&self) -> TWR_R {
        TWR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Row Cycle Delay and Row Refresh Cycle"]
    #[inline(always)]
    pub fn trc_trfc(&self) -> TRC_TRFC_R {
        TRC_TRFC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Row Precharge Delay"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Row to Column Delay"]
    #[inline(always)]
    pub fn trcd(&self) -> TRCD_R {
        TRCD_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Active to Precharge Delay"]
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Exit Self Refresh to Active Delay"]
    #[inline(always)]
    pub fn txsr(&self) -> TXSR_R {
        TXSR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of Column Bits"]
    #[inline(always)]
    pub fn nc(&mut self) -> NC_W {
        NC_W { w: self }
    }
    #[doc = "Bits 2:3 - Number of Row Bits"]
    #[inline(always)]
    pub fn nr(&mut self) -> NR_W {
        NR_W { w: self }
    }
    #[doc = "Bit 4 - Number of Banks"]
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W {
        NB_W { w: self }
    }
    #[doc = "Bits 5:6 - CAS Latency"]
    #[inline(always)]
    pub fn cas(&mut self) -> CAS_W {
        CAS_W { w: self }
    }
    #[doc = "Bit 7 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&mut self) -> DBW_W {
        DBW_W { w: self }
    }
    #[doc = "Bits 8:11 - Write Recovery Delay"]
    #[inline(always)]
    pub fn twr(&mut self) -> TWR_W {
        TWR_W { w: self }
    }
    #[doc = "Bits 12:15 - Row Cycle Delay and Row Refresh Cycle"]
    #[inline(always)]
    pub fn trc_trfc(&mut self) -> TRC_TRFC_W {
        TRC_TRFC_W { w: self }
    }
    #[doc = "Bits 16:19 - Row Precharge Delay"]
    #[inline(always)]
    pub fn trp(&mut self) -> TRP_W {
        TRP_W { w: self }
    }
    #[doc = "Bits 20:23 - Row to Column Delay"]
    #[inline(always)]
    pub fn trcd(&mut self) -> TRCD_W {
        TRCD_W { w: self }
    }
    #[doc = "Bits 24:27 - Active to Precharge Delay"]
    #[inline(always)]
    pub fn tras(&mut self) -> TRAS_W {
        TRAS_W { w: self }
    }
    #[doc = "Bits 28:31 - Exit Self Refresh to Active Delay"]
    #[inline(always)]
    pub fn txsr(&mut self) -> TXSR_W {
        TXSR_W { w: self }
    }
}
