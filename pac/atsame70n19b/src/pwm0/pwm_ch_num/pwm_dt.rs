#[doc = "Reader of register PWM_DT"]
pub type R = crate::R<u32, super::PWM_DT>;
#[doc = "Writer for register PWM_DT"]
pub type W = crate::W<u32, super::PWM_DT>;
#[doc = "Register PWM_DT `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_DT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DTH`"]
pub type DTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DTH`"]
pub struct DTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `DTL`"]
pub type DTL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DTL`"]
pub struct DTL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Dead-Time Value for PWMHx Output"]
    #[inline(always)]
    pub fn dth(&self) -> DTH_R {
        DTH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Dead-Time Value for PWMLx Output"]
    #[inline(always)]
    pub fn dtl(&self) -> DTL_R {
        DTL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Dead-Time Value for PWMHx Output"]
    #[inline(always)]
    pub fn dth(&mut self) -> DTH_W {
        DTH_W { w: self }
    }
    #[doc = "Bits 16:31 - Dead-Time Value for PWMLx Output"]
    #[inline(always)]
    pub fn dtl(&mut self) -> DTL_W {
        DTL_W { w: self }
    }
}
