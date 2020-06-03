#[doc = "Reader of register US_RHR"]
pub type R = crate::R<u32, super::US_RHR>;
#[doc = "Reader of field `RXCHR`"]
pub type RXCHR_R = crate::R<u16, u16>;
#[doc = "Reader of field `RXSYNH`"]
pub type RXSYNH_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:8 - Received Character"]
    #[inline(always)]
    pub fn rxchr(&self) -> RXCHR_R {
        RXCHR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - Received Sync"]
    #[inline(always)]
    pub fn rxsynh(&self) -> RXSYNH_R {
        RXSYNH_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
