#[doc = "Writer for register XDMAC_CID"]
pub type W = crate::W<u32, super::XDMAC_CID>;
#[doc = "Register XDMAC_CID `reset()`'s with value 0"]
impl crate::ResetValue for super::XDMAC_CID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `BID`"]
pub struct BID_W<'a> {
    w: &'a mut W,
}
impl<'a> BID_W<'a> {
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
#[doc = "Write proxy for field `LID`"]
pub struct LID_W<'a> {
    w: &'a mut W,
}
impl<'a> LID_W<'a> {
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
#[doc = "Write proxy for field `DID`"]
pub struct DID_W<'a> {
    w: &'a mut W,
}
impl<'a> DID_W<'a> {
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
#[doc = "Write proxy for field `FID`"]
pub struct FID_W<'a> {
    w: &'a mut W,
}
impl<'a> FID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `RBEID`"]
pub struct RBEID_W<'a> {
    w: &'a mut W,
}
impl<'a> RBEID_W<'a> {
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
#[doc = "Write proxy for field `WBEID`"]
pub struct WBEID_W<'a> {
    w: &'a mut W,
}
impl<'a> WBEID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `ROID`"]
pub struct ROID_W<'a> {
    w: &'a mut W,
}
impl<'a> ROID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - End of Block Interrupt Disable Bit"]
    #[inline(always)]
    pub fn bid(&mut self) -> BID_W {
        BID_W { w: self }
    }
    #[doc = "Bit 1 - End of Linked List Interrupt Disable Bit"]
    #[inline(always)]
    pub fn lid(&mut self) -> LID_W {
        LID_W { w: self }
    }
    #[doc = "Bit 2 - End of Disable Interrupt Disable Bit"]
    #[inline(always)]
    pub fn did(&mut self) -> DID_W {
        DID_W { w: self }
    }
    #[doc = "Bit 3 - End of Flush Interrupt Disable Bit"]
    #[inline(always)]
    pub fn fid(&mut self) -> FID_W {
        FID_W { w: self }
    }
    #[doc = "Bit 4 - Read Bus Error Interrupt Disable Bit"]
    #[inline(always)]
    pub fn rbeid(&mut self) -> RBEID_W {
        RBEID_W { w: self }
    }
    #[doc = "Bit 5 - Write Bus Error Interrupt Disable Bit"]
    #[inline(always)]
    pub fn wbeid(&mut self) -> WBEID_W {
        WBEID_W { w: self }
    }
    #[doc = "Bit 6 - Request Overflow Error Interrupt Disable Bit"]
    #[inline(always)]
    pub fn roid(&mut self) -> ROID_W {
        ROID_W { w: self }
    }
}
