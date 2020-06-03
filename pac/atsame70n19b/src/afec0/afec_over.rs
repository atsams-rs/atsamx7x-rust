#[doc = "Reader of register AFEC_OVER"]
pub type R = crate::R<u32, super::AFEC_OVER>;
#[doc = "Reader of field `OVRE0`"]
pub type OVRE0_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE1`"]
pub type OVRE1_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE2`"]
pub type OVRE2_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE3`"]
pub type OVRE3_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE4`"]
pub type OVRE4_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE5`"]
pub type OVRE5_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE6`"]
pub type OVRE6_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE7`"]
pub type OVRE7_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE8`"]
pub type OVRE8_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE9`"]
pub type OVRE9_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE10`"]
pub type OVRE10_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVRE11`"]
pub type OVRE11_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Overrun Error 0"]
    #[inline(always)]
    pub fn ovre0(&self) -> OVRE0_R {
        OVRE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overrun Error 1"]
    #[inline(always)]
    pub fn ovre1(&self) -> OVRE1_R {
        OVRE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Overrun Error 2"]
    #[inline(always)]
    pub fn ovre2(&self) -> OVRE2_R {
        OVRE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun Error 3"]
    #[inline(always)]
    pub fn ovre3(&self) -> OVRE3_R {
        OVRE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Overrun Error 4"]
    #[inline(always)]
    pub fn ovre4(&self) -> OVRE4_R {
        OVRE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overrun Error 5"]
    #[inline(always)]
    pub fn ovre5(&self) -> OVRE5_R {
        OVRE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Overrun Error 6"]
    #[inline(always)]
    pub fn ovre6(&self) -> OVRE6_R {
        OVRE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Overrun Error 7"]
    #[inline(always)]
    pub fn ovre7(&self) -> OVRE7_R {
        OVRE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Overrun Error 8"]
    #[inline(always)]
    pub fn ovre8(&self) -> OVRE8_R {
        OVRE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Overrun Error 9"]
    #[inline(always)]
    pub fn ovre9(&self) -> OVRE9_R {
        OVRE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Overrun Error 10"]
    #[inline(always)]
    pub fn ovre10(&self) -> OVRE10_R {
        OVRE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Overrun Error 11"]
    #[inline(always)]
    pub fn ovre11(&self) -> OVRE11_R {
        OVRE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
