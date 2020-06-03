#[doc = "Reader of register SPI_RDR"]
pub type R = crate::R<u32, super::SPI_RDR>;
#[doc = "Reader of field `RD`"]
pub type RD_R = crate::R<u16, u16>;
#[doc = "Reader of field `PCS`"]
pub type PCS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Receive Data"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
