#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWM_CMR {
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
#[doc = "Possible values of the field `CPRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPRER {
    #[doc = "Peripheral clock"]
    MCK,
    #[doc = "Peripheral clock/2"]
    MCK_DIV_2,
    #[doc = "Peripheral clock/4"]
    MCK_DIV_4,
    #[doc = "Peripheral clock/8"]
    MCK_DIV_8,
    #[doc = "Peripheral clock/16"]
    MCK_DIV_16,
    #[doc = "Peripheral clock/32"]
    MCK_DIV_32,
    #[doc = "Peripheral clock/64"]
    MCK_DIV_64,
    #[doc = "Peripheral clock/128"]
    MCK_DIV_128,
    #[doc = "Peripheral clock/256"]
    MCK_DIV_256,
    #[doc = "Peripheral clock/512"]
    MCK_DIV_512,
    #[doc = "Peripheral clock/1024"]
    MCK_DIV_1024,
    #[doc = "Clock A"]
    CLKA,
    #[doc = "Clock B"]
    CLKB,
}
impl crate::ToBits<u8> for CPRER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CPRER::MCK => 0,
            CPRER::MCK_DIV_2 => 1,
            CPRER::MCK_DIV_4 => 2,
            CPRER::MCK_DIV_8 => 3,
            CPRER::MCK_DIV_16 => 4,
            CPRER::MCK_DIV_32 => 5,
            CPRER::MCK_DIV_64 => 6,
            CPRER::MCK_DIV_128 => 7,
            CPRER::MCK_DIV_256 => 8,
            CPRER::MCK_DIV_512 => 9,
            CPRER::MCK_DIV_1024 => 10,
            CPRER::CLKA => 11,
            CPRER::CLKB => 12,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CPRE_R = crate::FR<u8, CPRER>;
impl CPRE_R {
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == CPRER::MCK
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_2`"]
    #[inline(always)]
    pub fn is_mck_div_2(&self) -> bool {
        *self == CPRER::MCK_DIV_2
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_4`"]
    #[inline(always)]
    pub fn is_mck_div_4(&self) -> bool {
        *self == CPRER::MCK_DIV_4
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_8`"]
    #[inline(always)]
    pub fn is_mck_div_8(&self) -> bool {
        *self == CPRER::MCK_DIV_8
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_16`"]
    #[inline(always)]
    pub fn is_mck_div_16(&self) -> bool {
        *self == CPRER::MCK_DIV_16
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_32`"]
    #[inline(always)]
    pub fn is_mck_div_32(&self) -> bool {
        *self == CPRER::MCK_DIV_32
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_64`"]
    #[inline(always)]
    pub fn is_mck_div_64(&self) -> bool {
        *self == CPRER::MCK_DIV_64
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_128`"]
    #[inline(always)]
    pub fn is_mck_div_128(&self) -> bool {
        *self == CPRER::MCK_DIV_128
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_256`"]
    #[inline(always)]
    pub fn is_mck_div_256(&self) -> bool {
        *self == CPRER::MCK_DIV_256
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_512`"]
    #[inline(always)]
    pub fn is_mck_div_512(&self) -> bool {
        *self == CPRER::MCK_DIV_512
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_1024`"]
    #[inline(always)]
    pub fn is_mck_div_1024(&self) -> bool {
        *self == CPRER::MCK_DIV_1024
    }
    #[doc = "Checks if the value of the field is `CLKA`"]
    #[inline(always)]
    pub fn is_clka(&self) -> bool {
        *self == CPRER::CLKA
    }
    #[doc = "Checks if the value of the field is `CLKB`"]
    #[inline(always)]
    pub fn is_clkb(&self) -> bool {
        *self == CPRER::CLKB
    }
}
#[doc = "Values that can be written to the field `CPRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPREW {
    #[doc = "Peripheral clock"]
    MCK,
    #[doc = "Peripheral clock/2"]
    MCK_DIV_2,
    #[doc = "Peripheral clock/4"]
    MCK_DIV_4,
    #[doc = "Peripheral clock/8"]
    MCK_DIV_8,
    #[doc = "Peripheral clock/16"]
    MCK_DIV_16,
    #[doc = "Peripheral clock/32"]
    MCK_DIV_32,
    #[doc = "Peripheral clock/64"]
    MCK_DIV_64,
    #[doc = "Peripheral clock/128"]
    MCK_DIV_128,
    #[doc = "Peripheral clock/256"]
    MCK_DIV_256,
    #[doc = "Peripheral clock/512"]
    MCK_DIV_512,
    #[doc = "Peripheral clock/1024"]
    MCK_DIV_1024,
    #[doc = "Clock A"]
    CLKA,
    #[doc = "Clock B"]
    CLKB,
}
impl CPREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CPREW::MCK => 0,
            CPREW::MCK_DIV_2 => 1,
            CPREW::MCK_DIV_4 => 2,
            CPREW::MCK_DIV_8 => 3,
            CPREW::MCK_DIV_16 => 4,
            CPREW::MCK_DIV_32 => 5,
            CPREW::MCK_DIV_64 => 6,
            CPREW::MCK_DIV_128 => 7,
            CPREW::MCK_DIV_256 => 8,
            CPREW::MCK_DIV_512 => 9,
            CPREW::MCK_DIV_1024 => 10,
            CPREW::CLKA => 11,
            CPREW::CLKB => 12,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CPREW<'a> {
    w: &'a mut W,
}
impl<'a> _CPREW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPREW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(CPREW::MCK)
    }
    #[doc = "Peripheral clock/2"]
    #[inline(always)]
    pub fn mck_div_2(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_2)
    }
    #[doc = "Peripheral clock/4"]
    #[inline(always)]
    pub fn mck_div_4(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_4)
    }
    #[doc = "Peripheral clock/8"]
    #[inline(always)]
    pub fn mck_div_8(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_8)
    }
    #[doc = "Peripheral clock/16"]
    #[inline(always)]
    pub fn mck_div_16(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_16)
    }
    #[doc = "Peripheral clock/32"]
    #[inline(always)]
    pub fn mck_div_32(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_32)
    }
    #[doc = "Peripheral clock/64"]
    #[inline(always)]
    pub fn mck_div_64(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_64)
    }
    #[doc = "Peripheral clock/128"]
    #[inline(always)]
    pub fn mck_div_128(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_128)
    }
    #[doc = "Peripheral clock/256"]
    #[inline(always)]
    pub fn mck_div_256(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_256)
    }
    #[doc = "Peripheral clock/512"]
    #[inline(always)]
    pub fn mck_div_512(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_512)
    }
    #[doc = "Peripheral clock/1024"]
    #[inline(always)]
    pub fn mck_div_1024(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_1024)
    }
    #[doc = "Clock A"]
    #[inline(always)]
    pub fn clka(self) -> &'a mut W {
        self.variant(CPREW::CLKA)
    }
    #[doc = "Clock B"]
    #[inline(always)]
    pub fn clkb(self) -> &'a mut W {
        self.variant(CPREW::CLKB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CALG_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CALGW<'a> {
    w: &'a mut W,
}
impl<'a> _CALGW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CPOL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CES_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CESW<'a> {
    w: &'a mut W,
}
impl<'a> _CESW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type UPDS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _UPDSW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDSW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DPOLI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DPOLIW<'a> {
    w: &'a mut W,
}
impl<'a> _DPOLIW<'a> {
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
#[doc = r"Reader of the field"]
pub type TCTS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TCTSW<'a> {
    w: &'a mut W,
}
impl<'a> _TCTSW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DTE_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DTEW<'a> {
    w: &'a mut W,
}
impl<'a> _DTEW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DTHI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DTHIW<'a> {
    w: &'a mut W,
}
impl<'a> _DTHIW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DTLI_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DTLIW<'a> {
    w: &'a mut W,
}
impl<'a> _DTLIW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PPM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PPMW<'a> {
    w: &'a mut W,
}
impl<'a> _PPMW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    pub fn cpre(&self) -> CPRE_R {
        CPRE_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    pub fn calg(&self) -> CALG_R {
        CALG_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Counter Event Selection"]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Update Selection"]
    #[inline(always)]
    pub fn upds(&self) -> UPDS_R {
        UPDS_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Disabled Polarity Inverted"]
    #[inline(always)]
    pub fn dpoli(&self) -> DPOLI_R {
        DPOLI_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Timer Counter Trigger Selection"]
    #[inline(always)]
    pub fn tcts(&self) -> TCTS_R {
        TCTS_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Dead-Time Generator Enable"]
    #[inline(always)]
    pub fn dte(&self) -> DTE_R {
        DTE_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Dead-Time PWMHx Output Inverted"]
    #[inline(always)]
    pub fn dthi(&self) -> DTHI_R {
        DTHI_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Dead-Time PWMLx Output Inverted"]
    #[inline(always)]
    pub fn dtli(&self) -> DTLI_R {
        DTLI_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Push-Pull Mode"]
    #[inline(always)]
    pub fn ppm(&self) -> PPM_R {
        PPM_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline(always)]
    pub fn cpre(&mut self) -> _CPREW {
        _CPREW { w: self }
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline(always)]
    pub fn calg(&mut self) -> _CALGW {
        _CALGW { w: self }
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 10 - Counter Event Selection"]
    #[inline(always)]
    pub fn ces(&mut self) -> _CESW {
        _CESW { w: self }
    }
    #[doc = "Bit 11 - Update Selection"]
    #[inline(always)]
    pub fn upds(&mut self) -> _UPDSW {
        _UPDSW { w: self }
    }
    #[doc = "Bit 12 - Disabled Polarity Inverted"]
    #[inline(always)]
    pub fn dpoli(&mut self) -> _DPOLIW {
        _DPOLIW { w: self }
    }
    #[doc = "Bit 13 - Timer Counter Trigger Selection"]
    #[inline(always)]
    pub fn tcts(&mut self) -> _TCTSW {
        _TCTSW { w: self }
    }
    #[doc = "Bit 16 - Dead-Time Generator Enable"]
    #[inline(always)]
    pub fn dte(&mut self) -> _DTEW {
        _DTEW { w: self }
    }
    #[doc = "Bit 17 - Dead-Time PWMHx Output Inverted"]
    #[inline(always)]
    pub fn dthi(&mut self) -> _DTHIW {
        _DTHIW { w: self }
    }
    #[doc = "Bit 18 - Dead-Time PWMLx Output Inverted"]
    #[inline(always)]
    pub fn dtli(&mut self) -> _DTLIW {
        _DTLIW { w: self }
    }
    #[doc = "Bit 19 - Push-Pull Mode"]
    #[inline(always)]
    pub fn ppm(&mut self) -> _PPMW {
        _PPMW { w: self }
    }
}
