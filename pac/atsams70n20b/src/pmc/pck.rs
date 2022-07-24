#[doc = "Register `PCK[%s]` reader"]
pub struct R(crate::R<PCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCK[%s]` writer"]
pub struct W(crate::W<PCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCK_SPEC>;
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
impl From<crate::W<PCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Programmable Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSS_A {
    #[doc = "0: SLCK is selected"]
    SLOW_CLK = 0,
    #[doc = "1: MAINCK is selected"]
    MAIN_CLK = 1,
    #[doc = "2: PLLACK is selected"]
    PLLA_CLK = 2,
    #[doc = "3: UPLLCKDIV is selected"]
    UPLL_CLK = 3,
    #[doc = "4: MCK is selected"]
    MCK = 4,
}
impl From<CSS_A> for u8 {
    #[inline(always)]
    fn from(variant: CSS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CSS` reader - Programmable Clock Source Selection"]
pub type CSS_R = crate::FieldReader<u8, CSS_A>;
impl CSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSS_A> {
        match self.bits {
            0 => Some(CSS_A::SLOW_CLK),
            1 => Some(CSS_A::MAIN_CLK),
            2 => Some(CSS_A::PLLA_CLK),
            3 => Some(CSS_A::UPLL_CLK),
            4 => Some(CSS_A::MCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SLOW_CLK`"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        *self == CSS_A::SLOW_CLK
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == CSS_A::MAIN_CLK
    }
    #[doc = "Checks if the value of the field is `PLLA_CLK`"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        *self == CSS_A::PLLA_CLK
    }
    #[doc = "Checks if the value of the field is `UPLL_CLK`"]
    #[inline(always)]
    pub fn is_upll_clk(&self) -> bool {
        *self == CSS_A::UPLL_CLK
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == CSS_A::MCK
    }
}
#[doc = "Field `CSS` writer - Programmable Clock Source Selection"]
pub type CSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCK_SPEC, u8, CSS_A, 3, O>;
impl<'a, const O: u8> CSS_W<'a, O> {
    #[doc = "SLCK is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut W {
        self.variant(CSS_A::SLOW_CLK)
    }
    #[doc = "MAINCK is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(CSS_A::MAIN_CLK)
    }
    #[doc = "PLLACK is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut W {
        self.variant(CSS_A::PLLA_CLK)
    }
    #[doc = "UPLLCKDIV is selected"]
    #[inline(always)]
    pub fn upll_clk(self) -> &'a mut W {
        self.variant(CSS_A::UPLL_CLK)
    }
    #[doc = "MCK is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(CSS_A::MCK)
    }
}
#[doc = "Field `PRES` reader - Programmable Clock Prescaler"]
pub type PRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRES` writer - Programmable Clock Prescaler"]
pub type PRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCK_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:2 - Programmable Clock Source Selection"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:11 - Programmable Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 4) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Programmable Clock Source Selection"]
    #[inline(always)]
    pub fn css(&mut self) -> CSS_W<0> {
        CSS_W::new(self)
    }
    #[doc = "Bits 4:11 - Programmable Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&mut self) -> PRES_W<4> {
        PRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Programmable Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pck](index.html) module"]
pub struct PCK_SPEC;
impl crate::RegisterSpec for PCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pck::R](R) reader structure"]
impl crate::Readable for PCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pck::W](W) writer structure"]
impl crate::Writable for PCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCK[%s]
to value 0"]
impl crate::Resettable for PCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
