#[doc = "Writer for register RTC_SCCR"]
pub type W = crate::W<u32, super::RTC_SCCR>;
#[doc = "Register RTC_SCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_SCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ACKCLR`"]
pub struct ACKCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKCLR_W<'a> {
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
#[doc = "Write proxy for field `ALRCLR`"]
pub struct ALRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRCLR_W<'a> {
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
#[doc = "Write proxy for field `SECCLR`"]
pub struct SECCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SECCLR_W<'a> {
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
#[doc = "Write proxy for field `TIMCLR`"]
pub struct TIMCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMCLR_W<'a> {
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
#[doc = "Write proxy for field `CALCLR`"]
pub struct CALCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CALCLR_W<'a> {
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
#[doc = "Write proxy for field `TDERRCLR`"]
pub struct TDERRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TDERRCLR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Acknowledge Clear"]
    #[inline(always)]
    pub fn ackclr(&mut self) -> ACKCLR_W {
        ACKCLR_W { w: self }
    }
    #[doc = "Bit 1 - Alarm Clear"]
    #[inline(always)]
    pub fn alrclr(&mut self) -> ALRCLR_W {
        ALRCLR_W { w: self }
    }
    #[doc = "Bit 2 - Second Clear"]
    #[inline(always)]
    pub fn secclr(&mut self) -> SECCLR_W {
        SECCLR_W { w: self }
    }
    #[doc = "Bit 3 - Time Clear"]
    #[inline(always)]
    pub fn timclr(&mut self) -> TIMCLR_W {
        TIMCLR_W { w: self }
    }
    #[doc = "Bit 4 - Calendar Clear"]
    #[inline(always)]
    pub fn calclr(&mut self) -> CALCLR_W {
        CALCLR_W { w: self }
    }
    #[doc = "Bit 5 - Time and/or Date Free Running Error Clear"]
    #[inline(always)]
    pub fn tderrclr(&mut self) -> TDERRCLR_W {
        TDERRCLR_W { w: self }
    }
}
