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
#[doc = "Field `QSPIEN` writer - QSPI Enable"]
pub type QSPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `QSPIDIS` writer - QSPI Disable"]
pub type QSPIDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SWRST` writer - QSPI Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `LASTXFER` writer - Last Transfer"]
pub type LASTXFER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - QSPI Enable"]
    #[inline(always)]
    pub fn qspien(&mut self) -> QSPIEN_W<0> {
        QSPIEN_W::new(self)
    }
    #[doc = "Bit 1 - QSPI Disable"]
    #[inline(always)]
    pub fn qspidis(&mut self) -> QSPIDIS_W<1> {
        QSPIDIS_W::new(self)
    }
    #[doc = "Bit 7 - QSPI Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W<7> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 24 - Last Transfer"]
    #[inline(always)]
    pub fn lastxfer(&mut self) -> LASTXFER_W<24> {
        LASTXFER_W::new(self)
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
