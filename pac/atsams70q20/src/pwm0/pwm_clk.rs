#[doc = "Reader of register PWM_CLK"]
pub type R = crate::R<u32, super::PWM_CLK>;
#[doc = "Writer for register PWM_CLK"]
pub type W = crate::W<u32, super::PWM_CLK>;
#[doc = "Register PWM_CLK `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_CLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CLKA Divide Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVA_A {
    #[doc = "0: CLKA clock is turned off"]
    CLKA_POFF = 0,
    #[doc = "1: CLKA clock is clock selected by PREA"]
    PREA = 1,
}
impl From<DIVA_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVA`"]
pub type DIVA_R = crate::R<u8, DIVA_A>;
impl DIVA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVA_A::CLKA_POFF),
            1 => Val(DIVA_A::PREA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLKA_POFF`"]
    #[inline(always)]
    pub fn is_clka_poff(&self) -> bool {
        *self == DIVA_A::CLKA_POFF
    }
    #[doc = "Checks if the value of the field is `PREA`"]
    #[inline(always)]
    pub fn is_prea(&self) -> bool {
        *self == DIVA_A::PREA
    }
}
#[doc = "Write proxy for field `DIVA`"]
pub struct DIVA_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CLKA clock is turned off"]
    #[inline(always)]
    pub fn clka_poff(self) -> &'a mut W {
        self.variant(DIVA_A::CLKA_POFF)
    }
    #[doc = "CLKA clock is clock selected by PREA"]
    #[inline(always)]
    pub fn prea(self) -> &'a mut W {
        self.variant(DIVA_A::PREA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "CLKA Source Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PREA_A {
    #[doc = "0: Peripheral clock"]
    CLK = 0,
    #[doc = "1: Peripheral clock/2"]
    CLK_DIV2 = 1,
    #[doc = "2: Peripheral clock/4"]
    CLK_DIV4 = 2,
    #[doc = "3: Peripheral clock/8"]
    CLK_DIV8 = 3,
    #[doc = "4: Peripheral clock/16"]
    CLK_DIV16 = 4,
    #[doc = "5: Peripheral clock/32"]
    CLK_DIV32 = 5,
    #[doc = "6: Peripheral clock/64"]
    CLK_DIV64 = 6,
    #[doc = "7: Peripheral clock/128"]
    CLK_DIV128 = 7,
    #[doc = "8: Peripheral clock/256"]
    CLK_DIV256 = 8,
    #[doc = "9: Peripheral clock/512"]
    CLK_DIV512 = 9,
    #[doc = "10: Peripheral clock/1024"]
    CLK_DIV1024 = 10,
}
impl From<PREA_A> for u8 {
    #[inline(always)]
    fn from(variant: PREA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PREA`"]
pub type PREA_R = crate::R<u8, PREA_A>;
impl PREA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PREA_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PREA_A::CLK),
            1 => Val(PREA_A::CLK_DIV2),
            2 => Val(PREA_A::CLK_DIV4),
            3 => Val(PREA_A::CLK_DIV8),
            4 => Val(PREA_A::CLK_DIV16),
            5 => Val(PREA_A::CLK_DIV32),
            6 => Val(PREA_A::CLK_DIV64),
            7 => Val(PREA_A::CLK_DIV128),
            8 => Val(PREA_A::CLK_DIV256),
            9 => Val(PREA_A::CLK_DIV512),
            10 => Val(PREA_A::CLK_DIV1024),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLK`"]
    #[inline(always)]
    pub fn is_clk(&self) -> bool {
        *self == PREA_A::CLK
    }
    #[doc = "Checks if the value of the field is `CLK_DIV2`"]
    #[inline(always)]
    pub fn is_clk_div2(&self) -> bool {
        *self == PREA_A::CLK_DIV2
    }
    #[doc = "Checks if the value of the field is `CLK_DIV4`"]
    #[inline(always)]
    pub fn is_clk_div4(&self) -> bool {
        *self == PREA_A::CLK_DIV4
    }
    #[doc = "Checks if the value of the field is `CLK_DIV8`"]
    #[inline(always)]
    pub fn is_clk_div8(&self) -> bool {
        *self == PREA_A::CLK_DIV8
    }
    #[doc = "Checks if the value of the field is `CLK_DIV16`"]
    #[inline(always)]
    pub fn is_clk_div16(&self) -> bool {
        *self == PREA_A::CLK_DIV16
    }
    #[doc = "Checks if the value of the field is `CLK_DIV32`"]
    #[inline(always)]
    pub fn is_clk_div32(&self) -> bool {
        *self == PREA_A::CLK_DIV32
    }
    #[doc = "Checks if the value of the field is `CLK_DIV64`"]
    #[inline(always)]
    pub fn is_clk_div64(&self) -> bool {
        *self == PREA_A::CLK_DIV64
    }
    #[doc = "Checks if the value of the field is `CLK_DIV128`"]
    #[inline(always)]
    pub fn is_clk_div128(&self) -> bool {
        *self == PREA_A::CLK_DIV128
    }
    #[doc = "Checks if the value of the field is `CLK_DIV256`"]
    #[inline(always)]
    pub fn is_clk_div256(&self) -> bool {
        *self == PREA_A::CLK_DIV256
    }
    #[doc = "Checks if the value of the field is `CLK_DIV512`"]
    #[inline(always)]
    pub fn is_clk_div512(&self) -> bool {
        *self == PREA_A::CLK_DIV512
    }
    #[doc = "Checks if the value of the field is `CLK_DIV1024`"]
    #[inline(always)]
    pub fn is_clk_div1024(&self) -> bool {
        *self == PREA_A::CLK_DIV1024
    }
}
#[doc = "Write proxy for field `PREA`"]
pub struct PREA_W<'a> {
    w: &'a mut W,
}
impl<'a> PREA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREA_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn clk(self) -> &'a mut W {
        self.variant(PREA_A::CLK)
    }
    #[doc = "Peripheral clock/2"]
    #[inline(always)]
    pub fn clk_div2(self) -> &'a mut W {
        self.variant(PREA_A::CLK_DIV2)
    }
    #[doc = "Peripheral clock/4"]
    #[inline(always)]
    pub fn clk_div4(self) -> &'a mut W {
        self.variant(PREA_A::CLK_DIV4)
    }
    #[doc = "Peripheral clock/8"]
    #[inline(always)]
    pub fn clk_div8(self) -> &'a mut W {
        self.variant(PREA_A::CLK_DIV8)
    }
    #[doc = "Peripheral clock/16"]
    #[inline(always)]
    pub fn clk_div16(self) -> &'a mut W {
        self.variant(PREA_A::CLK_DIV16)
    }
    #[doc = "Peripheral clock/32"]
    #[inline(always)]
    pub fn clk_div32(self) -> &'a mut W {
        self.variant(PREA_A::CLK_DIV32)
    }
    #[doc = "Peripheral clock/64"]
    #[inline(always)]
    pub fn clk_div64(self) -> &'a mut W {
        self.variant(PREA_A::CLK_DIV64)
    }
    #[doc = "Peripheral clock/128"]
    #[inline(always)]
    pub fn clk_div128(self) -> &'a mut W {
        self.variant(PREA_A::CLK_DIV128)
    }
    #[doc = "Peripheral clock/256"]
    #[inline(always)]
    pub fn clk_div256(self) -> &'a mut W {
        self.variant(PREA_A::CLK_DIV256)
    }
    #[doc = "Peripheral clock/512"]
    #[inline(always)]
    pub fn clk_div512(self) -> &'a mut W {
        self.variant(PREA_A::CLK_DIV512)
    }
    #[doc = "Peripheral clock/1024"]
    #[inline(always)]
    pub fn clk_div1024(self) -> &'a mut W {
        self.variant(PREA_A::CLK_DIV1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "CLKB Divide Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVB_A {
    #[doc = "0: CLKB clock is turned off"]
    CLKB_POFF = 0,
    #[doc = "1: CLKB clock is clock selected by PREB"]
    PREB = 1,
}
impl From<DIVB_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVB`"]
pub type DIVB_R = crate::R<u8, DIVB_A>;
impl DIVB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVB_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVB_A::CLKB_POFF),
            1 => Val(DIVB_A::PREB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLKB_POFF`"]
    #[inline(always)]
    pub fn is_clkb_poff(&self) -> bool {
        *self == DIVB_A::CLKB_POFF
    }
    #[doc = "Checks if the value of the field is `PREB`"]
    #[inline(always)]
    pub fn is_preb(&self) -> bool {
        *self == DIVB_A::PREB
    }
}
#[doc = "Write proxy for field `DIVB`"]
pub struct DIVB_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CLKB clock is turned off"]
    #[inline(always)]
    pub fn clkb_poff(self) -> &'a mut W {
        self.variant(DIVB_A::CLKB_POFF)
    }
    #[doc = "CLKB clock is clock selected by PREB"]
    #[inline(always)]
    pub fn preb(self) -> &'a mut W {
        self.variant(DIVB_A::PREB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "CLKB Source Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PREB_A {
    #[doc = "0: Peripheral clock"]
    CLK = 0,
    #[doc = "1: Peripheral clock/2"]
    CLK_DIV2 = 1,
    #[doc = "2: Peripheral clock/4"]
    CLK_DIV4 = 2,
    #[doc = "3: Peripheral clock/8"]
    CLK_DIV8 = 3,
    #[doc = "4: Peripheral clock/16"]
    CLK_DIV16 = 4,
    #[doc = "5: Peripheral clock/32"]
    CLK_DIV32 = 5,
    #[doc = "6: Peripheral clock/64"]
    CLK_DIV64 = 6,
    #[doc = "7: Peripheral clock/128"]
    CLK_DIV128 = 7,
    #[doc = "8: Peripheral clock/256"]
    CLK_DIV256 = 8,
    #[doc = "9: Peripheral clock/512"]
    CLK_DIV512 = 9,
    #[doc = "10: Peripheral clock/1024"]
    CLK_DIV1024 = 10,
}
impl From<PREB_A> for u8 {
    #[inline(always)]
    fn from(variant: PREB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PREB`"]
pub type PREB_R = crate::R<u8, PREB_A>;
impl PREB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PREB_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PREB_A::CLK),
            1 => Val(PREB_A::CLK_DIV2),
            2 => Val(PREB_A::CLK_DIV4),
            3 => Val(PREB_A::CLK_DIV8),
            4 => Val(PREB_A::CLK_DIV16),
            5 => Val(PREB_A::CLK_DIV32),
            6 => Val(PREB_A::CLK_DIV64),
            7 => Val(PREB_A::CLK_DIV128),
            8 => Val(PREB_A::CLK_DIV256),
            9 => Val(PREB_A::CLK_DIV512),
            10 => Val(PREB_A::CLK_DIV1024),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLK`"]
    #[inline(always)]
    pub fn is_clk(&self) -> bool {
        *self == PREB_A::CLK
    }
    #[doc = "Checks if the value of the field is `CLK_DIV2`"]
    #[inline(always)]
    pub fn is_clk_div2(&self) -> bool {
        *self == PREB_A::CLK_DIV2
    }
    #[doc = "Checks if the value of the field is `CLK_DIV4`"]
    #[inline(always)]
    pub fn is_clk_div4(&self) -> bool {
        *self == PREB_A::CLK_DIV4
    }
    #[doc = "Checks if the value of the field is `CLK_DIV8`"]
    #[inline(always)]
    pub fn is_clk_div8(&self) -> bool {
        *self == PREB_A::CLK_DIV8
    }
    #[doc = "Checks if the value of the field is `CLK_DIV16`"]
    #[inline(always)]
    pub fn is_clk_div16(&self) -> bool {
        *self == PREB_A::CLK_DIV16
    }
    #[doc = "Checks if the value of the field is `CLK_DIV32`"]
    #[inline(always)]
    pub fn is_clk_div32(&self) -> bool {
        *self == PREB_A::CLK_DIV32
    }
    #[doc = "Checks if the value of the field is `CLK_DIV64`"]
    #[inline(always)]
    pub fn is_clk_div64(&self) -> bool {
        *self == PREB_A::CLK_DIV64
    }
    #[doc = "Checks if the value of the field is `CLK_DIV128`"]
    #[inline(always)]
    pub fn is_clk_div128(&self) -> bool {
        *self == PREB_A::CLK_DIV128
    }
    #[doc = "Checks if the value of the field is `CLK_DIV256`"]
    #[inline(always)]
    pub fn is_clk_div256(&self) -> bool {
        *self == PREB_A::CLK_DIV256
    }
    #[doc = "Checks if the value of the field is `CLK_DIV512`"]
    #[inline(always)]
    pub fn is_clk_div512(&self) -> bool {
        *self == PREB_A::CLK_DIV512
    }
    #[doc = "Checks if the value of the field is `CLK_DIV1024`"]
    #[inline(always)]
    pub fn is_clk_div1024(&self) -> bool {
        *self == PREB_A::CLK_DIV1024
    }
}
#[doc = "Write proxy for field `PREB`"]
pub struct PREB_W<'a> {
    w: &'a mut W,
}
impl<'a> PREB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn clk(self) -> &'a mut W {
        self.variant(PREB_A::CLK)
    }
    #[doc = "Peripheral clock/2"]
    #[inline(always)]
    pub fn clk_div2(self) -> &'a mut W {
        self.variant(PREB_A::CLK_DIV2)
    }
    #[doc = "Peripheral clock/4"]
    #[inline(always)]
    pub fn clk_div4(self) -> &'a mut W {
        self.variant(PREB_A::CLK_DIV4)
    }
    #[doc = "Peripheral clock/8"]
    #[inline(always)]
    pub fn clk_div8(self) -> &'a mut W {
        self.variant(PREB_A::CLK_DIV8)
    }
    #[doc = "Peripheral clock/16"]
    #[inline(always)]
    pub fn clk_div16(self) -> &'a mut W {
        self.variant(PREB_A::CLK_DIV16)
    }
    #[doc = "Peripheral clock/32"]
    #[inline(always)]
    pub fn clk_div32(self) -> &'a mut W {
        self.variant(PREB_A::CLK_DIV32)
    }
    #[doc = "Peripheral clock/64"]
    #[inline(always)]
    pub fn clk_div64(self) -> &'a mut W {
        self.variant(PREB_A::CLK_DIV64)
    }
    #[doc = "Peripheral clock/128"]
    #[inline(always)]
    pub fn clk_div128(self) -> &'a mut W {
        self.variant(PREB_A::CLK_DIV128)
    }
    #[doc = "Peripheral clock/256"]
    #[inline(always)]
    pub fn clk_div256(self) -> &'a mut W {
        self.variant(PREB_A::CLK_DIV256)
    }
    #[doc = "Peripheral clock/512"]
    #[inline(always)]
    pub fn clk_div512(self) -> &'a mut W {
        self.variant(PREB_A::CLK_DIV512)
    }
    #[doc = "Peripheral clock/1024"]
    #[inline(always)]
    pub fn clk_div1024(self) -> &'a mut W {
        self.variant(PREB_A::CLK_DIV1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CLKA Divide Factor"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - CLKA Source Clock Selection"]
    #[inline(always)]
    pub fn prea(&self) -> PREA_R {
        PREA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - CLKB Divide Factor"]
    #[inline(always)]
    pub fn divb(&self) -> DIVB_R {
        DIVB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - CLKB Source Clock Selection"]
    #[inline(always)]
    pub fn preb(&self) -> PREB_R {
        PREB_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CLKA Divide Factor"]
    #[inline(always)]
    pub fn diva(&mut self) -> DIVA_W {
        DIVA_W { w: self }
    }
    #[doc = "Bits 8:11 - CLKA Source Clock Selection"]
    #[inline(always)]
    pub fn prea(&mut self) -> PREA_W {
        PREA_W { w: self }
    }
    #[doc = "Bits 16:23 - CLKB Divide Factor"]
    #[inline(always)]
    pub fn divb(&mut self) -> DIVB_W {
        DIVB_W { w: self }
    }
    #[doc = "Bits 24:27 - CLKB Source Clock Selection"]
    #[inline(always)]
    pub fn preb(&mut self) -> PREB_W {
        PREB_W { w: self }
    }
}
