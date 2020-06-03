#[doc = "Reader of register MCAN_TOCC"]
pub type R = crate::R<u32, super::MCAN_TOCC>;
#[doc = "Writer for register MCAN_TOCC"]
pub type W = crate::W<u32, super::MCAN_TOCC>;
#[doc = "Register MCAN_TOCC `reset()`'s with value 0"]
impl crate::ResetValue for super::MCAN_TOCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `ETOC`"]
pub type ETOC_R = crate::R<bool, ETOC_A>;
impl ETOC_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `ETOC`"]
pub struct ETOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETOC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
#[doc = "Reader of field `TOS`"]
pub type TOS_R = crate::R<u8, TOS_A>;
impl TOS_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `TOS`"]
pub struct TOS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `TOP`"]
pub type TOP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOP`"]
pub struct TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable Timeout Counter"]
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 0x03) as u8)
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
    pub fn etoc(&mut self) -> ETOC_W {
        ETOC_W { w: self }
    }
    #[doc = "Bits 1:2 - Timeout Select"]
    #[inline(always)]
    pub fn tos(&mut self) -> TOS_W {
        TOS_W { w: self }
    }
    #[doc = "Bits 16:31 - Timeout Period"]
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W {
        TOP_W { w: self }
    }
}
