#[doc = "Reader of register PMC_FSPR"]
pub type R = crate::R<u32, super::PMC_FSPR>;
#[doc = "Writer for register PMC_FSPR"]
pub type W = crate::W<u32, super::PMC_FSPR>;
#[doc = "Register PMC_FSPR `reset()`'s with value 0"]
impl crate::ResetValue for super::PMC_FSPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FSTP0`"]
pub type FSTP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTP0`"]
pub struct FSTP0_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP0_W<'a> {
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
#[doc = "Reader of field `FSTP1`"]
pub type FSTP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTP1`"]
pub struct FSTP1_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP1_W<'a> {
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
#[doc = "Reader of field `FSTP2`"]
pub type FSTP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTP2`"]
pub struct FSTP2_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP2_W<'a> {
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
#[doc = "Reader of field `FSTP3`"]
pub type FSTP3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTP3`"]
pub struct FSTP3_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP3_W<'a> {
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
#[doc = "Reader of field `FSTP4`"]
pub type FSTP4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTP4`"]
pub struct FSTP4_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP4_W<'a> {
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
#[doc = "Reader of field `FSTP5`"]
pub type FSTP5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTP5`"]
pub struct FSTP5_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP5_W<'a> {
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
#[doc = "Reader of field `FSTP6`"]
pub type FSTP6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTP6`"]
pub struct FSTP6_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP6_W<'a> {
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
#[doc = "Reader of field `FSTP7`"]
pub type FSTP7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTP7`"]
pub struct FSTP7_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP7_W<'a> {
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
#[doc = "Reader of field `FSTP8`"]
pub type FSTP8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTP8`"]
pub struct FSTP8_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP8_W<'a> {
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
#[doc = "Reader of field `FSTP9`"]
pub type FSTP9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTP9`"]
pub struct FSTP9_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP9_W<'a> {
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
#[doc = "Reader of field `FSTP10`"]
pub type FSTP10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTP10`"]
pub struct FSTP10_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP10_W<'a> {
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
#[doc = "Reader of field `FSTP11`"]
pub type FSTP11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTP11`"]
pub struct FSTP11_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP11_W<'a> {
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
#[doc = "Reader of field `FSTP12`"]
pub type FSTP12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTP12`"]
pub struct FSTP12_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `FSTP13`"]
pub type FSTP13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTP13`"]
pub struct FSTP13_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `FSTP14`"]
pub type FSTP14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTP14`"]
pub struct FSTP14_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `FSTP15`"]
pub type FSTP15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTP15`"]
pub struct FSTP15_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTP15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Fast Startup Input Polarity 0"]
    #[inline(always)]
    pub fn fstp0(&self) -> FSTP0_R {
        FSTP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast Startup Input Polarity 1"]
    #[inline(always)]
    pub fn fstp1(&self) -> FSTP1_R {
        FSTP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast Startup Input Polarity 2"]
    #[inline(always)]
    pub fn fstp2(&self) -> FSTP2_R {
        FSTP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fast Startup Input Polarity 3"]
    #[inline(always)]
    pub fn fstp3(&self) -> FSTP3_R {
        FSTP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fast Startup Input Polarity 4"]
    #[inline(always)]
    pub fn fstp4(&self) -> FSTP4_R {
        FSTP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fast Startup Input Polarity 5"]
    #[inline(always)]
    pub fn fstp5(&self) -> FSTP5_R {
        FSTP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast Startup Input Polarity 6"]
    #[inline(always)]
    pub fn fstp6(&self) -> FSTP6_R {
        FSTP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast Startup Input Polarity 7"]
    #[inline(always)]
    pub fn fstp7(&self) -> FSTP7_R {
        FSTP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast Startup Input Polarity 8"]
    #[inline(always)]
    pub fn fstp8(&self) -> FSTP8_R {
        FSTP8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast Startup Input Polarity 9"]
    #[inline(always)]
    pub fn fstp9(&self) -> FSTP9_R {
        FSTP9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast Startup Input Polarity 10"]
    #[inline(always)]
    pub fn fstp10(&self) -> FSTP10_R {
        FSTP10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fast Startup Input Polarity 11"]
    #[inline(always)]
    pub fn fstp11(&self) -> FSTP11_R {
        FSTP11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast Startup Input Polarity 12"]
    #[inline(always)]
    pub fn fstp12(&self) -> FSTP12_R {
        FSTP12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast Startup Input Polarity 13"]
    #[inline(always)]
    pub fn fstp13(&self) -> FSTP13_R {
        FSTP13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast Startup Input Polarity 14"]
    #[inline(always)]
    pub fn fstp14(&self) -> FSTP14_R {
        FSTP14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast Startup Input Polarity 15"]
    #[inline(always)]
    pub fn fstp15(&self) -> FSTP15_R {
        FSTP15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast Startup Input Polarity 0"]
    #[inline(always)]
    pub fn fstp0(&mut self) -> FSTP0_W {
        FSTP0_W { w: self }
    }
    #[doc = "Bit 1 - Fast Startup Input Polarity 1"]
    #[inline(always)]
    pub fn fstp1(&mut self) -> FSTP1_W {
        FSTP1_W { w: self }
    }
    #[doc = "Bit 2 - Fast Startup Input Polarity 2"]
    #[inline(always)]
    pub fn fstp2(&mut self) -> FSTP2_W {
        FSTP2_W { w: self }
    }
    #[doc = "Bit 3 - Fast Startup Input Polarity 3"]
    #[inline(always)]
    pub fn fstp3(&mut self) -> FSTP3_W {
        FSTP3_W { w: self }
    }
    #[doc = "Bit 4 - Fast Startup Input Polarity 4"]
    #[inline(always)]
    pub fn fstp4(&mut self) -> FSTP4_W {
        FSTP4_W { w: self }
    }
    #[doc = "Bit 5 - Fast Startup Input Polarity 5"]
    #[inline(always)]
    pub fn fstp5(&mut self) -> FSTP5_W {
        FSTP5_W { w: self }
    }
    #[doc = "Bit 6 - Fast Startup Input Polarity 6"]
    #[inline(always)]
    pub fn fstp6(&mut self) -> FSTP6_W {
        FSTP6_W { w: self }
    }
    #[doc = "Bit 7 - Fast Startup Input Polarity 7"]
    #[inline(always)]
    pub fn fstp7(&mut self) -> FSTP7_W {
        FSTP7_W { w: self }
    }
    #[doc = "Bit 8 - Fast Startup Input Polarity 8"]
    #[inline(always)]
    pub fn fstp8(&mut self) -> FSTP8_W {
        FSTP8_W { w: self }
    }
    #[doc = "Bit 9 - Fast Startup Input Polarity 9"]
    #[inline(always)]
    pub fn fstp9(&mut self) -> FSTP9_W {
        FSTP9_W { w: self }
    }
    #[doc = "Bit 10 - Fast Startup Input Polarity 10"]
    #[inline(always)]
    pub fn fstp10(&mut self) -> FSTP10_W {
        FSTP10_W { w: self }
    }
    #[doc = "Bit 11 - Fast Startup Input Polarity 11"]
    #[inline(always)]
    pub fn fstp11(&mut self) -> FSTP11_W {
        FSTP11_W { w: self }
    }
    #[doc = "Bit 12 - Fast Startup Input Polarity 12"]
    #[inline(always)]
    pub fn fstp12(&mut self) -> FSTP12_W {
        FSTP12_W { w: self }
    }
    #[doc = "Bit 13 - Fast Startup Input Polarity 13"]
    #[inline(always)]
    pub fn fstp13(&mut self) -> FSTP13_W {
        FSTP13_W { w: self }
    }
    #[doc = "Bit 14 - Fast Startup Input Polarity 14"]
    #[inline(always)]
    pub fn fstp14(&mut self) -> FSTP14_W {
        FSTP14_W { w: self }
    }
    #[doc = "Bit 15 - Fast Startup Input Polarity 15"]
    #[inline(always)]
    pub fn fstp15(&mut self) -> FSTP15_W {
        FSTP15_W { w: self }
    }
}
