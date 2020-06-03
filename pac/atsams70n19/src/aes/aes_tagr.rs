#[doc = "Reader of register AES_TAGR[%s]"]
pub type R = crate::R<u32, super::AES_TAGR>;
#[doc = "Reader of field `TAG`"]
pub type TAG_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - GCM Authentication Tag x"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
