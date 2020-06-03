#[doc = "Reader of register US_LONDL"]
pub type R = crate::R<u32, super::US_LONDL>;
#[doc = "Writer for register US_LONDL"]
pub type W = crate::W<u32, super::US_LONDL>;
#[doc = "Register US_LONDL `reset()`'s with value 0"]
impl crate::ResetValue for super::US_LONDL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LONDL`"]
pub type LONDL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LONDL`"]
pub struct LONDL_W<'a> {
    w: &'a mut W,
}
impl<'a> LONDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - LON Data Length"]
    #[inline(always)]
    pub fn londl(&self) -> LONDL_R {
        LONDL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LON Data Length"]
    #[inline(always)]
    pub fn londl(&mut self) -> LONDL_W {
        LONDL_W { w: self }
    }
}
