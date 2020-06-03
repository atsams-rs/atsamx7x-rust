#[doc = "Reader of register GMAC_RXLPITIME"]
pub type R = crate::R<u32, super::GMAC_RXLPITIME>;
#[doc = "Reader of field `LPITIME`"]
pub type LPITIME_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Time in LPI (cleared on read)"]
    #[inline(always)]
    pub fn lpitime(&self) -> LPITIME_R {
        LPITIME_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
