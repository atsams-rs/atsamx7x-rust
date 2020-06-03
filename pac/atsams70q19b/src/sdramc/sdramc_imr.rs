#[doc = "Reader of register SDRAMC_IMR"]
pub type R = crate::R<u32, super::SDRAMC_IMR>;
#[doc = "Reader of field `RES`"]
pub type RES_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Refresh Error Interrupt Mask"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new((self.bits & 0x01) != 0)
    }
}
