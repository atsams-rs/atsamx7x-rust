#[doc = "Reader of register TC_RB"]
pub type R = crate::R<u32, super::TC_RB>;
#[doc = "Writer for register TC_RB"]
pub type W = crate::W<u32, super::TC_RB>;
#[doc = "Register TC_RB `reset()`'s with value 0"]
impl crate::ResetValue for super::TC_RB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RB`"]
pub type RB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RB`"]
pub struct RB_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Register B"]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register B"]
    #[inline(always)]
    pub fn rb(&mut self) -> RB_W {
        RB_W { w: self }
    }
}
