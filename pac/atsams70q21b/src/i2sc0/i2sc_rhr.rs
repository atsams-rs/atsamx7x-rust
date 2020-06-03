#[doc = "Reader of register I2SC_RHR"]
pub type R = crate::R<u32, super::I2SC_RHR>;
#[doc = "Reader of field `RHR`"]
pub type RHR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receiver Holding Register"]
    #[inline(always)]
    pub fn rhr(&self) -> RHR_R {
        RHR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
