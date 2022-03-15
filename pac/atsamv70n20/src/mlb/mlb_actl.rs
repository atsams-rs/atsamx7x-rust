#[doc = "Register `MLB_ACTL` reader"]
pub struct R(crate::R<MLB_ACTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MLB_ACTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MLB_ACTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MLB_ACTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MLB_ACTL` writer"]
pub struct W(crate::W<MLB_ACTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MLB_ACTL_SPEC>;
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
impl From<crate::W<MLB_ACTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MLB_ACTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCE` reader - Software Clear Enable"]
pub struct SCE_R(crate::FieldReader<bool, bool>);
impl SCE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCE` writer - Software Clear Enable"]
pub struct SCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCE_W<'a> {
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
#[doc = "Field `SMX` reader - AHB Interrupt Mux Enable"]
pub struct SMX_R(crate::FieldReader<bool, bool>);
impl SMX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SMX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMX` writer - AHB Interrupt Mux Enable"]
pub struct SMX_W<'a> {
    w: &'a mut W,
}
impl<'a> SMX_W<'a> {
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
#[doc = "Field `DMA_MODE` reader - DMA Mode"]
pub struct DMA_MODE_R(crate::FieldReader<bool, bool>);
impl DMA_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_MODE` writer - DMA Mode"]
pub struct DMA_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "DMA Packet Buffering Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MPB_A {
    #[doc = "0: Single-packet mode"]
    SINGLE_PACKET = 0,
    #[doc = "1: Multiple-packet mode"]
    MULTIPLE_PACKET = 1,
}
impl From<MPB_A> for bool {
    #[inline(always)]
    fn from(variant: MPB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MPB` reader - DMA Packet Buffering Mode"]
pub struct MPB_R(crate::FieldReader<bool, MPB_A>);
impl MPB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MPB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MPB_A {
        match self.bits {
            false => MPB_A::SINGLE_PACKET,
            true => MPB_A::MULTIPLE_PACKET,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_PACKET`"]
    #[inline(always)]
    pub fn is_single_packet(&self) -> bool {
        **self == MPB_A::SINGLE_PACKET
    }
    #[doc = "Checks if the value of the field is `MULTIPLE_PACKET`"]
    #[inline(always)]
    pub fn is_multiple_packet(&self) -> bool {
        **self == MPB_A::MULTIPLE_PACKET
    }
}
impl core::ops::Deref for MPB_R {
    type Target = crate::FieldReader<bool, MPB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPB` writer - DMA Packet Buffering Mode"]
pub struct MPB_W<'a> {
    w: &'a mut W,
}
impl<'a> MPB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MPB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single-packet mode"]
    #[inline(always)]
    pub fn single_packet(self) -> &'a mut W {
        self.variant(MPB_A::SINGLE_PACKET)
    }
    #[doc = "Multiple-packet mode"]
    #[inline(always)]
    pub fn multiple_packet(self) -> &'a mut W {
        self.variant(MPB_A::MULTIPLE_PACKET)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Software Clear Enable"]
    #[inline(always)]
    pub fn sce(&self) -> SCE_R {
        SCE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AHB Interrupt Mux Enable"]
    #[inline(always)]
    pub fn smx(&self) -> SMX_R {
        SMX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA Mode"]
    #[inline(always)]
    pub fn dma_mode(&self) -> DMA_MODE_R {
        DMA_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DMA Packet Buffering Mode"]
    #[inline(always)]
    pub fn mpb(&self) -> MPB_R {
        MPB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Clear Enable"]
    #[inline(always)]
    pub fn sce(&mut self) -> SCE_W {
        SCE_W { w: self }
    }
    #[doc = "Bit 1 - AHB Interrupt Mux Enable"]
    #[inline(always)]
    pub fn smx(&mut self) -> SMX_W {
        SMX_W { w: self }
    }
    #[doc = "Bit 2 - DMA Mode"]
    #[inline(always)]
    pub fn dma_mode(&mut self) -> DMA_MODE_W {
        DMA_MODE_W { w: self }
    }
    #[doc = "Bit 4 - DMA Packet Buffering Mode"]
    #[inline(always)]
    pub fn mpb(&mut self) -> MPB_W {
        MPB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlb_actl](index.html) module"]
pub struct MLB_ACTL_SPEC;
impl crate::RegisterSpec for MLB_ACTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mlb_actl::R](R) reader structure"]
impl crate::Readable for MLB_ACTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mlb_actl::W](W) writer structure"]
impl crate::Writable for MLB_ACTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MLB_ACTL to value 0"]
impl crate::Resettable for MLB_ACTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
