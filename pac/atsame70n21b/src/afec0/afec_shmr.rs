#[doc = "Reader of register AFEC_SHMR"]
pub type R = crate::R<u32, super::AFEC_SHMR>;
#[doc = "Writer for register AFEC_SHMR"]
pub type W = crate::W<u32, super::AFEC_SHMR>;
#[doc = "Register AFEC_SHMR `reset()`'s with value 0"]
impl crate::ResetValue for super::AFEC_SHMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DUAL0`"]
pub type DUAL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUAL0`"]
pub struct DUAL0_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL0_W<'a> {
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
#[doc = "Reader of field `DUAL1`"]
pub type DUAL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUAL1`"]
pub struct DUAL1_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL1_W<'a> {
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
#[doc = "Reader of field `DUAL2`"]
pub type DUAL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUAL2`"]
pub struct DUAL2_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL2_W<'a> {
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
#[doc = "Reader of field `DUAL3`"]
pub type DUAL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUAL3`"]
pub struct DUAL3_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL3_W<'a> {
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
#[doc = "Reader of field `DUAL4`"]
pub type DUAL4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUAL4`"]
pub struct DUAL4_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DUAL5`"]
pub type DUAL5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUAL5`"]
pub struct DUAL5_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DUAL6`"]
pub type DUAL6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUAL6`"]
pub struct DUAL6_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL6_W<'a> {
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
#[doc = "Reader of field `DUAL7`"]
pub type DUAL7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUAL7`"]
pub struct DUAL7_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `DUAL8`"]
pub type DUAL8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUAL8`"]
pub struct DUAL8_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL8_W<'a> {
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
#[doc = "Reader of field `DUAL9`"]
pub type DUAL9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUAL9`"]
pub struct DUAL9_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL9_W<'a> {
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
#[doc = "Reader of field `DUAL10`"]
pub type DUAL10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUAL10`"]
pub struct DUAL10_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL10_W<'a> {
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
#[doc = "Reader of field `DUAL11`"]
pub type DUAL11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DUAL11`"]
pub struct DUAL11_W<'a> {
    w: &'a mut W,
}
impl<'a> DUAL11_W<'a> {
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
    #[doc = "Bit 0 - Dual Sample & Hold for channel 0"]
    #[inline(always)]
    pub fn dual0(&self) -> DUAL0_R {
        DUAL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Dual Sample & Hold for channel 1"]
    #[inline(always)]
    pub fn dual1(&self) -> DUAL1_R {
        DUAL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Dual Sample & Hold for channel 2"]
    #[inline(always)]
    pub fn dual2(&self) -> DUAL2_R {
        DUAL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Dual Sample & Hold for channel 3"]
    #[inline(always)]
    pub fn dual3(&self) -> DUAL3_R {
        DUAL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Dual Sample & Hold for channel 4"]
    #[inline(always)]
    pub fn dual4(&self) -> DUAL4_R {
        DUAL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Dual Sample & Hold for channel 5"]
    #[inline(always)]
    pub fn dual5(&self) -> DUAL5_R {
        DUAL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Dual Sample & Hold for channel 6"]
    #[inline(always)]
    pub fn dual6(&self) -> DUAL6_R {
        DUAL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Dual Sample & Hold for channel 7"]
    #[inline(always)]
    pub fn dual7(&self) -> DUAL7_R {
        DUAL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Dual Sample & Hold for channel 8"]
    #[inline(always)]
    pub fn dual8(&self) -> DUAL8_R {
        DUAL8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Dual Sample & Hold for channel 9"]
    #[inline(always)]
    pub fn dual9(&self) -> DUAL9_R {
        DUAL9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Dual Sample & Hold for channel 10"]
    #[inline(always)]
    pub fn dual10(&self) -> DUAL10_R {
        DUAL10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Dual Sample & Hold for channel 11"]
    #[inline(always)]
    pub fn dual11(&self) -> DUAL11_R {
        DUAL11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Dual Sample & Hold for channel 0"]
    #[inline(always)]
    pub fn dual0(&mut self) -> DUAL0_W {
        DUAL0_W { w: self }
    }
    #[doc = "Bit 1 - Dual Sample & Hold for channel 1"]
    #[inline(always)]
    pub fn dual1(&mut self) -> DUAL1_W {
        DUAL1_W { w: self }
    }
    #[doc = "Bit 2 - Dual Sample & Hold for channel 2"]
    #[inline(always)]
    pub fn dual2(&mut self) -> DUAL2_W {
        DUAL2_W { w: self }
    }
    #[doc = "Bit 3 - Dual Sample & Hold for channel 3"]
    #[inline(always)]
    pub fn dual3(&mut self) -> DUAL3_W {
        DUAL3_W { w: self }
    }
    #[doc = "Bit 4 - Dual Sample & Hold for channel 4"]
    #[inline(always)]
    pub fn dual4(&mut self) -> DUAL4_W {
        DUAL4_W { w: self }
    }
    #[doc = "Bit 5 - Dual Sample & Hold for channel 5"]
    #[inline(always)]
    pub fn dual5(&mut self) -> DUAL5_W {
        DUAL5_W { w: self }
    }
    #[doc = "Bit 6 - Dual Sample & Hold for channel 6"]
    #[inline(always)]
    pub fn dual6(&mut self) -> DUAL6_W {
        DUAL6_W { w: self }
    }
    #[doc = "Bit 7 - Dual Sample & Hold for channel 7"]
    #[inline(always)]
    pub fn dual7(&mut self) -> DUAL7_W {
        DUAL7_W { w: self }
    }
    #[doc = "Bit 8 - Dual Sample & Hold for channel 8"]
    #[inline(always)]
    pub fn dual8(&mut self) -> DUAL8_W {
        DUAL8_W { w: self }
    }
    #[doc = "Bit 9 - Dual Sample & Hold for channel 9"]
    #[inline(always)]
    pub fn dual9(&mut self) -> DUAL9_W {
        DUAL9_W { w: self }
    }
    #[doc = "Bit 10 - Dual Sample & Hold for channel 10"]
    #[inline(always)]
    pub fn dual10(&mut self) -> DUAL10_W {
        DUAL10_W { w: self }
    }
    #[doc = "Bit 11 - Dual Sample & Hold for channel 11"]
    #[inline(always)]
    pub fn dual11(&mut self) -> DUAL11_W {
        DUAL11_W { w: self }
    }
}
