#[doc = "Reader of register ACC_IMR"]
pub type R = crate::R<u32, super::ACC_IMR>;
#[doc = "Reader of field `CE`"]
pub type CE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Comparison Edge"]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new((self.bits & 0x01) != 0)
    }
}
