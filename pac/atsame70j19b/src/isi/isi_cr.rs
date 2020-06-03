#[doc = "Writer for register ISI_CR"]
pub type W = crate::W<u32, super::ISI_CR>;
#[doc = "Register ISI_CR `reset()`'s with value 0"]
impl crate::ResetValue for super::ISI_CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `ISI_EN`"]
pub struct ISI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISI_EN_W<'a> {
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
#[doc = "Write proxy for field `ISI_DIS`"]
pub struct ISI_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ISI_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write proxy for field `ISI_SRST`"]
pub struct ISI_SRST_W<'a> {
    w: &'a mut W,
}
impl<'a> ISI_SRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `ISI_CDC`"]
pub struct ISI_CDC_W<'a> {
    w: &'a mut W,
}
impl<'a> ISI_CDC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - ISI Module Enable Request"]
    #[inline(always)]
    pub fn isi_en(&mut self) -> ISI_EN_W {
        ISI_EN_W { w: self }
    }
    #[doc = "Bit 1 - ISI Module Disable Request"]
    #[inline(always)]
    pub fn isi_dis(&mut self) -> ISI_DIS_W {
        ISI_DIS_W { w: self }
    }
    #[doc = "Bit 2 - ISI Software Reset Request"]
    #[inline(always)]
    pub fn isi_srst(&mut self) -> ISI_SRST_W {
        ISI_SRST_W { w: self }
    }
    #[doc = "Bit 8 - ISI Codec Request"]
    #[inline(always)]
    pub fn isi_cdc(&mut self) -> ISI_CDC_W {
        ISI_CDC_W { w: self }
    }
}
