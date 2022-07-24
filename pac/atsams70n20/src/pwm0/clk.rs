#[doc = "Register `CLK` reader"]
pub struct R(crate::R<CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK` writer"]
pub struct W(crate::W<CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_SPEC>;
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
impl From<crate::W<CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `DIVA` reader - CLKA Divide Factor"]
pub type DIVA_R = crate::FieldReader<u8, DIVA_A>;
impl DIVA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVA_A> {
        match self.bits {
            0 => Some(DIVA_A::CLKA_POFF),
            1 => Some(DIVA_A::PREA),
            _ => None,
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
#[doc = "Field `DIVA` writer - CLKA Divide Factor"]
pub type DIVA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_SPEC, u8, DIVA_A, 8, O>;
impl<'a, const O: u8> DIVA_W<'a, O> {
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
#[doc = "Field `PREA` reader - CLKA Source Clock Selection"]
pub type PREA_R = crate::FieldReader<u8, PREA_A>;
impl PREA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PREA_A> {
        match self.bits {
            0 => Some(PREA_A::CLK),
            1 => Some(PREA_A::CLK_DIV2),
            2 => Some(PREA_A::CLK_DIV4),
            3 => Some(PREA_A::CLK_DIV8),
            4 => Some(PREA_A::CLK_DIV16),
            5 => Some(PREA_A::CLK_DIV32),
            6 => Some(PREA_A::CLK_DIV64),
            7 => Some(PREA_A::CLK_DIV128),
            8 => Some(PREA_A::CLK_DIV256),
            9 => Some(PREA_A::CLK_DIV512),
            10 => Some(PREA_A::CLK_DIV1024),
            _ => None,
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
#[doc = "Field `PREA` writer - CLKA Source Clock Selection"]
pub type PREA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_SPEC, u8, PREA_A, 4, O>;
impl<'a, const O: u8> PREA_W<'a, O> {
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
#[doc = "Field `DIVB` reader - CLKB Divide Factor"]
pub type DIVB_R = crate::FieldReader<u8, DIVB_A>;
impl DIVB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIVB_A> {
        match self.bits {
            0 => Some(DIVB_A::CLKB_POFF),
            1 => Some(DIVB_A::PREB),
            _ => None,
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
#[doc = "Field `DIVB` writer - CLKB Divide Factor"]
pub type DIVB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_SPEC, u8, DIVB_A, 8, O>;
impl<'a, const O: u8> DIVB_W<'a, O> {
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
#[doc = "Field `PREB` reader - CLKB Source Clock Selection"]
pub type PREB_R = crate::FieldReader<u8, PREB_A>;
impl PREB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PREB_A> {
        match self.bits {
            0 => Some(PREB_A::CLK),
            1 => Some(PREB_A::CLK_DIV2),
            2 => Some(PREB_A::CLK_DIV4),
            3 => Some(PREB_A::CLK_DIV8),
            4 => Some(PREB_A::CLK_DIV16),
            5 => Some(PREB_A::CLK_DIV32),
            6 => Some(PREB_A::CLK_DIV64),
            7 => Some(PREB_A::CLK_DIV128),
            8 => Some(PREB_A::CLK_DIV256),
            9 => Some(PREB_A::CLK_DIV512),
            10 => Some(PREB_A::CLK_DIV1024),
            _ => None,
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
#[doc = "Field `PREB` writer - CLKB Source Clock Selection"]
pub type PREB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_SPEC, u8, PREB_A, 4, O>;
impl<'a, const O: u8> PREB_W<'a, O> {
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
    pub fn diva(&mut self) -> DIVA_W<0> {
        DIVA_W::new(self)
    }
    #[doc = "Bits 8:11 - CLKA Source Clock Selection"]
    #[inline(always)]
    pub fn prea(&mut self) -> PREA_W<8> {
        PREA_W::new(self)
    }
    #[doc = "Bits 16:23 - CLKB Divide Factor"]
    #[inline(always)]
    pub fn divb(&mut self) -> DIVB_W<16> {
        DIVB_W::new(self)
    }
    #[doc = "Bits 24:27 - CLKB Source Clock Selection"]
    #[inline(always)]
    pub fn preb(&mut self) -> PREB_W<24> {
        PREB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk](index.html) module"]
pub struct CLK_SPEC;
impl crate::RegisterSpec for CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk::R](R) reader structure"]
impl crate::Readable for CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk::W](W) writer structure"]
impl crate::Writable for CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK to value 0"]
impl crate::Resettable for CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
