#[doc = "Register `BTP` reader"]
pub struct R(crate::R<BTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTP` writer"]
pub struct W(crate::W<BTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTP_SPEC>;
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
impl From<crate::W<BTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SJW` reader - (Re) Synchronization Jump Width"]
pub type SJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SJW` writer - (Re) Synchronization Jump Width"]
pub type SJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTP_SPEC, u8, u8, 4, O>;
#[doc = "Field `TSEG2` reader - Time Segment After Sample Point"]
pub type TSEG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSEG2` writer - Time Segment After Sample Point"]
pub type TSEG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTP_SPEC, u8, u8, 4, O>;
#[doc = "Field `TSEG1` reader - Time Segment Before Sample Point"]
pub type TSEG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSEG1` writer - Time Segment Before Sample Point"]
pub type TSEG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTP_SPEC, u8, u8, 6, O>;
#[doc = "Field `BRP` reader - Baud Rate Prescaler"]
pub type BRP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BRP` writer - Baud Rate Prescaler"]
pub type BRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BTP_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:3 - (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Time Segment After Sample Point"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn tseg1(&self) -> TSEG1_R {
        TSEG1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:25 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - (Re) Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W<0> {
        SJW_W::new(self)
    }
    #[doc = "Bits 4:7 - Time Segment After Sample Point"]
    #[inline(always)]
    pub fn tseg2(&mut self) -> TSEG2_W<4> {
        TSEG2_W::new(self)
    }
    #[doc = "Bits 8:13 - Time Segment Before Sample Point"]
    #[inline(always)]
    pub fn tseg1(&mut self) -> TSEG1_W<8> {
        TSEG1_W::new(self)
    }
    #[doc = "Bits 16:25 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W<16> {
        BRP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit Timing and Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btp](index.html) module"]
pub struct BTP_SPEC;
impl crate::RegisterSpec for BTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btp::R](R) reader structure"]
impl crate::Readable for BTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btp::W](W) writer structure"]
impl crate::Writable for BTP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BTP to value 0"]
impl crate::Resettable for BTP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
