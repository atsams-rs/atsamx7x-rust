#[doc = "Reader of register DACC_WPSR"]
pub type R = crate::R<u32, super::DACC_WPSR>;
#[doc = "Reader of field `WPVS`"]
pub type WPVS_R = crate::R<bool, bool>;
#[doc = "Reader of field `WPVSRC`"]
pub type WPVSRC_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Write Protection Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WPVS_R {
        WPVS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Write Protection Violation Source"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WPVSRC_R {
        WPVSRC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
