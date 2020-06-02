#[doc = "Reader of register ISI_DMA_P_CTRL"]
pub type R = crate::R<u32, super::ISI_DMA_P_CTRL>;
#[doc = "Writer for register ISI_DMA_P_CTRL"]
pub type W = crate::W<u32, super::ISI_DMA_P_CTRL>;
#[doc = "Register ISI_DMA_P_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ISI_DMA_P_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `P_FETCH`"]
pub type P_FETCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P_FETCH`"]
pub struct P_FETCH_W<'a> {
    w: &'a mut W,
}
impl<'a> P_FETCH_W<'a> {
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
#[doc = "Reader of field `P_WB`"]
pub type P_WB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P_WB`"]
pub struct P_WB_W<'a> {
    w: &'a mut W,
}
impl<'a> P_WB_W<'a> {
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
#[doc = "Reader of field `P_IEN`"]
pub type P_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P_IEN`"]
pub struct P_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> P_IEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `P_DONE`"]
pub type P_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `P_DONE`"]
pub struct P_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> P_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Descriptor Fetch Control Bit"]
    #[inline(always)]
    pub fn p_fetch(&self) -> P_FETCH_R {
        P_FETCH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Descriptor Writeback Control Bit"]
    #[inline(always)]
    pub fn p_wb(&self) -> P_WB_R {
        P_WB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transfer Done Flag Control"]
    #[inline(always)]
    pub fn p_ien(&self) -> P_IEN_R {
        P_IEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Preview Transfer Done"]
    #[inline(always)]
    pub fn p_done(&self) -> P_DONE_R {
        P_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Descriptor Fetch Control Bit"]
    #[inline(always)]
    pub fn p_fetch(&mut self) -> P_FETCH_W {
        P_FETCH_W { w: self }
    }
    #[doc = "Bit 1 - Descriptor Writeback Control Bit"]
    #[inline(always)]
    pub fn p_wb(&mut self) -> P_WB_W {
        P_WB_W { w: self }
    }
    #[doc = "Bit 2 - Transfer Done Flag Control"]
    #[inline(always)]
    pub fn p_ien(&mut self) -> P_IEN_W {
        P_IEN_W { w: self }
    }
    #[doc = "Bit 3 - Preview Transfer Done"]
    #[inline(always)]
    pub fn p_done(&mut self) -> P_DONE_W {
        P_DONE_W { w: self }
    }
}
