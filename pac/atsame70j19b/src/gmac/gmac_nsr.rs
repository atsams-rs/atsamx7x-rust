#[doc = "Reader of register GMAC_NSR"]
pub type R = crate::R<u32, super::GMAC_NSR>;
#[doc = "Reader of field `MDIO`"]
pub type MDIO_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDLE`"]
pub type IDLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXLPIS`"]
pub type RXLPIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - MDIO Input Status"]
    #[inline(always)]
    pub fn mdio(&self) -> MDIO_R {
        MDIO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PHY Management Logic Idle"]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LPI Indication"]
    #[inline(always)]
    pub fn rxlpis(&self) -> RXLPIS_R {
        RXLPIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
