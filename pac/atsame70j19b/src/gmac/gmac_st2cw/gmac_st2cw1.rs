#[doc = "Reader of register GMAC_ST2CW1"]
pub type R = crate::R<u32, super::GMAC_ST2CW1>;
#[doc = "Writer for register GMAC_ST2CW1"]
pub type W = crate::W<u32, super::GMAC_ST2CW1>;
#[doc = "Register GMAC_ST2CW1 `reset()`'s with value 0"]
impl crate::ResetValue for super::GMAC_ST2CW1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OFFSVAL`"]
pub type OFFSVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OFFSVAL`"]
pub struct OFFSVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Ethernet Frame Offset Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OFFSSTRT_A {
    #[doc = "0: Offset from the start of the frame"]
    FRAMESTART = 0,
    #[doc = "1: Offset from the byte after the EtherType field"]
    ETHERTYPE = 1,
    #[doc = "2: Offset from the byte after the IP header field"]
    IP = 2,
    #[doc = "3: Offset from the byte after the TCP/UDP header field"]
    TCP_UDP = 3,
}
impl From<OFFSSTRT_A> for u8 {
    #[inline(always)]
    fn from(variant: OFFSSTRT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OFFSSTRT`"]
pub type OFFSSTRT_R = crate::R<u8, OFFSSTRT_A>;
impl OFFSSTRT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFSSTRT_A {
        match self.bits {
            0 => OFFSSTRT_A::FRAMESTART,
            1 => OFFSSTRT_A::ETHERTYPE,
            2 => OFFSSTRT_A::IP,
            3 => OFFSSTRT_A::TCP_UDP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FRAMESTART`"]
    #[inline(always)]
    pub fn is_framestart(&self) -> bool {
        *self == OFFSSTRT_A::FRAMESTART
    }
    #[doc = "Checks if the value of the field is `ETHERTYPE`"]
    #[inline(always)]
    pub fn is_ethertype(&self) -> bool {
        *self == OFFSSTRT_A::ETHERTYPE
    }
    #[doc = "Checks if the value of the field is `IP`"]
    #[inline(always)]
    pub fn is_ip(&self) -> bool {
        *self == OFFSSTRT_A::IP
    }
    #[doc = "Checks if the value of the field is `TCP_UDP`"]
    #[inline(always)]
    pub fn is_tcp_udp(&self) -> bool {
        *self == OFFSSTRT_A::TCP_UDP
    }
}
#[doc = "Write proxy for field `OFFSSTRT`"]
pub struct OFFSSTRT_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSSTRT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFFSSTRT_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Offset from the start of the frame"]
    #[inline(always)]
    pub fn framestart(self) -> &'a mut W {
        self.variant(OFFSSTRT_A::FRAMESTART)
    }
    #[doc = "Offset from the byte after the EtherType field"]
    #[inline(always)]
    pub fn ethertype(self) -> &'a mut W {
        self.variant(OFFSSTRT_A::ETHERTYPE)
    }
    #[doc = "Offset from the byte after the IP header field"]
    #[inline(always)]
    pub fn ip(self) -> &'a mut W {
        self.variant(OFFSSTRT_A::IP)
    }
    #[doc = "Offset from the byte after the TCP/UDP header field"]
    #[inline(always)]
    pub fn tcp_udp(self) -> &'a mut W {
        self.variant(OFFSSTRT_A::TCP_UDP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Offset Value in Bytes"]
    #[inline(always)]
    pub fn offsval(&self) -> OFFSVAL_R {
        OFFSVAL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:8 - Ethernet Frame Offset Start"]
    #[inline(always)]
    pub fn offsstrt(&self) -> OFFSSTRT_R {
        OFFSSTRT_R::new(((self.bits >> 7) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Offset Value in Bytes"]
    #[inline(always)]
    pub fn offsval(&mut self) -> OFFSVAL_W {
        OFFSVAL_W { w: self }
    }
    #[doc = "Bits 7:8 - Ethernet Frame Offset Start"]
    #[inline(always)]
    pub fn offsstrt(&mut self) -> OFFSSTRT_W {
        OFFSSTRT_W { w: self }
    }
}
