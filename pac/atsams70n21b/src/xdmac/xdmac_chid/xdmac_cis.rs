#[doc = "Reader of register XDMAC_CIS"]
pub type R = crate::R<u32, super::XDMAC_CIS>;
#[doc = "Reader of field `BIS`"]
pub type BIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `LIS`"]
pub type LIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIS`"]
pub type DIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FIS`"]
pub type FIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RBEIS`"]
pub type RBEIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `WBEIS`"]
pub type WBEIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `ROIS`"]
pub type ROIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - End of Block Interrupt Status Bit"]
    #[inline(always)]
    pub fn bis(&self) -> BIS_R {
        BIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Status Bit"]
    #[inline(always)]
    pub fn lis(&self) -> LIS_R {
        LIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of Disable Interrupt Status Bit"]
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of Flush Interrupt Status Bit"]
    #[inline(always)]
    pub fn fis(&self) -> FIS_R {
        FIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Status Bit"]
    #[inline(always)]
    pub fn rbeis(&self) -> RBEIS_R {
        RBEIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Status Bit"]
    #[inline(always)]
    pub fn wbeis(&self) -> WBEIS_R {
        WBEIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Status Bit"]
    #[inline(always)]
    pub fn rois(&self) -> ROIS_R {
        ROIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
