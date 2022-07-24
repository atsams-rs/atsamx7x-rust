#[doc = "Register `CNDC` reader"]
pub struct R(crate::R<CNDC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNDC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNDC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNDC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNDC` writer"]
pub struct W(crate::W<CNDC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNDC_SPEC>;
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
impl From<crate::W<CNDC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNDC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel x Next Descriptor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDE_A {
    #[doc = "0: Descriptor fetch is disabled."]
    DSCR_FETCH_DIS = 0,
    #[doc = "1: Descriptor fetch is enabled."]
    DSCR_FETCH_EN = 1,
}
impl From<NDE_A> for bool {
    #[inline(always)]
    fn from(variant: NDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NDE` reader - Channel x Next Descriptor Enable"]
pub type NDE_R = crate::BitReader<NDE_A>;
impl NDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDE_A {
        match self.bits {
            false => NDE_A::DSCR_FETCH_DIS,
            true => NDE_A::DSCR_FETCH_EN,
        }
    }
    #[doc = "Checks if the value of the field is `DSCR_FETCH_DIS`"]
    #[inline(always)]
    pub fn is_dscr_fetch_dis(&self) -> bool {
        *self == NDE_A::DSCR_FETCH_DIS
    }
    #[doc = "Checks if the value of the field is `DSCR_FETCH_EN`"]
    #[inline(always)]
    pub fn is_dscr_fetch_en(&self) -> bool {
        *self == NDE_A::DSCR_FETCH_EN
    }
}
#[doc = "Field `NDE` writer - Channel x Next Descriptor Enable"]
pub type NDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDC_SPEC, NDE_A, O>;
impl<'a, const O: u8> NDE_W<'a, O> {
    #[doc = "Descriptor fetch is disabled."]
    #[inline(always)]
    pub fn dscr_fetch_dis(self) -> &'a mut W {
        self.variant(NDE_A::DSCR_FETCH_DIS)
    }
    #[doc = "Descriptor fetch is enabled."]
    #[inline(always)]
    pub fn dscr_fetch_en(self) -> &'a mut W {
        self.variant(NDE_A::DSCR_FETCH_EN)
    }
}
#[doc = "Channel x Next Descriptor Source Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDSUP_A {
    #[doc = "0: Source parameters remain unchanged."]
    SRC_PARAMS_UNCHANGED = 0,
    #[doc = "1: Source parameters are updated when the descriptor is retrieved."]
    SRC_PARAMS_UPDATED = 1,
}
impl From<NDSUP_A> for bool {
    #[inline(always)]
    fn from(variant: NDSUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NDSUP` reader - Channel x Next Descriptor Source Update"]
pub type NDSUP_R = crate::BitReader<NDSUP_A>;
impl NDSUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDSUP_A {
        match self.bits {
            false => NDSUP_A::SRC_PARAMS_UNCHANGED,
            true => NDSUP_A::SRC_PARAMS_UPDATED,
        }
    }
    #[doc = "Checks if the value of the field is `SRC_PARAMS_UNCHANGED`"]
    #[inline(always)]
    pub fn is_src_params_unchanged(&self) -> bool {
        *self == NDSUP_A::SRC_PARAMS_UNCHANGED
    }
    #[doc = "Checks if the value of the field is `SRC_PARAMS_UPDATED`"]
    #[inline(always)]
    pub fn is_src_params_updated(&self) -> bool {
        *self == NDSUP_A::SRC_PARAMS_UPDATED
    }
}
#[doc = "Field `NDSUP` writer - Channel x Next Descriptor Source Update"]
pub type NDSUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDC_SPEC, NDSUP_A, O>;
impl<'a, const O: u8> NDSUP_W<'a, O> {
    #[doc = "Source parameters remain unchanged."]
    #[inline(always)]
    pub fn src_params_unchanged(self) -> &'a mut W {
        self.variant(NDSUP_A::SRC_PARAMS_UNCHANGED)
    }
    #[doc = "Source parameters are updated when the descriptor is retrieved."]
    #[inline(always)]
    pub fn src_params_updated(self) -> &'a mut W {
        self.variant(NDSUP_A::SRC_PARAMS_UPDATED)
    }
}
#[doc = "Channel x Next Descriptor Destination Update\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDDUP_A {
    #[doc = "0: Destination parameters remain unchanged."]
    DST_PARAMS_UNCHANGED = 0,
    #[doc = "1: Destination parameters are updated when the descriptor is retrieved."]
    DST_PARAMS_UPDATED = 1,
}
impl From<NDDUP_A> for bool {
    #[inline(always)]
    fn from(variant: NDDUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NDDUP` reader - Channel x Next Descriptor Destination Update"]
pub type NDDUP_R = crate::BitReader<NDDUP_A>;
impl NDDUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDDUP_A {
        match self.bits {
            false => NDDUP_A::DST_PARAMS_UNCHANGED,
            true => NDDUP_A::DST_PARAMS_UPDATED,
        }
    }
    #[doc = "Checks if the value of the field is `DST_PARAMS_UNCHANGED`"]
    #[inline(always)]
    pub fn is_dst_params_unchanged(&self) -> bool {
        *self == NDDUP_A::DST_PARAMS_UNCHANGED
    }
    #[doc = "Checks if the value of the field is `DST_PARAMS_UPDATED`"]
    #[inline(always)]
    pub fn is_dst_params_updated(&self) -> bool {
        *self == NDDUP_A::DST_PARAMS_UPDATED
    }
}
#[doc = "Field `NDDUP` writer - Channel x Next Descriptor Destination Update"]
pub type NDDUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNDC_SPEC, NDDUP_A, O>;
impl<'a, const O: u8> NDDUP_W<'a, O> {
    #[doc = "Destination parameters remain unchanged."]
    #[inline(always)]
    pub fn dst_params_unchanged(self) -> &'a mut W {
        self.variant(NDDUP_A::DST_PARAMS_UNCHANGED)
    }
    #[doc = "Destination parameters are updated when the descriptor is retrieved."]
    #[inline(always)]
    pub fn dst_params_updated(self) -> &'a mut W {
        self.variant(NDDUP_A::DST_PARAMS_UPDATED)
    }
}
#[doc = "Channel x Next Descriptor View\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NDVIEW_A {
    #[doc = "0: Next Descriptor View 0"]
    NDV0 = 0,
    #[doc = "1: Next Descriptor View 1"]
    NDV1 = 1,
    #[doc = "2: Next Descriptor View 2"]
    NDV2 = 2,
    #[doc = "3: Next Descriptor View 3"]
    NDV3 = 3,
}
impl From<NDVIEW_A> for u8 {
    #[inline(always)]
    fn from(variant: NDVIEW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NDVIEW` reader - Channel x Next Descriptor View"]
pub type NDVIEW_R = crate::FieldReader<u8, NDVIEW_A>;
impl NDVIEW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDVIEW_A {
        match self.bits {
            0 => NDVIEW_A::NDV0,
            1 => NDVIEW_A::NDV1,
            2 => NDVIEW_A::NDV2,
            3 => NDVIEW_A::NDV3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NDV0`"]
    #[inline(always)]
    pub fn is_ndv0(&self) -> bool {
        *self == NDVIEW_A::NDV0
    }
    #[doc = "Checks if the value of the field is `NDV1`"]
    #[inline(always)]
    pub fn is_ndv1(&self) -> bool {
        *self == NDVIEW_A::NDV1
    }
    #[doc = "Checks if the value of the field is `NDV2`"]
    #[inline(always)]
    pub fn is_ndv2(&self) -> bool {
        *self == NDVIEW_A::NDV2
    }
    #[doc = "Checks if the value of the field is `NDV3`"]
    #[inline(always)]
    pub fn is_ndv3(&self) -> bool {
        *self == NDVIEW_A::NDV3
    }
}
#[doc = "Field `NDVIEW` writer - Channel x Next Descriptor View"]
pub type NDVIEW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CNDC_SPEC, u8, NDVIEW_A, 2, O>;
impl<'a, const O: u8> NDVIEW_W<'a, O> {
    #[doc = "Next Descriptor View 0"]
    #[inline(always)]
    pub fn ndv0(self) -> &'a mut W {
        self.variant(NDVIEW_A::NDV0)
    }
    #[doc = "Next Descriptor View 1"]
    #[inline(always)]
    pub fn ndv1(self) -> &'a mut W {
        self.variant(NDVIEW_A::NDV1)
    }
    #[doc = "Next Descriptor View 2"]
    #[inline(always)]
    pub fn ndv2(self) -> &'a mut W {
        self.variant(NDVIEW_A::NDV2)
    }
    #[doc = "Next Descriptor View 3"]
    #[inline(always)]
    pub fn ndv3(self) -> &'a mut W {
        self.variant(NDVIEW_A::NDV3)
    }
}
impl R {
    #[doc = "Bit 0 - Channel x Next Descriptor Enable"]
    #[inline(always)]
    pub fn nde(&self) -> NDE_R {
        NDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Next Descriptor Source Update"]
    #[inline(always)]
    pub fn ndsup(&self) -> NDSUP_R {
        NDSUP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x Next Descriptor Destination Update"]
    #[inline(always)]
    pub fn nddup(&self) -> NDDUP_R {
        NDDUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Channel x Next Descriptor View"]
    #[inline(always)]
    pub fn ndview(&self) -> NDVIEW_R {
        NDVIEW_R::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel x Next Descriptor Enable"]
    #[inline(always)]
    pub fn nde(&mut self) -> NDE_W<0> {
        NDE_W::new(self)
    }
    #[doc = "Bit 1 - Channel x Next Descriptor Source Update"]
    #[inline(always)]
    pub fn ndsup(&mut self) -> NDSUP_W<1> {
        NDSUP_W::new(self)
    }
    #[doc = "Bit 2 - Channel x Next Descriptor Destination Update"]
    #[inline(always)]
    pub fn nddup(&mut self) -> NDDUP_W<2> {
        NDDUP_W::new(self)
    }
    #[doc = "Bits 3:4 - Channel x Next Descriptor View"]
    #[inline(always)]
    pub fn ndview(&mut self) -> NDVIEW_W<3> {
        NDVIEW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Next Descriptor Control Register (chid = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndc](index.html) module"]
pub struct CNDC_SPEC;
impl crate::RegisterSpec for CNDC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cndc::R](R) reader structure"]
impl crate::Readable for CNDC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cndc::W](W) writer structure"]
impl crate::Writable for CNDC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNDC to value 0"]
impl crate::Resettable for CNDC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
