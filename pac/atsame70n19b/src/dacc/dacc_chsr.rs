#[doc = "Reader of register DACC_CHSR"]
pub type R = crate::R<u32, super::DACC_CHSR>;
#[doc = "Reader of field `CH0`"]
pub type CH0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CH1`"]
pub type CH1_R = crate::R<bool, bool>;
#[doc = "Reader of field `DACRDY0`"]
pub type DACRDY0_R = crate::R<bool, bool>;
#[doc = "Reader of field `DACRDY1`"]
pub type DACRDY1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Status"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Status"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DAC Ready Flag"]
    #[inline(always)]
    pub fn dacrdy0(&self) -> DACRDY0_R {
        DACRDY0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DAC Ready Flag"]
    #[inline(always)]
    pub fn dacrdy1(&self) -> DACRDY1_R {
        DACRDY1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
