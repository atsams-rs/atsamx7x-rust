#[doc = "Writer for register PWM_CMUPD3"]
pub type W = crate::W<u32, super::PWM_CMUPD3>;
#[doc = "Register PWM_CMUPD3 `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_CMUPD3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CPOLUP`"]
pub struct CPOLUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOLUP_W<'a> {
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
#[doc = "Write proxy for field `CPOLINVUP`"]
pub struct CPOLINVUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOLINVUP_W<'a> {
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
impl W {
    #[doc = "Bit 9 - Channel Polarity Update"]
    #[inline(always)]
    pub fn cpolup(&mut self) -> CPOLUP_W {
        CPOLUP_W { w: self }
    }
    #[doc = "Bit 13 - Channel Polarity Inversion Update"]
    #[inline(always)]
    pub fn cpolinvup(&mut self) -> CPOLINVUP_W {
        CPOLINVUP_W { w: self }
    }
}
