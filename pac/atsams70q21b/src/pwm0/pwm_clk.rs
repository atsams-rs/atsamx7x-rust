#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWM_CLK {
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
#[doc = "Possible values of the field `DIVA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVAR {
    #[doc = "CLKA clock is turned off"]
    CLKA_POFF,
    #[doc = "CLKA clock is clock selected by PREA"]
    PREA,
}
impl crate::ToBits<u8> for DIVAR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DIVAR::CLKA_POFF => 0,
            DIVAR::PREA => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DIVA_R = crate::FR<u8, DIVAR>;
impl DIVA_R {
    #[doc = "Checks if the value of the field is `CLKA_POFF`"]
    #[inline(always)]
    pub fn is_clka_poff(&self) -> bool {
        *self == DIVAR::CLKA_POFF
    }
    #[doc = "Checks if the value of the field is `PREA`"]
    #[inline(always)]
    pub fn is_prea(&self) -> bool {
        *self == DIVAR::PREA
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIVAW::CLKA_POFF => 0,
            DIVAW::PREA => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DIVAW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVAW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CLKA clock is turned off"]
    #[inline(always)]
    pub fn clka_poff(self) -> &'a mut W {
        self.variant(DIVAW::CLKA_POFF)
    }
    #[doc = "CLKA clock is clock selected by PREA"]
    #[inline(always)]
    pub fn prea(self) -> &'a mut W {
        self.variant(DIVAW::PREA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
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
}
impl crate::ToBits<u8> for PREAR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
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
        }
    }
}
#[doc = r"Reader of the field"]
pub type PREA_R = crate::FR<u8, PREAR>;
impl PREA_R {
    #[doc = "Checks if the value of the field is `CLK`"]
    #[inline(always)]
    pub fn is_clk(&self) -> bool {
        *self == PREAR::CLK
    }
    #[doc = "Checks if the value of the field is `CLK_DIV2`"]
    #[inline(always)]
    pub fn is_clk_div2(&self) -> bool {
        *self == PREAR::CLK_DIV2
    }
    #[doc = "Checks if the value of the field is `CLK_DIV4`"]
    #[inline(always)]
    pub fn is_clk_div4(&self) -> bool {
        *self == PREAR::CLK_DIV4
    }
    #[doc = "Checks if the value of the field is `CLK_DIV8`"]
    #[inline(always)]
    pub fn is_clk_div8(&self) -> bool {
        *self == PREAR::CLK_DIV8
    }
    #[doc = "Checks if the value of the field is `CLK_DIV16`"]
    #[inline(always)]
    pub fn is_clk_div16(&self) -> bool {
        *self == PREAR::CLK_DIV16
    }
    #[doc = "Checks if the value of the field is `CLK_DIV32`"]
    #[inline(always)]
    pub fn is_clk_div32(&self) -> bool {
        *self == PREAR::CLK_DIV32
    }
    #[doc = "Checks if the value of the field is `CLK_DIV64`"]
    #[inline(always)]
    pub fn is_clk_div64(&self) -> bool {
        *self == PREAR::CLK_DIV64
    }
    #[doc = "Checks if the value of the field is `CLK_DIV128`"]
    #[inline(always)]
    pub fn is_clk_div128(&self) -> bool {
        *self == PREAR::CLK_DIV128
    }
    #[doc = "Checks if the value of the field is `CLK_DIV256`"]
    #[inline(always)]
    pub fn is_clk_div256(&self) -> bool {
        *self == PREAR::CLK_DIV256
    }
    #[doc = "Checks if the value of the field is `CLK_DIV512`"]
    #[inline(always)]
    pub fn is_clk_div512(&self) -> bool {
        *self == PREAR::CLK_DIV512
    }
    #[doc = "Checks if the value of the field is `CLK_DIV1024`"]
    #[inline(always)]
    pub fn is_clk_div1024(&self) -> bool {
        *self == PREAR::CLK_DIV1024
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
    #[inline(always)]
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
#[doc = r"Proxy"]
pub struct _PREAW<'a> {
    w: &'a mut W,
}
impl<'a> _PREAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREAW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn clk(self) -> &'a mut W {
        self.variant(PREAW::CLK)
    }
    #[doc = "Peripheral clock/2"]
    #[inline(always)]
    pub fn clk_div2(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV2)
    }
    #[doc = "Peripheral clock/4"]
    #[inline(always)]
    pub fn clk_div4(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV4)
    }
    #[doc = "Peripheral clock/8"]
    #[inline(always)]
    pub fn clk_div8(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV8)
    }
    #[doc = "Peripheral clock/16"]
    #[inline(always)]
    pub fn clk_div16(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV16)
    }
    #[doc = "Peripheral clock/32"]
    #[inline(always)]
    pub fn clk_div32(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV32)
    }
    #[doc = "Peripheral clock/64"]
    #[inline(always)]
    pub fn clk_div64(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV64)
    }
    #[doc = "Peripheral clock/128"]
    #[inline(always)]
    pub fn clk_div128(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV128)
    }
    #[doc = "Peripheral clock/256"]
    #[inline(always)]
    pub fn clk_div256(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV256)
    }
    #[doc = "Peripheral clock/512"]
    #[inline(always)]
    pub fn clk_div512(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV512)
    }
    #[doc = "Peripheral clock/1024"]
    #[inline(always)]
    pub fn clk_div1024(self) -> &'a mut W {
        self.variant(PREAW::CLK_DIV1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `DIVB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVBR {
    #[doc = "CLKB clock is turned off"]
    CLKB_POFF,
    #[doc = "CLKB clock is clock selected by PREB"]
    PREB,
}
impl crate::ToBits<u8> for DIVBR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DIVBR::CLKB_POFF => 0,
            DIVBR::PREB => 1,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DIVB_R = crate::FR<u8, DIVBR>;
impl DIVB_R {
    #[doc = "Checks if the value of the field is `CLKB_POFF`"]
    #[inline(always)]
    pub fn is_clkb_poff(&self) -> bool {
        *self == DIVBR::CLKB_POFF
    }
    #[doc = "Checks if the value of the field is `PREB`"]
    #[inline(always)]
    pub fn is_preb(&self) -> bool {
        *self == DIVBR::PREB
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIVBW::CLKB_POFF => 0,
            DIVBW::PREB => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DIVBW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVBW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVBW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CLKB clock is turned off"]
    #[inline(always)]
    pub fn clkb_poff(self) -> &'a mut W {
        self.variant(DIVBW::CLKB_POFF)
    }
    #[doc = "CLKB clock is clock selected by PREB"]
    #[inline(always)]
    pub fn preb(self) -> &'a mut W {
        self.variant(DIVBW::PREB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
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
}
impl crate::ToBits<u8> for PREBR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
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
        }
    }
}
#[doc = r"Reader of the field"]
pub type PREB_R = crate::FR<u8, PREBR>;
impl PREB_R {
    #[doc = "Checks if the value of the field is `CLK`"]
    #[inline(always)]
    pub fn is_clk(&self) -> bool {
        *self == PREBR::CLK
    }
    #[doc = "Checks if the value of the field is `CLK_DIV2`"]
    #[inline(always)]
    pub fn is_clk_div2(&self) -> bool {
        *self == PREBR::CLK_DIV2
    }
    #[doc = "Checks if the value of the field is `CLK_DIV4`"]
    #[inline(always)]
    pub fn is_clk_div4(&self) -> bool {
        *self == PREBR::CLK_DIV4
    }
    #[doc = "Checks if the value of the field is `CLK_DIV8`"]
    #[inline(always)]
    pub fn is_clk_div8(&self) -> bool {
        *self == PREBR::CLK_DIV8
    }
    #[doc = "Checks if the value of the field is `CLK_DIV16`"]
    #[inline(always)]
    pub fn is_clk_div16(&self) -> bool {
        *self == PREBR::CLK_DIV16
    }
    #[doc = "Checks if the value of the field is `CLK_DIV32`"]
    #[inline(always)]
    pub fn is_clk_div32(&self) -> bool {
        *self == PREBR::CLK_DIV32
    }
    #[doc = "Checks if the value of the field is `CLK_DIV64`"]
    #[inline(always)]
    pub fn is_clk_div64(&self) -> bool {
        *self == PREBR::CLK_DIV64
    }
    #[doc = "Checks if the value of the field is `CLK_DIV128`"]
    #[inline(always)]
    pub fn is_clk_div128(&self) -> bool {
        *self == PREBR::CLK_DIV128
    }
    #[doc = "Checks if the value of the field is `CLK_DIV256`"]
    #[inline(always)]
    pub fn is_clk_div256(&self) -> bool {
        *self == PREBR::CLK_DIV256
    }
    #[doc = "Checks if the value of the field is `CLK_DIV512`"]
    #[inline(always)]
    pub fn is_clk_div512(&self) -> bool {
        *self == PREBR::CLK_DIV512
    }
    #[doc = "Checks if the value of the field is `CLK_DIV1024`"]
    #[inline(always)]
    pub fn is_clk_div1024(&self) -> bool {
        *self == PREBR::CLK_DIV1024
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
    #[inline(always)]
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
#[doc = r"Proxy"]
pub struct _PREBW<'a> {
    w: &'a mut W,
}
impl<'a> _PREBW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREBW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn clk(self) -> &'a mut W {
        self.variant(PREBW::CLK)
    }
    #[doc = "Peripheral clock/2"]
    #[inline(always)]
    pub fn clk_div2(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV2)
    }
    #[doc = "Peripheral clock/4"]
    #[inline(always)]
    pub fn clk_div4(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV4)
    }
    #[doc = "Peripheral clock/8"]
    #[inline(always)]
    pub fn clk_div8(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV8)
    }
    #[doc = "Peripheral clock/16"]
    #[inline(always)]
    pub fn clk_div16(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV16)
    }
    #[doc = "Peripheral clock/32"]
    #[inline(always)]
    pub fn clk_div32(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV32)
    }
    #[doc = "Peripheral clock/64"]
    #[inline(always)]
    pub fn clk_div64(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV64)
    }
    #[doc = "Peripheral clock/128"]
    #[inline(always)]
    pub fn clk_div128(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV128)
    }
    #[doc = "Peripheral clock/256"]
    #[inline(always)]
    pub fn clk_div256(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV256)
    }
    #[doc = "Peripheral clock/512"]
    #[inline(always)]
    pub fn clk_div512(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV512)
    }
    #[doc = "Peripheral clock/1024"]
    #[inline(always)]
    pub fn clk_div1024(self) -> &'a mut W {
        self.variant(PREBW::CLK_DIV1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - CLKA Divide Factor"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new((self.bits() & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - CLKA Source Clock Selection"]
    #[inline(always)]
    pub fn prea(&self) -> PREA_R {
        PREA_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - CLKB Divide Factor"]
    #[inline(always)]
    pub fn divb(&self) -> DIVB_R {
        DIVB_R::new(((self.bits() >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - CLKB Source Clock Selection"]
    #[inline(always)]
    pub fn preb(&self) -> PREB_R {
        PREB_R::new(((self.bits() >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - CLKA Divide Factor"]
    #[inline(always)]
    pub fn diva(&mut self) -> _DIVAW {
        _DIVAW { w: self }
    }
    #[doc = "Bits 8:11 - CLKA Source Clock Selection"]
    #[inline(always)]
    pub fn prea(&mut self) -> _PREAW {
        _PREAW { w: self }
    }
    #[doc = "Bits 16:23 - CLKB Divide Factor"]
    #[inline(always)]
    pub fn divb(&mut self) -> _DIVBW {
        _DIVBW { w: self }
    }
    #[doc = "Bits 24:27 - CLKB Source Clock Selection"]
    #[inline(always)]
    pub fn preb(&mut self) -> _PREBW {
        _PREBW { w: self }
    }
}
