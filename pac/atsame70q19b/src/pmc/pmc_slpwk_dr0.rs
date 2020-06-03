#[doc = "Writer for register PMC_SLPWK_DR0"]
pub type W = crate::W<u32, super::PMC_SLPWK_DR0>;
#[doc = "Register PMC_SLPWK_DR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PMC_SLPWK_DR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `PID7`"]
pub struct PID7_W<'a> {
    w: &'a mut W,
}
impl<'a> PID7_W<'a> {
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
#[doc = "Write proxy for field `PID8`"]
pub struct PID8_W<'a> {
    w: &'a mut W,
}
impl<'a> PID8_W<'a> {
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
#[doc = "Write proxy for field `PID9`"]
pub struct PID9_W<'a> {
    w: &'a mut W,
}
impl<'a> PID9_W<'a> {
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
#[doc = "Write proxy for field `PID10`"]
pub struct PID10_W<'a> {
    w: &'a mut W,
}
impl<'a> PID10_W<'a> {
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
#[doc = "Write proxy for field `PID11`"]
pub struct PID11_W<'a> {
    w: &'a mut W,
}
impl<'a> PID11_W<'a> {
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
#[doc = "Write proxy for field `PID12`"]
pub struct PID12_W<'a> {
    w: &'a mut W,
}
impl<'a> PID12_W<'a> {
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
#[doc = "Write proxy for field `PID13`"]
pub struct PID13_W<'a> {
    w: &'a mut W,
}
impl<'a> PID13_W<'a> {
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
#[doc = "Write proxy for field `PID14`"]
pub struct PID14_W<'a> {
    w: &'a mut W,
}
impl<'a> PID14_W<'a> {
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
#[doc = "Write proxy for field `PID15`"]
pub struct PID15_W<'a> {
    w: &'a mut W,
}
impl<'a> PID15_W<'a> {
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
#[doc = "Write proxy for field `PID16`"]
pub struct PID16_W<'a> {
    w: &'a mut W,
}
impl<'a> PID16_W<'a> {
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
#[doc = "Write proxy for field `PID17`"]
pub struct PID17_W<'a> {
    w: &'a mut W,
}
impl<'a> PID17_W<'a> {
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
#[doc = "Write proxy for field `PID18`"]
pub struct PID18_W<'a> {
    w: &'a mut W,
}
impl<'a> PID18_W<'a> {
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
#[doc = "Write proxy for field `PID19`"]
pub struct PID19_W<'a> {
    w: &'a mut W,
}
impl<'a> PID19_W<'a> {
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
#[doc = "Write proxy for field `PID20`"]
pub struct PID20_W<'a> {
    w: &'a mut W,
}
impl<'a> PID20_W<'a> {
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
#[doc = "Write proxy for field `PID21`"]
pub struct PID21_W<'a> {
    w: &'a mut W,
}
impl<'a> PID21_W<'a> {
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
#[doc = "Write proxy for field `PID22`"]
pub struct PID22_W<'a> {
    w: &'a mut W,
}
impl<'a> PID22_W<'a> {
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
#[doc = "Write proxy for field `PID23`"]
pub struct PID23_W<'a> {
    w: &'a mut W,
}
impl<'a> PID23_W<'a> {
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
#[doc = "Write proxy for field `PID24`"]
pub struct PID24_W<'a> {
    w: &'a mut W,
}
impl<'a> PID24_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Write proxy for field `PID25`"]
pub struct PID25_W<'a> {
    w: &'a mut W,
}
impl<'a> PID25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Write proxy for field `PID26`"]
pub struct PID26_W<'a> {
    w: &'a mut W,
}
impl<'a> PID26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Write proxy for field `PID27`"]
pub struct PID27_W<'a> {
    w: &'a mut W,
}
impl<'a> PID27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Write proxy for field `PID28`"]
pub struct PID28_W<'a> {
    w: &'a mut W,
}
impl<'a> PID28_W<'a> {
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
#[doc = "Write proxy for field `PID29`"]
pub struct PID29_W<'a> {
    w: &'a mut W,
}
impl<'a> PID29_W<'a> {
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
#[doc = "Write proxy for field `PID30`"]
pub struct PID30_W<'a> {
    w: &'a mut W,
}
impl<'a> PID30_W<'a> {
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
#[doc = "Write proxy for field `PID31`"]
pub struct PID31_W<'a> {
    w: &'a mut W,
}
impl<'a> PID31_W<'a> {
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
impl W {
    #[doc = "Bit 7 - Peripheral 7 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid7(&mut self) -> PID7_W {
        PID7_W { w: self }
    }
    #[doc = "Bit 8 - Peripheral 8 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid8(&mut self) -> PID8_W {
        PID8_W { w: self }
    }
    #[doc = "Bit 9 - Peripheral 9 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid9(&mut self) -> PID9_W {
        PID9_W { w: self }
    }
    #[doc = "Bit 10 - Peripheral 10 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid10(&mut self) -> PID10_W {
        PID10_W { w: self }
    }
    #[doc = "Bit 11 - Peripheral 11 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid11(&mut self) -> PID11_W {
        PID11_W { w: self }
    }
    #[doc = "Bit 12 - Peripheral 12 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid12(&mut self) -> PID12_W {
        PID12_W { w: self }
    }
    #[doc = "Bit 13 - Peripheral 13 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid13(&mut self) -> PID13_W {
        PID13_W { w: self }
    }
    #[doc = "Bit 14 - Peripheral 14 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid14(&mut self) -> PID14_W {
        PID14_W { w: self }
    }
    #[doc = "Bit 15 - Peripheral 15 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid15(&mut self) -> PID15_W {
        PID15_W { w: self }
    }
    #[doc = "Bit 16 - Peripheral 16 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid16(&mut self) -> PID16_W {
        PID16_W { w: self }
    }
    #[doc = "Bit 17 - Peripheral 17 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid17(&mut self) -> PID17_W {
        PID17_W { w: self }
    }
    #[doc = "Bit 18 - Peripheral 18 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid18(&mut self) -> PID18_W {
        PID18_W { w: self }
    }
    #[doc = "Bit 19 - Peripheral 19 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid19(&mut self) -> PID19_W {
        PID19_W { w: self }
    }
    #[doc = "Bit 20 - Peripheral 20 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid20(&mut self) -> PID20_W {
        PID20_W { w: self }
    }
    #[doc = "Bit 21 - Peripheral 21 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid21(&mut self) -> PID21_W {
        PID21_W { w: self }
    }
    #[doc = "Bit 22 - Peripheral 22 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid22(&mut self) -> PID22_W {
        PID22_W { w: self }
    }
    #[doc = "Bit 23 - Peripheral 23 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid23(&mut self) -> PID23_W {
        PID23_W { w: self }
    }
    #[doc = "Bit 24 - Peripheral 24 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid24(&mut self) -> PID24_W {
        PID24_W { w: self }
    }
    #[doc = "Bit 25 - Peripheral 25 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid25(&mut self) -> PID25_W {
        PID25_W { w: self }
    }
    #[doc = "Bit 26 - Peripheral 26 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid26(&mut self) -> PID26_W {
        PID26_W { w: self }
    }
    #[doc = "Bit 27 - Peripheral 27 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid27(&mut self) -> PID27_W {
        PID27_W { w: self }
    }
    #[doc = "Bit 28 - Peripheral 28 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid28(&mut self) -> PID28_W {
        PID28_W { w: self }
    }
    #[doc = "Bit 29 - Peripheral 29 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid29(&mut self) -> PID29_W {
        PID29_W { w: self }
    }
    #[doc = "Bit 30 - Peripheral 30 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid30(&mut self) -> PID30_W {
        PID30_W { w: self }
    }
    #[doc = "Bit 31 - Peripheral 31 SleepWalking Disable"]
    #[inline(always)]
    pub fn pid31(&mut self) -> PID31_W {
        PID31_W { w: self }
    }
}
