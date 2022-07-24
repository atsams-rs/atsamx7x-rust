#[doc = "Register `GCFG` reader"]
pub struct R(crate::R<GCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CGDISREG` reader - Configuration Registers Clock Gating Disable"]
pub type CGDISREG_R = crate::BitReader<bool>;
#[doc = "Field `CGDISPIPE` reader - Pipeline Clock Gating Disable"]
pub type CGDISPIPE_R = crate::BitReader<bool>;
#[doc = "Field `CGDISFIFO` reader - FIFO Clock Gating Disable"]
pub type CGDISFIFO_R = crate::BitReader<bool>;
#[doc = "Field `CGDISIF` reader - Bus Interface Clock Gating Disable"]
pub type CGDISIF_R = crate::BitReader<bool>;
#[doc = "Field `BXKBEN` reader - Boundary X Kilobyte Enable"]
pub type BXKBEN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Configuration Registers Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisreg(&self) -> CGDISREG_R {
        CGDISREG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pipeline Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdispipe(&self) -> CGDISPIPE_R {
        CGDISPIPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisfifo(&self) -> CGDISFIFO_R {
        CGDISFIFO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bus Interface Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisif(&self) -> CGDISIF_R {
        CGDISIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Boundary X Kilobyte Enable"]
    #[inline(always)]
    pub fn bxkben(&self) -> BXKBEN_R {
        BXKBEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Global Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcfg](index.html) module"]
pub struct GCFG_SPEC;
impl crate::RegisterSpec for GCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcfg::R](R) reader structure"]
impl crate::Readable for GCFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GCFG to value 0"]
impl crate::Resettable for GCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
