#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMC_PCR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct PIDR {
    bits: u8,
}
impl PIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `GCLKCSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCLKCSSR {
    #[doc = "Slow clock is selected"]
    SLOW_CLK,
    #[doc = "Main clock is selected"]
    MAIN_CLK,
    #[doc = "PLLACK is selected"]
    PLLA_CLK,
    #[doc = "UPLL Clock is selected"]
    UPLL_CLK,
    #[doc = "Master Clock is selected"]
    MCK_CLK,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GCLKCSSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GCLKCSSR::SLOW_CLK => 0,
            GCLKCSSR::MAIN_CLK => 1,
            GCLKCSSR::PLLA_CLK => 2,
            GCLKCSSR::UPLL_CLK => 3,
            GCLKCSSR::MCK_CLK => 4,
            GCLKCSSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GCLKCSSR {
        match value {
            0 => GCLKCSSR::SLOW_CLK,
            1 => GCLKCSSR::MAIN_CLK,
            2 => GCLKCSSR::PLLA_CLK,
            3 => GCLKCSSR::UPLL_CLK,
            4 => GCLKCSSR::MCK_CLK,
            i => GCLKCSSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SLOW_CLK`"]
    #[inline]
    pub fn is_slow_clk(&self) -> bool {
        *self == GCLKCSSR::SLOW_CLK
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline]
    pub fn is_main_clk(&self) -> bool {
        *self == GCLKCSSR::MAIN_CLK
    }
    #[doc = "Checks if the value of the field is `PLLA_CLK`"]
    #[inline]
    pub fn is_plla_clk(&self) -> bool {
        *self == GCLKCSSR::PLLA_CLK
    }
    #[doc = "Checks if the value of the field is `UPLL_CLK`"]
    #[inline]
    pub fn is_upll_clk(&self) -> bool {
        *self == GCLKCSSR::UPLL_CLK
    }
    #[doc = "Checks if the value of the field is `MCK_CLK`"]
    #[inline]
    pub fn is_mck_clk(&self) -> bool {
        *self == GCLKCSSR::MCK_CLK
    }
}
#[doc = r" Value of the field"]
pub struct CMDR {
    bits: bool,
}
impl CMDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct GCLKDIVR {
    bits: u8,
}
impl GCLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ENR {
    bits: bool,
}
impl ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct GCLKENR {
    bits: bool,
}
impl GCLKENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _PIDW<'a> {
    w: &'a mut W,
}
impl<'a> _PIDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GCLKCSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCLKCSSW {
    #[doc = "Slow clock is selected"]
    SLOW_CLK,
    #[doc = "Main clock is selected"]
    MAIN_CLK,
    #[doc = "PLLACK is selected"]
    PLLA_CLK,
    #[doc = "UPLL Clock is selected"]
    UPLL_CLK,
    #[doc = "Master Clock is selected"]
    MCK_CLK,
}
impl GCLKCSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GCLKCSSW::SLOW_CLK => 0,
            GCLKCSSW::MAIN_CLK => 1,
            GCLKCSSW::PLLA_CLK => 2,
            GCLKCSSW::UPLL_CLK => 3,
            GCLKCSSW::MCK_CLK => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GCLKCSSW<'a> {
    w: &'a mut W,
}
impl<'a> _GCLKCSSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GCLKCSSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Slow clock is selected"]
    #[inline]
    pub fn slow_clk(self) -> &'a mut W {
        self.variant(GCLKCSSW::SLOW_CLK)
    }
    #[doc = "Main clock is selected"]
    #[inline]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(GCLKCSSW::MAIN_CLK)
    }
    #[doc = "PLLACK is selected"]
    #[inline]
    pub fn plla_clk(self) -> &'a mut W {
        self.variant(GCLKCSSW::PLLA_CLK)
    }
    #[doc = "UPLL Clock is selected"]
    #[inline]
    pub fn upll_clk(self) -> &'a mut W {
        self.variant(GCLKCSSW::UPLL_CLK)
    }
    #[doc = "Master Clock is selected"]
    #[inline]
    pub fn mck_clk(self) -> &'a mut W {
        self.variant(GCLKCSSW::MCK_CLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMDW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GCLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _GCLKDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _GCLKENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - Peripheral ID"]
    #[inline]
    pub fn pid(&self) -> PIDR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PIDR { bits }
    }
    #[doc = "Bits 8:10 - Generic Clock Source Selection"]
    #[inline]
    pub fn gclkcss(&self) -> GCLKCSSR {
        GCLKCSSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Command"]
    #[inline]
    pub fn cmd(&self) -> CMDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CMDR { bits }
    }
    #[doc = "Bits 20:27 - Generic Clock Division Ratio"]
    #[inline]
    pub fn gclkdiv(&self) -> GCLKDIVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GCLKDIVR { bits }
    }
    #[doc = "Bit 28 - Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
    #[doc = "Bit 29 - Generic Clock Enable"]
    #[inline]
    pub fn gclken(&self) -> GCLKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GCLKENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Peripheral ID"]
    #[inline]
    pub fn pid(&mut self) -> _PIDW {
        _PIDW { w: self }
    }
    #[doc = "Bits 8:10 - Generic Clock Source Selection"]
    #[inline]
    pub fn gclkcss(&mut self) -> _GCLKCSSW {
        _GCLKCSSW { w: self }
    }
    #[doc = "Bit 12 - Command"]
    #[inline]
    pub fn cmd(&mut self) -> _CMDW {
        _CMDW { w: self }
    }
    #[doc = "Bits 20:27 - Generic Clock Division Ratio"]
    #[inline]
    pub fn gclkdiv(&mut self) -> _GCLKDIVW {
        _GCLKDIVW { w: self }
    }
    #[doc = "Bit 28 - Enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bit 29 - Generic Clock Enable"]
    #[inline]
    pub fn gclken(&mut self) -> _GCLKENW {
        _GCLKENW { w: self }
    }
}
