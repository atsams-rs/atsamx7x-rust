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
#[doc = "Field `TXRDY` reader - Transmit Ready"]
pub type TXRDY_R = crate::BitReader<bool>;
#[doc = "Field `TXEMPTY` reader - Transmit Empty"]
pub type TXEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `RXRDY` reader - Receive Ready"]
pub type RXRDY_R = crate::BitReader<bool>;
#[doc = "Field `OVRUN` reader - Receive Overrun"]
pub type OVRUN_R = crate::BitReader<bool>;
#[doc = "Field `CP0` reader - Compare 0"]
pub type CP0_R = crate::BitReader<bool>;
#[doc = "Field `CP1` reader - Compare 1"]
pub type CP1_R = crate::BitReader<bool>;
#[doc = "Field `TXSYN` reader - Transmit Sync"]
pub type TXSYN_R = crate::BitReader<bool>;
#[doc = "Field `RXSYN` reader - Receive Sync"]
pub type RXSYN_R = crate::BitReader<bool>;
#[doc = "Field `TXEN` reader - Transmit Enable"]
pub type TXEN_R = crate::BitReader<bool>;
#[doc = "Field `RXEN` reader - Receive Enable"]
pub type RXEN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Transmit Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Overrun"]
    #[inline(always)]
    pub fn ovrun(&self) -> OVRUN_R {
        OVRUN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Compare 0"]
    #[inline(always)]
    pub fn cp0(&self) -> CP0_R {
        CP0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare 1"]
    #[inline(always)]
    pub fn cp1(&self) -> CP1_R {
        CP1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit Sync"]
    #[inline(always)]
    pub fn txsyn(&self) -> TXSYN_R {
        TXSYN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receive Sync"]
    #[inline(always)]
    pub fn rxsyn(&self) -> RXSYN_R {
        RXSYN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmit Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new(((self.bits >> 17) & 1) != 0)
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
