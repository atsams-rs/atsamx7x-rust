#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWM_CMR {
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
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CPRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
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
            CPRER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CPRER {
        match value {
            0 => CPRER::MCK,
            1 => CPRER::MCK_DIV_2,
            2 => CPRER::MCK_DIV_4,
            3 => CPRER::MCK_DIV_8,
            4 => CPRER::MCK_DIV_16,
            5 => CPRER::MCK_DIV_32,
            6 => CPRER::MCK_DIV_64,
            7 => CPRER::MCK_DIV_128,
            8 => CPRER::MCK_DIV_256,
            9 => CPRER::MCK_DIV_512,
            10 => CPRER::MCK_DIV_1024,
            11 => CPRER::CLKA,
            12 => CPRER::CLKB,
            i => CPRER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline]
    pub fn is_mck(&self) -> bool {
        *self == CPRER::MCK
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_2`"]
    #[inline]
    pub fn is_mck_div_2(&self) -> bool {
        *self == CPRER::MCK_DIV_2
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_4`"]
    #[inline]
    pub fn is_mck_div_4(&self) -> bool {
        *self == CPRER::MCK_DIV_4
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_8`"]
    #[inline]
    pub fn is_mck_div_8(&self) -> bool {
        *self == CPRER::MCK_DIV_8
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_16`"]
    #[inline]
    pub fn is_mck_div_16(&self) -> bool {
        *self == CPRER::MCK_DIV_16
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_32`"]
    #[inline]
    pub fn is_mck_div_32(&self) -> bool {
        *self == CPRER::MCK_DIV_32
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_64`"]
    #[inline]
    pub fn is_mck_div_64(&self) -> bool {
        *self == CPRER::MCK_DIV_64
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_128`"]
    #[inline]
    pub fn is_mck_div_128(&self) -> bool {
        *self == CPRER::MCK_DIV_128
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_256`"]
    #[inline]
    pub fn is_mck_div_256(&self) -> bool {
        *self == CPRER::MCK_DIV_256
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_512`"]
    #[inline]
    pub fn is_mck_div_512(&self) -> bool {
        *self == CPRER::MCK_DIV_512
    }
    #[doc = "Checks if the value of the field is `MCK_DIV_1024`"]
    #[inline]
    pub fn is_mck_div_1024(&self) -> bool {
        *self == CPRER::MCK_DIV_1024
    }
    #[doc = "Checks if the value of the field is `CLKA`"]
    #[inline]
    pub fn is_clka(&self) -> bool {
        *self == CPRER::CLKA
    }
    #[doc = "Checks if the value of the field is `CLKB`"]
    #[inline]
    pub fn is_clkb(&self) -> bool {
        *self == CPRER::CLKB
    }
}
#[doc = r" Value of the field"]
pub struct CALGR {
    bits: bool,
}
impl CALGR {
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
pub struct CPOLR {
    bits: bool,
}
impl CPOLR {
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
pub struct CESR {
    bits: bool,
}
impl CESR {
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
pub struct UPDSR {
    bits: bool,
}
impl UPDSR {
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
pub struct DPOLIR {
    bits: bool,
}
impl DPOLIR {
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
pub struct TCTSR {
    bits: bool,
}
impl TCTSR {
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
pub struct DTER {
    bits: bool,
}
impl DTER {
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
pub struct DTHIR {
    bits: bool,
}
impl DTHIR {
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
pub struct DTLIR {
    bits: bool,
}
impl DTLIR {
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
pub struct PPMR {
    bits: bool,
}
impl PPMR {
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
    #[inline]
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
#[doc = r" Proxy"]
pub struct _CPREW<'a> {
    w: &'a mut W,
}
impl<'a> _CPREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPREW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Peripheral clock"]
    #[inline]
    pub fn mck(self) -> &'a mut W {
        self.variant(CPREW::MCK)
    }
    #[doc = "Peripheral clock/2"]
    #[inline]
    pub fn mck_div_2(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_2)
    }
    #[doc = "Peripheral clock/4"]
    #[inline]
    pub fn mck_div_4(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_4)
    }
    #[doc = "Peripheral clock/8"]
    #[inline]
    pub fn mck_div_8(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_8)
    }
    #[doc = "Peripheral clock/16"]
    #[inline]
    pub fn mck_div_16(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_16)
    }
    #[doc = "Peripheral clock/32"]
    #[inline]
    pub fn mck_div_32(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_32)
    }
    #[doc = "Peripheral clock/64"]
    #[inline]
    pub fn mck_div_64(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_64)
    }
    #[doc = "Peripheral clock/128"]
    #[inline]
    pub fn mck_div_128(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_128)
    }
    #[doc = "Peripheral clock/256"]
    #[inline]
    pub fn mck_div_256(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_256)
    }
    #[doc = "Peripheral clock/512"]
    #[inline]
    pub fn mck_div_512(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_512)
    }
    #[doc = "Peripheral clock/1024"]
    #[inline]
    pub fn mck_div_1024(self) -> &'a mut W {
        self.variant(CPREW::MCK_DIV_1024)
    }
    #[doc = "Clock A"]
    #[inline]
    pub fn clka(self) -> &'a mut W {
        self.variant(CPREW::CLKA)
    }
    #[doc = "Clock B"]
    #[inline]
    pub fn clkb(self) -> &'a mut W {
        self.variant(CPREW::CLKB)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CALGW<'a> {
    w: &'a mut W,
}
impl<'a> _CALGW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CESW<'a> {
    w: &'a mut W,
}
impl<'a> _CESW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UPDSW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDSW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DPOLIW<'a> {
    w: &'a mut W,
}
impl<'a> _DPOLIW<'a> {
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
pub struct _TCTSW<'a> {
    w: &'a mut W,
}
impl<'a> _TCTSW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTEW<'a> {
    w: &'a mut W,
}
impl<'a> _DTEW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTHIW<'a> {
    w: &'a mut W,
}
impl<'a> _DTHIW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DTLIW<'a> {
    w: &'a mut W,
}
impl<'a> _DTLIW<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PPMW<'a> {
    w: &'a mut W,
}
impl<'a> _PPMW<'a> {
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
        const OFFSET: u8 = 19;
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
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline]
    pub fn cpre(&self) -> CPRER {
        CPRER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline]
    pub fn calg(&self) -> CALGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CALGR { bits }
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CPOLR { bits }
    }
    #[doc = "Bit 10 - Counter Event Selection"]
    #[inline]
    pub fn ces(&self) -> CESR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CESR { bits }
    }
    #[doc = "Bit 11 - Update Selection"]
    #[inline]
    pub fn upds(&self) -> UPDSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UPDSR { bits }
    }
    #[doc = "Bit 12 - Disabled Polarity Inverted"]
    #[inline]
    pub fn dpoli(&self) -> DPOLIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DPOLIR { bits }
    }
    #[doc = "Bit 13 - Timer Counter Trigger Selection"]
    #[inline]
    pub fn tcts(&self) -> TCTSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TCTSR { bits }
    }
    #[doc = "Bit 16 - Dead-Time Generator Enable"]
    #[inline]
    pub fn dte(&self) -> DTER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTER { bits }
    }
    #[doc = "Bit 17 - Dead-Time PWMHx Output Inverted"]
    #[inline]
    pub fn dthi(&self) -> DTHIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTHIR { bits }
    }
    #[doc = "Bit 18 - Dead-Time PWMLx Output Inverted"]
    #[inline]
    pub fn dtli(&self) -> DTLIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DTLIR { bits }
    }
    #[doc = "Bit 19 - Push-Pull Mode"]
    #[inline]
    pub fn ppm(&self) -> PPMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PPMR { bits }
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
    #[doc = "Bits 0:3 - Channel Pre-scaler"]
    #[inline]
    pub fn cpre(&mut self) -> _CPREW {
        _CPREW { w: self }
    }
    #[doc = "Bit 8 - Channel Alignment"]
    #[inline]
    pub fn calg(&mut self) -> _CALGW {
        _CALGW { w: self }
    }
    #[doc = "Bit 9 - Channel Polarity"]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 10 - Counter Event Selection"]
    #[inline]
    pub fn ces(&mut self) -> _CESW {
        _CESW { w: self }
    }
    #[doc = "Bit 11 - Update Selection"]
    #[inline]
    pub fn upds(&mut self) -> _UPDSW {
        _UPDSW { w: self }
    }
    #[doc = "Bit 12 - Disabled Polarity Inverted"]
    #[inline]
    pub fn dpoli(&mut self) -> _DPOLIW {
        _DPOLIW { w: self }
    }
    #[doc = "Bit 13 - Timer Counter Trigger Selection"]
    #[inline]
    pub fn tcts(&mut self) -> _TCTSW {
        _TCTSW { w: self }
    }
    #[doc = "Bit 16 - Dead-Time Generator Enable"]
    #[inline]
    pub fn dte(&mut self) -> _DTEW {
        _DTEW { w: self }
    }
    #[doc = "Bit 17 - Dead-Time PWMHx Output Inverted"]
    #[inline]
    pub fn dthi(&mut self) -> _DTHIW {
        _DTHIW { w: self }
    }
    #[doc = "Bit 18 - Dead-Time PWMLx Output Inverted"]
    #[inline]
    pub fn dtli(&mut self) -> _DTLIW {
        _DTLIW { w: self }
    }
    #[doc = "Bit 19 - Push-Pull Mode"]
    #[inline]
    pub fn ppm(&mut self) -> _PPMW {
        _PPMW { w: self }
    }
}
