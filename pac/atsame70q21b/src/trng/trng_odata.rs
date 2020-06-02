#[doc = "Reader of register TRNG_ODATA"]
pub type R = crate::R<u32, super::TRNG_ODATA>;
#[doc = "Reader of field `ODATA`"]
pub type ODATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Output Data"]
    #[inline(always)]
    pub fn odata(&self) -> ODATA_R {
        ODATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
