#[doc = "Reader of register TRNG_ISR"]
pub type R = crate::R<u32, super::TRNG_ISR>;
#[doc = "Reader of field `DATRDY`"]
pub type DATRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Data Ready"]
    #[inline(always)]
    pub fn datrdy(&self) -> DATRDY_R {
        DATRDY_R::new((self.bits & 0x01) != 0)
    }
}
