#[doc = "Writer for register PWM_IDR2"]
pub type W = crate::W<u32, super::PWM_IDR2>;
#[doc = "Register PWM_IDR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_IDR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `WRDY`"]
pub struct WRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> WRDY_W<'a> {
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
#[doc = "Write proxy for field `UNRE`"]
pub struct UNRE_W<'a> {
    w: &'a mut W,
}
impl<'a> UNRE_W<'a> {
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
#[doc = "Write proxy for field `CMPM0`"]
pub struct CMPM0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPM0_W<'a> {
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
#[doc = "Write proxy for field `CMPM1`"]
pub struct CMPM1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPM1_W<'a> {
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
#[doc = "Write proxy for field `CMPM2`"]
pub struct CMPM2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPM2_W<'a> {
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
#[doc = "Write proxy for field `CMPM3`"]
pub struct CMPM3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPM3_W<'a> {
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
#[doc = "Write proxy for field `CMPM4`"]
pub struct CMPM4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPM4_W<'a> {
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
#[doc = "Write proxy for field `CMPM5`"]
pub struct CMPM5_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPM5_W<'a> {
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
#[doc = "Write proxy for field `CMPM6`"]
pub struct CMPM6_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPM6_W<'a> {
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
#[doc = "Write proxy for field `CMPM7`"]
pub struct CMPM7_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPM7_W<'a> {
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
#[doc = "Write proxy for field `CMPU0`"]
pub struct CMPU0_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPU0_W<'a> {
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
#[doc = "Write proxy for field `CMPU1`"]
pub struct CMPU1_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPU1_W<'a> {
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
#[doc = "Write proxy for field `CMPU2`"]
pub struct CMPU2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPU2_W<'a> {
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
#[doc = "Write proxy for field `CMPU3`"]
pub struct CMPU3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPU3_W<'a> {
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
#[doc = "Write proxy for field `CMPU4`"]
pub struct CMPU4_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPU4_W<'a> {
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
#[doc = "Write proxy for field `CMPU5`"]
pub struct CMPU5_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPU5_W<'a> {
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
#[doc = "Write proxy for field `CMPU6`"]
pub struct CMPU6_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPU6_W<'a> {
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
#[doc = "Write proxy for field `CMPU7`"]
pub struct CMPU7_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPU7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Write Ready for Synchronous Channels Update Interrupt Disable"]
    #[inline(always)]
    pub fn wrdy(&mut self) -> WRDY_W {
        WRDY_W { w: self }
    }
    #[doc = "Bit 3 - Synchronous Channels Update Underrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn unre(&mut self) -> UNRE_W {
        UNRE_W { w: self }
    }
    #[doc = "Bit 8 - Comparison 0 Match Interrupt Disable"]
    #[inline(always)]
    pub fn cmpm0(&mut self) -> CMPM0_W {
        CMPM0_W { w: self }
    }
    #[doc = "Bit 9 - Comparison 1 Match Interrupt Disable"]
    #[inline(always)]
    pub fn cmpm1(&mut self) -> CMPM1_W {
        CMPM1_W { w: self }
    }
    #[doc = "Bit 10 - Comparison 2 Match Interrupt Disable"]
    #[inline(always)]
    pub fn cmpm2(&mut self) -> CMPM2_W {
        CMPM2_W { w: self }
    }
    #[doc = "Bit 11 - Comparison 3 Match Interrupt Disable"]
    #[inline(always)]
    pub fn cmpm3(&mut self) -> CMPM3_W {
        CMPM3_W { w: self }
    }
    #[doc = "Bit 12 - Comparison 4 Match Interrupt Disable"]
    #[inline(always)]
    pub fn cmpm4(&mut self) -> CMPM4_W {
        CMPM4_W { w: self }
    }
    #[doc = "Bit 13 - Comparison 5 Match Interrupt Disable"]
    #[inline(always)]
    pub fn cmpm5(&mut self) -> CMPM5_W {
        CMPM5_W { w: self }
    }
    #[doc = "Bit 14 - Comparison 6 Match Interrupt Disable"]
    #[inline(always)]
    pub fn cmpm6(&mut self) -> CMPM6_W {
        CMPM6_W { w: self }
    }
    #[doc = "Bit 15 - Comparison 7 Match Interrupt Disable"]
    #[inline(always)]
    pub fn cmpm7(&mut self) -> CMPM7_W {
        CMPM7_W { w: self }
    }
    #[doc = "Bit 16 - Comparison 0 Update Interrupt Disable"]
    #[inline(always)]
    pub fn cmpu0(&mut self) -> CMPU0_W {
        CMPU0_W { w: self }
    }
    #[doc = "Bit 17 - Comparison 1 Update Interrupt Disable"]
    #[inline(always)]
    pub fn cmpu1(&mut self) -> CMPU1_W {
        CMPU1_W { w: self }
    }
    #[doc = "Bit 18 - Comparison 2 Update Interrupt Disable"]
    #[inline(always)]
    pub fn cmpu2(&mut self) -> CMPU2_W {
        CMPU2_W { w: self }
    }
    #[doc = "Bit 19 - Comparison 3 Update Interrupt Disable"]
    #[inline(always)]
    pub fn cmpu3(&mut self) -> CMPU3_W {
        CMPU3_W { w: self }
    }
    #[doc = "Bit 20 - Comparison 4 Update Interrupt Disable"]
    #[inline(always)]
    pub fn cmpu4(&mut self) -> CMPU4_W {
        CMPU4_W { w: self }
    }
    #[doc = "Bit 21 - Comparison 5 Update Interrupt Disable"]
    #[inline(always)]
    pub fn cmpu5(&mut self) -> CMPU5_W {
        CMPU5_W { w: self }
    }
    #[doc = "Bit 22 - Comparison 6 Update Interrupt Disable"]
    #[inline(always)]
    pub fn cmpu6(&mut self) -> CMPU6_W {
        CMPU6_W { w: self }
    }
    #[doc = "Bit 23 - Comparison 7 Update Interrupt Disable"]
    #[inline(always)]
    pub fn cmpu7(&mut self) -> CMPU7_W {
        CMPU7_W { w: self }
    }
}
