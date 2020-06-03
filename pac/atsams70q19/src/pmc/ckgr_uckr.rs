#[doc = "Reader of register CKGR_UCKR"]
pub type R = crate::R<u32, super::CKGR_UCKR>;
#[doc = "Writer for register CKGR_UCKR"]
pub type W = crate::W<u32, super::CKGR_UCKR>;
#[doc = "Register CKGR_UCKR `reset()`'s with value 0"]
impl crate::ResetValue for super::CKGR_UCKR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UPLLEN`"]
pub type UPLLEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPLLEN`"]
pub struct UPLLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UPLLEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `UPLLCOUNT`"]
pub type UPLLCOUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UPLLCOUNT`"]
pub struct UPLLCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> UPLLCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - UTMI PLL Enable"]
    #[inline(always)]
    pub fn upllen(&self) -> UPLLEN_R {
        UPLLEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - UTMI PLL Start-up Time"]
    #[inline(always)]
    pub fn upllcount(&self) -> UPLLCOUNT_R {
        UPLLCOUNT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - UTMI PLL Enable"]
    #[inline(always)]
    pub fn upllen(&mut self) -> UPLLEN_W {
        UPLLEN_W { w: self }
    }
    #[doc = "Bits 20:23 - UTMI PLL Start-up Time"]
    #[inline(always)]
    pub fn upllcount(&mut self) -> UPLLCOUNT_W {
        UPLLCOUNT_W { w: self }
    }
}
