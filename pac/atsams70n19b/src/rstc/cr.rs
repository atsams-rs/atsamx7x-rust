#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROCRST` writer - Processor Reset"]
pub type PROCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EXTRST` writer - External Reset"]
pub type EXTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "System Reset Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEYSELECT_AW {
    #[doc = "165: Writing any other value in this field aborts the write operation."]
    PASSWD = 165,
}
impl From<KEYSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEYSELECT_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` writer - System Reset Key"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, KEYSELECT_AW, 8, O>;
impl<'a, const O: u8> KEY_W<'a, O> {
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEYSELECT_AW::PASSWD)
    }
}
impl W {
    #[doc = "Bit 0 - Processor Reset"]
    #[inline(always)]
    pub fn procrst(&mut self) -> PROCRST_W<0> {
        PROCRST_W::new(self)
    }
    #[doc = "Bit 3 - External Reset"]
    #[inline(always)]
    pub fn extrst(&mut self) -> EXTRST_W<3> {
        EXTRST_W::new(self)
    }
    #[doc = "Bits 24:31 - System Reset Key"]
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
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
