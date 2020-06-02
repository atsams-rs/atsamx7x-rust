#[doc = "Reader of register SDRAMC_ISR"]
pub type R = crate::R<u32, super::SDRAMC_ISR>;
#[doc = "Reader of field `RES`"]
pub type RES_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Refresh Error Status (cleared on read)"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits & 0x01) != 0)
    }
}
