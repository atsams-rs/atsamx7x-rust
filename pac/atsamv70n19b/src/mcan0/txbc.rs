#[doc = "Register `TXBC` reader"]
pub struct R(crate::R<TXBC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXBC` writer"]
pub struct W(crate::W<TXBC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXBC_SPEC>;
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
impl From<crate::W<TXBC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXBC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBSA` reader - Tx Buffers Start Address"]
pub type TBSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TBSA` writer - Tx Buffers Start Address"]
pub type TBSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXBC_SPEC, u16, u16, 14, O>;
#[doc = "Field `NDTB` reader - Number of Dedicated Transmit Buffers"]
pub type NDTB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NDTB` writer - Number of Dedicated Transmit Buffers"]
pub type NDTB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXBC_SPEC, u8, u8, 6, O>;
#[doc = "Field `TFQS` reader - Transmit FIFO/Queue Size"]
pub type TFQS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFQS` writer - Transmit FIFO/Queue Size"]
pub type TFQS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXBC_SPEC, u8, u8, 6, O>;
#[doc = "Field `TFQM` reader - Tx FIFO/Queue Mode"]
pub type TFQM_R = crate::BitReader<bool>;
#[doc = "Field `TFQM` writer - Tx FIFO/Queue Mode"]
pub type TFQM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TXBC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 2:15 - Tx Buffers Start Address"]
    #[inline(always)]
    pub fn tbsa(&self) -> TBSA_R {
        TBSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - Number of Dedicated Transmit Buffers"]
    #[inline(always)]
    pub fn ndtb(&self) -> NDTB_R {
        NDTB_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Transmit FIFO/Queue Size"]
    #[inline(always)]
    pub fn tfqs(&self) -> TFQS_R {
        TFQS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Tx FIFO/Queue Mode"]
    #[inline(always)]
    pub fn tfqm(&self) -> TFQM_R {
        TFQM_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Tx Buffers Start Address"]
    #[inline(always)]
    pub fn tbsa(&mut self) -> TBSA_W<2> {
        TBSA_W::new(self)
    }
    #[doc = "Bits 16:21 - Number of Dedicated Transmit Buffers"]
    #[inline(always)]
    pub fn ndtb(&mut self) -> NDTB_W<16> {
        NDTB_W::new(self)
    }
    #[doc = "Bits 24:29 - Transmit FIFO/Queue Size"]
    #[inline(always)]
    pub fn tfqs(&mut self) -> TFQS_W<24> {
        TFQS_W::new(self)
    }
    #[doc = "Bit 30 - Tx FIFO/Queue Mode"]
    #[inline(always)]
    pub fn tfqm(&mut self) -> TFQM_W<30> {
        TFQM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Buffer Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txbc](index.html) module"]
pub struct TXBC_SPEC;
impl crate::RegisterSpec for TXBC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txbc::R](R) reader structure"]
impl crate::Readable for TXBC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txbc::W](W) writer structure"]
impl crate::Writable for TXBC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXBC to value 0"]
impl crate::Resettable for TXBC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
