#[doc = "Reader of register GMAC_CBSCR"]
pub type R = crate::R<u32, super::GMAC_CBSCR>;
#[doc = "Writer for register GMAC_CBSCR"]
pub type W = crate::W<u32, super::GMAC_CBSCR>;
#[doc = "Register GMAC_CBSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::GMAC_CBSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QBE`"]
pub type QBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QBE`"]
pub struct QBE_W<'a> {
    w: &'a mut W,
}
impl<'a> QBE_W<'a> {
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
#[doc = "Reader of field `QAE`"]
pub type QAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QAE`"]
pub struct QAE_W<'a> {
    w: &'a mut W,
}
impl<'a> QAE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Queue B CBS Enable"]
    #[inline(always)]
    pub fn qbe(&self) -> QBE_R {
        QBE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Queue A CBS Enable"]
    #[inline(always)]
    pub fn qae(&self) -> QAE_R {
        QAE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Queue B CBS Enable"]
    #[inline(always)]
    pub fn qbe(&mut self) -> QBE_W {
        QBE_W { w: self }
    }
    #[doc = "Bit 1 - Queue A CBS Enable"]
    #[inline(always)]
    pub fn qae(&mut self) -> QAE_W {
        QAE_W { w: self }
    }
}
