#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PMC_SCSR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type HCLKS_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type USBCLK_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PCK0_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PCK1_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PCK2_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PCK3_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PCK4_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PCK5_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type PCK6_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - HCLK Status"]
    #[inline(always)]
    pub fn hclks(&self) -> HCLKS_R {
        HCLKS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 5 - USB FS Clock Status"]
    #[inline(always)]
    pub fn usbclk(&self) -> USBCLK_R {
        USBCLK_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Programmable Clock 0 Output Status"]
    #[inline(always)]
    pub fn pck0(&self) -> PCK0_R {
        PCK0_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Status"]
    #[inline(always)]
    pub fn pck1(&self) -> PCK1_R {
        PCK1_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Programmable Clock 2 Output Status"]
    #[inline(always)]
    pub fn pck2(&self) -> PCK2_R {
        PCK2_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Programmable Clock 3 Output Status"]
    #[inline(always)]
    pub fn pck3(&self) -> PCK3_R {
        PCK3_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Programmable Clock 4 Output Status"]
    #[inline(always)]
    pub fn pck4(&self) -> PCK4_R {
        PCK4_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Programmable Clock 5 Output Status"]
    #[inline(always)]
    pub fn pck5(&self) -> PCK5_R {
        PCK5_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Programmable Clock 6 Output Status"]
    #[inline(always)]
    pub fn pck6(&self) -> PCK6_R {
        PCK6_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
}
