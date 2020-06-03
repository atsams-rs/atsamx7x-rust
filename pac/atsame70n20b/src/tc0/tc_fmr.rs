#[doc = "Reader of register TC_FMR"]
pub type R = crate::R<u32, super::TC_FMR>;
#[doc = "Writer for register TC_FMR"]
pub type W = crate::W<u32, super::TC_FMR>;
#[doc = "Register TC_FMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TC_FMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENCF0`"]
pub type ENCF0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENCF0`"]
pub struct ENCF0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCF0_W<'a> {
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
#[doc = "Reader of field `ENCF1`"]
pub type ENCF1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENCF1`"]
pub struct ENCF1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCF1_W<'a> {
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
    #[doc = "Bit 0 - Enable Compare Fault Channel 0"]
    #[inline(always)]
    pub fn encf0(&self) -> ENCF0_R {
        ENCF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Compare Fault Channel 1"]
    #[inline(always)]
    pub fn encf1(&self) -> ENCF1_R {
        ENCF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Compare Fault Channel 0"]
    #[inline(always)]
    pub fn encf0(&mut self) -> ENCF0_W {
        ENCF0_W { w: self }
    }
    #[doc = "Bit 1 - Enable Compare Fault Channel 1"]
    #[inline(always)]
    pub fn encf1(&mut self) -> ENCF1_W {
        ENCF1_W { w: self }
    }
}
