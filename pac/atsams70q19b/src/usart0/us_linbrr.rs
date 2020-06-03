#[doc = "Reader of register US_LINBRR"]
pub type R = crate::R<u32, super::US_LINBRR>;
#[doc = "Reader of field `LINCD`"]
pub type LINCD_R = crate::R<u16, u16>;
#[doc = "Reader of field `LINFP`"]
pub type LINFP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Clock Divider after Synchronization"]
    #[inline(always)]
    pub fn lincd(&self) -> LINCD_R {
        LINCD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Fractional Part after Synchronization"]
    #[inline(always)]
    pub fn linfp(&self) -> LINFP_R {
        LINFP_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
