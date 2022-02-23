#[doc = "Register `MLB_HCTL` reader"]
pub struct R(crate::R<MLB_HCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MLB_HCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MLB_HCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MLB_HCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MLB_HCTL` writer"]
pub struct W(crate::W<MLB_HCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MLB_HCTL_SPEC>;
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
impl From<crate::W<MLB_HCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MLB_HCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RST0` reader - Address Generation Unit 0 Software Reset"]
pub struct RST0_R(crate::FieldReader<bool, bool>);
impl RST0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST0` writer - Address Generation Unit 0 Software Reset"]
pub struct RST0_W<'a> {
    w: &'a mut W,
}
impl<'a> RST0_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `RST1` reader - Address Generation Unit 1 Software Reset"]
pub struct RST1_R(crate::FieldReader<bool, bool>);
impl RST1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST1` writer - Address Generation Unit 1 Software Reset"]
pub struct RST1_W<'a> {
    w: &'a mut W,
}
impl<'a> RST1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `EN` reader - HBI Enable"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - HBI Enable"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Address Generation Unit 0 Software Reset"]
    #[inline(always)]
    pub fn rst0(&self) -> RST0_R {
        RST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Address Generation Unit 1 Software Reset"]
    #[inline(always)]
    pub fn rst1(&self) -> RST1_R {
        RST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 15 - HBI Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Address Generation Unit 0 Software Reset"]
    #[inline(always)]
    pub fn rst0(&mut self) -> RST0_W {
        RST0_W { w: self }
    }
    #[doc = "Bit 1 - Address Generation Unit 1 Software Reset"]
    #[inline(always)]
    pub fn rst1(&mut self) -> RST1_W {
        RST1_W { w: self }
    }
    #[doc = "Bit 15 - HBI Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlb_hctl](index.html) module"]
pub struct MLB_HCTL_SPEC;
impl crate::RegisterSpec for MLB_HCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mlb_hctl::R](R) reader structure"]
impl crate::Readable for MLB_HCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mlb_hctl::W](W) writer structure"]
impl crate::Writable for MLB_HCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MLB_HCTL to value 0"]
impl crate::Resettable for MLB_HCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
