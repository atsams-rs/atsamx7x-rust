#[doc = "Reader of register DACC_ISR"]
pub type R = crate::R<u32, super::DACC_ISR>;
#[doc = "Reader of field `TXRDY0`"]
pub type TXRDY0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRDY1`"]
pub type TXRDY1_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC0`"]
pub type EOC0_R = crate::R<bool, bool>;
#[doc = "Reader of field `EOC1`"]
pub type EOC1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Transmit Ready Interrupt Flag of channel 0"]
    #[inline(always)]
    pub fn txrdy0(&self) -> TXRDY0_R {
        TXRDY0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Ready Interrupt Flag of channel 1"]
    #[inline(always)]
    pub fn txrdy1(&self) -> TXRDY1_R {
        TXRDY1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End of Conversion Interrupt Flag of channel 0"]
    #[inline(always)]
    pub fn eoc0(&self) -> EOC0_R {
        EOC0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Flag of channel 1"]
    #[inline(always)]
    pub fn eoc1(&self) -> EOC1_R {
        EOC1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
