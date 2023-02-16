#[doc = "Register `TSCC` reader"]
pub struct R(crate::R<TSCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSCC` writer"]
pub struct W(crate::W<TSCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSCC_SPEC>;
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
impl From<crate::W<TSCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSS` reader - Timestamp Select"]
pub type TSS_R = crate::FieldReader<u8, TSSSELECT_A>;
#[doc = "Timestamp Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSSSELECT_A {
    #[doc = "0: Timestamp counter value always 0x0000"]
    ALWAYS_0 = 0,
    #[doc = "1: Timestamp counter value incremented according to TCP"]
    TCP_INC = 1,
    #[doc = "2: External timestamp counter value used"]
    EXT_TIMESTAMP = 2,
}
impl From<TSSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TSSSELECT_A) -> Self {
        variant as _
    }
}
impl TSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSSSELECT_A> {
        match self.bits {
            0 => Some(TSSSELECT_A::ALWAYS_0),
            1 => Some(TSSSELECT_A::TCP_INC),
            2 => Some(TSSSELECT_A::EXT_TIMESTAMP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_0`"]
    #[inline(always)]
    pub fn is_always_0(&self) -> bool {
        *self == TSSSELECT_A::ALWAYS_0
    }
    #[doc = "Checks if the value of the field is `TCP_INC`"]
    #[inline(always)]
    pub fn is_tcp_inc(&self) -> bool {
        *self == TSSSELECT_A::TCP_INC
    }
    #[doc = "Checks if the value of the field is `EXT_TIMESTAMP`"]
    #[inline(always)]
    pub fn is_ext_timestamp(&self) -> bool {
        *self == TSSSELECT_A::EXT_TIMESTAMP
    }
}
#[doc = "Field `TSS` writer - Timestamp Select"]
pub type TSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSCC_SPEC, u8, TSSSELECT_A, 2, O>;
impl<'a, const O: u8> TSS_W<'a, O> {
    #[doc = "Timestamp counter value always 0x0000"]
    #[inline(always)]
    pub fn always_0(self) -> &'a mut W {
        self.variant(TSSSELECT_A::ALWAYS_0)
    }
    #[doc = "Timestamp counter value incremented according to TCP"]
    #[inline(always)]
    pub fn tcp_inc(self) -> &'a mut W {
        self.variant(TSSSELECT_A::TCP_INC)
    }
    #[doc = "External timestamp counter value used"]
    #[inline(always)]
    pub fn ext_timestamp(self) -> &'a mut W {
        self.variant(TSSSELECT_A::EXT_TIMESTAMP)
    }
}
#[doc = "Field `TCP` reader - Timestamp Counter Prescaler"]
pub type TCP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TCP` writer - Timestamp Counter Prescaler"]
pub type TCP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TSCC_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Timestamp Select"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler"]
    #[inline(always)]
    pub fn tcp(&self) -> TCP_R {
        TCP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timestamp Select"]
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W<0> {
        TSS_W::new(self)
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler"]
    #[inline(always)]
    pub fn tcp(&mut self) -> TCP_W<16> {
        TCP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timestamp Counter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tscc](index.html) module"]
pub struct TSCC_SPEC;
impl crate::RegisterSpec for TSCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tscc::R](R) reader structure"]
impl crate::Readable for TSCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tscc::W](W) writer structure"]
impl crate::Writable for TSCC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSCC to value 0"]
impl crate::Resettable for TSCC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
