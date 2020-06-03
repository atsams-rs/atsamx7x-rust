#[doc = "Reader of register QSPI_RDR"]
pub type R = crate::R<u32, super::QSPI_RDR>;
#[doc = "Reader of field `RD`"]
pub type RD_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Data"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new((self.bits & 0xffff) as u16)
    }
}
