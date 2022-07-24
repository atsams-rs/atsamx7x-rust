#[doc = "Register `TOCC` reader"]
pub struct R(crate::R<TOCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOCC` writer"]
pub struct W(crate::W<TOCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TOCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable Timeout Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETOC_A {
    #[doc = "0: Timeout Counter disabled."]
    NO_TIMEOUT = 0,
    #[doc = "1: Timeout Counter enabled."]
    TOS_CONTROLLED = 1,
}
impl From<ETOC_A> for bool {
    #[inline(always)]
    fn from(variant: ETOC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETOC` reader - Enable Timeout Counter"]
pub type ETOC_R = crate::BitReader<ETOC_A>;
impl ETOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETOC_A {
        match self.bits {
            false => ETOC_A::NO_TIMEOUT,
            true => ETOC_A::TOS_CONTROLLED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIMEOUT`"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == ETOC_A::NO_TIMEOUT
    }
    #[doc = "Checks if the value of the field is `TOS_CONTROLLED`"]
    #[inline(always)]
    pub fn is_tos_controlled(&self) -> bool {
        *self == ETOC_A::TOS_CONTROLLED
    }
}
#[doc = "Field `ETOC` writer - Enable Timeout Counter"]
pub type ETOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TOCC_SPEC, ETOC_A, O>;
impl<'a, const O: u8> ETOC_W<'a, O> {
    #[doc = "Timeout Counter disabled."]
    #[inline(always)]
    pub fn no_timeout(self) -> &'a mut W {
        self.variant(ETOC_A::NO_TIMEOUT)
    }
    #[doc = "Timeout Counter enabled."]
    #[inline(always)]
    pub fn tos_controlled(self) -> &'a mut W {
        self.variant(ETOC_A::TOS_CONTROLLED)
    }
}
#[doc = "Timeout Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TOS_A {
    #[doc = "0: Continuous operation"]
    CONTINUOUS = 0,
    #[doc = "1: Timeout controlled by Tx Event FIFO"]
    TX_EV_TIMEOUT = 1,
    #[doc = "2: Timeout controlled by Receive FIFO 0"]
    RX0_EV_TIMEOUT = 2,
    #[doc = "3: Timeout controlled by Receive FIFO 1"]
    RX1_EV_TIMEOUT = 3,
}
impl From<TOS_A> for u8 {
    #[inline(always)]
    fn from(variant: TOS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TOS` reader - Timeout Select"]
pub type TOS_R = crate::FieldReader<u8, TOS_A>;
impl TOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOS_A {
        match self.bits {
            0 => TOS_A::CONTINUOUS,
            1 => TOS_A::TX_EV_TIMEOUT,
            2 => TOS_A::RX0_EV_TIMEOUT,
            3 => TOS_A::RX1_EV_TIMEOUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == TOS_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `TX_EV_TIMEOUT`"]
    #[inline(always)]
    pub fn is_tx_ev_timeout(&self) -> bool {
        *self == TOS_A::TX_EV_TIMEOUT
    }
    #[doc = "Checks if the value of the field is `RX0_EV_TIMEOUT`"]
    #[inline(always)]
    pub fn is_rx0_ev_timeout(&self) -> bool {
        *self == TOS_A::RX0_EV_TIMEOUT
    }
    #[doc = "Checks if the value of the field is `RX1_EV_TIMEOUT`"]
    #[inline(always)]
    pub fn is_rx1_ev_timeout(&self) -> bool {
        *self == TOS_A::RX1_EV_TIMEOUT
    }
}
#[doc = "Field `TOS` writer - Timeout Select"]
pub type TOS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TOCC_SPEC, u8, TOS_A, 2, O>;
impl<'a, const O: u8> TOS_W<'a, O> {
    #[doc = "Continuous operation"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TOS_A::CONTINUOUS)
    }
    #[doc = "Timeout controlled by Tx Event FIFO"]
    #[inline(always)]
    pub fn tx_ev_timeout(self) -> &'a mut W {
        self.variant(TOS_A::TX_EV_TIMEOUT)
    }
    #[doc = "Timeout controlled by Receive FIFO 0"]
    #[inline(always)]
    pub fn rx0_ev_timeout(self) -> &'a mut W {
        self.variant(TOS_A::RX0_EV_TIMEOUT)
    }
    #[doc = "Timeout controlled by Receive FIFO 1"]
    #[inline(always)]
    pub fn rx1_ev_timeout(self) -> &'a mut W {
        self.variant(TOS_A::RX1_EV_TIMEOUT)
    }
}
#[doc = "Field `TOP` reader - Timeout Period"]
pub type TOP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOP` writer - Timeout Period"]
pub type TOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOCC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    pub fn etoc(&mut self) -> ETOC_W<0> {
        ETOC_W::new(self)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    pub fn tos(&mut self) -> TOS_W<1> {
        TOS_W::new(self)
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W<16> {
        TOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timeout Counter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tocc](index.html) module"]
pub struct TOCC_SPEC;
impl crate::RegisterSpec for TOCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tocc::R](R) reader structure"]
impl crate::Readable for TOCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tocc::W](W) writer structure"]
impl crate::Writable for TOCC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TOCC to value 0"]
impl crate::Resettable for TOCC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
