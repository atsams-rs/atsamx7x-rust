#[doc = "Register `MLB_MLBC1` reader"]
pub struct R(crate::R<MLB_MLBC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MLB_MLBC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MLB_MLBC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MLB_MLBC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MLB_MLBC1` writer"]
pub struct W(crate::W<MLB_MLBC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MLB_MLBC1_SPEC>;
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
impl From<crate::W<MLB_MLBC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MLB_MLBC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - MediaLB Lock Error Status (cleared by writing a 0)"]
pub struct LOCK_R(crate::FieldReader<bool, bool>);
impl LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` writer - MediaLB Lock Error Status (cleared by writing a 0)"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CLKM` reader - MediaLB Clock Missing Status (cleared by writing a 0)"]
pub struct CLKM_R(crate::FieldReader<bool, bool>);
impl CLKM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLKM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKM` writer - MediaLB Clock Missing Status (cleared by writing a 0)"]
pub struct CLKM_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `NDA` reader - Node Device Address"]
pub struct NDA_R(crate::FieldReader<u8, u8>);
impl NDA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDA` writer - Node Device Address"]
pub struct NDA_W<'a> {
    w: &'a mut W,
}
impl<'a> NDA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 6 - MediaLB Lock Error Status (cleared by writing a 0)"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - MediaLB Clock Missing Status (cleared by writing a 0)"]
    #[inline(always)]
    pub fn clkm(&self) -> CLKM_R {
        CLKM_R::new(((self.bits >> 7) & 0x01) != 0)
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
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Bit 7 - MediaLB Clock Missing Status (cleared by writing a 0)"]
    #[inline(always)]
    pub fn clkm(&mut self) -> CLKM_W {
        CLKM_W { w: self }
    }
    #[doc = "Bits 8:15 - Node Device Address"]
    #[inline(always)]
    pub fn nda(&mut self) -> NDA_W {
        NDA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MediaLB Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlb_mlbc1](index.html) module"]
pub struct MLB_MLBC1_SPEC;
impl crate::RegisterSpec for MLB_MLBC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mlb_mlbc1::R](R) reader structure"]
impl crate::Readable for MLB_MLBC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mlb_mlbc1::W](W) writer structure"]
impl crate::Writable for MLB_MLBC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MLB_MLBC1 to value 0"]
impl crate::Resettable for MLB_MLBC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
