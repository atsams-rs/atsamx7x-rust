#[doc = "Reader of register MCAN_RXBC"]
pub type R = crate::R<u32, super::MCAN_RXBC>;
#[doc = "Writer for register MCAN_RXBC"]
pub type W = crate::W<u32, super::MCAN_RXBC>;
#[doc = "Register MCAN_RXBC `reset()`'s with value 0"]
impl crate::ResetValue for super::MCAN_RXBC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RBSA`"]
pub type RBSA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RBSA`"]
pub struct RBSA_W<'a> {
    w: &'a mut W,
}
impl<'a> RBSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | (((value as u32) & 0x3fff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - Receive Buffer Start Address"]
    #[inline(always)]
    pub fn rbsa(&self) -> RBSA_R {
        RBSA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:15 - Receive Buffer Start Address"]
    #[inline(always)]
    pub fn rbsa(&mut self) -> RBSA_W {
        RBSA_W { w: self }
    }
}
