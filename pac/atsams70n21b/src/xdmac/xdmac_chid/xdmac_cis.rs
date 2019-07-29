#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::XDMAC_CIS {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type BIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type LIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type DIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type FIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type RBEIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type WBEIS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type ROIS_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - End of Block Interrupt Status Bit"]
    #[inline(always)]
    pub fn bis(&self) -> BIS_R {
        BIS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Status Bit"]
    #[inline(always)]
    pub fn lis(&self) -> LIS_R {
        LIS_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of Disable Interrupt Status Bit"]
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of Flush Interrupt Status Bit"]
    #[inline(always)]
    pub fn fis(&self) -> FIS_R {
        FIS_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Status Bit"]
    #[inline(always)]
    pub fn rbeis(&self) -> RBEIS_R {
        RBEIS_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Status Bit"]
    #[inline(always)]
    pub fn wbeis(&self) -> WBEIS_R {
        WBEIS_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Status Bit"]
    #[inline(always)]
    pub fn rois(&self) -> ROIS_R {
        ROIS_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
}
