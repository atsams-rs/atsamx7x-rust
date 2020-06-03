#[doc = "Reader of register XDMAC_CIM"]
pub type R = crate::R<u32, super::XDMAC_CIM>;
#[doc = "Reader of field `BIM`"]
pub type BIM_R = crate::R<bool, bool>;
#[doc = "Reader of field `LIM`"]
pub type LIM_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIM`"]
pub type DIM_R = crate::R<bool, bool>;
#[doc = "Reader of field `FIM`"]
pub type FIM_R = crate::R<bool, bool>;
#[doc = "Reader of field `RBEIM`"]
pub type RBEIM_R = crate::R<bool, bool>;
#[doc = "Reader of field `WBEIM`"]
pub type WBEIM_R = crate::R<bool, bool>;
#[doc = "Reader of field `ROIM`"]
pub type ROIM_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - End of Block Interrupt Mask Bit"]
    #[inline(always)]
    pub fn bim(&self) -> BIM_R {
        BIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Mask Bit"]
    #[inline(always)]
    pub fn lim(&self) -> LIM_R {
        LIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of Disable Interrupt Mask Bit"]
    #[inline(always)]
    pub fn dim(&self) -> DIM_R {
        DIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of Flush Interrupt Mask Bit"]
    #[inline(always)]
    pub fn fim(&self) -> FIM_R {
        FIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Mask Bit"]
    #[inline(always)]
    pub fn rbeim(&self) -> RBEIM_R {
        RBEIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Mask Bit"]
    #[inline(always)]
    pub fn wbeim(&self) -> WBEIM_R {
        WBEIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Mask Bit"]
    #[inline(always)]
    pub fn roim(&self) -> ROIM_R {
        ROIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
