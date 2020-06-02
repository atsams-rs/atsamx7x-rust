#[doc = "Reader of register PWM_IMR1"]
pub type R = crate::R<u32, super::PWM_IMR1>;
#[doc = "Reader of field `CHID0`"]
pub type CHID0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHID1`"]
pub type CHID1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHID2`"]
pub type CHID2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHID3`"]
pub type CHID3_R = crate::R<bool, bool>;
#[doc = "Reader of field `FCHID0`"]
pub type FCHID0_R = crate::R<bool, bool>;
#[doc = "Reader of field `FCHID1`"]
pub type FCHID1_R = crate::R<bool, bool>;
#[doc = "Reader of field `FCHID2`"]
pub type FCHID2_R = crate::R<bool, bool>;
#[doc = "Reader of field `FCHID3`"]
pub type FCHID3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Counter Event on Channel 0 Interrupt Mask"]
    #[inline(always)]
    pub fn chid0(&self) -> CHID0_R {
        CHID0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Counter Event on Channel 1 Interrupt Mask"]
    #[inline(always)]
    pub fn chid1(&self) -> CHID1_R {
        CHID1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Counter Event on Channel 2 Interrupt Mask"]
    #[inline(always)]
    pub fn chid2(&self) -> CHID2_R {
        CHID2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Counter Event on Channel 3 Interrupt Mask"]
    #[inline(always)]
    pub fn chid3(&self) -> CHID3_R {
        CHID3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fault Protection Trigger on Channel 0 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid0(&self) -> FCHID0_R {
        FCHID0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fault Protection Trigger on Channel 1 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid1(&self) -> FCHID1_R {
        FCHID1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fault Protection Trigger on Channel 2 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid2(&self) -> FCHID2_R {
        FCHID2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fault Protection Trigger on Channel 3 Interrupt Mask"]
    #[inline(always)]
    pub fn fchid3(&self) -> FCHID3_R {
        FCHID3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
