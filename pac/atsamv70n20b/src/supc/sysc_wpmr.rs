#[doc = "Register `SYSC_WPMR` reader"]
pub struct R(crate::R<SYSC_WPMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSC_WPMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSC_WPMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSC_WPMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSC_WPMR` writer"]
pub struct W(crate::W<SYSC_WPMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSC_WPMR_SPEC>;
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
impl From<crate::W<SYSC_WPMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSC_WPMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WPEN` reader - Write Protection Enable"]
pub type WPEN_R = crate::BitReader<bool>;
#[doc = "Field `WPEN` writer - Write Protection Enable"]
pub type WPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSC_WPMR_SPEC, bool, O>;
#[doc = "Write Protection Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum WPKEY_A {
    #[doc = "5395523: Writing any other value in this field aborts the write operation of the WPEN bit. Always reads as 0."]
    PASSWD = 5395523,
}
impl From<WPKEY_A> for u32 {
    #[inline(always)]
    fn from(variant: WPKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WPKEY` reader - Write Protection Key"]
pub type WPKEY_R = crate::FieldReader<u32, WPKEY_A>;
impl WPKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WPKEY_A> {
        match self.bits {
            5395523 => Some(WPKEY_A::PASSWD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == WPKEY_A::PASSWD
    }
}
#[doc = "Field `WPKEY` writer - Write Protection Key"]
pub type WPKEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYSC_WPMR_SPEC, u32, WPKEY_A, 24, O>;
impl<'a, const O: u8> WPKEY_W<'a, O> {
    #[doc = "Writing any other value in this field aborts the write operation of the WPEN bit. Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(WPKEY_A::PASSWD)
    }
}
impl R {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WPEN_R {
        WPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protection Key"]
    #[inline(always)]
    pub fn wpkey(&self) -> WPKEY_R {
        WPKEY_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&mut self) -> WPEN_W<0> {
        WPEN_W::new(self)
    }
    #[doc = "Bits 8:31 - Write Protection Key"]
    #[inline(always)]
    pub fn wpkey(&mut self) -> WPKEY_W<8> {
        WPKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Protection Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysc_wpmr](index.html) module"]
pub struct SYSC_WPMR_SPEC;
impl crate::RegisterSpec for SYSC_WPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysc_wpmr::R](R) reader structure"]
impl crate::Readable for SYSC_WPMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysc_wpmr::W](W) writer structure"]
impl crate::Writable for SYSC_WPMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSC_WPMR to value 0"]
impl crate::Resettable for SYSC_WPMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
