#[doc = "Reader of register EEFC_FRR"]
pub type R = crate::R<u32, super::EEFC_FRR>;
#[doc = "Reader of field `FVALUE`"]
pub type FVALUE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Flash Result Value"]
    #[inline(always)]
    pub fn fvalue(&self) -> FVALUE_R {
        FVALUE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
