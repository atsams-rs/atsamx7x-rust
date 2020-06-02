#[doc = "Reader of register GMAC_DCFGR"]
pub type R = crate::R<u32, super::GMAC_DCFGR>;
#[doc = "Writer for register GMAC_DCFGR"]
pub type W = crate::W<u32, super::GMAC_DCFGR>;
#[doc = "Register GMAC_DCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::GMAC_DCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Fixed Burst Length for DMA Data Operations:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FBLDO_A {
    #[doc = "1: 00001: Always use SINGLE AHB bursts"]
    SINGLE = 1,
    #[doc = "4: 001xx: Attempt to use INCR4 AHB bursts (Default)"]
    INCR4 = 4,
    #[doc = "8: 01xxx: Attempt to use INCR8 AHB bursts"]
    INCR8 = 8,
    #[doc = "16: 1xxxx: Attempt to use INCR16 AHB bursts"]
    INCR16 = 16,
}
impl From<FBLDO_A> for u8 {
    #[inline(always)]
    fn from(variant: FBLDO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FBLDO`"]
pub type FBLDO_R = crate::R<u8, FBLDO_A>;
impl FBLDO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FBLDO_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(FBLDO_A::SINGLE),
            4 => Val(FBLDO_A::INCR4),
            8 => Val(FBLDO_A::INCR8),
            16 => Val(FBLDO_A::INCR16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == FBLDO_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `INCR4`"]
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        *self == FBLDO_A::INCR4
    }
    #[doc = "Checks if the value of the field is `INCR8`"]
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        *self == FBLDO_A::INCR8
    }
    #[doc = "Checks if the value of the field is `INCR16`"]
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        *self == FBLDO_A::INCR16
    }
}
#[doc = "Write proxy for field `FBLDO`"]
pub struct FBLDO_W<'a> {
    w: &'a mut W,
}
impl<'a> FBLDO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FBLDO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "00001: Always use SINGLE AHB bursts"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(FBLDO_A::SINGLE)
    }
    #[doc = "001xx: Attempt to use INCR4 AHB bursts (Default)"]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut W {
        self.variant(FBLDO_A::INCR4)
    }
    #[doc = "01xxx: Attempt to use INCR8 AHB bursts"]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut W {
        self.variant(FBLDO_A::INCR8)
    }
    #[doc = "1xxxx: Attempt to use INCR16 AHB bursts"]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut W {
        self.variant(FBLDO_A::INCR16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `ESMA`"]
pub type ESMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESMA`"]
pub struct ESMA_W<'a> {
    w: &'a mut W,
}
impl<'a> ESMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ESPA`"]
pub type ESPA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ESPA`"]
pub struct ESPA_W<'a> {
    w: &'a mut W,
}
impl<'a> ESPA_W<'a> {
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
#[doc = "Receiver Packet Buffer Memory Size Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXBMS_A {
    #[doc = "0: 4/8 Kbyte Memory Size"]
    EIGHTH = 0,
    #[doc = "1: 4/4 Kbytes Memory Size"]
    QUARTER = 1,
    #[doc = "2: 4/2 Kbytes Memory Size"]
    HALF = 2,
    #[doc = "3: 4 Kbytes Memory Size"]
    FULL = 3,
}
impl From<RXBMS_A> for u8 {
    #[inline(always)]
    fn from(variant: RXBMS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXBMS`"]
pub type RXBMS_R = crate::R<u8, RXBMS_A>;
impl RXBMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXBMS_A {
        match self.bits {
            0 => RXBMS_A::EIGHTH,
            1 => RXBMS_A::QUARTER,
            2 => RXBMS_A::HALF,
            3 => RXBMS_A::FULL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EIGHTH`"]
    #[inline(always)]
    pub fn is_eighth(&self) -> bool {
        *self == RXBMS_A::EIGHTH
    }
    #[doc = "Checks if the value of the field is `QUARTER`"]
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == RXBMS_A::QUARTER
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == RXBMS_A::HALF
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RXBMS_A::FULL
    }
}
#[doc = "Write proxy for field `RXBMS`"]
pub struct RXBMS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBMS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXBMS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "4/8 Kbyte Memory Size"]
    #[inline(always)]
    pub fn eighth(self) -> &'a mut W {
        self.variant(RXBMS_A::EIGHTH)
    }
    #[doc = "4/4 Kbytes Memory Size"]
    #[inline(always)]
    pub fn quarter(self) -> &'a mut W {
        self.variant(RXBMS_A::QUARTER)
    }
    #[doc = "4/2 Kbytes Memory Size"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(RXBMS_A::HALF)
    }
    #[doc = "4 Kbytes Memory Size"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(RXBMS_A::FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `TXPBMS`"]
pub type TXPBMS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPBMS`"]
pub struct TXPBMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPBMS_W<'a> {
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
#[doc = "Reader of field `TXCOEN`"]
pub type TXCOEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXCOEN`"]
pub struct TXCOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCOEN_W<'a> {
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
#[doc = "Reader of field `DRBS`"]
pub type DRBS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRBS`"]
pub struct DRBS_W<'a> {
    w: &'a mut W,
}
impl<'a> DRBS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DDRP`"]
pub type DDRP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRP`"]
pub struct DDRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRP_W<'a> {
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
impl R {
    #[doc = "Bits 0:4 - Fixed Burst Length for DMA Data Operations:"]
    #[inline(always)]
    pub fn fbldo(&self) -> FBLDO_R {
        FBLDO_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Endian Swap Mode Enable for Management Descriptor Accesses"]
    #[inline(always)]
    pub fn esma(&self) -> ESMA_R {
        ESMA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endian Swap Mode Enable for Packet Data Accesses"]
    #[inline(always)]
    pub fn espa(&self) -> ESPA_R {
        ESPA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Receiver Packet Buffer Memory Size Select"]
    #[inline(always)]
    pub fn rxbms(&self) -> RXBMS_R {
        RXBMS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Transmitter Packet Buffer Memory Size Select"]
    #[inline(always)]
    pub fn txpbms(&self) -> TXPBMS_R {
        TXPBMS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmitter Checksum Generation Offload Enable"]
    #[inline(always)]
    pub fn txcoen(&self) -> TXCOEN_R {
        TXCOEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - DMA Receive Buffer Size"]
    #[inline(always)]
    pub fn drbs(&self) -> DRBS_R {
        DRBS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - DMA Discard Receive Packets"]
    #[inline(always)]
    pub fn ddrp(&self) -> DDRP_R {
        DDRP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Fixed Burst Length for DMA Data Operations:"]
    #[inline(always)]
    pub fn fbldo(&mut self) -> FBLDO_W {
        FBLDO_W { w: self }
    }
    #[doc = "Bit 6 - Endian Swap Mode Enable for Management Descriptor Accesses"]
    #[inline(always)]
    pub fn esma(&mut self) -> ESMA_W {
        ESMA_W { w: self }
    }
    #[doc = "Bit 7 - Endian Swap Mode Enable for Packet Data Accesses"]
    #[inline(always)]
    pub fn espa(&mut self) -> ESPA_W {
        ESPA_W { w: self }
    }
    #[doc = "Bits 8:9 - Receiver Packet Buffer Memory Size Select"]
    #[inline(always)]
    pub fn rxbms(&mut self) -> RXBMS_W {
        RXBMS_W { w: self }
    }
    #[doc = "Bit 10 - Transmitter Packet Buffer Memory Size Select"]
    #[inline(always)]
    pub fn txpbms(&mut self) -> TXPBMS_W {
        TXPBMS_W { w: self }
    }
    #[doc = "Bit 11 - Transmitter Checksum Generation Offload Enable"]
    #[inline(always)]
    pub fn txcoen(&mut self) -> TXCOEN_W {
        TXCOEN_W { w: self }
    }
    #[doc = "Bits 16:23 - DMA Receive Buffer Size"]
    #[inline(always)]
    pub fn drbs(&mut self) -> DRBS_W {
        DRBS_W { w: self }
    }
    #[doc = "Bit 24 - DMA Discard Receive Packets"]
    #[inline(always)]
    pub fn ddrp(&mut self) -> DDRP_W {
        DDRP_W { w: self }
    }
}
