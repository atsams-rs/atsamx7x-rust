#[doc = "Register `XDMAC_GCFG` reader"]
pub struct R(crate::R<XDMAC_GCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XDMAC_GCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XDMAC_GCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XDMAC_GCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CGDISREG` reader - Configuration Registers Clock Gating Disable"]
pub struct CGDISREG_R(crate::FieldReader<bool, bool>);
impl CGDISREG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CGDISREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CGDISREG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CGDISPIPE` reader - Pipeline Clock Gating Disable"]
pub struct CGDISPIPE_R(crate::FieldReader<bool, bool>);
impl CGDISPIPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CGDISPIPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CGDISPIPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CGDISFIFO` reader - FIFO Clock Gating Disable"]
pub struct CGDISFIFO_R(crate::FieldReader<bool, bool>);
impl CGDISFIFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CGDISFIFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CGDISFIFO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CGDISIF` reader - Bus Interface Clock Gating Disable"]
pub struct CGDISIF_R(crate::FieldReader<bool, bool>);
impl CGDISIF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CGDISIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CGDISIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BXKBEN` reader - Boundary X Kilobyte Enable"]
pub struct BXKBEN_R(crate::FieldReader<bool, bool>);
impl BXKBEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BXKBEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BXKBEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Configuration Registers Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisreg(&self) -> CGDISREG_R {
        CGDISREG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pipeline Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdispipe(&self) -> CGDISPIPE_R {
        CGDISPIPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FIFO Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisfifo(&self) -> CGDISFIFO_R {
        CGDISFIFO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bus Interface Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisif(&self) -> CGDISIF_R {
        CGDISIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Boundary X Kilobyte Enable"]
    #[inline(always)]
    pub fn bxkben(&self) -> BXKBEN_R {
        BXKBEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "Global Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xdmac_gcfg](index.html) module"]
pub struct XDMAC_GCFG_SPEC;
impl crate::RegisterSpec for XDMAC_GCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xdmac_gcfg::R](R) reader structure"]
impl crate::Readable for XDMAC_GCFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets XDMAC_GCFG to value 0"]
impl crate::Resettable for XDMAC_GCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
