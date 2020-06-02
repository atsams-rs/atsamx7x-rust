#[doc = "Reader of register ISI_PSIZE"]
pub type R = crate::R<u32, super::ISI_PSIZE>;
#[doc = "Writer for register ISI_PSIZE"]
pub type W = crate::W<u32, super::ISI_PSIZE>;
#[doc = "Register ISI_PSIZE `reset()`'s with value 0"]
impl crate::ResetValue for super::ISI_PSIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PREV_VSIZE`"]
pub type PREV_VSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PREV_VSIZE`"]
pub struct PREV_VSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PREV_VSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `PREV_HSIZE`"]
pub type PREV_HSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PREV_HSIZE`"]
pub struct PREV_HSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PREV_HSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Vertical Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_vsize(&self) -> PREV_VSIZE_R {
        PREV_VSIZE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Horizontal Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_hsize(&self) -> PREV_HSIZE_R {
        PREV_HSIZE_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Vertical Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_vsize(&mut self) -> PREV_VSIZE_W {
        PREV_VSIZE_W { w: self }
    }
    #[doc = "Bits 16:25 - Horizontal Size for the Preview Path"]
    #[inline(always)]
    pub fn prev_hsize(&mut self) -> PREV_HSIZE_W {
        PREV_HSIZE_W { w: self }
    }
}
