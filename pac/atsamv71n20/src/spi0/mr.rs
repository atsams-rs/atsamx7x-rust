#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTR` reader - Master/Slave Mode"]
pub type MSTR_R = crate::BitReader<bool>;
#[doc = "Field `MSTR` writer - Master/Slave Mode"]
pub type MSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `PS` reader - Peripheral Select"]
pub type PS_R = crate::BitReader<bool>;
#[doc = "Field `PS` writer - Peripheral Select"]
pub type PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `PCSDEC` reader - Chip Select Decode"]
pub type PCSDEC_R = crate::BitReader<bool>;
#[doc = "Field `PCSDEC` writer - Chip Select Decode"]
pub type PCSDEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `MODFDIS` reader - Mode Fault Detection"]
pub type MODFDIS_R = crate::BitReader<bool>;
#[doc = "Field `MODFDIS` writer - Mode Fault Detection"]
pub type MODFDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `WDRBT` reader - Wait Data Read Before Transfer"]
pub type WDRBT_R = crate::BitReader<bool>;
#[doc = "Field `WDRBT` writer - Wait Data Read Before Transfer"]
pub type WDRBT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `LLB` reader - Local Loopback Enable"]
pub type LLB_R = crate::BitReader<bool>;
#[doc = "Field `LLB` writer - Local Loopback Enable"]
pub type LLB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `PCS` reader - Peripheral Chip Select"]
pub type PCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCS` writer - Peripheral Chip Select"]
pub type PCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 4, O>;
#[doc = "Field `DLYBCS` reader - Delay Between Chip Selects"]
pub type DLYBCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYBCS` writer - Delay Between Chip Selects"]
pub type DLYBCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline(always)]
    pub fn pcsdec(&self) -> PCSDEC_R {
        PCSDEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline(always)]
    pub fn modfdis(&self) -> MODFDIS_R {
        MODFDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&self) -> WDRBT_R {
        WDRBT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&self) -> LLB_R {
        LLB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline(always)]
    pub fn dlybcs(&self) -> DLYBCS_R {
        DLYBCS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W<0> {
        MSTR_W::new(self)
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<1> {
        PS_W::new(self)
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline(always)]
    pub fn pcsdec(&mut self) -> PCSDEC_W<2> {
        PCSDEC_W::new(self)
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline(always)]
    pub fn modfdis(&mut self) -> MODFDIS_W<4> {
        MODFDIS_W::new(self)
    }
    #[doc = "Bit 5 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&mut self) -> WDRBT_W<5> {
        WDRBT_W::new(self)
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&mut self) -> LLB_W<7> {
        LLB_W::new(self)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&mut self) -> PCS_W<16> {
        PCS_W::new(self)
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline(always)]
    pub fn dlybcs(&mut self) -> DLYBCS_W<24> {
        DLYBCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
