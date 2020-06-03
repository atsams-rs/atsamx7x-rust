#[doc = "Reader of register HSMCI_ARGR"]
pub type R = crate::R<u32, super::HSMCI_ARGR>;
#[doc = "Writer for register HSMCI_ARGR"]
pub type W = crate::W<u32, super::HSMCI_ARGR>;
#[doc = "Register HSMCI_ARGR `reset()`'s with value 0"]
impl crate::ResetValue for super::HSMCI_ARGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ARG`"]
pub type ARG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ARG`"]
pub struct ARG_W<'a> {
    w: &'a mut W,
}
impl<'a> ARG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    pub fn arg(&self) -> ARG_R {
        ARG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Command Argument"]
    #[inline(always)]
    pub fn arg(&mut self) -> ARG_W {
        ARG_W { w: self }
    }
}
