#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWM_CLK {
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
#[doc = "Possible values of the field `DIVA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVAR {
    #[doc = "CLKA clock is turned off"]
    CLKA_POFF,
    #[doc = "CLKA clock is clock selected by PREA"]
    PREA,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DIVAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIVAR::CLKA_POFF => 0,
            DIVAR::PREA => 1,
            DIVAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIVAR {
        match value {
            0 => DIVAR::CLKA_POFF,
            1 => DIVAR::PREA,
            i => DIVAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLKA_POFF`"]
    #[inline]
    pub fn is_clka_poff(&self) -> bool {
        *self == DIVAR::CLKA_POFF
    }
    #[doc = "Checks if the value of the field is `PREA`"]
    #[inline]
    pub fn is_prea(&self) -> bool {
        *self == DIVAR::PREA
    }
}
#[doc = "Possible values of the field `PREA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREAR {
    #[doc = "Peripheral clock"]
    CLK,
    #[doc = "Peripheral clock/2"]
    CLK_DIV2,
    #[doc = "Peripheral clock/4"]
    CLK_DIV4,
    #[doc = "Peripheral clock/8"]
    CLK_DIV8,
    #[doc = "Peripheral clock/16"]
    CLK_DIV16,
    #[doc = "Peripheral clock/32"]
    CLK_DIV32,
    #[doc = "Peripheral clock/64"]
    CLK_DIV64,
    #[doc = "Peripheral clock/128"]
    CLK_DIV128,
    #[doc = "Peripheral clock/256"]
    CLK_DIV256,
    #[doc = "Peripheral clock/512"]
    CLK_DIV512,
    #[doc = "Peripheral clock/1024"]
    CLK_DIV1024,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PREAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PREAR::CLK => 0,
            PREAR::CLK_DIV2 => 1,
            PREAR::CLK_DIV4 => 2,
            PREAR::CLK_DIV8 => 3,
            PREAR::CLK_DIV16 => 4,
            PREAR::CLK_DIV32 => 5,
            PREAR::CLK_DIV64 => 6,
            PREAR::CLK_DIV128 => 7,
            PREAR::CLK_DIV256 => 8,
            PREAR::CLK_DIV512 => 9,
            PREAR::CLK_DIV1024 => 10,
            PREAR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PREAR {
        match value {
            0 => PREAR::CLK,
            1 => PREAR::CLK_DIV2,
            2 => PREAR::CLK_DIV4,
            3 => PREAR::CLK_DIV8,
            4 => PREAR::CLK_DIV16,
            5 => PREAR::CLK_DIV32,
            6 => PREAR::CLK_DIV64,
            7 => PREAR::CLK_DIV128,
            8 => PREAR::CLK_DIV256,
            9 => PREAR::CLK_DIV512,
            10 => PREAR::CLK_DIV1024,
            i => PREAR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLK`"]
    #[inline]
    pub fn is_clk(&self) -> bool {
        *self == PREAR::CLK
    }
    #[doc = "Checks if the value of the field is `CLK_DIV2`"]
    #[inline]
    pub fn is_clk_div2(&self) -> bool {
        *self == PREAR::CLK_DIV2
    }
    #[doc = "Checks if the value of the field is `CLK_DIV4`"]
    #[inline]
    pub fn is_clk_div4(&self) -> bool {
        *self == PREAR::CLK_DIV4
    }
    #[doc = "Checks if the value of the field is `CLK_DIV8`"]
    #[inline]
    pub fn is_clk_div8(&self) -> bool {
        *self == PREAR::CLK_DIV8
    }
    #[doc = "Checks if the value of the field is `CLK_DIV16`"]
    #[inline]
    pub fn is_clk_div16(&self) -> bool {
        *self == PREAR::CLK_DIV16
    }
    #[doc = "Checks if the value of the field is `CLK_DIV32`"]
    #[inline]
    pub fn is_clk_div32(&self) -> bool {
        *self == PREAR::CLK_DIV32
    }
    #[doc = "Checks if the value of the field is `CLK_DIV64`"]
    #[inline]
    pub fn is_clk_div64(&self) -> bool {
        *self == PREAR::CLK_DIV64
    }
    #[doc = "Checks if the value of the field is `CLK_DIV128`"]
    #[inline]
    pub fn is_clk_div128(&self) -> bool {
        *self == PREAR::CLK_DIV128
    }
    #[doc = "Checks if the value of the field is `CLK_DIV256`"]
    #[inline]
    pub fn is_clk_div256(&self) -> bool {
        *self == PREAR::CLK_DIV256
    }
    #[doc = "Checks if the value of the field is `CLK_DIV512`"]
    #[inline]
    pub fn is_clk_div512(&self) -> bool {
        *self == PREAR::CLK_DIV512
    }
    #[doc = "Checks if the value of the field is `CLK_DIV1024`"]
    #[inline]
    pub fn is_clk_div1024(&self) -> bool {
        *self == PREAR::CLK_DIV1024
    }
}
#[doc = "Possible values of the field `DIVB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVBR {
    #[doc = "CLKB clock is turned off"]
    CLKB_POFF,
    #[doc = "CLKB clock is clock selected by PREB"]
    PREB,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DIVBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIVBR::CLKB_POFF => 0,
            DIVBR::PREB => 1,
            DIVBR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIVBR {
        match value {
            0 => DIVBR::CLKB_POFF,
            1 => DIVBR::PREB,
            i => DIVBR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLKB_POFF`"]
    #[inline]
    pub fn is_clkb_poff(&self) -> bool {
        *self == DIVBR::CLKB_POFF
    }
    #[doc = "Checks if the value of the field is `PREB`"]
    #[inline]
    pub fn is_preb(&self) -> bool {
        *self == DIVBR::PREB
    }
}
#[doc = "Possible values of the field `PREB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREBR {
    #[doc = "Peripheral clock"]
    CLK,
    #[doc = "Peripheral clock/2"]
    CLK_DIV2,
    #[doc = "Peripheral clock/4"]
    CLK_DIV4,
    #[doc = "Peripheral clock/8"]
    CLK_DIV8,
    #[doc = "Peripheral clock/16"]
    CLK_DIV16,
    #[doc = "Peripheral clock/32"]
    CLK_DIV32,
    #[doc = "Peripheral clock/64"]
    CLK_DIV64,
    #[doc = "Peripheral clock/128"]
    CLK_DIV128,
    #[doc = "Peripheral clock/256"]
    CLK_DIV256,
    #[doc = "Peripheral clock/512"]
    CLK_DIV512,
    #[doc = "Peripheral clock/1024"]
    CLK_DIV1024,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PREBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PREBR::CLK => 0,
            PREBR::CLK_DIV2 => 1,
            PREBR::CLK_DIV4 => 2,
            PREBR::CLK_DIV8 => 3,
            PREBR::CLK_DIV16 => 4,
            PREBR::CLK_DIV32 => 5,
            PREBR::CLK_DIV64 => 6,
            PREBR::CLK_DIV128 => 7,
            PREBR::CLK_DIV256 => 8,
            PREBR::CLK_DIV512 => 9,
            PREBR::CLK_DIV1024 => 10,
            PREBR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PREBR {
        match value {
            0 => PREBR::CLK,
            1 => PREBR::CLK_DIV2,
            2 => PREBR::CLK_DIV4,
            3 => PREBR::CLK_DIV8,
            4 => PREBR::CLK_DIV16,
            5 => PREBR::CLK_DIV32,
            6 => PREBR::CLK_DIV64,
            7 => PREBR::CLK_DIV128,
            8 => PREBR::CLK_DIV256,
            9 => PREBR::CLK_DIV512,
            10 => PREBR::CLK_DIV1024,
            i => PREBR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLK`"]
    #[inline]
    pub fn is_clk(&self) -> bool {
        *self == PREBR::CLK
    }
    #[doc = "Checks if the value of the field is `CLK_DIV2`"]
    #[inline]
    pub fn is_clk_div2(&self) -> bool {
        *self == PREBR::CLK_DIV2
    }
    #[doc = "Checks if the value of the field is `CLK_DIV4`"]
    #[inline]
    pub fn is_clk_div4(&self) -> bool {
        *self == PREBR::CLK_DIV4
    }
    #[doc = "Checks if the value of the field is `CLK_DIV8`"]
    #[inline]
    pub fn is_clk_div8(&self) -> bool {
        *self == PREBR::CLK_DIV8
    }
    #[doc = "Checks if the value of the field is `CLK_DIV16`"]
    #[inline]
    pub fn is_clk_div16(&self) -> bool {
        *self == PREBR::CLK_DIV16
    }
    #[doc = "Checks if the value of the field is `CLK_DIV32`"]
    #[inline]
    pub fn is_clk_div32(&self) -> bool {
        *self == PREBR::CLK_DIV32
    }
    #[doc = "Checks if the value of the field is `CLK_DIV64`"]
    #[inline]
    pub fn is_clk_div64(&self) -> bool {
        *self == PREBR::CLK_DIV64
    }
    #[doc = "Checks if the value of the field is `CLK_DIV128`"]
    #[inline]
    pub fn is_clk_div128(&self) -> bool {
        *self == PREBR::CLK_DIV128
    }
    #[doc = "Checks if the value of the field is `CLK_DIV256`"]
    #[inline]
    pub fn is_clk_div256(&self) -> bool {
        *self == PREBR::CLK_DIV256
    }
    #[doc = "Checks if the value of the field is `CLK_DIV512`"]
    #[inline]
    pub fn is_clk_div512(&self) -> bool {
        *self == PREBR::CLK_DIV512
    }
    #[doc = "Checks if the value of the field is `CLK_DIV1024`"]
    #[inline]
    pub fn is_clk_div1024(&self) -> bool {
        *self == PREBR::CLK_DIV1024
    }
}
#[doc = "Values that can be written to the field `DIVA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVAW {
    #[doc = "CLKA clock is turned off"]
    CLKA_POFF,
    #[doc = "CLKA clock is clock selected by PREA"]
    PREA,
}
impl DIVAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIVAW::CLKA_POFF => 0,
            DIVAW::PREA => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVAW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVAW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CLKA clock is turned off"]
    #[inline]
    pub fn clka_poff(self) -> &'a mut W {
        self.variant(DIVAW::CLKA_POFF)
    }
    #[doc = "CLKA clock is clock selected by PREA"]
    #[inline]
    pub fn prea(self) -> &'a mut W {
        self.variant(DIVAW::PREA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PREA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREAW {
    #[doc = "Peripheral clock"]
    CLK,
    #[doc = "Peripheral clock/2"]
    CLK_DIV2,
    #[doc = "Peripheral clock/4"]
    CLK_DIV4,
    #[doc = "Peripheral clock/8"]
    CLK_DIV8,
    #[doc = "Peripheral clock/16"]
    CLK_DIV16,
    #[doc = "Peripheral clock/32"]
    CLK_DIV32,
    #[doc = "Peripheral clock/64"]
    CLK_DIV64,
    #[doc = "Peripheral clock/128"]
    CLK_DIV128,
    #[doc = "Peripheral clock/256"]
    CLK_DIV256,
    #[doc = "Peripheral clock/512"]
    CLK_DIV512,
    #[doc = "Peripheral clock/1024"]
    CLK_DIV1024,
}
impl PREAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PREAW::CLK => 0,
            PREAW::CLK_DIV2 => 1,
            PREAW::CLK_DIV4 => 2,
            PREAW::CLK_DIV8 => 3,
            PREAW::CLK_DIV16 => 4,
            PREAW::CLK_DIV32 => 5,
            PREAW::CLK_DIV64 => 6,
            PREAW::CLK_DIV128 => 7,
            PREAW::CLK_DIV256 => 8,
            PREAW::CLK_DIV512 => 9,
            PREAW::CLK_DIV1024 => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PREAW<'a> {
    w: &'a mut W,
}
impl<'a> _PREAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PREAW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Peripheral clock"]
    #[inline]
    pub fn clk(self) -> &'a mut W {
        self.variant(PREAW::CLK)
    }
    #[doc = "Peripheral clock/2"]
    #[inline]
    pub fn clk_div2(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV2)
    }
    #[doc = "Peripheral clock/4"]
    #[inline]
    pub fn clk_div4(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV4)
    }
    #[doc = "Peripheral clock/8"]
    #[inline]
    pub fn clk_div8(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV8)
    }
    #[doc = "Peripheral clock/16"]
    #[inline]
    pub fn clk_div16(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV16)
    }
    #[doc = "Peripheral clock/32"]
    #[inline]
    pub fn clk_div32(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV32)
    }
    #[doc = "Peripheral clock/64"]
    #[inline]
    pub fn clk_div64(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV64)
    }
    #[doc = "Peripheral clock/128"]
    #[inline]
    pub fn clk_div128(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV128)
    }
    #[doc = "Peripheral clock/256"]
    #[inline]
    pub fn clk_div256(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV256)
    }
    #[doc = "Peripheral clock/512"]
    #[inline]
    pub fn clk_div512(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV512)
    }
    #[doc = "Peripheral clock/1024"]
    #[inline]
    pub fn clk_div1024(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV1024)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIVB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVBW {
    #[doc = "CLKB clock is turned off"]
    CLKB_POFF,
    #[doc = "CLKB clock is clock selected by PREB"]
    PREB,
}
impl DIVBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIVBW::CLKB_POFF => 0,
            DIVBW::PREB => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVBW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVBW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CLKB clock is turned off"]
    #[inline]
    pub fn clkb_poff(self) -> &'a mut W {
        self.variant(DIVBW::CLKB_POFF)
    }
    #[doc = "CLKB clock is clock selected by PREB"]
    #[inline]
    pub fn preb(self) -> &'a mut W {
        self.variant(DIVBW::PREB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PREB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREBW {
    #[doc = "Peripheral clock"]
    CLK,
    #[doc = "Peripheral clock/2"]
    CLK_DIV2,
    #[doc = "Peripheral clock/4"]
    CLK_DIV4,
    #[doc = "Peripheral clock/8"]
    CLK_DIV8,
    #[doc = "Peripheral clock/16"]
    CLK_DIV16,
    #[doc = "Peripheral clock/32"]
    CLK_DIV32,
    #[doc = "Peripheral clock/64"]
    CLK_DIV64,
    #[doc = "Peripheral clock/128"]
    CLK_DIV128,
    #[doc = "Peripheral clock/256"]
    CLK_DIV256,
    #[doc = "Peripheral clock/512"]
    CLK_DIV512,
    #[doc = "Peripheral clock/1024"]
    CLK_DIV1024,
}
impl PREBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PREBW::CLK => 0,
            PREBW::CLK_DIV2 => 1,
            PREBW::CLK_DIV4 => 2,
            PREBW::CLK_DIV8 => 3,
            PREBW::CLK_DIV16 => 4,
            PREBW::CLK_DIV32 => 5,
            PREBW::CLK_DIV64 => 6,
            PREBW::CLK_DIV128 => 7,
            PREBW::CLK_DIV256 => 8,
            PREBW::CLK_DIV512 => 9,
            PREBW::CLK_DIV1024 => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PREBW<'a> {
    w: &'a mut W,
}
impl<'a> _PREBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PREBW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Peripheral clock"]
    #[inline]
    pub fn clk(self) -> &'a mut W {
        self.variant(PREBW::CLK)
    }
    #[doc = "Peripheral clock/2"]
    #[inline]
    pub fn clk_div2(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV2)
    }
    #[doc = "Peripheral clock/4"]
    #[inline]
    pub fn clk_div4(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV4)
    }
    #[doc = "Peripheral clock/8"]
    #[inline]
    pub fn clk_div8(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV8)
    }
    #[doc = "Peripheral clock/16"]
    #[inline]
    pub fn clk_div16(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV16)
    }
    #[doc = "Peripheral clock/32"]
    #[inline]
    pub fn clk_div32(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV32)
    }
    #[doc = "Peripheral clock/64"]
    #[inline]
    pub fn clk_div64(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV64)
    }
    #[doc = "Peripheral clock/128"]
    #[inline]
    pub fn clk_div128(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV128)
    }
    #[doc = "Peripheral clock/256"]
    #[inline]
    pub fn clk_div256(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV256)
    }
    #[doc = "Peripheral clock/512"]
    #[inline]
    pub fn clk_div512(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV512)
    }
    #[doc = "Peripheral clock/1024"]
    #[inline]
    pub fn clk_div1024(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV1024)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:7 - CLKA Divide Factor"]
    #[inline]
    pub fn diva(&self) -> DIVAR {
        DIVAR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - CLKA Source Clock Selection"]
    #[inline]
    pub fn prea(&self) -> PREAR {
        PREAR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - CLKB Divide Factor"]
    #[inline]
    pub fn divb(&self) -> DIVBR {
        DIVBR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - CLKB Source Clock Selection"]
    #[inline]
    pub fn preb(&self) -> PREBR {
        PREBR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:7 - CLKA Divide Factor"]
    #[inline]
    pub fn diva(&mut self) -> _DIVAW {
        _DIVAW { w: self }
    }
    #[doc = "Bits 8:11 - CLKA Source Clock Selection"]
    #[inline]
    pub fn prea(&mut self) -> _PREAW {
        _PREAW { w: self }
    }
    #[doc = "Bits 16:23 - CLKB Divide Factor"]
    #[inline]
    pub fn divb(&mut self) -> _DIVBW {
        _DIVBW { w: self }
    }
    #[doc = "Bits 24:27 - CLKB Source Clock Selection"]
    #[inline]
    pub fn preb(&mut self) -> _PREBW {
        _PREBW { w: self }
    }
}
