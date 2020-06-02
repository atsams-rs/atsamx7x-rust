#[doc = "Reader of register PMC_PCR"]
pub type R = crate::R<u32, super::PMC_PCR>;
#[doc = "Writer for register PMC_PCR"]
pub type W = crate::W<u32, super::PMC_PCR>;
#[doc = "Register PMC_PCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PMC_PCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PID`"]
pub type PID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PID`"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Generic Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GCLKCSS_A {
    #[doc = "0: Slow clock is selected"]
    SLOW_CLK = 0,
    #[doc = "1: Main clock is selected"]
    MAIN_CLK = 1,
    #[doc = "2: PLLACK is selected"]
    PLLA_CLK = 2,
    #[doc = "3: UPLL Clock is selected"]
    UPLL_CLK = 3,
    #[doc = "4: Master Clock is selected"]
    MCK_CLK = 4,
}
impl From<GCLKCSS_A> for u8 {
    #[inline(always)]
    fn from(variant: GCLKCSS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GCLKCSS`"]
pub type GCLKCSS_R = crate::R<u8, GCLKCSS_A>;
impl GCLKCSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GCLKCSS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GCLKCSS_A::SLOW_CLK),
            1 => Val(GCLKCSS_A::MAIN_CLK),
            2 => Val(GCLKCSS_A::PLLA_CLK),
            3 => Val(GCLKCSS_A::UPLL_CLK),
            4 => Val(GCLKCSS_A::MCK_CLK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLOW_CLK`"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        *self == GCLKCSS_A::SLOW_CLK
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == GCLKCSS_A::MAIN_CLK
    }
    #[doc = "Checks if the value of the field is `PLLA_CLK`"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        *self == GCLKCSS_A::PLLA_CLK
    }
    #[doc = "Checks if the value of the field is `UPLL_CLK`"]
    #[inline(always)]
    pub fn is_upll_clk(&self) -> bool {
        *self == GCLKCSS_A::UPLL_CLK
    }
    #[doc = "Checks if the value of the field is `MCK_CLK`"]
    #[inline(always)]
    pub fn is_mck_clk(&self) -> bool {
        *self == GCLKCSS_A::MCK_CLK
    }
}
#[doc = "Write proxy for field `GCLKCSS`"]
pub struct GCLKCSS_W<'a> {
    w: &'a mut W,
}
impl<'a> GCLKCSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GCLKCSS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Slow clock is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut W {
        self.variant(GCLKCSS_A::SLOW_CLK)
    }
    #[doc = "Main clock is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(GCLKCSS_A::MAIN_CLK)
    }
    #[doc = "PLLACK is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut W {
        self.variant(GCLKCSS_A::PLLA_CLK)
    }
    #[doc = "UPLL Clock is selected"]
    #[inline(always)]
    pub fn upll_clk(self) -> &'a mut W {
        self.variant(GCLKCSS_A::UPLL_CLK)
    }
    #[doc = "Master Clock is selected"]
    #[inline(always)]
    pub fn mck_clk(self) -> &'a mut W {
        self.variant(GCLKCSS_A::MCK_CLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `CMD`"]
pub type CMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `GCLKDIV`"]
pub type GCLKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GCLKDIV`"]
pub struct GCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> GCLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | (((value as u32) & 0xff) << 20);
        self.w
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `GCLKEN`"]
pub type GCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCLKEN`"]
pub struct GCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GCLKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Peripheral ID"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - Generic Clock Source Selection"]
    #[inline(always)]
    pub fn gclkcss(&self) -> GCLKCSS_R {
        GCLKCSS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Command"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 20:27 - Generic Clock Division Ratio"]
    #[inline(always)]
    pub fn gclkdiv(&self) -> GCLKDIV_R {
        GCLKDIV_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 28 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Generic Clock Enable"]
    #[inline(always)]
    pub fn gclken(&self) -> GCLKEN_R {
        GCLKEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Peripheral ID"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
    #[doc = "Bits 8:10 - Generic Clock Source Selection"]
    #[inline(always)]
    pub fn gclkcss(&mut self) -> GCLKCSS_W {
        GCLKCSS_W { w: self }
    }
    #[doc = "Bit 12 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Bits 20:27 - Generic Clock Division Ratio"]
    #[inline(always)]
    pub fn gclkdiv(&mut self) -> GCLKDIV_W {
        GCLKDIV_W { w: self }
    }
    #[doc = "Bit 28 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 29 - Generic Clock Enable"]
    #[inline(always)]
    pub fn gclken(&mut self) -> GCLKEN_W {
        GCLKEN_W { w: self }
    }
}
