#[doc = "Reader of register MCAN_TEST"]
pub type R = crate::R<u32, super::MCAN_TEST>;
#[doc = "Writer for register MCAN_TEST"]
pub type W = crate::W<u32, super::MCAN_TEST>;
#[doc = "Register MCAN_TEST `reset()`'s with value 0"]
impl crate::ResetValue for super::MCAN_TEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Loop Back Mode (read/write)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBCK_A {
    #[doc = "0: Reset value. Loop Back mode is disabled."]
    DISABLED = 0,
    #[doc = "1: Loop Back mode is enabled (see Section 6.1.9)."]
    ENABLED = 1,
}
impl From<LBCK_A> for bool {
    #[inline(always)]
    fn from(variant: LBCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LBCK`"]
pub type LBCK_R = crate::R<bool, LBCK_A>;
impl LBCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBCK_A {
        match self.bits {
            false => LBCK_A::DISABLED,
            true => LBCK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LBCK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LBCK_A::ENABLED
    }
}
#[doc = "Write proxy for field `LBCK`"]
pub struct LBCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LBCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset value. Loop Back mode is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LBCK_A::DISABLED)
    }
    #[doc = "Loop Back mode is enabled (see Section 6.1.9)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LBCK_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Control of Transmit Pin (read/write)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_A {
    #[doc = "0: Reset value, CANTX controlled by the CAN Core, updated at the end of the CAN bit time."]
    RESET = 0,
    #[doc = "1: Sample Point can be monitored at pin CANTX."]
    SAMPLE_POINT_MONITORING = 1,
    #[doc = "2: Dominant ('0') level at pin CANTX."]
    DOMINANT = 2,
    #[doc = "3: Recessive ('1') at pin CANTX."]
    RECESSIVE = 3,
}
impl From<TX_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TX`"]
pub type TX_R = crate::R<u8, TX_A>;
impl TX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            0 => TX_A::RESET,
            1 => TX_A::SAMPLE_POINT_MONITORING,
            2 => TX_A::DOMINANT,
            3 => TX_A::RECESSIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == TX_A::RESET
    }
    #[doc = "Checks if the value of the field is `SAMPLE_POINT_MONITORING`"]
    #[inline(always)]
    pub fn is_sample_point_monitoring(&self) -> bool {
        *self == TX_A::SAMPLE_POINT_MONITORING
    }
    #[doc = "Checks if the value of the field is `DOMINANT`"]
    #[inline(always)]
    pub fn is_dominant(&self) -> bool {
        *self == TX_A::DOMINANT
    }
    #[doc = "Checks if the value of the field is `RECESSIVE`"]
    #[inline(always)]
    pub fn is_recessive(&self) -> bool {
        *self == TX_A::RECESSIVE
    }
}
#[doc = "Write proxy for field `TX`"]
pub struct TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Reset value, CANTX controlled by the CAN Core, updated at the end of the CAN bit time."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(TX_A::RESET)
    }
    #[doc = "Sample Point can be monitored at pin CANTX."]
    #[inline(always)]
    pub fn sample_point_monitoring(self) -> &'a mut W {
        self.variant(TX_A::SAMPLE_POINT_MONITORING)
    }
    #[doc = "Dominant ('0') level at pin CANTX."]
    #[inline(always)]
    pub fn dominant(self) -> &'a mut W {
        self.variant(TX_A::DOMINANT)
    }
    #[doc = "Recessive ('1') at pin CANTX."]
    #[inline(always)]
    pub fn recessive(self) -> &'a mut W {
        self.variant(TX_A::RECESSIVE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `RX`"]
pub type RX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX`"]
pub struct RX_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Loop Back Mode (read/write)"]
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin (read/write)"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Receive Pin (read-only)"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Loop Back Mode (read/write)"]
    #[inline(always)]
    pub fn lbck(&mut self) -> LBCK_W {
        LBCK_W { w: self }
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin (read/write)"]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W { w: self }
    }
    #[doc = "Bit 7 - Receive Pin (read-only)"]
    #[inline(always)]
    pub fn rx(&mut self) -> RX_W {
        RX_W { w: self }
    }
}
