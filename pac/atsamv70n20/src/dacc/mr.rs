#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Max Speed Mode for Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXS0_A {
    #[doc = "0: External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    TRIG_EVENT = 0,
    #[doc = "1: Max speed mode enabled."]
    MAXIMUM = 1,
}
impl From<MAXS0_A> for bool {
    #[inline(always)]
    fn from(variant: MAXS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAXS0` reader - Max Speed Mode for Channel 0"]
pub type MAXS0_R = crate::BitReader<MAXS0_A>;
impl MAXS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAXS0_A {
        match self.bits {
            false => MAXS0_A::TRIG_EVENT,
            true => MAXS0_A::MAXIMUM,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG_EVENT`"]
    #[inline(always)]
    pub fn is_trig_event(&self) -> bool {
        *self == MAXS0_A::TRIG_EVENT
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == MAXS0_A::MAXIMUM
    }
}
#[doc = "Field `MAXS0` writer - Max Speed Mode for Channel 0"]
pub type MAXS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, MAXS0_A, O>;
impl<'a, const O: u8> MAXS0_W<'a, O> {
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    #[inline(always)]
    pub fn trig_event(self) -> &'a mut W {
        self.variant(MAXS0_A::TRIG_EVENT)
    }
    #[doc = "Max speed mode enabled."]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(MAXS0_A::MAXIMUM)
    }
}
#[doc = "Max Speed Mode for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXS1_A {
    #[doc = "0: External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    TRIG_EVENT = 0,
    #[doc = "1: Max speed mode enabled."]
    MAXIMUM = 1,
}
impl From<MAXS1_A> for bool {
    #[inline(always)]
    fn from(variant: MAXS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAXS1` reader - Max Speed Mode for Channel 1"]
pub type MAXS1_R = crate::BitReader<MAXS1_A>;
impl MAXS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAXS1_A {
        match self.bits {
            false => MAXS1_A::TRIG_EVENT,
            true => MAXS1_A::MAXIMUM,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG_EVENT`"]
    #[inline(always)]
    pub fn is_trig_event(&self) -> bool {
        *self == MAXS1_A::TRIG_EVENT
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == MAXS1_A::MAXIMUM
    }
}
#[doc = "Field `MAXS1` writer - Max Speed Mode for Channel 1"]
pub type MAXS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, MAXS1_A, O>;
impl<'a, const O: u8> MAXS1_W<'a, O> {
    #[doc = "External trigger mode or Free-running mode enabled. (See TRGENx.DACC_TRIGR.)"]
    #[inline(always)]
    pub fn trig_event(self) -> &'a mut W {
        self.variant(MAXS1_A::TRIG_EVENT)
    }
    #[doc = "Max speed mode enabled."]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(MAXS1_A::MAXIMUM)
    }
}
#[doc = "Word Transfer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WORD_A {
    #[doc = "0: One data to convert is written to the FIFO per access to DACC."]
    DISABLED = 0,
    #[doc = "1: Two data to convert are written to the FIFO per access to DACC (reduces the number of requests to DMA and the number of system bus accesses)."]
    ENABLED = 1,
}
impl From<WORD_A> for bool {
    #[inline(always)]
    fn from(variant: WORD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WORD` reader - Word Transfer Mode"]
pub type WORD_R = crate::BitReader<WORD_A>;
impl WORD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WORD_A {
        match self.bits {
            false => WORD_A::DISABLED,
            true => WORD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WORD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WORD_A::ENABLED
    }
}
#[doc = "Field `WORD` writer - Word Transfer Mode"]
pub type WORD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, WORD_A, O>;
impl<'a, const O: u8> WORD_W<'a, O> {
    #[doc = "One data to convert is written to the FIFO per access to DACC."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WORD_A::DISABLED)
    }
    #[doc = "Two data to convert are written to the FIFO per access to DACC (reduces the number of requests to DMA and the number of system bus accesses)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WORD_A::ENABLED)
    }
}
#[doc = "Field `ZERO` reader - Must always be written to 0."]
pub type ZERO_R = crate::BitReader<bool>;
#[doc = "Field `ZERO` writer - Must always be written to 0."]
pub type ZERO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Differential Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFF_A {
    #[doc = "0: DAC0 and DAC1 are single-ended outputs."]
    DISABLED = 0,
    #[doc = "1: DACP and DACN are differential outputs. The differential level is configured by the channel 0 value."]
    ENABLED = 1,
}
impl From<DIFF_A> for bool {
    #[inline(always)]
    fn from(variant: DIFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIFF` reader - Differential Mode"]
pub type DIFF_R = crate::BitReader<DIFF_A>;
impl DIFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFF_A {
        match self.bits {
            false => DIFF_A::DISABLED,
            true => DIFF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIFF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIFF_A::ENABLED
    }
}
#[doc = "Field `DIFF` writer - Differential Mode"]
pub type DIFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, DIFF_A, O>;
impl<'a, const O: u8> DIFF_W<'a, O> {
    #[doc = "DAC0 and DAC1 are single-ended outputs."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIFF_A::DISABLED)
    }
    #[doc = "DACP and DACN are differential outputs. The differential level is configured by the channel 0 value."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIFF_A::ENABLED)
    }
}
#[doc = "Field `PRESCALER` reader - Peripheral Clock to DAC Clock Ratio"]
pub type PRESCALER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESCALER` writer - Peripheral Clock to DAC Clock Ratio"]
pub type PRESCALER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Max Speed Mode for Channel 0"]
    #[inline(always)]
    pub fn maxs0(&self) -> MAXS0_R {
        MAXS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Max Speed Mode for Channel 1"]
    #[inline(always)]
    pub fn maxs1(&self) -> MAXS1_R {
        MAXS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Word Transfer Mode"]
    #[inline(always)]
    pub fn word(&self) -> WORD_R {
        WORD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Must always be written to 0."]
    #[inline(always)]
    pub fn zero(&self) -> ZERO_R {
        ZERO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 23 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Peripheral Clock to DAC Clock Ratio"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Max Speed Mode for Channel 0"]
    #[inline(always)]
    pub fn maxs0(&mut self) -> MAXS0_W<0> {
        MAXS0_W::new(self)
    }
    #[doc = "Bit 1 - Max Speed Mode for Channel 1"]
    #[inline(always)]
    pub fn maxs1(&mut self) -> MAXS1_W<1> {
        MAXS1_W::new(self)
    }
    #[doc = "Bit 4 - Word Transfer Mode"]
    #[inline(always)]
    pub fn word(&mut self) -> WORD_W<4> {
        WORD_W::new(self)
    }
    #[doc = "Bit 5 - Must always be written to 0."]
    #[inline(always)]
    pub fn zero(&mut self) -> ZERO_W<5> {
        ZERO_W::new(self)
    }
    #[doc = "Bit 23 - Differential Mode"]
    #[inline(always)]
    pub fn diff(&mut self) -> DIFF_W<23> {
        DIFF_W::new(self)
    }
    #[doc = "Bits 24:27 - Peripheral Clock to DAC Clock Ratio"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W<24> {
        PRESCALER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
