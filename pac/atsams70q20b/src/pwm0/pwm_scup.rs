#[doc = "Reader of register PWM_SCUP"]
pub type R = crate::R<u32, super::PWM_SCUP>;
#[doc = "Writer for register PWM_SCUP"]
pub type W = crate::W<u32, super::PWM_SCUP>;
#[doc = "Register PWM_SCUP `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_SCUP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UPR`"]
pub type UPR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UPR`"]
pub struct UPR_W<'a> {
    w: &'a mut W,
}
impl<'a> UPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `UPRCNT`"]
pub type UPRCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UPRCNT`"]
pub struct UPRCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> UPRCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Update Period"]
    #[inline(always)]
    pub fn upr(&self) -> UPR_R {
        UPR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Update Period Counter"]
    #[inline(always)]
    pub fn uprcnt(&self) -> UPRCNT_R {
        UPRCNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Update Period"]
    #[inline(always)]
    pub fn upr(&mut self) -> UPR_W {
        UPR_W { w: self }
    }
    #[doc = "Bits 4:7 - Update Period Counter"]
    #[inline(always)]
    pub fn uprcnt(&mut self) -> UPRCNT_W {
        UPRCNT_W { w: self }
    }
}
