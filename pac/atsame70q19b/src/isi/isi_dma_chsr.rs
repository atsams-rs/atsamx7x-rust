#[doc = "Reader of register ISI_DMA_CHSR"]
pub type R = crate::R<u32, super::ISI_DMA_CHSR>;
#[doc = "Reader of field `P_CH_S`"]
pub type P_CH_S_R = crate::R<bool, bool>;
#[doc = "Reader of field `C_CH_S`"]
pub type C_CH_S_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Preview DMA Channel Status"]
    #[inline(always)]
    pub fn p_ch_s(&self) -> P_CH_S_R {
        P_CH_S_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Code DMA Channel Status"]
    #[inline(always)]
    pub fn c_ch_s(&self) -> C_CH_S_R {
        C_CH_S_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
