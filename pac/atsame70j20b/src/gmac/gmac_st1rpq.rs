#[doc = "Reader of register GMAC_ST1RPQ[%s]"]
pub type R = crate::R<u32, super::GMAC_ST1RPQ>;
#[doc = "Writer for register GMAC_ST1RPQ[%s]"]
pub type W = crate::W<u32, super::GMAC_ST1RPQ>;
#[doc = "Register GMAC_ST1RPQ[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::GMAC_ST1RPQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QNB`"]
pub type QNB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `QNB`"]
pub struct QNB_W<'a> {
    w: &'a mut W,
}
impl<'a> QNB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `DSTCM`"]
pub type DSTCM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DSTCM`"]
pub struct DSTCM_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | (((value as u32) & 0xff) << 4);
        self.w
    }
}
#[doc = "Reader of field `UDPM`"]
pub type UDPM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UDPM`"]
pub struct UDPM_W<'a> {
    w: &'a mut W,
}
impl<'a> UDPM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 12)) | (((value as u32) & 0xffff) << 12);
        self.w
    }
}
#[doc = "Reader of field `DSTCE`"]
pub type DSTCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSTCE`"]
pub struct DSTCE_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `UDPE`"]
pub type UDPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UDPE`"]
pub struct UDPE_W<'a> {
    w: &'a mut W,
}
impl<'a> UDPE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Queue Number (0-5)"]
    #[inline(always)]
    pub fn qnb(&self) -> QNB_R {
        QNB_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:11 - Differentiated Services or Traffic Class Match"]
    #[inline(always)]
    pub fn dstcm(&self) -> DSTCM_R {
        DSTCM_R::new(((self.bits >> 4) & 0xff) as u8)
    }
    #[doc = "Bits 12:27 - UDP Port Match"]
    #[inline(always)]
    pub fn udpm(&self) -> UDPM_R {
        UDPM_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bit 28 - Differentiated Services or Traffic Class Match Enable"]
    #[inline(always)]
    pub fn dstce(&self) -> DSTCE_R {
        DSTCE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - UDP Port Match Enable"]
    #[inline(always)]
    pub fn udpe(&self) -> UDPE_R {
        UDPE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Queue Number (0-5)"]
    #[inline(always)]
    pub fn qnb(&mut self) -> QNB_W {
        QNB_W { w: self }
    }
    #[doc = "Bits 4:11 - Differentiated Services or Traffic Class Match"]
    #[inline(always)]
    pub fn dstcm(&mut self) -> DSTCM_W {
        DSTCM_W { w: self }
    }
    #[doc = "Bits 12:27 - UDP Port Match"]
    #[inline(always)]
    pub fn udpm(&mut self) -> UDPM_W {
        UDPM_W { w: self }
    }
    #[doc = "Bit 28 - Differentiated Services or Traffic Class Match Enable"]
    #[inline(always)]
    pub fn dstce(&mut self) -> DSTCE_W {
        DSTCE_W { w: self }
    }
    #[doc = "Bit 29 - UDP Port Match Enable"]
    #[inline(always)]
    pub fn udpe(&mut self) -> UDPE_W {
        UDPE_W { w: self }
    }
}
