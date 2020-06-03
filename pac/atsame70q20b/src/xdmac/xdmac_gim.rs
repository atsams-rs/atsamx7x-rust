#[doc = "Reader of register XDMAC_GIM"]
pub type R = crate::R<u32, super::XDMAC_GIM>;
#[doc = "Reader of field `IM0`"]
pub type IM0_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM1`"]
pub type IM1_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM2`"]
pub type IM2_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM3`"]
pub type IM3_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM4`"]
pub type IM4_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM5`"]
pub type IM5_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM6`"]
pub type IM6_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM7`"]
pub type IM7_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM8`"]
pub type IM8_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM9`"]
pub type IM9_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM10`"]
pub type IM10_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM11`"]
pub type IM11_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM12`"]
pub type IM12_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM13`"]
pub type IM13_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM14`"]
pub type IM14_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM15`"]
pub type IM15_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM16`"]
pub type IM16_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM17`"]
pub type IM17_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM18`"]
pub type IM18_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM19`"]
pub type IM19_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM20`"]
pub type IM20_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM21`"]
pub type IM21_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM22`"]
pub type IM22_R = crate::R<bool, bool>;
#[doc = "Reader of field `IM23`"]
pub type IM23_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - XDMAC Channel 0 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im0(&self) -> IM0_R {
        IM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im1(&self) -> IM1_R {
        IM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im2(&self) -> IM2_R {
        IM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im3(&self) -> IM3_R {
        IM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im4(&self) -> IM4_R {
        IM4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im5(&self) -> IM5_R {
        IM5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im6(&self) -> IM6_R {
        IM6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im7(&self) -> IM7_R {
        IM7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im8(&self) -> IM8_R {
        IM8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im9(&self) -> IM9_R {
        IM9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im10(&self) -> IM10_R {
        IM10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im11(&self) -> IM11_R {
        IM11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im12(&self) -> IM12_R {
        IM12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im13(&self) -> IM13_R {
        IM13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im14(&self) -> IM14_R {
        IM14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im15(&self) -> IM15_R {
        IM15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im16(&self) -> IM16_R {
        IM16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im17(&self) -> IM17_R {
        IM17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im18(&self) -> IM18_R {
        IM18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im19(&self) -> IM19_R {
        IM19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im20(&self) -> IM20_R {
        IM20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im21(&self) -> IM21_R {
        IM21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im22(&self) -> IM22_R {
        IM22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Interrupt Mask Bit"]
    #[inline(always)]
    pub fn im23(&self) -> IM23_R {
        IM23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
