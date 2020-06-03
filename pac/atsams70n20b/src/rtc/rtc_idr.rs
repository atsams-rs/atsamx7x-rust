#[doc = "Writer for register RTC_IDR"]
pub type W = crate::W<u32, super::RTC_IDR>;
#[doc = "Register RTC_IDR `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_IDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ACKDIS`"]
pub struct ACKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKDIS_W<'a> {
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
#[doc = "Write proxy for field `ALRDIS`"]
pub struct ALRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRDIS_W<'a> {
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
#[doc = "Write proxy for field `SECDIS`"]
pub struct SECDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SECDIS_W<'a> {
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
#[doc = "Write proxy for field `TIMDIS`"]
pub struct TIMDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMDIS_W<'a> {
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
#[doc = "Write proxy for field `CALDIS`"]
pub struct CALDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CALDIS_W<'a> {
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
#[doc = "Write proxy for field `TDERRDIS`"]
pub struct TDERRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TDERRDIS_W<'a> {
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
    #[doc = "Bit 0 - Acknowledge Update Interrupt Disable"]
    #[inline(always)]
    pub fn ackdis(&mut self) -> ACKDIS_W {
        ACKDIS_W { w: self }
    }
    #[doc = "Bit 1 - Alarm Interrupt Disable"]
    #[inline(always)]
    pub fn alrdis(&mut self) -> ALRDIS_W {
        ALRDIS_W { w: self }
    }
    #[doc = "Bit 2 - Second Event Interrupt Disable"]
    #[inline(always)]
    pub fn secdis(&mut self) -> SECDIS_W {
        SECDIS_W { w: self }
    }
    #[doc = "Bit 3 - Time Event Interrupt Disable"]
    #[inline(always)]
    pub fn timdis(&mut self) -> TIMDIS_W {
        TIMDIS_W { w: self }
    }
    #[doc = "Bit 4 - Calendar Event Interrupt Disable"]
    #[inline(always)]
    pub fn caldis(&mut self) -> CALDIS_W {
        CALDIS_W { w: self }
    }
    #[doc = "Bit 5 - Time and/or Date Error Interrupt Disable"]
    #[inline(always)]
    pub fn tderrdis(&mut self) -> TDERRDIS_W {
        TDERRDIS_W { w: self }
    }
}
