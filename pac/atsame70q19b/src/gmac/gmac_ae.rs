#[doc = "Reader of register GMAC_AE"]
pub type R = crate::R<u32, super::GMAC_AE>;
#[doc = "Reader of field `AER`"]
pub type AER_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Alignment Errors"]
    #[inline(always)]
    pub fn aer(&self) -> AER_R {
        AER_R::new((self.bits & 0x03ff) as u16)
    }
}
