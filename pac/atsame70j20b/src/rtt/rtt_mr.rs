#[doc = "Reader of register RTT_MR"]
pub type R = crate::R<u32, super::RTT_MR>;
#[doc = "Writer for register RTT_MR"]
pub type W = crate::W<u32, super::RTT_MR>;
#[doc = "Register RTT_MR `reset()`'s with value 0"]
impl crate::ResetValue for super::RTT_MR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTPRES`"]
pub type RTPRES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTPRES`"]
pub struct RTPRES_W<'a> {
    w: &'a mut W,
}
impl<'a> RTPRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ALMIEN`"]
pub type ALMIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALMIEN`"]
pub struct ALMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMIEN_W<'a> {
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
#[doc = "Reader of field `RTTINCIEN`"]
pub type RTTINCIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTTINCIEN`"]
pub struct RTTINCIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTTINCIEN_W<'a> {
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
#[doc = "Reader of field `RTTRST`"]
pub type RTTRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTTRST`"]
pub struct RTTRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RTTRST_W<'a> {
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
#[doc = "Reader of field `RTTDIS`"]
pub type RTTDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTTDIS`"]
pub struct RTTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTTDIS_W<'a> {
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
#[doc = "Reader of field `RTC1HZ`"]
pub type RTC1HZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC1HZ`"]
pub struct RTC1HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC1HZ_W<'a> {
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
impl R {
    #[doc = "Bits 0:15 - Real-time Timer Prescaler Value"]
    #[inline(always)]
    pub fn rtpres(&self) -> RTPRES_R {
        RTPRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn almien(&self) -> ALMIEN_R {
        ALMIEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Real-time Timer Increment Interrupt Enable"]
    #[inline(always)]
    pub fn rttincien(&self) -> RTTINCIEN_R {
        RTTINCIEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Real-time Timer Restart"]
    #[inline(always)]
    pub fn rttrst(&self) -> RTTRST_R {
        RTTRST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Real-time Timer Disable"]
    #[inline(always)]
    pub fn rttdis(&self) -> RTTDIS_R {
        RTTDIS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Real-Time Clock 1Hz Clock Selection"]
    #[inline(always)]
    pub fn rtc1hz(&self) -> RTC1HZ_R {
        RTC1HZ_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Real-time Timer Prescaler Value"]
    #[inline(always)]
    pub fn rtpres(&mut self) -> RTPRES_W {
        RTPRES_W { w: self }
    }
    #[doc = "Bit 16 - Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn almien(&mut self) -> ALMIEN_W {
        ALMIEN_W { w: self }
    }
    #[doc = "Bit 17 - Real-time Timer Increment Interrupt Enable"]
    #[inline(always)]
    pub fn rttincien(&mut self) -> RTTINCIEN_W {
        RTTINCIEN_W { w: self }
    }
    #[doc = "Bit 18 - Real-time Timer Restart"]
    #[inline(always)]
    pub fn rttrst(&mut self) -> RTTRST_W {
        RTTRST_W { w: self }
    }
    #[doc = "Bit 20 - Real-time Timer Disable"]
    #[inline(always)]
    pub fn rttdis(&mut self) -> RTTDIS_W {
        RTTDIS_W { w: self }
    }
    #[doc = "Bit 24 - Real-Time Clock 1Hz Clock Selection"]
    #[inline(always)]
    pub fn rtc1hz(&mut self) -> RTC1HZ_W {
        RTC1HZ_W { w: self }
    }
}
