#[doc = "Reader of register CCFG_PCCR"]
pub type R = crate::R<u32, super::CCFG_PCCR>;
#[doc = "Writer for register CCFG_PCCR"]
pub type W = crate::W<u32, super::CCFG_PCCR>;
#[doc = "Register CCFG_PCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CCFG_PCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TC0CC`"]
pub type TC0CC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TC0CC`"]
pub struct TC0CC_W<'a> {
    w: &'a mut W,
}
impl<'a> TC0CC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `I2SC0CC`"]
pub type I2SC0CC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2SC0CC`"]
pub struct I2SC0CC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SC0CC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `I2SC1CC`"]
pub type I2SC1CC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2SC1CC`"]
pub struct I2SC1CC_W<'a> {
    w: &'a mut W,
}
impl<'a> I2SC1CC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 20 - TC0 Clock Configuration"]
    #[inline(always)]
    pub fn tc0cc(&self) -> TC0CC_R {
        TC0CC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2SC0 Clock Configuration"]
    #[inline(always)]
    pub fn i2sc0cc(&self) -> I2SC0CC_R {
        I2SC0CC_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - I2SC1 Clock Configuration"]
    #[inline(always)]
    pub fn i2sc1cc(&self) -> I2SC1CC_R {
        I2SC1CC_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - TC0 Clock Configuration"]
    #[inline(always)]
    pub fn tc0cc(&mut self) -> TC0CC_W {
        TC0CC_W { w: self }
    }
    #[doc = "Bit 21 - I2SC0 Clock Configuration"]
    #[inline(always)]
    pub fn i2sc0cc(&mut self) -> I2SC0CC_W {
        I2SC0CC_W { w: self }
    }
    #[doc = "Bit 22 - I2SC1 Clock Configuration"]
    #[inline(always)]
    pub fn i2sc1cc(&mut self) -> I2SC1CC_W {
        I2SC1CC_W { w: self }
    }
}
