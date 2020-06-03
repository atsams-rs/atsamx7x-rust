#[doc = "Reader of register CCFG_DYNCKG"]
pub type R = crate::R<u32, super::CCFG_DYNCKG>;
#[doc = "Writer for register CCFG_DYNCKG"]
pub type W = crate::W<u32, super::CCFG_DYNCKG>;
#[doc = "Register CCFG_DYNCKG `reset()`'s with value 0"]
impl crate::ResetValue for super::CCFG_DYNCKG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MATCKG`"]
pub type MATCKG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MATCKG`"]
pub struct MATCKG_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCKG_W<'a> {
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
#[doc = "Reader of field `BRIDCKG`"]
pub type BRIDCKG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BRIDCKG`"]
pub struct BRIDCKG_W<'a> {
    w: &'a mut W,
}
impl<'a> BRIDCKG_W<'a> {
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
#[doc = "Reader of field `EFCCKG`"]
pub type EFCCKG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFCCKG`"]
pub struct EFCCKG_W<'a> {
    w: &'a mut W,
}
impl<'a> EFCCKG_W<'a> {
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
impl R {
    #[doc = "Bit 0 - MATRIX Dynamic Clock Gating"]
    #[inline(always)]
    pub fn matckg(&self) -> MATCKG_R {
        MATCKG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bridge Dynamic Clock Gating Enable"]
    #[inline(always)]
    pub fn bridckg(&self) -> BRIDCKG_R {
        BRIDCKG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EFC Dynamic Clock Gating Enable"]
    #[inline(always)]
    pub fn efcckg(&self) -> EFCCKG_R {
        EFCCKG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MATRIX Dynamic Clock Gating"]
    #[inline(always)]
    pub fn matckg(&mut self) -> MATCKG_W {
        MATCKG_W { w: self }
    }
    #[doc = "Bit 1 - Bridge Dynamic Clock Gating Enable"]
    #[inline(always)]
    pub fn bridckg(&mut self) -> BRIDCKG_W {
        BRIDCKG_W { w: self }
    }
    #[doc = "Bit 2 - EFC Dynamic Clock Gating Enable"]
    #[inline(always)]
    pub fn efcckg(&mut self) -> EFCCKG_W {
        EFCCKG_W { w: self }
    }
}
