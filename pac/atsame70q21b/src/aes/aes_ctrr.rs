#[doc = "Reader of register AES_CTRR"]
pub type R = crate::R<u32, super::AES_CTRR>;
#[doc = "Reader of field `CTR`"]
pub type CTR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - GCM Encryption Counter"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
