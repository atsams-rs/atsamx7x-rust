#[doc = "Reader of register MCAN_RXF1S"]
pub type R = crate::R<u32, super::MCAN_RXF1S>;
#[doc = "Reader of field `F1FL`"]
pub type F1FL_R = crate::R<u8, u8>;
#[doc = "Reader of field `F1GI`"]
pub type F1GI_R = crate::R<u8, u8>;
#[doc = "Reader of field `F1PI`"]
pub type F1PI_R = crate::R<u8, u8>;
#[doc = "Reader of field `F1F`"]
pub type F1F_R = crate::R<bool, bool>;
#[doc = "Reader of field `RF1L`"]
pub type RF1L_R = crate::R<bool, bool>;
#[doc = "Debug Message Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMS_A {
    #[doc = "0: Idle state, wait for reception of debug messages, DMA request is cleared."]
    IDLE = 0,
    #[doc = "1: Debug message A received."]
    MSG_A = 1,
    #[doc = "2: Debug messages A, B received."]
    MSG_AB = 2,
    #[doc = "3: Debug messages A, B, C received, DMA request is set."]
    MSG_ABC = 3,
}
impl From<DMS_A> for u8 {
    #[inline(always)]
    fn from(variant: DMS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DMS`"]
pub type DMS_R = crate::R<u8, DMS_A>;
impl DMS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMS_A {
        match self.bits {
            0 => DMS_A::IDLE,
            1 => DMS_A::MSG_A,
            2 => DMS_A::MSG_AB,
            3 => DMS_A::MSG_ABC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == DMS_A::IDLE
    }
    #[doc = "Checks if the value of the field is `MSG_A`"]
    #[inline(always)]
    pub fn is_msg_a(&self) -> bool {
        *self == DMS_A::MSG_A
    }
    #[doc = "Checks if the value of the field is `MSG_AB`"]
    #[inline(always)]
    pub fn is_msg_ab(&self) -> bool {
        *self == DMS_A::MSG_AB
    }
    #[doc = "Checks if the value of the field is `MSG_ABC`"]
    #[inline(always)]
    pub fn is_msg_abc(&self) -> bool {
        *self == DMS_A::MSG_ABC
    }
}
impl R {
    #[doc = "Bits 0:6 - Receive FIFO 1 Fill Level"]
    #[inline(always)]
    pub fn f1fl(&self) -> F1FL_R {
        F1FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Receive FIFO 1 Get Index"]
    #[inline(always)]
    pub fn f1gi(&self) -> F1GI_R {
        F1GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Receive FIFO 1 Put Index"]
    #[inline(always)]
    pub fn f1pi(&self) -> F1PI_R {
        F1PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Receive FIFO 1 Fill Level"]
    #[inline(always)]
    pub fn f1f(&self) -> F1F_R {
        F1F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Receive FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Debug Message Status"]
    #[inline(always)]
    pub fn dms(&self) -> DMS_R {
        DMS_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
