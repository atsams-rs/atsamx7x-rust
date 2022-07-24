#[doc = "Register `IMRPQ[%s]` reader"]
pub struct R(crate::R<IMRPQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMRPQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMRPQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMRPQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMRPQ[%s]` writer"]
pub struct W(crate::W<IMRPQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMRPQ_SPEC>;
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
impl From<crate::W<IMRPQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMRPQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCOMP` reader - Receive Complete"]
pub type RCOMP_R = crate::BitReader<bool>;
#[doc = "Field `RCOMP` writer - Receive Complete"]
pub type RCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMRPQ_SPEC, bool, O>;
#[doc = "Field `RXUBR` reader - RX Used Bit Read"]
pub type RXUBR_R = crate::BitReader<bool>;
#[doc = "Field `RXUBR` writer - RX Used Bit Read"]
pub type RXUBR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMRPQ_SPEC, bool, O>;
#[doc = "Field `RLEX` reader - Retry Limit Exceeded or Late Collision"]
pub type RLEX_R = crate::BitReader<bool>;
#[doc = "Field `RLEX` writer - Retry Limit Exceeded or Late Collision"]
pub type RLEX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMRPQ_SPEC, bool, O>;
#[doc = "Field `AHB` reader - AHB Error"]
pub type AHB_R = crate::BitReader<bool>;
#[doc = "Field `AHB` writer - AHB Error"]
pub type AHB_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMRPQ_SPEC, bool, O>;
#[doc = "Field `TCOMP` reader - Transmit Complete"]
pub type TCOMP_R = crate::BitReader<bool>;
#[doc = "Field `TCOMP` writer - Transmit Complete"]
pub type TCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMRPQ_SPEC, bool, O>;
#[doc = "Field `ROVR` reader - Receive Overrun"]
pub type ROVR_R = crate::BitReader<bool>;
#[doc = "Field `ROVR` writer - Receive Overrun"]
pub type ROVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMRPQ_SPEC, bool, O>;
#[doc = "Field `HRESP` reader - HRESP Not OK"]
pub type HRESP_R = crate::BitReader<bool>;
#[doc = "Field `HRESP` writer - HRESP Not OK"]
pub type HRESP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IMRPQ_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&self) -> RCOMP_R {
        RCOMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&self) -> RXUBR_R {
        RXUBR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded or Late Collision"]
    #[inline(always)]
    pub fn rlex(&self) -> RLEX_R {
        RLEX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AHB Error"]
    #[inline(always)]
    pub fn ahb(&self) -> AHB_R {
        AHB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&self) -> TCOMP_R {
        TCOMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&self) -> ROVR_R {
        ROVR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    pub fn hresp(&self) -> HRESP_R {
        HRESP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&mut self) -> RCOMP_W<1> {
        RCOMP_W::new(self)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&mut self) -> RXUBR_W<2> {
        RXUBR_W::new(self)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded or Late Collision"]
    #[inline(always)]
    pub fn rlex(&mut self) -> RLEX_W<5> {
        RLEX_W::new(self)
    }
    #[doc = "Bit 6 - AHB Error"]
    #[inline(always)]
    pub fn ahb(&mut self) -> AHB_W<6> {
        AHB_W::new(self)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&mut self) -> TCOMP_W<7> {
        TCOMP_W::new(self)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&mut self) -> ROVR_W<10> {
        ROVR_W::new(self)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    pub fn hresp(&mut self) -> HRESP_W<11> {
        HRESP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Register Priority Queue (1..5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imrpq](index.html) module"]
pub struct IMRPQ_SPEC;
impl crate::RegisterSpec for IMRPQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imrpq::R](R) reader structure"]
impl crate::Readable for IMRPQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imrpq::W](W) writer structure"]
impl crate::Writable for IMRPQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMRPQ[%s]
to value 0"]
impl crate::Resettable for IMRPQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
