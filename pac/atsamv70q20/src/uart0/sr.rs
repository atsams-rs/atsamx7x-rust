#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRDY` reader - Receiver Ready"]
pub type RXRDY_R = crate::BitReader<bool>;
#[doc = "Field `TXRDY` reader - Transmitter Ready"]
pub type TXRDY_R = crate::BitReader<bool>;
#[doc = "Field `OVRE` reader - Overrun Error"]
pub type OVRE_R = crate::BitReader<bool>;
#[doc = "Field `FRAME` reader - Framing Error"]
pub type FRAME_R = crate::BitReader<bool>;
#[doc = "Field `PARE` reader - Parity Error"]
pub type PARE_R = crate::BitReader<bool>;
#[doc = "Field `TXEMPTY` reader - Transmitter Empty"]
pub type TXEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `CMP` reader - Comparison Match"]
pub type CMP_R = crate::BitReader<bool>;
#[doc = "Field `SWES` reader - SleepWalking Enable Status"]
pub type SWES_R = crate::BitReader<bool>;
#[doc = "Field `CLKREQ` reader - Clock Request"]
pub type CLKREQ_R = crate::BitReader<bool>;
#[doc = "Field `WKUPREQ` reader - Wake-Up Request"]
pub type WKUPREQ_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Receiver Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Framing Error"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Parity Error"]
    #[inline(always)]
    pub fn pare(&self) -> PARE_R {
        PARE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmitter Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparison Match"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - SleepWalking Enable Status"]
    #[inline(always)]
    pub fn swes(&self) -> SWES_R {
        SWES_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Clock Request"]
    #[inline(always)]
    pub fn clkreq(&self) -> CLKREQ_R {
        CLKREQ_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Wake-Up Request"]
    #[inline(always)]
    pub fn wkupreq(&self) -> WKUPREQ_R {
        WKUPREQ_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
