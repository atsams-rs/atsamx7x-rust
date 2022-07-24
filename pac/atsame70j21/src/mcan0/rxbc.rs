#[doc = "Register `RXBC` reader"]
pub struct R(crate::R<RXBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXBC` writer"]
pub struct W(crate::W<RXBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXBC_SPEC>;
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
impl From<crate::W<RXBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RBSA` reader - Receive Buffer Start Address"]
pub type RBSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RBSA` writer - Receive Buffer Start Address"]
pub type RBSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXBC_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 2:15 - Receive Buffer Start Address"]
    #[inline(always)]
    pub fn rbsa(&self) -> RBSA_R {
        RBSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:15 - Receive Buffer Start Address"]
    #[inline(always)]
    pub fn rbsa(&mut self) -> RBSA_W<2> {
        RBSA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Rx Buffer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxbc](index.html) module"]
pub struct RXBC_SPEC;
impl crate::RegisterSpec for RXBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxbc::R](R) reader structure"]
impl crate::Readable for RXBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxbc::W](W) writer structure"]
impl crate::Writable for RXBC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXBC to value 0"]
impl crate::Resettable for RXBC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
