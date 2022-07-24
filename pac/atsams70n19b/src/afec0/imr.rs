#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EOC0` reader - End of Conversion Interrupt Mask 0"]
pub type EOC0_R = crate::BitReader<bool>;
#[doc = "Field `EOC1` reader - End of Conversion Interrupt Mask 1"]
pub type EOC1_R = crate::BitReader<bool>;
#[doc = "Field `EOC2` reader - End of Conversion Interrupt Mask 2"]
pub type EOC2_R = crate::BitReader<bool>;
#[doc = "Field `EOC3` reader - End of Conversion Interrupt Mask 3"]
pub type EOC3_R = crate::BitReader<bool>;
#[doc = "Field `EOC4` reader - End of Conversion Interrupt Mask 4"]
pub type EOC4_R = crate::BitReader<bool>;
#[doc = "Field `EOC5` reader - End of Conversion Interrupt Mask 5"]
pub type EOC5_R = crate::BitReader<bool>;
#[doc = "Field `EOC6` reader - End of Conversion Interrupt Mask 6"]
pub type EOC6_R = crate::BitReader<bool>;
#[doc = "Field `EOC7` reader - End of Conversion Interrupt Mask 7"]
pub type EOC7_R = crate::BitReader<bool>;
#[doc = "Field `EOC8` reader - End of Conversion Interrupt Mask 8"]
pub type EOC8_R = crate::BitReader<bool>;
#[doc = "Field `EOC9` reader - End of Conversion Interrupt Mask 9"]
pub type EOC9_R = crate::BitReader<bool>;
#[doc = "Field `EOC10` reader - End of Conversion Interrupt Mask 10"]
pub type EOC10_R = crate::BitReader<bool>;
#[doc = "Field `EOC11` reader - End of Conversion Interrupt Mask 11"]
pub type EOC11_R = crate::BitReader<bool>;
#[doc = "Field `DRDY` reader - Data Ready Interrupt Mask"]
pub type DRDY_R = crate::BitReader<bool>;
#[doc = "Field `GOVRE` reader - General Overrun Error Interrupt Mask"]
pub type GOVRE_R = crate::BitReader<bool>;
#[doc = "Field `COMPE` reader - Comparison Event Interrupt Mask"]
pub type COMPE_R = crate::BitReader<bool>;
#[doc = "Field `TEMPCHG` reader - Temperature Change Interrupt Mask"]
pub type TEMPCHG_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - End of Conversion Interrupt Mask 0"]
    #[inline(always)]
    pub fn eoc0(&self) -> EOC0_R {
        EOC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Conversion Interrupt Mask 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Conversion Interrupt Mask 2"]
    #[inline(always)]
    pub fn eoc2(&self) -> EOC2_R {
        EOC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Conversion Interrupt Mask 3"]
    #[inline(always)]
    pub fn eoc3(&self) -> EOC3_R {
        EOC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Mask 4"]
    #[inline(always)]
    pub fn eoc4(&self) -> EOC4_R {
        EOC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Mask 5"]
    #[inline(always)]
    pub fn eoc5(&self) -> EOC5_R {
        EOC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of Conversion Interrupt Mask 6"]
    #[inline(always)]
    pub fn eoc6(&self) -> EOC6_R {
        EOC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - End of Conversion Interrupt Mask 7"]
    #[inline(always)]
    pub fn eoc7(&self) -> EOC7_R {
        EOC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - End of Conversion Interrupt Mask 8"]
    #[inline(always)]
    pub fn eoc8(&self) -> EOC8_R {
        EOC8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - End of Conversion Interrupt Mask 9"]
    #[inline(always)]
    pub fn eoc9(&self) -> EOC9_R {
        EOC9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - End of Conversion Interrupt Mask 10"]
    #[inline(always)]
    pub fn eoc10(&self) -> EOC10_R {
        EOC10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - End of Conversion Interrupt Mask 11"]
    #[inline(always)]
    pub fn eoc11(&self) -> EOC11_R {
        EOC11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 24 - Data Ready Interrupt Mask"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - General Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn govre(&self) -> GOVRE_R {
        GOVRE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Comparison Event Interrupt Mask"]
    #[inline(always)]
    pub fn compe(&self) -> COMPE_R {
        COMPE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 30 - Temperature Change Interrupt Mask"]
    #[inline(always)]
    pub fn tempchg(&self) -> TEMPCHG_R {
        TEMPCHG_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "AFEC Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
