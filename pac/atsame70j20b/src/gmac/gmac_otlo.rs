#[doc = "Reader of register GMAC_OTLO"]
pub type R = crate::R<u32, super::GMAC_OTLO>;
#[doc = "Reader of field `TXO`"]
pub type TXO_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmitted Octets"]
    #[inline(always)]
    pub fn txo(&self) -> TXO_R {
        TXO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
