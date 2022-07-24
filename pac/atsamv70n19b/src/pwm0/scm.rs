#[doc = "Register `SCM` reader"]
pub struct R(crate::R<SCM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCM` writer"]
pub struct W(crate::W<SCM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCM_SPEC>;
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
impl From<crate::W<SCM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNC0` reader - Synchronous Channel 0"]
pub type SYNC0_R = crate::BitReader<bool>;
#[doc = "Field `SYNC0` writer - Synchronous Channel 0"]
pub type SYNC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCM_SPEC, bool, O>;
#[doc = "Field `SYNC1` reader - Synchronous Channel 1"]
pub type SYNC1_R = crate::BitReader<bool>;
#[doc = "Field `SYNC1` writer - Synchronous Channel 1"]
pub type SYNC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCM_SPEC, bool, O>;
#[doc = "Field `SYNC2` reader - Synchronous Channel 2"]
pub type SYNC2_R = crate::BitReader<bool>;
#[doc = "Field `SYNC2` writer - Synchronous Channel 2"]
pub type SYNC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCM_SPEC, bool, O>;
#[doc = "Field `SYNC3` reader - Synchronous Channel 3"]
pub type SYNC3_R = crate::BitReader<bool>;
#[doc = "Field `SYNC3` writer - Synchronous Channel 3"]
pub type SYNC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCM_SPEC, bool, O>;
#[doc = "Synchronous Channels Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UPDM_A {
    #[doc = "0: Manual write of double buffer registers and manual update of synchronous channels"]
    MODE0 = 0,
    #[doc = "1: Manual write of double buffer registers and automatic update of synchronous channels"]
    MODE1 = 1,
    #[doc = "2: Automatic write of duty-cycle update registers by the DMA Controller and automatic update of synchronous channels"]
    MODE2 = 2,
}
impl From<UPDM_A> for u8 {
    #[inline(always)]
    fn from(variant: UPDM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UPDM` reader - Synchronous Channels Update Mode"]
pub type UPDM_R = crate::FieldReader<u8, UPDM_A>;
impl UPDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UPDM_A> {
        match self.bits {
            0 => Some(UPDM_A::MODE0),
            1 => Some(UPDM_A::MODE1),
            2 => Some(UPDM_A::MODE2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == UPDM_A::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == UPDM_A::MODE1
    }
    #[doc = "Checks if the value of the field is `MODE2`"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == UPDM_A::MODE2
    }
}
#[doc = "Field `UPDM` writer - Synchronous Channels Update Mode"]
pub type UPDM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCM_SPEC, u8, UPDM_A, 2, O>;
impl<'a, const O: u8> UPDM_W<'a, O> {
    #[doc = "Manual write of double buffer registers and manual update of synchronous channels"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(UPDM_A::MODE0)
    }
    #[doc = "Manual write of double buffer registers and automatic update of synchronous channels"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(UPDM_A::MODE1)
    }
    #[doc = "Automatic write of duty-cycle update registers by the DMA Controller and automatic update of synchronous channels"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut W {
        self.variant(UPDM_A::MODE2)
    }
}
#[doc = "Field `PTRM` reader - DMA Controller Transfer Request Mode"]
pub type PTRM_R = crate::BitReader<bool>;
#[doc = "Field `PTRM` writer - DMA Controller Transfer Request Mode"]
pub type PTRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCM_SPEC, bool, O>;
#[doc = "Field `PTRCS` reader - DMA Controller Transfer Request Comparison Selection"]
pub type PTRCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PTRCS` writer - DMA Controller Transfer Request Comparison Selection"]
pub type PTRCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCM_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Synchronous Channel 0"]
    #[inline(always)]
    pub fn sync0(&self) -> SYNC0_R {
        SYNC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronous Channel 1"]
    #[inline(always)]
    pub fn sync1(&self) -> SYNC1_R {
        SYNC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronous Channel 2"]
    #[inline(always)]
    pub fn sync2(&self) -> SYNC2_R {
        SYNC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronous Channel 3"]
    #[inline(always)]
    pub fn sync3(&self) -> SYNC3_R {
        SYNC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Synchronous Channels Update Mode"]
    #[inline(always)]
    pub fn updm(&self) -> UPDM_R {
        UPDM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - DMA Controller Transfer Request Mode"]
    #[inline(always)]
    pub fn ptrm(&self) -> PTRM_R {
        PTRM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - DMA Controller Transfer Request Comparison Selection"]
    #[inline(always)]
    pub fn ptrcs(&self) -> PTRCS_R {
        PTRCS_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronous Channel 0"]
    #[inline(always)]
    pub fn sync0(&mut self) -> SYNC0_W<0> {
        SYNC0_W::new(self)
    }
    #[doc = "Bit 1 - Synchronous Channel 1"]
    #[inline(always)]
    pub fn sync1(&mut self) -> SYNC1_W<1> {
        SYNC1_W::new(self)
    }
    #[doc = "Bit 2 - Synchronous Channel 2"]
    #[inline(always)]
    pub fn sync2(&mut self) -> SYNC2_W<2> {
        SYNC2_W::new(self)
    }
    #[doc = "Bit 3 - Synchronous Channel 3"]
    #[inline(always)]
    pub fn sync3(&mut self) -> SYNC3_W<3> {
        SYNC3_W::new(self)
    }
    #[doc = "Bits 16:17 - Synchronous Channels Update Mode"]
    #[inline(always)]
    pub fn updm(&mut self) -> UPDM_W<16> {
        UPDM_W::new(self)
    }
    #[doc = "Bit 20 - DMA Controller Transfer Request Mode"]
    #[inline(always)]
    pub fn ptrm(&mut self) -> PTRM_W<20> {
        PTRM_W::new(self)
    }
    #[doc = "Bits 21:23 - DMA Controller Transfer Request Comparison Selection"]
    #[inline(always)]
    pub fn ptrcs(&mut self) -> PTRCS_W<21> {
        PTRCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Sync Channels Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scm](index.html) module"]
pub struct SCM_SPEC;
impl crate::RegisterSpec for SCM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scm::R](R) reader structure"]
impl crate::Readable for SCM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scm::W](W) writer structure"]
impl crate::Writable for SCM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCM to value 0"]
impl crate::Resettable for SCM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
