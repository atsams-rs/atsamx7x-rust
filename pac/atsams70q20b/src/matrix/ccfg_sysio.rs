#[doc = "Register `CCFG_SYSIO` reader"]
pub struct R(crate::R<CCFG_SYSIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_SYSIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_SYSIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_SYSIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_SYSIO` writer"]
pub struct W(crate::W<CCFG_SYSIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_SYSIO_SPEC>;
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
impl From<crate::W<CCFG_SYSIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_SYSIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSIO4` reader - PB4 or TDI Assignment"]
pub type SYSIO4_R = crate::BitReader<bool>;
#[doc = "Field `SYSIO4` writer - PB4 or TDI Assignment"]
pub type SYSIO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFG_SYSIO_SPEC, bool, O>;
#[doc = "Field `SYSIO5` reader - PB5 or TDO/TRACESWO Assignment"]
pub type SYSIO5_R = crate::BitReader<bool>;
#[doc = "Field `SYSIO5` writer - PB5 or TDO/TRACESWO Assignment"]
pub type SYSIO5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFG_SYSIO_SPEC, bool, O>;
#[doc = "Field `SYSIO6` reader - PB6 or TMS/SWDIO Assignment"]
pub type SYSIO6_R = crate::BitReader<bool>;
#[doc = "Field `SYSIO6` writer - PB6 or TMS/SWDIO Assignment"]
pub type SYSIO6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFG_SYSIO_SPEC, bool, O>;
#[doc = "Field `SYSIO7` reader - PB7 or TCK/SWCLK Assignment"]
pub type SYSIO7_R = crate::BitReader<bool>;
#[doc = "Field `SYSIO7` writer - PB7 or TCK/SWCLK Assignment"]
pub type SYSIO7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFG_SYSIO_SPEC, bool, O>;
#[doc = "Field `SYSIO12` reader - PB12 or ERASE Assignment"]
pub type SYSIO12_R = crate::BitReader<bool>;
#[doc = "Field `SYSIO12` writer - PB12 or ERASE Assignment"]
pub type SYSIO12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCFG_SYSIO_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - PB4 or TDI Assignment"]
    #[inline(always)]
    pub fn sysio4(&self) -> SYSIO4_R {
        SYSIO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PB5 or TDO/TRACESWO Assignment"]
    #[inline(always)]
    pub fn sysio5(&self) -> SYSIO5_R {
        SYSIO5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PB6 or TMS/SWDIO Assignment"]
    #[inline(always)]
    pub fn sysio6(&self) -> SYSIO6_R {
        SYSIO6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PB7 or TCK/SWCLK Assignment"]
    #[inline(always)]
    pub fn sysio7(&self) -> SYSIO7_R {
        SYSIO7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - PB12 or ERASE Assignment"]
    #[inline(always)]
    pub fn sysio12(&self) -> SYSIO12_R {
        SYSIO12_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - PB4 or TDI Assignment"]
    #[inline(always)]
    pub fn sysio4(&mut self) -> SYSIO4_W<4> {
        SYSIO4_W::new(self)
    }
    #[doc = "Bit 5 - PB5 or TDO/TRACESWO Assignment"]
    #[inline(always)]
    pub fn sysio5(&mut self) -> SYSIO5_W<5> {
        SYSIO5_W::new(self)
    }
    #[doc = "Bit 6 - PB6 or TMS/SWDIO Assignment"]
    #[inline(always)]
    pub fn sysio6(&mut self) -> SYSIO6_W<6> {
        SYSIO6_W::new(self)
    }
    #[doc = "Bit 7 - PB7 or TCK/SWCLK Assignment"]
    #[inline(always)]
    pub fn sysio7(&mut self) -> SYSIO7_W<7> {
        SYSIO7_W::new(self)
    }
    #[doc = "Bit 12 - PB12 or ERASE Assignment"]
    #[inline(always)]
    pub fn sysio12(&mut self) -> SYSIO12_W<12> {
        SYSIO12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System I/O Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_sysio](index.html) module"]
pub struct CCFG_SYSIO_SPEC;
impl crate::RegisterSpec for CCFG_SYSIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_sysio::R](R) reader structure"]
impl crate::Readable for CCFG_SYSIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_sysio::W](W) writer structure"]
impl crate::Writable for CCFG_SYSIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCFG_SYSIO to value 0"]
impl crate::Resettable for CCFG_SYSIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
