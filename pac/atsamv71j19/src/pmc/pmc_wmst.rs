#[doc = "Register `PMC_WMST` reader"]
pub struct R(crate::R<PMC_WMST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_WMST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_WMST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_WMST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC_WMST` writer"]
pub struct W(crate::W<PMC_WMST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_WMST_SPEC>;
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
impl From<crate::W<PMC_WMST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_WMST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WMST` reader - Wait Mode Startup Time"]
pub struct WMST_R(crate::FieldReader<u8, u8>);
impl WMST_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WMST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WMST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WMST` writer - Wait Mode Startup Time"]
pub struct WMST_W<'a> {
    w: &'a mut W,
}
impl<'a> WMST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Write Access Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEY_A {
    #[doc = "90: Writing any other value in this field aborts the write operation.Always reads as 0."]
    PASSWD = 90,
}
impl From<KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` reader - Write Access Password"]
pub struct KEY_R(crate::FieldReader<u8, KEY_A>);
impl KEY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_A> {
        match self.bits {
            90 => Some(KEY_A::PASSWD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        **self == KEY_A::PASSWD
    }
}
impl core::ops::Deref for KEY_R {
    type Target = crate::FieldReader<u8, KEY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY` writer - Write Access Password"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEY_A::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Wait Mode Startup Time"]
    #[inline(always)]
    pub fn wmst(&self) -> WMST_R {
        WMST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Write Access Password"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait Mode Startup Time"]
    #[inline(always)]
    pub fn wmst(&mut self) -> WMST_W {
        WMST_W { w: self }
    }
    #[doc = "Bits 24:31 - Write Access Password"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wait Mode Startup Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_wmst](index.html) module"]
pub struct PMC_WMST_SPEC;
impl crate::RegisterSpec for PMC_WMST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_wmst::R](R) reader structure"]
impl crate::Readable for PMC_WMST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc_wmst::W](W) writer structure"]
impl crate::Writable for PMC_WMST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC_WMST to value 0"]
impl crate::Resettable for PMC_WMST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
