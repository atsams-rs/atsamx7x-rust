#[doc = "Reader of register CHIPID_EXID"]
pub type R = crate::R<u32, super::CHIPID_EXID>;
#[doc = "Reader of field `EXID`"]
pub type EXID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Chip ID Extension"]
    #[inline(always)]
    pub fn exid(&self) -> EXID_R {
        EXID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
