#[doc = "Reader of register SSC_RHR"]
pub type R = crate::R<u32, super::SSC_RHR>;
#[doc = "Reader of field `RDAT`"]
pub type RDAT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Data"]
    #[inline(always)]
    pub fn rdat(&self) -> RDAT_R {
        RDAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
