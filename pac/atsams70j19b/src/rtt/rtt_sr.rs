#[doc = "Reader of register RTT_SR"]
pub type R = crate::R<u32, super::RTT_SR>;
#[doc = "Reader of field `ALMS`"]
pub type ALMS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTTINC`"]
pub type RTTINC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Real-time Alarm Status (cleared on read)"]
    #[inline(always)]
    pub fn alms(&self) -> ALMS_R {
        ALMS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Prescaler Roll-over Status (cleared on read)"]
    #[inline(always)]
    pub fn rttinc(&self) -> RTTINC_R {
        RTTINC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
