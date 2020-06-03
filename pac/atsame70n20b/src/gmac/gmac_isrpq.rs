#[doc = "Reader of register GMAC_ISRPQ[%s]"]
pub type R = crate::R<u32, super::GMAC_ISRPQ>;
#[doc = "Reader of field `RCOMP`"]
pub type RCOMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXUBR`"]
pub type RXUBR_R = crate::R<bool, bool>;
#[doc = "Reader of field `RLEX`"]
pub type RLEX_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFC`"]
pub type TFC_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCOMP`"]
pub type TCOMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `ROVR`"]
pub type ROVR_R = crate::R<bool, bool>;
#[doc = "Reader of field `HRESP`"]
pub type HRESP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 1 - Receive Complete"]
    #[inline(always)]
    pub fn rcomp(&self) -> RCOMP_R {
        RCOMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX Used Bit Read"]
    #[inline(always)]
    pub fn rxubr(&self) -> RXUBR_R {
        RXUBR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Retry Limit Exceeded or Late Collision"]
    #[inline(always)]
    pub fn rlex(&self) -> RLEX_R {
        RLEX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit Frame Corruption Due to AHB Error"]
    #[inline(always)]
    pub fn tfc(&self) -> TFC_R {
        TFC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmit Complete"]
    #[inline(always)]
    pub fn tcomp(&self) -> TCOMP_R {
        TCOMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Receive Overrun"]
    #[inline(always)]
    pub fn rovr(&self) -> ROVR_R {
        ROVR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HRESP Not OK"]
    #[inline(always)]
    pub fn hresp(&self) -> HRESP_R {
        HRESP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
