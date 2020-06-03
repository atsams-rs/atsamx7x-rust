#[doc = "Reader of register TC_QIMR"]
pub type R = crate::R<u32, super::TC_QIMR>;
#[doc = "Reader of field `IDX`"]
pub type IDX_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIRCHG`"]
pub type DIRCHG_R = crate::R<bool, bool>;
#[doc = "Reader of field `QERR`"]
pub type QERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Index"]
    #[inline(always)]
    pub fn idx(&self) -> IDX_R {
        IDX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Direction Change"]
    #[inline(always)]
    pub fn dirchg(&self) -> DIRCHG_R {
        DIRCHG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Quadrature Error"]
    #[inline(always)]
    pub fn qerr(&self) -> QERR_R {
        QERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
