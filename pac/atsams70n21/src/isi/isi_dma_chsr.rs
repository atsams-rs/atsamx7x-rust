#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ISI_DMA_CHSR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type P_CH_S_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type C_CH_S_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Preview DMA Channel Status"]
    #[inline(always)]
    pub fn p_ch_s(&self) -> P_CH_S_R {
        P_CH_S_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Code DMA Channel Status"]
    #[inline(always)]
    pub fn c_ch_s(&self) -> C_CH_S_R {
        C_CH_S_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
}
