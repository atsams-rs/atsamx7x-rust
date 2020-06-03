#[doc = "Writer for register USBHS_SCR"]
pub type W = crate::W<u32, super::USBHS_SCR>;
#[doc = "Register USBHS_SCR `reset()`'s with value 0"]
impl crate::ResetValue for super::USBHS_SCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `RDERRIC`"]
pub struct RDERRIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RDERRIC_W<'a> {
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
impl W {
    #[doc = "Bit 4 - Remote Device Connection Error Interrupt Clear"]
    #[inline(always)]
    pub fn rderric(&mut self) -> RDERRIC_W {
        RDERRIC_W { w: self }
    }
}
