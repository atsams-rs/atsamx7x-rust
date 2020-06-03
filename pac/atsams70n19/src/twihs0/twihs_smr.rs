#[doc = "Reader of register TWIHS_SMR"]
pub type R = crate::R<u32, super::TWIHS_SMR>;
#[doc = "Writer for register TWIHS_SMR"]
pub type W = crate::W<u32, super::TWIHS_SMR>;
#[doc = "Register TWIHS_SMR `reset()`'s with value 0"]
impl crate::ResetValue for super::TWIHS_SMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NACKEN`"]
pub type NACKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NACKEN`"]
pub struct NACKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NACKEN_W<'a> {
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
#[doc = "Reader of field `SMDA`"]
pub type SMDA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMDA`"]
pub struct SMDA_W<'a> {
    w: &'a mut W,
}
impl<'a> SMDA_W<'a> {
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
#[doc = "Reader of field `SMHH`"]
pub type SMHH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMHH`"]
pub struct SMHH_W<'a> {
    w: &'a mut W,
}
impl<'a> SMHH_W<'a> {
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
#[doc = "Reader of field `SCLWSDIS`"]
pub type SCLWSDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCLWSDIS`"]
pub struct SCLWSDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLWSDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `MASK`"]
pub type MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MASK`"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SADR`"]
pub type SADR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SADR`"]
pub struct SADR_W<'a> {
    w: &'a mut W,
}
impl<'a> SADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `SADR1EN`"]
pub type SADR1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SADR1EN`"]
pub struct SADR1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SADR1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `SADR2EN`"]
pub type SADR2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SADR2EN`"]
pub struct SADR2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SADR2EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SADR3EN`"]
pub type SADR3EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SADR3EN`"]
pub struct SADR3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SADR3EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `DATAMEN`"]
pub type DATAMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATAMEN`"]
pub struct DATAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Slave Receiver Data Phase NACK enable"]
    #[inline(always)]
    pub fn nacken(&self) -> NACKEN_R {
        NACKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - SMBus Default Address"]
    #[inline(always)]
    pub fn smda(&self) -> SMDA_R {
        SMDA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SMBus Host Header"]
    #[inline(always)]
    pub fn smhh(&self) -> SMHH_R {
        SMHH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Clock Wait State Disable"]
    #[inline(always)]
    pub fn sclwsdis(&self) -> SCLWSDIS_R {
        SCLWSDIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Slave Address"]
    #[inline(always)]
    pub fn sadr(&self) -> SADR_R {
        SADR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 28 - Slave Address 1 Enable"]
    #[inline(always)]
    pub fn sadr1en(&self) -> SADR1EN_R {
        SADR1EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Slave Address 2 Enable"]
    #[inline(always)]
    pub fn sadr2en(&self) -> SADR2EN_R {
        SADR2EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Slave Address 3 Enable"]
    #[inline(always)]
    pub fn sadr3en(&self) -> SADR3EN_R {
        SADR3EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Data Matching Enable"]
    #[inline(always)]
    pub fn datamen(&self) -> DATAMEN_R {
        DATAMEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Receiver Data Phase NACK enable"]
    #[inline(always)]
    pub fn nacken(&mut self) -> NACKEN_W {
        NACKEN_W { w: self }
    }
    #[doc = "Bit 2 - SMBus Default Address"]
    #[inline(always)]
    pub fn smda(&mut self) -> SMDA_W {
        SMDA_W { w: self }
    }
    #[doc = "Bit 3 - SMBus Host Header"]
    #[inline(always)]
    pub fn smhh(&mut self) -> SMHH_W {
        SMHH_W { w: self }
    }
    #[doc = "Bit 6 - Clock Wait State Disable"]
    #[inline(always)]
    pub fn sclwsdis(&mut self) -> SCLWSDIS_W {
        SCLWSDIS_W { w: self }
    }
    #[doc = "Bits 8:14 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Bits 16:22 - Slave Address"]
    #[inline(always)]
    pub fn sadr(&mut self) -> SADR_W {
        SADR_W { w: self }
    }
    #[doc = "Bit 28 - Slave Address 1 Enable"]
    #[inline(always)]
    pub fn sadr1en(&mut self) -> SADR1EN_W {
        SADR1EN_W { w: self }
    }
    #[doc = "Bit 29 - Slave Address 2 Enable"]
    #[inline(always)]
    pub fn sadr2en(&mut self) -> SADR2EN_W {
        SADR2EN_W { w: self }
    }
    #[doc = "Bit 30 - Slave Address 3 Enable"]
    #[inline(always)]
    pub fn sadr3en(&mut self) -> SADR3EN_W {
        SADR3EN_W { w: self }
    }
    #[doc = "Bit 31 - Data Matching Enable"]
    #[inline(always)]
    pub fn datamen(&mut self) -> DATAMEN_W {
        DATAMEN_W { w: self }
    }
}
