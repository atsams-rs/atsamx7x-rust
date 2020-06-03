#[doc = "Reader of register XDMAC_CNDA"]
pub type R = crate::R<u32, super::XDMAC_CNDA>;
#[doc = "Writer for register XDMAC_CNDA"]
pub type W = crate::W<u32, super::XDMAC_CNDA>;
#[doc = "Register XDMAC_CNDA `reset()`'s with value 0"]
impl crate::ResetValue for super::XDMAC_CNDA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NDAIF`"]
pub type NDAIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NDAIF`"]
pub struct NDAIF_W<'a> {
    w: &'a mut W,
}
impl<'a> NDAIF_W<'a> {
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
#[doc = "Reader of field `NDA`"]
pub type NDA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NDA`"]
pub struct NDA_W<'a> {
    w: &'a mut W,
}
impl<'a> NDA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel x Next Descriptor Interface"]
    #[inline(always)]
    pub fn ndaif(&self) -> NDAIF_R {
        NDAIF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:31 - Channel x Next Descriptor Address"]
    #[inline(always)]
    pub fn nda(&self) -> NDA_R {
        NDA_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Channel x Next Descriptor Interface"]
    #[inline(always)]
    pub fn ndaif(&mut self) -> NDAIF_W {
        NDAIF_W { w: self }
    }
    #[doc = "Bits 2:31 - Channel x Next Descriptor Address"]
    #[inline(always)]
    pub fn nda(&mut self) -> NDA_W {
        NDA_W { w: self }
    }
}
