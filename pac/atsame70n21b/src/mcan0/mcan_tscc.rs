#[doc = "Reader of register MCAN_TSCC"]
pub type R = crate::R<u32, super::MCAN_TSCC>;
#[doc = "Writer for register MCAN_TSCC"]
pub type W = crate::W<u32, super::MCAN_TSCC>;
#[doc = "Register MCAN_TSCC `reset()`'s with value 0"]
impl crate::ResetValue for super::MCAN_TSCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Timestamp Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSS_A {
    #[doc = "0: Timestamp counter value always 0x0000"]
    ALWAYS_0 = 0,
    #[doc = "1: Timestamp counter value incremented according to TCP"]
    TCP_INC = 1,
    #[doc = "2: External timestamp counter value used"]
    EXT_TIMESTAMP = 2,
}
impl From<TSS_A> for u8 {
    #[inline(always)]
    fn from(variant: TSS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TSS`"]
pub type TSS_R = crate::R<u8, TSS_A>;
impl TSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TSS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TSS_A::ALWAYS_0),
            1 => Val(TSS_A::TCP_INC),
            2 => Val(TSS_A::EXT_TIMESTAMP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_0`"]
    #[inline(always)]
    pub fn is_always_0(&self) -> bool {
        *self == TSS_A::ALWAYS_0
    }
    #[doc = "Checks if the value of the field is `TCP_INC`"]
    #[inline(always)]
    pub fn is_tcp_inc(&self) -> bool {
        *self == TSS_A::TCP_INC
    }
    #[doc = "Checks if the value of the field is `EXT_TIMESTAMP`"]
    #[inline(always)]
    pub fn is_ext_timestamp(&self) -> bool {
        *self == TSS_A::EXT_TIMESTAMP
    }
}
#[doc = "Write proxy for field `TSS`"]
pub struct TSS_W<'a> {
    w: &'a mut W,
}
impl<'a> TSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Timestamp counter value always 0x0000"]
    #[inline(always)]
    pub fn always_0(self) -> &'a mut W {
        self.variant(TSS_A::ALWAYS_0)
    }
    #[doc = "Timestamp counter value incremented according to TCP"]
    #[inline(always)]
    pub fn tcp_inc(self) -> &'a mut W {
        self.variant(TSS_A::TCP_INC)
    }
    #[doc = "External timestamp counter value used"]
    #[inline(always)]
    pub fn ext_timestamp(self) -> &'a mut W {
        self.variant(TSS_A::EXT_TIMESTAMP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `TCP`"]
pub type TCP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TCP`"]
pub struct TCP_W<'a> {
    w: &'a mut W,
}
impl<'a> TCP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Timestamp Select"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler"]
    #[inline(always)]
    pub fn tcp(&self) -> TCP_R {
        TCP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timestamp Select"]
    #[inline(always)]
    pub fn tss(&mut self) -> TSS_W {
        TSS_W { w: self }
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler"]
    #[inline(always)]
    pub fn tcp(&mut self) -> TCP_W {
        TCP_W { w: self }
    }
}
