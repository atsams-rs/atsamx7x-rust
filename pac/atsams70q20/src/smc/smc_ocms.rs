#[doc = "Reader of register SMC_OCMS"]
pub type R = crate::R<u32, super::SMC_OCMS>;
#[doc = "Writer for register SMC_OCMS"]
pub type W = crate::W<u32, super::SMC_OCMS>;
#[doc = "Register SMC_OCMS `reset()`'s with value 0"]
impl crate::ResetValue for super::SMC_OCMS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SMSE`"]
pub type SMSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMSE`"]
pub struct SMSE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMSE_W<'a> {
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
#[doc = "Reader of field `CS0SE`"]
pub type CS0SE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CS0SE`"]
pub struct CS0SE_W<'a> {
    w: &'a mut W,
}
impl<'a> CS0SE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `CS1SE`"]
pub type CS1SE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CS1SE`"]
pub struct CS1SE_W<'a> {
    w: &'a mut W,
}
impl<'a> CS1SE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CS2SE`"]
pub type CS2SE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CS2SE`"]
pub struct CS2SE_W<'a> {
    w: &'a mut W,
}
impl<'a> CS2SE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CS3SE`"]
pub type CS3SE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CS3SE`"]
pub struct CS3SE_W<'a> {
    w: &'a mut W,
}
impl<'a> CS3SE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn smse(&self) -> SMSE_R {
        SMSE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Chip Select 0 Scrambling Enable"]
    #[inline(always)]
    pub fn cs0se(&self) -> CS0SE_R {
        CS0SE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Chip Select 1 Scrambling Enable"]
    #[inline(always)]
    pub fn cs1se(&self) -> CS1SE_R {
        CS1SE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Chip Select 2 Scrambling Enable"]
    #[inline(always)]
    pub fn cs2se(&self) -> CS2SE_R {
        CS2SE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Chip Select 3 Scrambling Enable"]
    #[inline(always)]
    pub fn cs3se(&self) -> CS3SE_R {
        CS3SE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Static Memory Controller Scrambling Enable"]
    #[inline(always)]
    pub fn smse(&mut self) -> SMSE_W {
        SMSE_W { w: self }
    }
    #[doc = "Bit 8 - Chip Select 0 Scrambling Enable"]
    #[inline(always)]
    pub fn cs0se(&mut self) -> CS0SE_W {
        CS0SE_W { w: self }
    }
    #[doc = "Bit 9 - Chip Select 1 Scrambling Enable"]
    #[inline(always)]
    pub fn cs1se(&mut self) -> CS1SE_W {
        CS1SE_W { w: self }
    }
    #[doc = "Bit 10 - Chip Select 2 Scrambling Enable"]
    #[inline(always)]
    pub fn cs2se(&mut self) -> CS2SE_W {
        CS2SE_W { w: self }
    }
    #[doc = "Bit 11 - Chip Select 3 Scrambling Enable"]
    #[inline(always)]
    pub fn cs3se(&mut self) -> CS3SE_W {
        CS3SE_W { w: self }
    }
}
