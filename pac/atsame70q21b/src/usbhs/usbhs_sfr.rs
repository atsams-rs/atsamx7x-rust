#[doc = "Writer for register USBHS_SFR"]
pub type W = crate::W<u32, super::USBHS_SFR>;
#[doc = "Register USBHS_SFR `reset()`'s with value 0"]
impl crate::ResetValue for super::USBHS_SFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RDERRIS`"]
pub struct RDERRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RDERRIS_W<'a> {
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
#[doc = "Write proxy for field `VBUSRQS`"]
pub struct VBUSRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSRQS_W<'a> {
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
impl W {
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt Set"]
    #[inline(always)]
    pub fn rderris(&mut self) -> RDERRIS_W {
        RDERRIS_W { w: self }
    }
    #[doc = "Bit 9 - VBUS Request Set"]
    #[inline(always)]
    pub fn vbusrqs(&mut self) -> VBUSRQS_W {
        VBUSRQS_W { w: self }
    }
}
