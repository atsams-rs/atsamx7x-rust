#[doc = "Register `MDR` reader"]
pub struct R(crate::R<MDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDR` writer"]
pub struct W(crate::W<MDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDR_SPEC>;
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
impl From<crate::W<MDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Memory Device Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MD_A {
    #[doc = "0: SDRAM"]
    SDRAM = 0,
    #[doc = "1: Low-power SDRAM"]
    LPSDRAM = 1,
}
impl From<MD_A> for u8 {
    #[inline(always)]
    fn from(variant: MD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MD` reader - Memory Device Type"]
pub type MD_R = crate::FieldReader<u8, MD_A>;
impl MD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MD_A> {
        match self.bits {
            0 => Some(MD_A::SDRAM),
            1 => Some(MD_A::LPSDRAM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SDRAM`"]
    #[inline(always)]
    pub fn is_sdram(&self) -> bool {
        *self == MD_A::SDRAM
    }
    #[doc = "Checks if the value of the field is `LPSDRAM`"]
    #[inline(always)]
    pub fn is_lpsdram(&self) -> bool {
        *self == MD_A::LPSDRAM
    }
}
#[doc = "Field `MD` writer - Memory Device Type"]
pub type MD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDR_SPEC, u8, MD_A, 2, O>;
impl<'a, const O: u8> MD_W<'a, O> {
    #[doc = "SDRAM"]
    #[inline(always)]
    pub fn sdram(self) -> &'a mut W {
        self.variant(MD_A::SDRAM)
    }
    #[doc = "Low-power SDRAM"]
    #[inline(always)]
    pub fn lpsdram(self) -> &'a mut W {
        self.variant(MD_A::LPSDRAM)
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory Device Type"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory Device Type"]
    #[inline(always)]
    pub fn md(&mut self) -> MD_W<0> {
        MD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAMC Memory Device Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdr](index.html) module"]
pub struct MDR_SPEC;
impl crate::RegisterSpec for MDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdr::R](R) reader structure"]
impl crate::Readable for MDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdr::W](W) writer structure"]
impl crate::Writable for MDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDR to value 0"]
impl crate::Resettable for MDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
