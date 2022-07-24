#[doc = "Register `ST2CW01` reader"]
pub struct R(crate::R<ST2CW01_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ST2CW01_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ST2CW01_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ST2CW01_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ST2CW01` writer"]
pub struct W(crate::W<ST2CW01_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ST2CW01_SPEC>;
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
impl From<crate::W<ST2CW01_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ST2CW01_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASKVAL` reader - Mask Value"]
pub type MASKVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MASKVAL` writer - Mask Value"]
pub type MASKVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ST2CW01_SPEC, u16, u16, 16, O>;
#[doc = "Field `COMPVAL` reader - Compare Value"]
pub type COMPVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMPVAL` writer - Compare Value"]
pub type COMPVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ST2CW01_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Mask Value"]
    #[inline(always)]
    pub fn maskval(&self) -> MASKVAL_R {
        MASKVAL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Compare Value"]
    #[inline(always)]
    pub fn compval(&self) -> COMPVAL_R {
        COMPVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mask Value"]
    #[inline(always)]
    pub fn maskval(&mut self) -> MASKVAL_W<0> {
        MASKVAL_W::new(self)
    }
    #[doc = "Bits 16:31 - Compare Value"]
    #[inline(always)]
    pub fn compval(&mut self) -> COMPVAL_W<16> {
        COMPVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Screening Type 2 Compare Word 0 Register (index = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st2cw01](index.html) module"]
pub struct ST2CW01_SPEC;
impl crate::RegisterSpec for ST2CW01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [st2cw01::R](R) reader structure"]
impl crate::Readable for ST2CW01_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [st2cw01::W](W) writer structure"]
impl crate::Writable for ST2CW01_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ST2CW01 to value 0"]
impl crate::Resettable for ST2CW01_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
