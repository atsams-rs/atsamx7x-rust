#[doc = "Reader of register MATRIX_PRBS"]
pub type R = crate::R<u32, super::MATRIX_PRBS>;
#[doc = "Writer for register MATRIX_PRBS"]
pub type W = crate::W<u32, super::MATRIX_PRBS>;
#[doc = "Register MATRIX_PRBS `reset()`'s with value 0"]
impl crate::ResetValue for super::MATRIX_PRBS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `M8PR`"]
pub type M8PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M8PR`"]
pub struct M8PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M8PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `M9PR`"]
pub type M9PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M9PR`"]
pub struct M9PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M9PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `M10PR`"]
pub type M10PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M10PR`"]
pub struct M10PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M10PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `M11PR`"]
pub type M11PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M11PR`"]
pub struct M11PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M11PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `M12PR`"]
pub type M12PR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `M12PR`"]
pub struct M12PR_W<'a> {
    w: &'a mut W,
}
impl<'a> M12PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Master 8 Priority"]
    #[inline(always)]
    pub fn m8pr(&self) -> M8PR_R {
        M8PR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Master 9 Priority"]
    #[inline(always)]
    pub fn m9pr(&self) -> M9PR_R {
        M9PR_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Master 10 Priority"]
    #[inline(always)]
    pub fn m10pr(&self) -> M10PR_R {
        M10PR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Master 11 Priority"]
    #[inline(always)]
    pub fn m11pr(&self) -> M11PR_R {
        M11PR_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Master 12 Priority"]
    #[inline(always)]
    pub fn m12pr(&self) -> M12PR_R {
        M12PR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master 8 Priority"]
    #[inline(always)]
    pub fn m8pr(&mut self) -> M8PR_W {
        M8PR_W { w: self }
    }
    #[doc = "Bits 4:5 - Master 9 Priority"]
    #[inline(always)]
    pub fn m9pr(&mut self) -> M9PR_W {
        M9PR_W { w: self }
    }
    #[doc = "Bits 8:9 - Master 10 Priority"]
    #[inline(always)]
    pub fn m10pr(&mut self) -> M10PR_W {
        M10PR_W { w: self }
    }
    #[doc = "Bits 12:13 - Master 11 Priority"]
    #[inline(always)]
    pub fn m11pr(&mut self) -> M11PR_W {
        M11PR_W { w: self }
    }
    #[doc = "Bits 16:17 - Master 12 Priority"]
    #[inline(always)]
    pub fn m12pr(&mut self) -> M12PR_W {
        M12PR_W { w: self }
    }
}
