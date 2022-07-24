#[doc = "Register `MMR` reader"]
pub struct R(crate::R<MMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMR` writer"]
pub struct W(crate::W<MMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMR_SPEC>;
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
impl From<crate::W<MMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Internal Device Address Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IADRSZ_A {
    #[doc = "0: No internal device address"]
    NONE = 0,
    #[doc = "1: One-byte internal device address"]
    _1_BYTE = 1,
    #[doc = "2: Two-byte internal device address"]
    _2_BYTE = 2,
    #[doc = "3: Three-byte internal device address"]
    _3_BYTE = 3,
}
impl From<IADRSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: IADRSZ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IADRSZ` reader - Internal Device Address Size"]
pub type IADRSZ_R = crate::FieldReader<u8, IADRSZ_A>;
impl IADRSZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IADRSZ_A {
        match self.bits {
            0 => IADRSZ_A::NONE,
            1 => IADRSZ_A::_1_BYTE,
            2 => IADRSZ_A::_2_BYTE,
            3 => IADRSZ_A::_3_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == IADRSZ_A::NONE
    }
    #[doc = "Checks if the value of the field is `_1_BYTE`"]
    #[inline(always)]
    pub fn is_1_byte(&self) -> bool {
        *self == IADRSZ_A::_1_BYTE
    }
    #[doc = "Checks if the value of the field is `_2_BYTE`"]
    #[inline(always)]
    pub fn is_2_byte(&self) -> bool {
        *self == IADRSZ_A::_2_BYTE
    }
    #[doc = "Checks if the value of the field is `_3_BYTE`"]
    #[inline(always)]
    pub fn is_3_byte(&self) -> bool {
        *self == IADRSZ_A::_3_BYTE
    }
}
#[doc = "Field `IADRSZ` writer - Internal Device Address Size"]
pub type IADRSZ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MMR_SPEC, u8, IADRSZ_A, 2, O>;
impl<'a, const O: u8> IADRSZ_W<'a, O> {
    #[doc = "No internal device address"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(IADRSZ_A::NONE)
    }
    #[doc = "One-byte internal device address"]
    #[inline(always)]
    pub fn _1_byte(self) -> &'a mut W {
        self.variant(IADRSZ_A::_1_BYTE)
    }
    #[doc = "Two-byte internal device address"]
    #[inline(always)]
    pub fn _2_byte(self) -> &'a mut W {
        self.variant(IADRSZ_A::_2_BYTE)
    }
    #[doc = "Three-byte internal device address"]
    #[inline(always)]
    pub fn _3_byte(self) -> &'a mut W {
        self.variant(IADRSZ_A::_3_BYTE)
    }
}
#[doc = "Field `MREAD` reader - Master Read Direction"]
pub type MREAD_R = crate::BitReader<bool>;
#[doc = "Field `MREAD` writer - Master Read Direction"]
pub type MREAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMR_SPEC, bool, O>;
#[doc = "Field `DADR` reader - Device Address"]
pub type DADR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DADR` writer - Device Address"]
pub type DADR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 8:9 - Internal Device Address Size"]
    #[inline(always)]
    pub fn iadrsz(&self) -> IADRSZ_R {
        IADRSZ_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Master Read Direction"]
    #[inline(always)]
    pub fn mread(&self) -> MREAD_R {
        MREAD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Device Address"]
    #[inline(always)]
    pub fn dadr(&self) -> DADR_R {
        DADR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - Internal Device Address Size"]
    #[inline(always)]
    pub fn iadrsz(&mut self) -> IADRSZ_W<8> {
        IADRSZ_W::new(self)
    }
    #[doc = "Bit 12 - Master Read Direction"]
    #[inline(always)]
    pub fn mread(&mut self) -> MREAD_W<12> {
        MREAD_W::new(self)
    }
    #[doc = "Bits 16:22 - Device Address"]
    #[inline(always)]
    pub fn dadr(&mut self) -> DADR_W<16> {
        DADR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmr](index.html) module"]
pub struct MMR_SPEC;
impl crate::RegisterSpec for MMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmr::R](R) reader structure"]
impl crate::Readable for MMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmr::W](W) writer structure"]
impl crate::Writable for MMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMR to value 0"]
impl crate::Resettable for MMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
