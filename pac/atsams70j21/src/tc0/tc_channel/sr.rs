#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COVFS` reader - Counter Overflow Status (cleared on read)"]
pub type COVFS_R = crate::BitReader<bool>;
#[doc = "Field `LOVRS` reader - Load Overrun Status (cleared on read)"]
pub type LOVRS_R = crate::BitReader<bool>;
#[doc = "Field `CPAS` reader - RA Compare Status (cleared on read)"]
pub type CPAS_R = crate::BitReader<bool>;
#[doc = "Field `CPBS` reader - RB Compare Status (cleared on read)"]
pub type CPBS_R = crate::BitReader<bool>;
#[doc = "Field `CPCS` reader - RC Compare Status (cleared on read)"]
pub type CPCS_R = crate::BitReader<bool>;
#[doc = "Field `LDRAS` reader - RA Loading Status (cleared on read)"]
pub type LDRAS_R = crate::BitReader<bool>;
#[doc = "Field `LDRBS` reader - RB Loading Status (cleared on read)"]
pub type LDRBS_R = crate::BitReader<bool>;
#[doc = "Field `ETRGS` reader - External Trigger Status (cleared on read)"]
pub type ETRGS_R = crate::BitReader<bool>;
#[doc = "Field `CLKSTA` reader - Clock Enabling Status"]
pub type CLKSTA_R = crate::BitReader<bool>;
#[doc = "Field `MTIOA` reader - TIOAx Mirror"]
pub type MTIOA_R = crate::BitReader<bool>;
#[doc = "Field `MTIOB` reader - TIOBx Mirror"]
pub type MTIOB_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Counter Overflow Status (cleared on read)"]
    #[inline(always)]
    pub fn covfs(&self) -> COVFS_R {
        COVFS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load Overrun Status (cleared on read)"]
    #[inline(always)]
    pub fn lovrs(&self) -> LOVRS_R {
        LOVRS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RA Compare Status (cleared on read)"]
    #[inline(always)]
    pub fn cpas(&self) -> CPAS_R {
        CPAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RB Compare Status (cleared on read)"]
    #[inline(always)]
    pub fn cpbs(&self) -> CPBS_R {
        CPBS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RC Compare Status (cleared on read)"]
    #[inline(always)]
    pub fn cpcs(&self) -> CPCS_R {
        CPCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RA Loading Status (cleared on read)"]
    #[inline(always)]
    pub fn ldras(&self) -> LDRAS_R {
        LDRAS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RB Loading Status (cleared on read)"]
    #[inline(always)]
    pub fn ldrbs(&self) -> LDRBS_R {
        LDRBS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Trigger Status (cleared on read)"]
    #[inline(always)]
    pub fn etrgs(&self) -> ETRGS_R {
        ETRGS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Clock Enabling Status"]
    #[inline(always)]
    pub fn clksta(&self) -> CLKSTA_R {
        CLKSTA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIOAx Mirror"]
    #[inline(always)]
    pub fn mtioa(&self) -> MTIOA_R {
        MTIOA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIOBx Mirror"]
    #[inline(always)]
    pub fn mtiob(&self) -> MTIOB_R {
        MTIOB_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "Status Register (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
