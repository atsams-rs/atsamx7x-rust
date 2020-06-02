#[doc = "Reader of register I2SC_SR"]
pub type R = crate::R<u32, super::I2SC_SR>;
#[doc = "Reader of field `RXEN`"]
pub type RXEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXRDY`"]
pub type RXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXOR`"]
pub type RXOR_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXEN`"]
pub type TXEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRDY`"]
pub type TXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXUR`"]
pub type TXUR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXORCH`"]
pub type RXORCH_R = crate::R<u8, u8>;
#[doc = "Reader of field `TXURCH`"]
pub type TXURCH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Receiver Enabled"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Overrun"]
    #[inline(always)]
    pub fn rxor(&self) -> RXOR_R {
        RXOR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmitter Enabled"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit Underrun"]
    #[inline(always)]
    pub fn txur(&self) -> TXUR_R {
        TXUR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Receive Overrun Channel"]
    #[inline(always)]
    pub fn rxorch(&self) -> RXORCH_R {
        RXORCH_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Transmit Underrun Channel"]
    #[inline(always)]
    pub fn txurch(&self) -> TXURCH_R {
        TXURCH_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
