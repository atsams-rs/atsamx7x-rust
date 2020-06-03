#[doc = "Reader of register ACC_ISR"]
pub type R = crate::R<u32, super::ACC_ISR>;
#[doc = "Reader of field `CE`"]
pub type CE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCO`"]
pub type SCO_R = crate::R<bool, bool>;
#[doc = "Reader of field `MASK`"]
pub type MASK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Comparison Edge (cleared on read)"]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Synchronized Comparator Output"]
    #[inline(always)]
    pub fn sco(&self) -> SCO_R {
        SCO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Flag Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
