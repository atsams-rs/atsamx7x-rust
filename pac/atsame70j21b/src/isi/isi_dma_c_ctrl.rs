#[doc = "Reader of register ISI_DMA_C_CTRL"]
pub type R = crate::R<u32, super::ISI_DMA_C_CTRL>;
#[doc = "Writer for register ISI_DMA_C_CTRL"]
pub type W = crate::W<u32, super::ISI_DMA_C_CTRL>;
#[doc = "Register ISI_DMA_C_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ISI_DMA_C_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `C_FETCH`"]
pub type C_FETCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_FETCH`"]
pub struct C_FETCH_W<'a> {
    w: &'a mut W,
}
impl<'a> C_FETCH_W<'a> {
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
#[doc = "Reader of field `C_WB`"]
pub type C_WB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_WB`"]
pub struct C_WB_W<'a> {
    w: &'a mut W,
}
impl<'a> C_WB_W<'a> {
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
#[doc = "Reader of field `C_IEN`"]
pub type C_IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_IEN`"]
pub struct C_IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> C_IEN_W<'a> {
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
#[doc = "Reader of field `C_DONE`"]
pub type C_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `C_DONE`"]
pub struct C_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> C_DONE_W<'a> {
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
    pub fn c_fetch(&self) -> C_FETCH_R {
        C_FETCH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Descriptor Writeback Control Bit"]
    #[inline(always)]
    pub fn c_wb(&self) -> C_WB_R {
        C_WB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transfer Done Flag Control"]
    #[inline(always)]
    pub fn c_ien(&self) -> C_IEN_R {
        C_IEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Codec Transfer Done"]
    #[inline(always)]
    pub fn c_done(&self) -> C_DONE_R {
        C_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Descriptor Fetch Control Bit"]
    #[inline(always)]
    pub fn c_fetch(&mut self) -> C_FETCH_W {
        C_FETCH_W { w: self }
    }
    #[doc = "Bit 1 - Descriptor Writeback Control Bit"]
    #[inline(always)]
    pub fn c_wb(&mut self) -> C_WB_W {
        C_WB_W { w: self }
    }
    #[doc = "Bit 2 - Transfer Done Flag Control"]
    #[inline(always)]
    pub fn c_ien(&mut self) -> C_IEN_W {
        C_IEN_W { w: self }
    }
    #[doc = "Bit 3 - Codec Transfer Done"]
    #[inline(always)]
    pub fn c_done(&mut self) -> C_DONE_W {
        C_DONE_W { w: self }
    }
}
