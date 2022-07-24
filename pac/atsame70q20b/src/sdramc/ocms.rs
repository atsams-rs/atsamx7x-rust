#[doc = "Register `OCMS` reader"]
pub struct R(crate::R<OCMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCMS` writer"]
pub struct W(crate::W<OCMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCMS_SPEC>;
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
impl From<crate::W<OCMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDR_SE` reader - SDRAM Memory Controller Scrambling Enable"]
pub type SDR_SE_R = crate::BitReader<bool>;
#[doc = "Field `SDR_SE` writer - SDRAM Memory Controller Scrambling Enable"]
pub type SDR_SE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OCMS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SDRAM Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn sdr_se(&self) -> SDR_SE_R {
        SDR_SE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDRAM Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn sdr_se(&mut self) -> SDR_SE_W<0> {
        SDR_SE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAMC OCMS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ocms](index.html) module"]
pub struct OCMS_SPEC;
impl crate::RegisterSpec for OCMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ocms::R](R) reader structure"]
impl crate::Readable for OCMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ocms::W](W) writer structure"]
impl crate::Writable for OCMS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OCMS to value 0"]
impl crate::Resettable for OCMS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
