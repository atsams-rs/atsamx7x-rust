#[doc = "Reader of register XDMAC_GTYPE"]
pub type R = crate::R<u32, super::XDMAC_GTYPE>;
#[doc = "Reader of field `NB_CH`"]
pub type NB_CH_R = crate::R<u8, u8>;
#[doc = "Reader of field `FIFO_SZ`"]
pub type FIFO_SZ_R = crate::R<u16, u16>;
#[doc = "Reader of field `NB_REQ`"]
pub type NB_REQ_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Number of Channels Minus One"]
    #[inline(always)]
    pub fn nb_ch(&self) -> NB_CH_R {
        NB_CH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:15 - Number of Bytes"]
    #[inline(always)]
    pub fn fifo_sz(&self) -> FIFO_SZ_R {
        FIFO_SZ_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:22 - Number of Peripheral Requests Minus One"]
    #[inline(always)]
    pub fn nb_req(&self) -> NB_REQ_R {
        NB_REQ_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
