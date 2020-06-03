#[doc = "Reader of register PMC_SLPWK_AIPR"]
pub type R = crate::R<u32, super::PMC_SLPWK_AIPR>;
#[doc = "Reader of field `AIP`"]
pub type AIP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Activity In Progress"]
    #[inline(always)]
    pub fn aip(&self) -> AIP_R {
        AIP_R::new((self.bits & 0x01) != 0)
    }
}
