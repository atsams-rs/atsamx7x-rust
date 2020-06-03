#[doc = "Reader of register MCAN_CUST"]
pub type R = crate::R<u32, super::MCAN_CUST>;
#[doc = "Writer for register MCAN_CUST"]
pub type W = crate::W<u32, super::MCAN_CUST>;
#[doc = "Register MCAN_CUST `reset()`'s with value 0"]
impl crate::ResetValue for super::MCAN_CUST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CSV`"]
pub type CSV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CSV`"]
pub struct CSV_W<'a> {
    w: &'a mut W,
}
impl<'a> CSV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Customer-specific Value"]
    #[inline(always)]
    pub fn csv(&self) -> CSV_R {
        CSV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Customer-specific Value"]
    #[inline(always)]
    pub fn csv(&mut self) -> CSV_W {
        CSV_W { w: self }
    }
}
