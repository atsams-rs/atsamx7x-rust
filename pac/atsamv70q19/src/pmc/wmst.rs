#[doc = "Register `WMST` reader"]
pub struct R(crate::R<WMST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WMST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WMST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WMST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WMST` writer"]
pub struct W(crate::W<WMST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WMST_SPEC>;
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
impl From<crate::W<WMST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WMST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WMST` reader - Wait Mode Startup Time"]
pub type WMST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WMST` writer - Wait Mode Startup Time"]
pub type WMST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WMST_SPEC, u8, u8, 8, O>;
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
pub type KEY_R = crate::FieldReader<u8, KEY_A>;
impl KEY_R {
    #[doc = "Get enumerated values variant"]
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
        *self == KEY_A::PASSWD
    }
}
#[doc = "Field `KEY` writer - Write Access Password"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WMST_SPEC, u8, KEY_A, 8, O>;
impl<'a, const O: u8> KEY_W<'a, O> {
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEY_A::PASSWD)
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
    pub fn wmst(&mut self) -> WMST_W<0> {
        WMST_W::new(self)
    }
    #[doc = "Bits 24:31 - Write Access Password"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<24> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wait Mode Startup Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmst](index.html) module"]
pub struct WMST_SPEC;
impl crate::RegisterSpec for WMST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wmst::R](R) reader structure"]
impl crate::Readable for WMST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wmst::W](W) writer structure"]
impl crate::Writable for WMST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WMST to value 0"]
impl crate::Resettable for WMST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
