#[doc = "Register `HSTPIPISR_ISO_MODE[%s]` reader"]
pub struct R(crate::R<HSTPIPISR_ISO_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTPIPISR_ISO_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTPIPISR_ISO_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTPIPISR_ISO_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXINI` reader - Received IN Data Interrupt"]
pub type RXINI_R = crate::BitReader<bool>;
#[doc = "Field `TXOUTI` reader - Transmitted OUT Data Interrupt"]
pub type TXOUTI_R = crate::BitReader<bool>;
#[doc = "Field `UNDERFI` reader - Underflow Interrupt"]
pub type UNDERFI_R = crate::BitReader<bool>;
#[doc = "Field `PERRI` reader - Pipe Error Interrupt"]
pub type PERRI_R = crate::BitReader<bool>;
#[doc = "Field `NAKEDI` reader - NAKed Interrupt"]
pub type NAKEDI_R = crate::BitReader<bool>;
#[doc = "Field `OVERFI` reader - Overflow Interrupt"]
pub type OVERFI_R = crate::BitReader<bool>;
#[doc = "Field `CRCERRI` reader - CRC Error Interrupt"]
pub type CRCERRI_R = crate::BitReader<bool>;
#[doc = "Field `SHORTPACKETI` reader - Short Packet Interrupt"]
pub type SHORTPACKETI_R = crate::BitReader<bool>;
#[doc = "Data Toggle Sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTSEQ_A {
    #[doc = "0: Data0 toggle sequence"]
    DATA0 = 0,
    #[doc = "1: Data1 toggle sequence"]
    DATA1 = 1,
}
impl From<DTSEQ_A> for u8 {
    #[inline(always)]
    fn from(variant: DTSEQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DTSEQ` reader - Data Toggle Sequence"]
pub type DTSEQ_R = crate::FieldReader<u8, DTSEQ_A>;
impl DTSEQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTSEQ_A> {
        match self.bits {
            0 => Some(DTSEQ_A::DATA0),
            1 => Some(DTSEQ_A::DATA1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == DTSEQ_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == DTSEQ_A::DATA1
    }
}
#[doc = "Number of Busy Banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NBUSYBK_A {
    #[doc = "0: 0 busy bank (all banks free)"]
    _0_BUSY = 0,
    #[doc = "1: 1 busy bank"]
    _1_BUSY = 1,
    #[doc = "2: 2 busy banks"]
    _2_BUSY = 2,
    #[doc = "3: 3 busy banks"]
    _3_BUSY = 3,
}
impl From<NBUSYBK_A> for u8 {
    #[inline(always)]
    fn from(variant: NBUSYBK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NBUSYBK` reader - Number of Busy Banks"]
pub type NBUSYBK_R = crate::FieldReader<u8, NBUSYBK_A>;
impl NBUSYBK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NBUSYBK_A {
        match self.bits {
            0 => NBUSYBK_A::_0_BUSY,
            1 => NBUSYBK_A::_1_BUSY,
            2 => NBUSYBK_A::_2_BUSY,
            3 => NBUSYBK_A::_3_BUSY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0_BUSY`"]
    #[inline(always)]
    pub fn is_0_busy(&self) -> bool {
        *self == NBUSYBK_A::_0_BUSY
    }
    #[doc = "Checks if the value of the field is `_1_BUSY`"]
    #[inline(always)]
    pub fn is_1_busy(&self) -> bool {
        *self == NBUSYBK_A::_1_BUSY
    }
    #[doc = "Checks if the value of the field is `_2_BUSY`"]
    #[inline(always)]
    pub fn is_2_busy(&self) -> bool {
        *self == NBUSYBK_A::_2_BUSY
    }
    #[doc = "Checks if the value of the field is `_3_BUSY`"]
    #[inline(always)]
    pub fn is_3_busy(&self) -> bool {
        *self == NBUSYBK_A::_3_BUSY
    }
}
#[doc = "Current Bank\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CURRBK_A {
    #[doc = "0: Current bank is bank0"]
    BANK0 = 0,
    #[doc = "1: Current bank is bank1"]
    BANK1 = 1,
    #[doc = "2: Current bank is bank2"]
    BANK2 = 2,
}
impl From<CURRBK_A> for u8 {
    #[inline(always)]
    fn from(variant: CURRBK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CURRBK` reader - Current Bank"]
pub type CURRBK_R = crate::FieldReader<u8, CURRBK_A>;
impl CURRBK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CURRBK_A> {
        match self.bits {
            0 => Some(CURRBK_A::BANK0),
            1 => Some(CURRBK_A::BANK1),
            2 => Some(CURRBK_A::BANK2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BANK0`"]
    #[inline(always)]
    pub fn is_bank0(&self) -> bool {
        *self == CURRBK_A::BANK0
    }
    #[doc = "Checks if the value of the field is `BANK1`"]
    #[inline(always)]
    pub fn is_bank1(&self) -> bool {
        *self == CURRBK_A::BANK1
    }
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == CURRBK_A::BANK2
    }
}
#[doc = "Field `RWALL` reader - Read/Write Allowed"]
pub type RWALL_R = crate::BitReader<bool>;
#[doc = "Field `CFGOK` reader - Configuration OK Status"]
pub type CFGOK_R = crate::BitReader<bool>;
#[doc = "Field `PBYCT` reader - Pipe Byte Count"]
pub type PBYCT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - Received IN Data Interrupt"]
    #[inline(always)]
    pub fn rxini(&self) -> RXINI_R {
        RXINI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt"]
    #[inline(always)]
    pub fn txouti(&self) -> TXOUTI_R {
        TXOUTI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Underflow Interrupt"]
    #[inline(always)]
    pub fn underfi(&self) -> UNDERFI_R {
        UNDERFI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt"]
    #[inline(always)]
    pub fn perri(&self) -> PERRI_R {
        PERRI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAKed Interrupt"]
    #[inline(always)]
    pub fn nakedi(&self) -> NAKEDI_R {
        NAKEDI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overfi(&self) -> OVERFI_R {
        OVERFI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC Error Interrupt"]
    #[inline(always)]
    pub fn crcerri(&self) -> CRCERRI_R {
        CRCERRI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt"]
    #[inline(always)]
    pub fn shortpacketi(&self) -> SHORTPACKETI_R {
        SHORTPACKETI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Data Toggle Sequence"]
    #[inline(always)]
    pub fn dtseq(&self) -> DTSEQ_R {
        DTSEQ_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Number of Busy Banks"]
    #[inline(always)]
    pub fn nbusybk(&self) -> NBUSYBK_R {
        NBUSYBK_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Current Bank"]
    #[inline(always)]
    pub fn currbk(&self) -> CURRBK_R {
        CURRBK_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Read/Write Allowed"]
    #[inline(always)]
    pub fn rwall(&self) -> RWALL_R {
        RWALL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Configuration OK Status"]
    #[inline(always)]
    pub fn cfgok(&self) -> CFGOK_R {
        CFGOK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:30 - Pipe Byte Count"]
    #[inline(always)]
    pub fn pbyct(&self) -> PBYCT_R {
        PBYCT_R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
}
#[doc = "Host Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipisr_iso_mode](index.html) module"]
pub struct HSTPIPISR_ISO_MODE_SPEC;
impl crate::RegisterSpec for HSTPIPISR_ISO_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstpipisr_iso_mode::R](R) reader structure"]
impl crate::Readable for HSTPIPISR_ISO_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HSTPIPISR_ISO_MODE[%s]
to value 0"]
impl crate::Resettable for HSTPIPISR_ISO_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
