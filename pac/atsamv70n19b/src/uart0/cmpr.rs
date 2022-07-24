#[doc = "Register `CMPR` reader"]
pub struct R(crate::R<CMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPR` writer"]
pub struct W(crate::W<CMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPR_SPEC>;
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
impl From<crate::W<CMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL1` reader - First Comparison Value for Received Character"]
pub type VAL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VAL1` writer - First Comparison Value for Received Character"]
pub type VAL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPR_SPEC, u8, u8, 8, O>;
#[doc = "Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPMODE_A {
    #[doc = "0: Any character is received and comparison function drives CMP flag."]
    FLAG_ONLY = 0,
    #[doc = "1: Comparison condition must be met to start reception."]
    START_CONDITION = 1,
}
impl From<CMPMODE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPMODE` reader - Comparison Mode"]
pub type CMPMODE_R = crate::BitReader<CMPMODE_A>;
impl CMPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMODE_A {
        match self.bits {
            false => CMPMODE_A::FLAG_ONLY,
            true => CMPMODE_A::START_CONDITION,
        }
    }
    #[doc = "Checks if the value of the field is `FLAG_ONLY`"]
    #[inline(always)]
    pub fn is_flag_only(&self) -> bool {
        *self == CMPMODE_A::FLAG_ONLY
    }
    #[doc = "Checks if the value of the field is `START_CONDITION`"]
    #[inline(always)]
    pub fn is_start_condition(&self) -> bool {
        *self == CMPMODE_A::START_CONDITION
    }
}
#[doc = "Field `CMPMODE` writer - Comparison Mode"]
pub type CMPMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPR_SPEC, CMPMODE_A, O>;
impl<'a, const O: u8> CMPMODE_W<'a, O> {
    #[doc = "Any character is received and comparison function drives CMP flag."]
    #[inline(always)]
    pub fn flag_only(self) -> &'a mut W {
        self.variant(CMPMODE_A::FLAG_ONLY)
    }
    #[doc = "Comparison condition must be met to start reception."]
    #[inline(always)]
    pub fn start_condition(self) -> &'a mut W {
        self.variant(CMPMODE_A::START_CONDITION)
    }
}
#[doc = "Field `CMPPAR` reader - Compare Parity"]
pub type CMPPAR_R = crate::BitReader<bool>;
#[doc = "Field `CMPPAR` writer - Compare Parity"]
pub type CMPPAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPR_SPEC, bool, O>;
#[doc = "Field `VAL2` reader - Second Comparison Value for Received Character"]
pub type VAL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VAL2` writer - Second Comparison Value for Received Character"]
pub type VAL2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - First Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val1(&self) -> VAL1_R {
        VAL1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CMPMODE_R {
        CMPMODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Compare Parity"]
    #[inline(always)]
    pub fn cmppar(&self) -> CMPPAR_R {
        CMPPAR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Second Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val2(&self) -> VAL2_R {
        VAL2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - First Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val1(&mut self) -> VAL1_W<0> {
        VAL1_W::new(self)
    }
    #[doc = "Bit 12 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&mut self) -> CMPMODE_W<12> {
        CMPMODE_W::new(self)
    }
    #[doc = "Bit 14 - Compare Parity"]
    #[inline(always)]
    pub fn cmppar(&mut self) -> CMPPAR_W<14> {
        CMPPAR_W::new(self)
    }
    #[doc = "Bits 16:23 - Second Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val2(&mut self) -> VAL2_W<16> {
        VAL2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparison Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpr](index.html) module"]
pub struct CMPR_SPEC;
impl crate::RegisterSpec for CMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpr::R](R) reader structure"]
impl crate::Readable for CMPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpr::W](W) writer structure"]
impl crate::Writable for CMPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPR to value 0"]
impl crate::Resettable for CMPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
