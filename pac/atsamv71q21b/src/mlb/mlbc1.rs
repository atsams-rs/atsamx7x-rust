#[doc = "Register `MLBC1` reader"]
pub struct R(crate::R<MLBC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MLBC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MLBC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MLBC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MLBC1` writer"]
pub struct W(crate::W<MLBC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MLBC1_SPEC>;
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
impl From<crate::W<MLBC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MLBC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - MediaLB Lock Error Status (cleared by writing a 0)"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - MediaLB Lock Error Status (cleared by writing a 0)"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MLBC1_SPEC, bool, O>;
#[doc = "Field `CLKM` reader - MediaLB Clock Missing Status (cleared by writing a 0)"]
pub type CLKM_R = crate::BitReader<bool>;
#[doc = "Field `CLKM` writer - MediaLB Clock Missing Status (cleared by writing a 0)"]
pub type CLKM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MLBC1_SPEC, bool, O>;
#[doc = "Field `NDA` reader - Node Device Address"]
pub type NDA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NDA` writer - Node Device Address"]
pub type NDA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MLBC1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 6 - MediaLB Lock Error Status (cleared by writing a 0)"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MediaLB Clock Missing Status (cleared by writing a 0)"]
    #[inline(always)]
    pub fn clkm(&self) -> CLKM_R {
        CLKM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Node Device Address"]
    #[inline(always)]
    pub fn nda(&self) -> NDA_R {
        NDA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 6 - MediaLB Lock Error Status (cleared by writing a 0)"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<6> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 7 - MediaLB Clock Missing Status (cleared by writing a 0)"]
    #[inline(always)]
    pub fn clkm(&mut self) -> CLKM_W<7> {
        CLKM_W::new(self)
    }
    #[doc = "Bits 8:15 - Node Device Address"]
    #[inline(always)]
    pub fn nda(&mut self) -> NDA_W<8> {
        NDA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MediaLB Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlbc1](index.html) module"]
pub struct MLBC1_SPEC;
impl crate::RegisterSpec for MLBC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mlbc1::R](R) reader structure"]
impl crate::Readable for MLBC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mlbc1::W](W) writer structure"]
impl crate::Writable for MLBC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MLBC1 to value 0"]
impl crate::Resettable for MLBC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
