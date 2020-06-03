#[doc = "Reader of register US_NER"]
pub type R = crate::R<u32, super::US_NER>;
#[doc = "Reader of field `NB_ERRORS`"]
pub type NB_ERRORS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of Errors"]
    #[inline(always)]
    pub fn nb_errors(&self) -> NB_ERRORS_R {
        NB_ERRORS_R::new((self.bits & 0xff) as u8)
    }
}
