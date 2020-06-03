#[doc = "Reader of register GMAC_BCFT"]
pub type R = crate::R<u32, super::GMAC_BCFT>;
#[doc = "Reader of field `BFTX`"]
pub type BFTX_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Broadcast Frames Transmitted without Error"]
    #[inline(always)]
    pub fn bftx(&self) -> BFTX_R {
        BFTX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
