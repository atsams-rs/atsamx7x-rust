#[doc = "Register `SDCR` reader"]
pub struct R(crate::R<SDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDCR` writer"]
pub struct W(crate::W<SDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDCR_SPEC>;
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
impl From<crate::W<SDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SDCard/SDIO Slot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDCSEL_A {
    #[doc = "0: Slot A is selected."]
    SLOTA = 0,
}
impl From<SDCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SDCSEL` reader - SDCard/SDIO Slot"]
pub type SDCSEL_R = crate::FieldReader<u8, SDCSEL_A>;
impl SDCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SDCSEL_A> {
        match self.bits {
            0 => Some(SDCSEL_A::SLOTA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SLOTA`"]
    #[inline(always)]
    pub fn is_slota(&self) -> bool {
        *self == SDCSEL_A::SLOTA
    }
}
#[doc = "Field `SDCSEL` writer - SDCard/SDIO Slot"]
pub type SDCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDCR_SPEC, u8, SDCSEL_A, 2, O>;
impl<'a, const O: u8> SDCSEL_W<'a, O> {
    #[doc = "Slot A is selected."]
    #[inline(always)]
    pub fn slota(self) -> &'a mut W {
        self.variant(SDCSEL_A::SLOTA)
    }
}
#[doc = "SDCard/SDIO Bus Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDCBUS_A {
    #[doc = "0: 1 bit"]
    _1 = 0,
    #[doc = "2: 4 bits"]
    _4 = 2,
    #[doc = "3: 8 bits"]
    _8 = 3,
}
impl From<SDCBUS_A> for u8 {
    #[inline(always)]
    fn from(variant: SDCBUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SDCBUS` reader - SDCard/SDIO Bus Width"]
pub type SDCBUS_R = crate::FieldReader<u8, SDCBUS_A>;
impl SDCBUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SDCBUS_A> {
        match self.bits {
            0 => Some(SDCBUS_A::_1),
            2 => Some(SDCBUS_A::_4),
            3 => Some(SDCBUS_A::_8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDCBUS_A::_1
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == SDCBUS_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == SDCBUS_A::_8
    }
}
#[doc = "Field `SDCBUS` writer - SDCard/SDIO Bus Width"]
pub type SDCBUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDCR_SPEC, u8, SDCBUS_A, 2, O>;
impl<'a, const O: u8> SDCBUS_W<'a, O> {
    #[doc = "1 bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDCBUS_A::_1)
    }
    #[doc = "4 bits"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(SDCBUS_A::_4)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(SDCBUS_A::_8)
    }
}
impl R {
    #[doc = "Bits 0:1 - SDCard/SDIO Slot"]
    #[inline(always)]
    pub fn sdcsel(&self) -> SDCSEL_R {
        SDCSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 6:7 - SDCard/SDIO Bus Width"]
    #[inline(always)]
    pub fn sdcbus(&self) -> SDCBUS_R {
        SDCBUS_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SDCard/SDIO Slot"]
    #[inline(always)]
    pub fn sdcsel(&mut self) -> SDCSEL_W<0> {
        SDCSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - SDCard/SDIO Bus Width"]
    #[inline(always)]
    pub fn sdcbus(&mut self) -> SDCBUS_W<6> {
        SDCBUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD/SDIO Card Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdcr](index.html) module"]
pub struct SDCR_SPEC;
impl crate::RegisterSpec for SDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdcr::R](R) reader structure"]
impl crate::Readable for SDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdcr::W](W) writer structure"]
impl crate::Writable for SDCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDCR to value 0"]
impl crate::Resettable for SDCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
