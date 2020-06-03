#[doc = "Reader of register PWM_FPV2"]
pub type R = crate::R<u32, super::PWM_FPV2>;
#[doc = "Writer for register PWM_FPV2"]
pub type W = crate::W<u32, super::PWM_FPV2>;
#[doc = "Register PWM_FPV2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_FPV2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FPZH0`"]
pub type FPZH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPZH0`"]
pub struct FPZH0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPZH0_W<'a> {
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
#[doc = "Reader of field `FPZH1`"]
pub type FPZH1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPZH1`"]
pub struct FPZH1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPZH1_W<'a> {
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
#[doc = "Reader of field `FPZH2`"]
pub type FPZH2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPZH2`"]
pub struct FPZH2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPZH2_W<'a> {
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
#[doc = "Reader of field `FPZH3`"]
pub type FPZH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPZH3`"]
pub struct FPZH3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPZH3_W<'a> {
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
#[doc = "Reader of field `FPZL0`"]
pub type FPZL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPZL0`"]
pub struct FPZL0_W<'a> {
    w: &'a mut W,
}
impl<'a> FPZL0_W<'a> {
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
#[doc = "Reader of field `FPZL1`"]
pub type FPZL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPZL1`"]
pub struct FPZL1_W<'a> {
    w: &'a mut W,
}
impl<'a> FPZL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `FPZL2`"]
pub type FPZL2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPZL2`"]
pub struct FPZL2_W<'a> {
    w: &'a mut W,
}
impl<'a> FPZL2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `FPZL3`"]
pub type FPZL3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPZL3`"]
pub struct FPZL3_W<'a> {
    w: &'a mut W,
}
impl<'a> FPZL3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Fault Protection to Hi-Z for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpzh0(&self) -> FPZH0_R {
        FPZH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault Protection to Hi-Z for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpzh1(&self) -> FPZH1_R {
        FPZH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault Protection to Hi-Z for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpzh2(&self) -> FPZH2_R {
        FPZH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault Protection to Hi-Z for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpzh3(&self) -> FPZH3_R {
        FPZH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Fault Protection to Hi-Z for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpzl0(&self) -> FPZL0_R {
        FPZL0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Fault Protection to Hi-Z for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpzl1(&self) -> FPZL1_R {
        FPZL1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Fault Protection to Hi-Z for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpzl2(&self) -> FPZL2_R {
        FPZL2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Fault Protection to Hi-Z for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpzl3(&self) -> FPZL3_R {
        FPZL3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Protection to Hi-Z for PWMH output on channel 0"]
    #[inline(always)]
    pub fn fpzh0(&mut self) -> FPZH0_W {
        FPZH0_W { w: self }
    }
    #[doc = "Bit 1 - Fault Protection to Hi-Z for PWMH output on channel 1"]
    #[inline(always)]
    pub fn fpzh1(&mut self) -> FPZH1_W {
        FPZH1_W { w: self }
    }
    #[doc = "Bit 2 - Fault Protection to Hi-Z for PWMH output on channel 2"]
    #[inline(always)]
    pub fn fpzh2(&mut self) -> FPZH2_W {
        FPZH2_W { w: self }
    }
    #[doc = "Bit 3 - Fault Protection to Hi-Z for PWMH output on channel 3"]
    #[inline(always)]
    pub fn fpzh3(&mut self) -> FPZH3_W {
        FPZH3_W { w: self }
    }
    #[doc = "Bit 16 - Fault Protection to Hi-Z for PWML output on channel 0"]
    #[inline(always)]
    pub fn fpzl0(&mut self) -> FPZL0_W {
        FPZL0_W { w: self }
    }
    #[doc = "Bit 17 - Fault Protection to Hi-Z for PWML output on channel 1"]
    #[inline(always)]
    pub fn fpzl1(&mut self) -> FPZL1_W {
        FPZL1_W { w: self }
    }
    #[doc = "Bit 18 - Fault Protection to Hi-Z for PWML output on channel 2"]
    #[inline(always)]
    pub fn fpzl2(&mut self) -> FPZL2_W {
        FPZL2_W { w: self }
    }
    #[doc = "Bit 19 - Fault Protection to Hi-Z for PWML output on channel 3"]
    #[inline(always)]
    pub fn fpzl3(&mut self) -> FPZL3_W {
        FPZL3_W { w: self }
    }
}
