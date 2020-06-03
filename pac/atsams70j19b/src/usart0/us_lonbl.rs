#[doc = "Reader of register US_LONBL"]
pub type R = crate::R<u32, super::US_LONBL>;
#[doc = "Reader of field `LONBL`"]
pub type LONBL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - LON Node Backlog Value"]
    #[inline(always)]
    pub fn lonbl(&self) -> LONBL_R {
        LONBL_R::new((self.bits & 0x3f) as u8)
    }
}
