#[doc = "Register `GTYPE` reader"]
pub struct R(crate::R<GTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTYPE` writer"]
pub struct W(crate::W<GTYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTYPE_SPEC>;
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
impl From<crate::W<GTYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NB_CH` reader - Number of Channels Minus One"]
pub type NB_CH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NB_CH` writer - Number of Channels Minus One"]
pub type NB_CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTYPE_SPEC, u8, u8, 5, O>;
#[doc = "Field `FIFO_SZ` reader - Number of Bytes"]
pub type FIFO_SZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FIFO_SZ` writer - Number of Bytes"]
pub type FIFO_SZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTYPE_SPEC, u16, u16, 11, O>;
#[doc = "Field `NB_REQ` reader - Number of Peripheral Requests Minus One"]
pub type NB_REQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NB_REQ` writer - Number of Peripheral Requests Minus One"]
pub type NB_REQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTYPE_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:4 - Number of Channels Minus One"]
    #[inline(always)]
    pub fn nb_ch(&self) -> NB_CH_R {
        NB_CH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:15 - Number of Bytes"]
    #[inline(always)]
    pub fn fifo_sz(&self) -> FIFO_SZ_R {
        FIFO_SZ_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:22 - Number of Peripheral Requests Minus One"]
    #[inline(always)]
    pub fn nb_req(&self) -> NB_REQ_R {
        NB_REQ_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of Channels Minus One"]
    #[inline(always)]
    pub fn nb_ch(&mut self) -> NB_CH_W<0> {
        NB_CH_W::new(self)
    }
    #[doc = "Bits 5:15 - Number of Bytes"]
    #[inline(always)]
    pub fn fifo_sz(&mut self) -> FIFO_SZ_W<5> {
        FIFO_SZ_W::new(self)
    }
    #[doc = "Bits 16:22 - Number of Peripheral Requests Minus One"]
    #[inline(always)]
    pub fn nb_req(&mut self) -> NB_REQ_W<16> {
        NB_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Type Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtype](index.html) module"]
pub struct GTYPE_SPEC;
impl crate::RegisterSpec for GTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtype::R](R) reader structure"]
impl crate::Readable for GTYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtype::W](W) writer structure"]
impl crate::Writable for GTYPE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GTYPE to value 0"]
impl crate::Resettable for GTYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
