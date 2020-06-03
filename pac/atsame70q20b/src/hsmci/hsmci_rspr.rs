#[doc = "Reader of register HSMCI_RSPR[%s]"]
pub type R = crate::R<u32, super::HSMCI_RSPR>;
#[doc = "Reader of field `RSP`"]
pub type RSP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Response"]
    #[inline(always)]
    pub fn rsp(&self) -> RSP_R {
        RSP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
