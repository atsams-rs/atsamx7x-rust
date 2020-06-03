#[doc = "Reader of register USBHS_DEVEPTIMR_INTRPT_MODE[%s]"]
pub type R = crate::R<u32, super::USBHS_DEVEPTIMR_INTRPT_MODE>;
#[doc = "Reader of field `TXINE`"]
pub type TXINE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXOUTE`"]
pub type RXOUTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXSTPE`"]
pub type RXSTPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `NAKOUTE`"]
pub type NAKOUTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `NAKINE`"]
pub type NAKINE_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVERFE`"]
pub type OVERFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `STALLEDE`"]
pub type STALLEDE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SHORTPACKETE`"]
pub type SHORTPACKETE_R = crate::R<bool, bool>;
#[doc = "Reader of field `NBUSYBKE`"]
pub type NBUSYBKE_R = crate::R<bool, bool>;
#[doc = "Reader of field `KILLBK`"]
pub type KILLBK_R = crate::R<bool, bool>;
#[doc = "Reader of field `FIFOCON`"]
pub type FIFOCON_R = crate::R<bool, bool>;
#[doc = "Reader of field `EPDISHDMA`"]
pub type EPDISHDMA_R = crate::R<bool, bool>;
#[doc = "Reader of field `NYETDIS`"]
pub type NYETDIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSTDT`"]
pub type RSTDT_R = crate::R<bool, bool>;
#[doc = "Reader of field `STALLRQ`"]
pub type STALLRQ_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt"]
    #[inline(always)]
    pub fn txine(&self) -> TXINE_R {
        TXINE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt"]
    #[inline(always)]
    pub fn rxoute(&self) -> RXOUTE_R {
        RXOUTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt"]
    #[inline(always)]
    pub fn rxstpe(&self) -> RXSTPE_R {
        RXSTPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt"]
    #[inline(always)]
    pub fn nakoute(&self) -> NAKOUTE_R {
        NAKOUTE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt"]
    #[inline(always)]
    pub fn nakine(&self) -> NAKINE_R {
        NAKINE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overflow Interrupt"]
    #[inline(always)]
    pub fn overfe(&self) -> OVERFE_R {
        OVERFE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - STALLed Interrupt"]
    #[inline(always)]
    pub fn stallede(&self) -> STALLEDE_R {
        STALLEDE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Short Packet Interrupt"]
    #[inline(always)]
    pub fn shortpackete(&self) -> SHORTPACKETE_R {
        SHORTPACKETE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt"]
    #[inline(always)]
    pub fn nbusybke(&self) -> NBUSYBKE_R {
        NBUSYBKE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Kill IN Bank"]
    #[inline(always)]
    pub fn killbk(&self) -> KILLBK_R {
        KILLBK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    pub fn fifocon(&self) -> FIFOCON_R {
        FIFOCON_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint Interrupts Disable HDMA Request"]
    #[inline(always)]
    pub fn epdishdma(&self) -> EPDISHDMA_R {
        EPDISHDMA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - NYET Token Disable"]
    #[inline(always)]
    pub fn nyetdis(&self) -> NYETDIS_R {
        NYETDIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Reset Data Toggle"]
    #[inline(always)]
    pub fn rstdt(&self) -> RSTDT_R {
        RSTDT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - STALL Request"]
    #[inline(always)]
    pub fn stallrq(&self) -> STALLRQ_R {
        STALLRQ_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
