#[doc = "Reader of register PWM_CCNT"]
pub type R = crate::R<u32, super::PWM_CCNT>;
#[doc = "Reader of field `CNT`"]
pub type CNT_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Channel Counter Register"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
