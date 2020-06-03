#[doc = "Reader of register AFEC_EMR"]
pub type R = crate::R<u32, super::AFEC_EMR>;
#[doc = "Writer for register AFEC_EMR"]
pub type W = crate::W<u32, super::AFEC_EMR>;
#[doc = "Register AFEC_EMR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFEC_EMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPMODE_A {
    #[doc = "0: Generates an event when the converted data is lower than the low threshold of the window."]
    LOW = 0,
    #[doc = "1: Generates an event when the converted data is higher than the high threshold of the window."]
    HIGH = 1,
    #[doc = "2: Generates an event when the converted data is in the comparison window."]
    IN = 2,
    #[doc = "3: Generates an event when the converted data is out of the comparison window."]
    OUT = 3,
}
impl From<CMPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMPMODE`"]
pub type CMPMODE_R = crate::R<u8, CMPMODE_A>;
impl CMPMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMODE_A {
        match self.bits {
            0 => CMPMODE_A::LOW,
            1 => CMPMODE_A::HIGH,
            2 => CMPMODE_A::IN,
            3 => CMPMODE_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CMPMODE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CMPMODE_A::HIGH
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == CMPMODE_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == CMPMODE_A::OUT
    }
}
#[doc = "Write proxy for field `CMPMODE`"]
pub struct CMPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CMPMODE_A::LOW)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CMPMODE_A::HIGH)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(CMPMODE_A::IN)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(CMPMODE_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `CMPSEL`"]
pub type CMPSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMPSEL`"]
pub struct CMPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Reader of field `CMPALL`"]
pub type CMPALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPALL`"]
pub struct CMPALL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPALL_W<'a> {
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
#[doc = "Reader of field `CMPFILTER`"]
pub type CMPFILTER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMPFILTER`"]
pub struct CMPFILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPFILTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RES_A {
    #[doc = "0: 12-bit resolution, AFE sample rate is maximum (no averaging)."]
    NO_AVERAGE = 0,
    #[doc = "2: 13-bit resolution, AFE sample rate divided by 4 (averaging)."]
    OSR4 = 2,
    #[doc = "3: 14-bit resolution, AFE sample rate divided by 16 (averaging)."]
    OSR16 = 3,
    #[doc = "4: 15-bit resolution, AFE sample rate divided by 64 (averaging)."]
    OSR64 = 4,
    #[doc = "5: 16-bit resolution, AFE sample rate divided by 256 (averaging)."]
    OSR256 = 5,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RES`"]
pub type RES_R = crate::R<u8, RES_A>;
impl RES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RES_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RES_A::NO_AVERAGE),
            2 => Val(RES_A::OSR4),
            3 => Val(RES_A::OSR16),
            4 => Val(RES_A::OSR64),
            5 => Val(RES_A::OSR256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO_AVERAGE`"]
    #[inline(always)]
    pub fn is_no_average(&self) -> bool {
        *self == RES_A::NO_AVERAGE
    }
    #[doc = "Checks if the value of the field is `OSR4`"]
    #[inline(always)]
    pub fn is_osr4(&self) -> bool {
        *self == RES_A::OSR4
    }
    #[doc = "Checks if the value of the field is `OSR16`"]
    #[inline(always)]
    pub fn is_osr16(&self) -> bool {
        *self == RES_A::OSR16
    }
    #[doc = "Checks if the value of the field is `OSR64`"]
    #[inline(always)]
    pub fn is_osr64(&self) -> bool {
        *self == RES_A::OSR64
    }
    #[doc = "Checks if the value of the field is `OSR256`"]
    #[inline(always)]
    pub fn is_osr256(&self) -> bool {
        *self == RES_A::OSR256
    }
}
#[doc = "Write proxy for field `RES`"]
pub struct RES_W<'a> {
    w: &'a mut W,
}
impl<'a> RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RES_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "12-bit resolution, AFE sample rate is maximum (no averaging)."]
    #[inline(always)]
    pub fn no_average(self) -> &'a mut W {
        self.variant(RES_A::NO_AVERAGE)
    }
    #[doc = "13-bit resolution, AFE sample rate divided by 4 (averaging)."]
    #[inline(always)]
    pub fn osr4(self) -> &'a mut W {
        self.variant(RES_A::OSR4)
    }
    #[doc = "14-bit resolution, AFE sample rate divided by 16 (averaging)."]
    #[inline(always)]
    pub fn osr16(self) -> &'a mut W {
        self.variant(RES_A::OSR16)
    }
    #[doc = "15-bit resolution, AFE sample rate divided by 64 (averaging)."]
    #[inline(always)]
    pub fn osr64(self) -> &'a mut W {
        self.variant(RES_A::OSR64)
    }
    #[doc = "16-bit resolution, AFE sample rate divided by 256 (averaging)."]
    #[inline(always)]
    pub fn osr256(self) -> &'a mut W {
        self.variant(RES_A::OSR256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `TAG`"]
pub type TAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAG`"]
pub struct TAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_W<'a> {
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
#[doc = "Reader of field `STM`"]
pub type STM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STM`"]
pub struct STM_W<'a> {
    w: &'a mut W,
}
impl<'a> STM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Sign Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIGNMODE_A {
    #[doc = "0: Single-Ended channels: Unsigned conversions.Differential channels: Signed conversions."]
    SE_UNSG_DF_SIGN = 0,
    #[doc = "1: Single-Ended channels: Signed conversions.Differential channels: Unsigned conversions."]
    SE_SIGN_DF_UNSG = 1,
    #[doc = "2: All channels: Unsigned conversions."]
    ALL_UNSIGNED = 2,
    #[doc = "3: All channels: Signed conversions."]
    ALL_SIGNED = 3,
}
impl From<SIGNMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIGNMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SIGNMODE`"]
pub type SIGNMODE_R = crate::R<u8, SIGNMODE_A>;
impl SIGNMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIGNMODE_A {
        match self.bits {
            0 => SIGNMODE_A::SE_UNSG_DF_SIGN,
            1 => SIGNMODE_A::SE_SIGN_DF_UNSG,
            2 => SIGNMODE_A::ALL_UNSIGNED,
            3 => SIGNMODE_A::ALL_SIGNED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE_UNSG_DF_SIGN`"]
    #[inline(always)]
    pub fn is_se_unsg_df_sign(&self) -> bool {
        *self == SIGNMODE_A::SE_UNSG_DF_SIGN
    }
    #[doc = "Checks if the value of the field is `SE_SIGN_DF_UNSG`"]
    #[inline(always)]
    pub fn is_se_sign_df_unsg(&self) -> bool {
        *self == SIGNMODE_A::SE_SIGN_DF_UNSG
    }
    #[doc = "Checks if the value of the field is `ALL_UNSIGNED`"]
    #[inline(always)]
    pub fn is_all_unsigned(&self) -> bool {
        *self == SIGNMODE_A::ALL_UNSIGNED
    }
    #[doc = "Checks if the value of the field is `ALL_SIGNED`"]
    #[inline(always)]
    pub fn is_all_signed(&self) -> bool {
        *self == SIGNMODE_A::ALL_SIGNED
    }
}
#[doc = "Write proxy for field `SIGNMODE`"]
pub struct SIGNMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGNMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIGNMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-Ended channels: Unsigned conversions.Differential channels: Signed conversions."]
    #[inline(always)]
    pub fn se_unsg_df_sign(self) -> &'a mut W {
        self.variant(SIGNMODE_A::SE_UNSG_DF_SIGN)
    }
    #[doc = "Single-Ended channels: Signed conversions.Differential channels: Unsigned conversions."]
    #[inline(always)]
    pub fn se_sign_df_unsg(self) -> &'a mut W {
        self.variant(SIGNMODE_A::SE_SIGN_DF_UNSG)
    }
    #[doc = "All channels: Unsigned conversions."]
    #[inline(always)]
    pub fn all_unsigned(self) -> &'a mut W {
        self.variant(SIGNMODE_A::ALL_UNSIGNED)
    }
    #[doc = "All channels: Signed conversions."]
    #[inline(always)]
    pub fn all_signed(self) -> &'a mut W {
        self.variant(SIGNMODE_A::ALL_SIGNED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CMPMODE_R {
        CMPMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 3:7 - Comparison Selected Channel"]
    #[inline(always)]
    pub fn cmpsel(&self) -> CMPSEL_R {
        CMPSEL_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    pub fn cmpall(&self) -> CMPALL_R {
        CMPALL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    pub fn cmpfilter(&self) -> CMPFILTER_R {
        CMPFILTER_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:18 - Resolution"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 24 - TAG of the AFEC_LDCR"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Single Trigger Mode"]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Sign Mode"]
    #[inline(always)]
    pub fn signmode(&self) -> SIGNMODE_R {
        SIGNMODE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&mut self) -> CMPMODE_W {
        CMPMODE_W { w: self }
    }
    #[doc = "Bits 3:7 - Comparison Selected Channel"]
    #[inline(always)]
    pub fn cmpsel(&mut self) -> CMPSEL_W {
        CMPSEL_W { w: self }
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    pub fn cmpall(&mut self) -> CMPALL_W {
        CMPALL_W { w: self }
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    pub fn cmpfilter(&mut self) -> CMPFILTER_W {
        CMPFILTER_W { w: self }
    }
    #[doc = "Bits 16:18 - Resolution"]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W {
        RES_W { w: self }
    }
    #[doc = "Bit 24 - TAG of the AFEC_LDCR"]
    #[inline(always)]
    pub fn tag(&mut self) -> TAG_W {
        TAG_W { w: self }
    }
    #[doc = "Bit 25 - Single Trigger Mode"]
    #[inline(always)]
    pub fn stm(&mut self) -> STM_W {
        STM_W { w: self }
    }
    #[doc = "Bits 28:29 - Sign Mode"]
    #[inline(always)]
    pub fn signmode(&mut self) -> SIGNMODE_W {
        SIGNMODE_W { w: self }
    }
}
