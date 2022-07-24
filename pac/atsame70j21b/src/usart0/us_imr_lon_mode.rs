#[doc = "Register `US_IMR_LON_MODE` reader"]
pub struct R(crate::R<US_IMR_LON_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_IMR_LON_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_IMR_LON_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_IMR_LON_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRDY` reader - RXRDY Interrupt Mask"]
pub type RXRDY_R = crate::BitReader<bool>;
#[doc = "Field `TXRDY` reader - TXRDY Interrupt Mask"]
pub type TXRDY_R = crate::BitReader<bool>;
#[doc = "Field `OVRE` reader - Overrun Error Interrupt Mask"]
pub type OVRE_R = crate::BitReader<bool>;
#[doc = "Field `LSFE` reader - LON Short Frame Error Interrupt Mask"]
pub type LSFE_R = crate::BitReader<bool>;
#[doc = "Field `LCRCE` reader - LON CRC Error Interrupt Mask"]
pub type LCRCE_R = crate::BitReader<bool>;
#[doc = "Field `TXEMPTY` reader - TXEMPTY Interrupt Mask"]
pub type TXEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `UNRE` reader - SPI Underrun Error Interrupt Mask"]
pub type UNRE_R = crate::BitReader<bool>;
#[doc = "Field `LTXD` reader - LON Transmission Done Interrupt Mask"]
pub type LTXD_R = crate::BitReader<bool>;
#[doc = "Field `LCOL` reader - LON Collision Interrupt Mask"]
pub type LCOL_R = crate::BitReader<bool>;
#[doc = "Field `LFET` reader - LON Frame Early Termination Interrupt Mask"]
pub type LFET_R = crate::BitReader<bool>;
#[doc = "Field `LRXD` reader - LON Reception Done Interrupt Mask"]
pub type LRXD_R = crate::BitReader<bool>;
#[doc = "Field `LBLOVFE` reader - LON Backlog Overflow Error Interrupt Mask"]
pub type LBLOVFE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - RXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LON Short Frame Error Interrupt Mask"]
    #[inline(always)]
    pub fn lsfe(&self) -> LSFE_R {
        LSFE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LON CRC Error Interrupt Mask"]
    #[inline(always)]
    pub fn lcrce(&self) -> LCRCE_R {
        LCRCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI Underrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 24 - LON Transmission Done Interrupt Mask"]
    #[inline(always)]
    pub fn ltxd(&self) -> LTXD_R {
        LTXD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LON Collision Interrupt Mask"]
    #[inline(always)]
    pub fn lcol(&self) -> LCOL_R {
        LCOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LON Frame Early Termination Interrupt Mask"]
    #[inline(always)]
    pub fn lfet(&self) -> LFET_R {
        LFET_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LON Reception Done Interrupt Mask"]
    #[inline(always)]
    pub fn lrxd(&self) -> LRXD_R {
        LRXD_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LON Backlog Overflow Error Interrupt Mask"]
    #[inline(always)]
    pub fn lblovfe(&self) -> LBLOVFE_R {
        LBLOVFE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_imr_lon_mode](index.html) module"]
pub struct US_IMR_LON_MODE_SPEC;
impl crate::RegisterSpec for US_IMR_LON_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_imr_lon_mode::R](R) reader structure"]
impl crate::Readable for US_IMR_LON_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets US_IMR_LON_MODE to value 0"]
impl crate::Resettable for US_IMR_LON_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
