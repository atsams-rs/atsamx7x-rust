#[doc = "Reader of register RSTC_MR"]
pub type R = crate::R<u32, super::RSTC_MR>;
#[doc = "Writer for register RSTC_MR"]
pub type W = crate::W<u32, super::RSTC_MR>;
#[doc = "Register RSTC_MR `reset()`'s with value 0"]
impl crate::ResetValue for super::RSTC_MR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `URSTEN`"]
pub type URSTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URSTEN`"]
pub struct URSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> URSTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `URSTIEN`"]
pub type URSTIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `URSTIEN`"]
pub struct URSTIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> URSTIEN_W<'a> {
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
#[doc = "Reader of field `ERSTL`"]
pub type ERSTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ERSTL`"]
pub struct ERSTL_W<'a> {
    w: &'a mut W,
}
impl<'a> ERSTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Write Access Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEY_A {
    #[doc = "165: Writing any other value in this field aborts the write operation.Always reads as 0."]
    PASSWD = 165,
}
impl From<KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<u8, KEY_A>;
impl KEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, KEY_A> {
        use crate::Variant::*;
        match self.bits {
            165 => Val(KEY_A::PASSWD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == KEY_A::PASSWD
    }
}
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEY_A::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - User Reset Enable"]
    #[inline(always)]
    pub fn ursten(&self) -> URSTEN_R {
        URSTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - User Reset Interrupt Enable"]
    #[inline(always)]
    pub fn urstien(&self) -> URSTIEN_R {
        URSTIEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - External Reset Length"]
    #[inline(always)]
    pub fn erstl(&self) -> ERSTL_R {
        ERSTL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Write Access Password"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - User Reset Enable"]
    #[inline(always)]
    pub fn ursten(&mut self) -> URSTEN_W {
        URSTEN_W { w: self }
    }
    #[doc = "Bit 4 - User Reset Interrupt Enable"]
    #[inline(always)]
    pub fn urstien(&mut self) -> URSTIEN_W {
        URSTIEN_W { w: self }
    }
    #[doc = "Bits 8:11 - External Reset Length"]
    #[inline(always)]
    pub fn erstl(&mut self) -> ERSTL_W {
        ERSTL_W { w: self }
    }
    #[doc = "Bits 24:31 - Write Access Password"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
