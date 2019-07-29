#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AFEC_OVER {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type OVRE0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVRE1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVRE2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVRE3_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVRE4_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVRE5_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVRE6_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVRE7_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVRE8_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVRE9_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVRE10_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type OVRE11_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Overrun Error 0"]
    #[inline(always)]
    pub fn ovre0(&self) -> OVRE0_R {
        OVRE0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overrun Error 1"]
    #[inline(always)]
    pub fn ovre1(&self) -> OVRE1_R {
        OVRE1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Overrun Error 2"]
    #[inline(always)]
    pub fn ovre2(&self) -> OVRE2_R {
        OVRE2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun Error 3"]
    #[inline(always)]
    pub fn ovre3(&self) -> OVRE3_R {
        OVRE3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Overrun Error 4"]
    #[inline(always)]
    pub fn ovre4(&self) -> OVRE4_R {
        OVRE4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overrun Error 5"]
    #[inline(always)]
    pub fn ovre5(&self) -> OVRE5_R {
        OVRE5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Overrun Error 6"]
    #[inline(always)]
    pub fn ovre6(&self) -> OVRE6_R {
        OVRE6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Overrun Error 7"]
    #[inline(always)]
    pub fn ovre7(&self) -> OVRE7_R {
        OVRE7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Overrun Error 8"]
    #[inline(always)]
    pub fn ovre8(&self) -> OVRE8_R {
        OVRE8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Overrun Error 9"]
    #[inline(always)]
    pub fn ovre9(&self) -> OVRE9_R {
        OVRE9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Overrun Error 10"]
    #[inline(always)]
    pub fn ovre10(&self) -> OVRE10_R {
        OVRE10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Overrun Error 11"]
    #[inline(always)]
    pub fn ovre11(&self) -> OVRE11_R {
        OVRE11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
}
