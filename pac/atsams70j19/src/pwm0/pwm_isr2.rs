#[doc = "Reader of register PWM_ISR2"]
pub type R = crate::R<u32, super::PWM_ISR2>;
#[doc = "Reader of field `WRDY`"]
pub type WRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `UNRE`"]
pub type UNRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPM0`"]
pub type CMPM0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPM1`"]
pub type CMPM1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPM2`"]
pub type CMPM2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPM3`"]
pub type CMPM3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPM4`"]
pub type CMPM4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPM5`"]
pub type CMPM5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPM6`"]
pub type CMPM6_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPM7`"]
pub type CMPM7_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPU0`"]
pub type CMPU0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPU1`"]
pub type CMPU1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPU2`"]
pub type CMPU2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPU3`"]
pub type CMPU3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPU4`"]
pub type CMPU4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPU5`"]
pub type CMPU5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPU6`"]
pub type CMPU6_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMPU7`"]
pub type CMPU7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Write Ready for Synchronous Channels Update"]
    #[inline(always)]
    pub fn wrdy(&self) -> WRDY_R {
        WRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Synchronous Channels Update Underrun Error"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Comparison 0 Match"]
    #[inline(always)]
    pub fn cmpm0(&self) -> CMPM0_R {
        CMPM0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Comparison 1 Match"]
    #[inline(always)]
    pub fn cmpm1(&self) -> CMPM1_R {
        CMPM1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Comparison 2 Match"]
    #[inline(always)]
    pub fn cmpm2(&self) -> CMPM2_R {
        CMPM2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Comparison 3 Match"]
    #[inline(always)]
    pub fn cmpm3(&self) -> CMPM3_R {
        CMPM3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comparison 4 Match"]
    #[inline(always)]
    pub fn cmpm4(&self) -> CMPM4_R {
        CMPM4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Comparison 5 Match"]
    #[inline(always)]
    pub fn cmpm5(&self) -> CMPM5_R {
        CMPM5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Comparison 6 Match"]
    #[inline(always)]
    pub fn cmpm6(&self) -> CMPM6_R {
        CMPM6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Comparison 7 Match"]
    #[inline(always)]
    pub fn cmpm7(&self) -> CMPM7_R {
        CMPM7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Comparison 0 Update"]
    #[inline(always)]
    pub fn cmpu0(&self) -> CMPU0_R {
        CMPU0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Comparison 1 Update"]
    #[inline(always)]
    pub fn cmpu1(&self) -> CMPU1_R {
        CMPU1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Comparison 2 Update"]
    #[inline(always)]
    pub fn cmpu2(&self) -> CMPU2_R {
        CMPU2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Comparison 3 Update"]
    #[inline(always)]
    pub fn cmpu3(&self) -> CMPU3_R {
        CMPU3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Comparison 4 Update"]
    #[inline(always)]
    pub fn cmpu4(&self) -> CMPU4_R {
        CMPU4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Comparison 5 Update"]
    #[inline(always)]
    pub fn cmpu5(&self) -> CMPU5_R {
        CMPU5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Comparison 6 Update"]
    #[inline(always)]
    pub fn cmpu6(&self) -> CMPU6_R {
        CMPU6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Comparison 7 Update"]
    #[inline(always)]
    pub fn cmpu7(&self) -> CMPU7_R {
        CMPU7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
