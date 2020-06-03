#[doc = "Reader of register GMAC_RBQBAPQ[%s]"]
pub type R = crate::R<u32, super::GMAC_RBQBAPQ>;
#[doc = "Writer for register GMAC_RBQBAPQ[%s]"]
pub type W = crate::W<u32, super::GMAC_RBQBAPQ>;
#[doc = "Register GMAC_RBQBAPQ[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::GMAC_RBQBAPQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXBQBA`"]
pub type RXBQBA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RXBQBA`"]
pub struct RXBQBA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBQBA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - Receive Buffer Queue Base Address"]
    #[inline(always)]
    pub fn rxbqba(&self) -> RXBQBA_R {
        RXBQBA_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31 - Receive Buffer Queue Base Address"]
    #[inline(always)]
    pub fn rxbqba(&mut self) -> RXBQBA_W {
        RXBQBA_W { w: self }
    }
}
