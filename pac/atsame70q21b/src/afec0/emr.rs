#[doc = "Register `EMR` reader"]
pub struct R(crate::R<EMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMR` writer"]
pub struct W(crate::W<EMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `CMPMODE` reader - Comparison Mode"]
pub type CMPMODE_R = crate::FieldReader<u8, CMPMODE_A>;
impl CMPMODE_R {
    #[doc = "Get enumerated values variant"]
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
    pub fn is_in(&self) -> bool {
        *self == CMPMODE_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == CMPMODE_A::OUT
    }
}
#[doc = "Field `CMPMODE` writer - Comparison Mode"]
pub type CMPMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EMR_SPEC, u8, CMPMODE_A, 2, O>;
impl<'a, const O: u8> CMPMODE_W<'a, O> {
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
}
#[doc = "Field `CMPSEL` reader - Comparison Selected Channel"]
pub type CMPSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMPSEL` writer - Comparison Selected Channel"]
pub type CMPSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EMR_SPEC, u8, u8, 5, O>;
#[doc = "Field `CMPALL` reader - Compare All Channels"]
pub type CMPALL_R = crate::BitReader<bool>;
#[doc = "Field `CMPALL` writer - Compare All Channels"]
pub type CMPALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `CMPFILTER` reader - Compare Event Filtering"]
pub type CMPFILTER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMPFILTER` writer - Compare Event Filtering"]
pub type CMPFILTER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EMR_SPEC, u8, u8, 2, O>;
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
#[doc = "Field `RES` reader - Resolution"]
pub type RES_R = crate::FieldReader<u8, RES_A>;
impl RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RES_A> {
        match self.bits {
            0 => Some(RES_A::NO_AVERAGE),
            2 => Some(RES_A::OSR4),
            3 => Some(RES_A::OSR16),
            4 => Some(RES_A::OSR64),
            5 => Some(RES_A::OSR256),
            _ => None,
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
#[doc = "Field `RES` writer - Resolution"]
pub type RES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EMR_SPEC, u8, RES_A, 3, O>;
impl<'a, const O: u8> RES_W<'a, O> {
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
}
#[doc = "Field `TAG` reader - TAG of the AFEC_LDCR"]
pub type TAG_R = crate::BitReader<bool>;
#[doc = "Field `TAG` writer - TAG of the AFEC_LDCR"]
pub type TAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
#[doc = "Field `STM` reader - Single Trigger Mode"]
pub type STM_R = crate::BitReader<bool>;
#[doc = "Field `STM` writer - Single Trigger Mode"]
pub type STM_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR_SPEC, bool, O>;
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
#[doc = "Field `SIGNMODE` reader - Sign Mode"]
pub type SIGNMODE_R = crate::FieldReader<u8, SIGNMODE_A>;
impl SIGNMODE_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `SIGNMODE` writer - Sign Mode"]
pub type SIGNMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EMR_SPEC, u8, SIGNMODE_A, 2, O>;
impl<'a, const O: u8> SIGNMODE_W<'a, O> {
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
}
impl R {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CMPMODE_R {
        CMPMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 3:7 - Comparison Selected Channel"]
    #[inline(always)]
    pub fn cmpsel(&self) -> CMPSEL_R {
        CMPSEL_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    pub fn cmpall(&self) -> CMPALL_R {
        CMPALL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    pub fn cmpfilter(&self) -> CMPFILTER_R {
        CMPFILTER_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Resolution"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - TAG of the AFEC_LDCR"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Single Trigger Mode"]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Sign Mode"]
    #[inline(always)]
    pub fn signmode(&self) -> SIGNMODE_R {
        SIGNMODE_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&mut self) -> CMPMODE_W<0> {
        CMPMODE_W::new(self)
    }
    #[doc = "Bits 3:7 - Comparison Selected Channel"]
    #[inline(always)]
    pub fn cmpsel(&mut self) -> CMPSEL_W<3> {
        CMPSEL_W::new(self)
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    pub fn cmpall(&mut self) -> CMPALL_W<9> {
        CMPALL_W::new(self)
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    pub fn cmpfilter(&mut self) -> CMPFILTER_W<12> {
        CMPFILTER_W::new(self)
    }
    #[doc = "Bits 16:18 - Resolution"]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W<16> {
        RES_W::new(self)
    }
    #[doc = "Bit 24 - TAG of the AFEC_LDCR"]
    #[inline(always)]
    pub fn tag(&mut self) -> TAG_W<24> {
        TAG_W::new(self)
    }
    #[doc = "Bit 25 - Single Trigger Mode"]
    #[inline(always)]
    pub fn stm(&mut self) -> STM_W<25> {
        STM_W::new(self)
    }
    #[doc = "Bits 28:29 - Sign Mode"]
    #[inline(always)]
    pub fn signmode(&mut self) -> SIGNMODE_W<28> {
        SIGNMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AFEC Extended Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr](index.html) module"]
pub struct EMR_SPEC;
impl crate::RegisterSpec for EMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emr::R](R) reader structure"]
impl crate::Readable for EMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emr::W](W) writer structure"]
impl crate::Writable for EMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
