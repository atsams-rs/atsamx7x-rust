#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMC_PCK {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `CSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSR {
    #[doc = "SLCK is selected"]
    SLOW_CLK,
    #[doc = "MAINCK is selected"]
    MAIN_CLK,
    #[doc = "PLLACK is selected"]
    PLLA_CLK,
    #[doc = "UPLLCKDIV is selected"]
    UPLL_CLK,
    #[doc = "MCK is selected"]
    MCK,
}
impl crate::ToBits<u8> for CSSR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CSSR::SLOW_CLK => 0,
            CSSR::MAIN_CLK => 1,
            CSSR::PLLA_CLK => 2,
            CSSR::UPLL_CLK => 3,
            CSSR::MCK => 4,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CSS_R = crate::FR<u8, CSSR>;
impl CSS_R {
    #[doc = "Checks if the value of the field is `SLOW_CLK`"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        *self == CSSR::SLOW_CLK
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == CSSR::MAIN_CLK
    }
    #[doc = "Checks if the value of the field is `PLLA_CLK`"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        *self == CSSR::PLLA_CLK
    }
    #[doc = "Checks if the value of the field is `UPLL_CLK`"]
    #[inline(always)]
    pub fn is_upll_clk(&self) -> bool {
        *self == CSSR::UPLL_CLK
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == CSSR::MCK
    }
}
#[doc = "Values that can be written to the field `CSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSSW {
    #[doc = "SLCK is selected"]
    SLOW_CLK,
    #[doc = "MAINCK is selected"]
    MAIN_CLK,
    #[doc = "PLLACK is selected"]
    PLLA_CLK,
    #[doc = "UPLLCKDIV is selected"]
    UPLL_CLK,
    #[doc = "MCK is selected"]
    MCK,
}
impl CSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSSW::SLOW_CLK => 0,
            CSSW::MAIN_CLK => 1,
            CSSW::PLLA_CLK => 2,
            CSSW::UPLL_CLK => 3,
            CSSW::MCK => 4,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CSSW<'a> {
    w: &'a mut W,
}
impl<'a> _CSSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SLCK is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut W {
        self.variant(CSSW::SLOW_CLK)
    }
    #[doc = "MAINCK is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(CSSW::MAIN_CLK)
    }
    #[doc = "PLLACK is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut W {
        self.variant(CSSW::PLLA_CLK)
    }
    #[doc = "UPLLCKDIV is selected"]
    #[inline(always)]
    pub fn upll_clk(self) -> &'a mut W {
        self.variant(CSSW::UPLL_CLK)
    }
    #[doc = "MCK is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(CSSW::MCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PRES_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PRESW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 4)) | (((value as u32) & 0xff) << 4);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Programmable Clock Source Selection"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new((self.bits() & 0x07) as u8)
    }
    #[doc = "Bits 4:11 - Programmable Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits() >> 4) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Programmable Clock Source Selection"]
    #[inline(always)]
    pub fn css(&mut self) -> _CSSW {
        _CSSW { w: self }
    }
    #[doc = "Bits 4:11 - Programmable Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&mut self) -> _PRESW {
        _PRESW { w: self }
    }
}
