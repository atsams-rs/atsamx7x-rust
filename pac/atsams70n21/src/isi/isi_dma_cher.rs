#[doc = "Writer for register ISI_DMA_CHER"]
pub type W = crate::W<u32, super::ISI_DMA_CHER>;
#[doc = "Register ISI_DMA_CHER `reset()`'s with value 0"]
impl crate::ResetValue for super::ISI_DMA_CHER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `P_CH_EN`"]
pub struct P_CH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> P_CH_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `C_CH_EN`"]
pub struct C_CH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> C_CH_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Preview Channel Enable"]
    #[inline(always)]
    pub fn p_ch_en(&mut self) -> P_CH_EN_W {
        P_CH_EN_W { w: self }
    }
    #[doc = "Bit 1 - Codec Channel Enable"]
    #[inline(always)]
    pub fn c_ch_en(&mut self) -> C_CH_EN_W {
        C_CH_EN_W { w: self }
    }
}
