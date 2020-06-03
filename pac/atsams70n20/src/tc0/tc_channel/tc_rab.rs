#[doc = "Reader of register TC_RAB"]
pub type R = crate::R<u32, super::TC_RAB>;
#[doc = "Reader of field `RAB`"]
pub type RAB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register A or Register B"]
    #[inline(always)]
    pub fn rab(&self) -> RAB_R {
        RAB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
