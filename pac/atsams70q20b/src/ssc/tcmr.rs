#[doc = "Register `TCMR` reader"]
pub struct R(crate::R<TCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCMR` writer"]
pub struct W(crate::W<TCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCMR_SPEC>;
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
impl From<crate::W<TCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKS` reader - Transmit Clock Selection"]
pub type CKS_R = crate::FieldReader<u8, CKSSELECT_A>;
#[doc = "Transmit Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKSSELECT_A {
    #[doc = "0: Divided Clock"]
    MCK = 0,
    #[doc = "1: RK Clock signal"]
    RK = 1,
    #[doc = "2: TK pin"]
    TK = 2,
}
impl From<CKSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CKSSELECT_A) -> Self {
        variant as _
    }
}
impl CKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKSSELECT_A> {
        match self.bits {
            0 => Some(CKSSELECT_A::MCK),
            1 => Some(CKSSELECT_A::RK),
            2 => Some(CKSSELECT_A::TK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == CKSSELECT_A::MCK
    }
    #[doc = "Checks if the value of the field is `RK`"]
    #[inline(always)]
    pub fn is_rk(&self) -> bool {
        *self == CKSSELECT_A::RK
    }
    #[doc = "Checks if the value of the field is `TK`"]
    #[inline(always)]
    pub fn is_tk(&self) -> bool {
        *self == CKSSELECT_A::TK
    }
}
#[doc = "Field `CKS` writer - Transmit Clock Selection"]
pub type CKS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCMR_SPEC, u8, CKSSELECT_A, 2, O>;
impl<'a, const O: u8> CKS_W<'a, O> {
    #[doc = "Divided Clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(CKSSELECT_A::MCK)
    }
    #[doc = "RK Clock signal"]
    #[inline(always)]
    pub fn rk(self) -> &'a mut W {
        self.variant(CKSSELECT_A::RK)
    }
    #[doc = "TK pin"]
    #[inline(always)]
    pub fn tk(self) -> &'a mut W {
        self.variant(CKSSELECT_A::TK)
    }
}
#[doc = "Field `CKO` reader - Transmit Clock Output Mode Selection"]
pub type CKO_R = crate::FieldReader<u8, CKOSELECT_A>;
#[doc = "Transmit Clock Output Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKOSELECT_A {
    #[doc = "0: None, TK pin is an input"]
    NONE = 0,
    #[doc = "1: Continuous Transmit Clock, TK pin is an output"]
    CONTINUOUS = 1,
    #[doc = "2: Transmit Clock only during data transfers, TK pin is an output"]
    TRANSFER = 2,
}
impl From<CKOSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CKOSELECT_A) -> Self {
        variant as _
    }
}
impl CKO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKOSELECT_A> {
        match self.bits {
            0 => Some(CKOSELECT_A::NONE),
            1 => Some(CKOSELECT_A::CONTINUOUS),
            2 => Some(CKOSELECT_A::TRANSFER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CKOSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CKOSELECT_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `TRANSFER`"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == CKOSELECT_A::TRANSFER
    }
}
#[doc = "Field `CKO` writer - Transmit Clock Output Mode Selection"]
pub type CKO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCMR_SPEC, u8, CKOSELECT_A, 3, O>;
impl<'a, const O: u8> CKO_W<'a, O> {
    #[doc = "None, TK pin is an input"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(CKOSELECT_A::NONE)
    }
    #[doc = "Continuous Transmit Clock, TK pin is an output"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CKOSELECT_A::CONTINUOUS)
    }
    #[doc = "Transmit Clock only during data transfers, TK pin is an output"]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut W {
        self.variant(CKOSELECT_A::TRANSFER)
    }
}
#[doc = "Field `CKI` reader - Transmit Clock Inversion"]
pub type CKI_R = crate::BitReader<bool>;
#[doc = "Field `CKI` writer - Transmit Clock Inversion"]
pub type CKI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCMR_SPEC, bool, O>;
#[doc = "Field `CKG` reader - Transmit Clock Gating Selection"]
pub type CKG_R = crate::FieldReader<u8, CKGSELECT_A>;
#[doc = "Transmit Clock Gating Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKGSELECT_A {
    #[doc = "0: None"]
    CONTINUOUS = 0,
    #[doc = "1: Transmit Clock enabled only if TF Low"]
    EN_TF_LOW = 1,
    #[doc = "2: Transmit Clock enabled only if TF High"]
    EN_TF_HIGH = 2,
}
impl From<CKGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CKGSELECT_A) -> Self {
        variant as _
    }
}
impl CKG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKGSELECT_A> {
        match self.bits {
            0 => Some(CKGSELECT_A::CONTINUOUS),
            1 => Some(CKGSELECT_A::EN_TF_LOW),
            2 => Some(CKGSELECT_A::EN_TF_HIGH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CKGSELECT_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `EN_TF_LOW`"]
    #[inline(always)]
    pub fn is_en_tf_low(&self) -> bool {
        *self == CKGSELECT_A::EN_TF_LOW
    }
    #[doc = "Checks if the value of the field is `EN_TF_HIGH`"]
    #[inline(always)]
    pub fn is_en_tf_high(&self) -> bool {
        *self == CKGSELECT_A::EN_TF_HIGH
    }
}
#[doc = "Field `CKG` writer - Transmit Clock Gating Selection"]
pub type CKG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCMR_SPEC, u8, CKGSELECT_A, 2, O>;
impl<'a, const O: u8> CKG_W<'a, O> {
    #[doc = "None"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CKGSELECT_A::CONTINUOUS)
    }
    #[doc = "Transmit Clock enabled only if TF Low"]
    #[inline(always)]
    pub fn en_tf_low(self) -> &'a mut W {
        self.variant(CKGSELECT_A::EN_TF_LOW)
    }
    #[doc = "Transmit Clock enabled only if TF High"]
    #[inline(always)]
    pub fn en_tf_high(self) -> &'a mut W {
        self.variant(CKGSELECT_A::EN_TF_HIGH)
    }
}
#[doc = "Field `START` reader - Transmit Start Selection"]
pub type START_R = crate::FieldReader<u8, STARTSELECT_A>;
#[doc = "Transmit Start Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STARTSELECT_A {
    #[doc = "0: Continuous, as soon as a word is written in the SSC_THR (if Transmit is enabled), and immediately after the end of transfer of the previous data"]
    CONTINUOUS = 0,
    #[doc = "1: Receive start"]
    RECEIVE = 1,
    #[doc = "2: Detection of a low level on TF signal"]
    TF_LOW = 2,
    #[doc = "3: Detection of a high level on TF signal"]
    TF_HIGH = 3,
    #[doc = "4: Detection of a falling edge on TF signal"]
    TF_FALLING = 4,
    #[doc = "5: Detection of a rising edge on TF signal"]
    TF_RISING = 5,
    #[doc = "6: Detection of any level change on TF signal"]
    TF_LEVEL = 6,
    #[doc = "7: Detection of any edge on TF signal"]
    TF_EDGE = 7,
}
impl From<STARTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTSELECT_A) -> Self {
        variant as _
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STARTSELECT_A> {
        match self.bits {
            0 => Some(STARTSELECT_A::CONTINUOUS),
            1 => Some(STARTSELECT_A::RECEIVE),
            2 => Some(STARTSELECT_A::TF_LOW),
            3 => Some(STARTSELECT_A::TF_HIGH),
            4 => Some(STARTSELECT_A::TF_FALLING),
            5 => Some(STARTSELECT_A::TF_RISING),
            6 => Some(STARTSELECT_A::TF_LEVEL),
            7 => Some(STARTSELECT_A::TF_EDGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == STARTSELECT_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `RECEIVE`"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == STARTSELECT_A::RECEIVE
    }
    #[doc = "Checks if the value of the field is `TF_LOW`"]
    #[inline(always)]
    pub fn is_tf_low(&self) -> bool {
        *self == STARTSELECT_A::TF_LOW
    }
    #[doc = "Checks if the value of the field is `TF_HIGH`"]
    #[inline(always)]
    pub fn is_tf_high(&self) -> bool {
        *self == STARTSELECT_A::TF_HIGH
    }
    #[doc = "Checks if the value of the field is `TF_FALLING`"]
    #[inline(always)]
    pub fn is_tf_falling(&self) -> bool {
        *self == STARTSELECT_A::TF_FALLING
    }
    #[doc = "Checks if the value of the field is `TF_RISING`"]
    #[inline(always)]
    pub fn is_tf_rising(&self) -> bool {
        *self == STARTSELECT_A::TF_RISING
    }
    #[doc = "Checks if the value of the field is `TF_LEVEL`"]
    #[inline(always)]
    pub fn is_tf_level(&self) -> bool {
        *self == STARTSELECT_A::TF_LEVEL
    }
    #[doc = "Checks if the value of the field is `TF_EDGE`"]
    #[inline(always)]
    pub fn is_tf_edge(&self) -> bool {
        *self == STARTSELECT_A::TF_EDGE
    }
}
#[doc = "Field `START` writer - Transmit Start Selection"]
pub type START_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCMR_SPEC, u8, STARTSELECT_A, 4, O>;
impl<'a, const O: u8> START_W<'a, O> {
    #[doc = "Continuous, as soon as a word is written in the SSC_THR (if Transmit is enabled), and immediately after the end of transfer of the previous data"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(STARTSELECT_A::CONTINUOUS)
    }
    #[doc = "Receive start"]
    #[inline(always)]
    pub fn receive(self) -> &'a mut W {
        self.variant(STARTSELECT_A::RECEIVE)
    }
    #[doc = "Detection of a low level on TF signal"]
    #[inline(always)]
    pub fn tf_low(self) -> &'a mut W {
        self.variant(STARTSELECT_A::TF_LOW)
    }
    #[doc = "Detection of a high level on TF signal"]
    #[inline(always)]
    pub fn tf_high(self) -> &'a mut W {
        self.variant(STARTSELECT_A::TF_HIGH)
    }
    #[doc = "Detection of a falling edge on TF signal"]
    #[inline(always)]
    pub fn tf_falling(self) -> &'a mut W {
        self.variant(STARTSELECT_A::TF_FALLING)
    }
    #[doc = "Detection of a rising edge on TF signal"]
    #[inline(always)]
    pub fn tf_rising(self) -> &'a mut W {
        self.variant(STARTSELECT_A::TF_RISING)
    }
    #[doc = "Detection of any level change on TF signal"]
    #[inline(always)]
    pub fn tf_level(self) -> &'a mut W {
        self.variant(STARTSELECT_A::TF_LEVEL)
    }
    #[doc = "Detection of any edge on TF signal"]
    #[inline(always)]
    pub fn tf_edge(self) -> &'a mut W {
        self.variant(STARTSELECT_A::TF_EDGE)
    }
}
#[doc = "Field `STTDLY` reader - Transmit Start Delay"]
pub type STTDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STTDLY` writer - Transmit Start Delay"]
pub type STTDLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCMR_SPEC, u8, u8, 8, O>;
#[doc = "Field `PERIOD` reader - Transmit Period Divider Selection"]
pub type PERIOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERIOD` writer - Transmit Period Divider Selection"]
pub type PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCMR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:1 - Transmit Clock Selection"]
    #[inline(always)]
    pub fn cks(&self) -> CKS_R {
        CKS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Transmit Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&self) -> CKO_R {
        CKO_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Transmit Clock Inversion"]
    #[inline(always)]
    pub fn cki(&self) -> CKI_R {
        CKI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Transmit Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&self) -> CKG_R {
        CKG_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Transmit Start Selection"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Transmit Start Delay"]
    #[inline(always)]
    pub fn sttdly(&self) -> STTDLY_R {
        STTDLY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transmit Period Divider Selection"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transmit Clock Selection"]
    #[inline(always)]
    pub fn cks(&mut self) -> CKS_W<0> {
        CKS_W::new(self)
    }
    #[doc = "Bits 2:4 - Transmit Clock Output Mode Selection"]
    #[inline(always)]
    pub fn cko(&mut self) -> CKO_W<2> {
        CKO_W::new(self)
    }
    #[doc = "Bit 5 - Transmit Clock Inversion"]
    #[inline(always)]
    pub fn cki(&mut self) -> CKI_W<5> {
        CKI_W::new(self)
    }
    #[doc = "Bits 6:7 - Transmit Clock Gating Selection"]
    #[inline(always)]
    pub fn ckg(&mut self) -> CKG_W<6> {
        CKG_W::new(self)
    }
    #[doc = "Bits 8:11 - Transmit Start Selection"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<8> {
        START_W::new(self)
    }
    #[doc = "Bits 16:23 - Transmit Start Delay"]
    #[inline(always)]
    pub fn sttdly(&mut self) -> STTDLY_W<16> {
        STTDLY_W::new(self)
    }
    #[doc = "Bits 24:31 - Transmit Period Divider Selection"]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W<24> {
        PERIOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Clock Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcmr](index.html) module"]
pub struct TCMR_SPEC;
impl crate::RegisterSpec for TCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcmr::R](R) reader structure"]
impl crate::Readable for TCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcmr::W](W) writer structure"]
impl crate::Writable for TCMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCMR to value 0"]
impl crate::Resettable for TCMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
