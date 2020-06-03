#[doc = "Reader of register SSC_RSHR"]
pub type R = crate::R<u32, super::SSC_RSHR>;
#[doc = "Reader of field `RSDAT`"]
pub type RSDAT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Synchronization Data"]
    #[inline(always)]
    pub fn rsdat(&self) -> RSDAT_R {
        RSDAT_R::new((self.bits & 0xffff) as u16)
    }
}
