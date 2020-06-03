#[doc = "Reader of register WDT_SR"]
pub type R = crate::R<u32, super::WDT_SR>;
#[doc = "Reader of field `WDUNF`"]
pub type WDUNF_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDERR`"]
pub type WDERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Watchdog Underflow (cleared on read)"]
    #[inline(always)]
    pub fn wdunf(&self) -> WDUNF_R {
        WDUNF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Watchdog Error (cleared on read)"]
    #[inline(always)]
    pub fn wderr(&self) -> WDERR_R {
        WDERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
