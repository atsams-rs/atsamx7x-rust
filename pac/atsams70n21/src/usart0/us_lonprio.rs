#[doc = "Reader of register US_LONPRIO"]
pub type R = crate::R<u32, super::US_LONPRIO>;
#[doc = "Writer for register US_LONPRIO"]
pub type W = crate::W<u32, super::US_LONPRIO>;
#[doc = "Register US_LONPRIO `reset()`'s with value 0"]
impl crate::ResetValue for super::US_LONPRIO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PSNB`"]
pub type PSNB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSNB`"]
pub struct PSNB_W<'a> {
    w: &'a mut W,
}
impl<'a> PSNB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `NPS`"]
pub type NPS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NPS`"]
pub struct NPS_W<'a> {
    w: &'a mut W,
}
impl<'a> NPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - LON Priority Slot Number"]
    #[inline(always)]
    pub fn psnb(&self) -> PSNB_R {
        PSNB_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - LON Node Priority Slot"]
    #[inline(always)]
    pub fn nps(&self) -> NPS_R {
        NPS_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - LON Priority Slot Number"]
    #[inline(always)]
    pub fn psnb(&mut self) -> PSNB_W {
        PSNB_W { w: self }
    }
    #[doc = "Bits 8:14 - LON Node Priority Slot"]
    #[inline(always)]
    pub fn nps(&mut self) -> NPS_W {
        NPS_W { w: self }
    }
}
