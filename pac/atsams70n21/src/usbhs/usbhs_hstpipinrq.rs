#[doc = "Reader of register USBHS_HSTPIPINRQ[%s]"]
pub type R = crate::R<u32, super::USBHS_HSTPIPINRQ>;
#[doc = "Writer for register USBHS_HSTPIPINRQ[%s]"]
pub type W = crate::W<u32, super::USBHS_HSTPIPINRQ>;
#[doc = "Register USBHS_HSTPIPINRQ[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::USBHS_HSTPIPINRQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INRQ`"]
pub type INRQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INRQ`"]
pub struct INRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> INRQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `INMODE`"]
pub type INMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INMODE`"]
pub struct INMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> INMODE_W<'a> {
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
impl R {
    #[doc = "Bits 0:7 - IN Request Number before Freeze"]
    #[inline(always)]
    pub fn inrq(&self) -> INRQ_R {
        INRQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - IN Request Mode"]
    #[inline(always)]
    pub fn inmode(&self) -> INMODE_R {
        INMODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - IN Request Number before Freeze"]
    #[inline(always)]
    pub fn inrq(&mut self) -> INRQ_W {
        INRQ_W { w: self }
    }
    #[doc = "Bit 8 - IN Request Mode"]
    #[inline(always)]
    pub fn inmode(&mut self) -> INMODE_W {
        INMODE_W { w: self }
    }
}
