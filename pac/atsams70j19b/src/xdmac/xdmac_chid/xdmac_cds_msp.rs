#[doc = "Reader of register XDMAC_CDS_MSP"]
pub type R = crate::R<u32, super::XDMAC_CDS_MSP>;
#[doc = "Writer for register XDMAC_CDS_MSP"]
pub type W = crate::W<u32, super::XDMAC_CDS_MSP>;
#[doc = "Register XDMAC_CDS_MSP `reset()`'s with value 0"]
impl crate::ResetValue for super::XDMAC_CDS_MSP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDS_MSP`"]
pub type SDS_MSP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SDS_MSP`"]
pub struct SDS_MSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SDS_MSP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `DDS_MSP`"]
pub type DDS_MSP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DDS_MSP`"]
pub struct DDS_MSP_W<'a> {
    w: &'a mut W,
}
impl<'a> DDS_MSP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Channel x Source Data stride or Memory Set Pattern"]
    #[inline(always)]
    pub fn sds_msp(&self) -> SDS_MSP_R {
        SDS_MSP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Channel x Destination Data Stride or Memory Set Pattern"]
    #[inline(always)]
    pub fn dds_msp(&self) -> DDS_MSP_R {
        DDS_MSP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel x Source Data stride or Memory Set Pattern"]
    #[inline(always)]
    pub fn sds_msp(&mut self) -> SDS_MSP_W {
        SDS_MSP_W { w: self }
    }
    #[doc = "Bits 16:31 - Channel x Destination Data Stride or Memory Set Pattern"]
    #[inline(always)]
    pub fn dds_msp(&mut self) -> DDS_MSP_W {
        DDS_MSP_W { w: self }
    }
}
