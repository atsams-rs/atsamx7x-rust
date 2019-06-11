#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFEC_EMR {
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
#[doc = "Possible values of the field `CMPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPMODER {
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    LOW,
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    HIGH,
    #[doc = "Generates an event when the converted data is in the comparison window."]
    IN,
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    OUT,
}
impl CMPMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMPMODER::LOW => 0,
            CMPMODER::HIGH => 1,
            CMPMODER::IN => 2,
            CMPMODER::OUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMPMODER {
        match value {
            0 => CMPMODER::LOW,
            1 => CMPMODER::HIGH,
            2 => CMPMODER::IN,
            3 => CMPMODER::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == CMPMODER::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == CMPMODER::HIGH
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline]
    pub fn is_in_(&self) -> bool {
        *self == CMPMODER::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline]
    pub fn is_out(&self) -> bool {
        *self == CMPMODER::OUT
    }
}
#[doc = r" Value of the field"]
pub struct CMPSELR {
    bits: u8,
}
impl CMPSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CMPALLR {
    bits: bool,
}
impl CMPALLR {
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
pub struct CMPFILTERR {
    bits: u8,
}
impl CMPFILTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESR {
    #[doc = "12-bit resolution, AFE sample rate is maximum (no averaging)."]
    NO_AVERAGE,
    #[doc = "13-bit resolution, AFE sample rate divided by 4 (averaging)."]
    OSR4,
    #[doc = "14-bit resolution, AFE sample rate divided by 16 (averaging)."]
    OSR16,
    #[doc = "15-bit resolution, AFE sample rate divided by 64 (averaging)."]
    OSR64,
    #[doc = "16-bit resolution, AFE sample rate divided by 256 (averaging)."]
    OSR256,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESR::NO_AVERAGE => 0,
            RESR::OSR4 => 2,
            RESR::OSR16 => 3,
            RESR::OSR64 => 4,
            RESR::OSR256 => 5,
            RESR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESR {
        match value {
            0 => RESR::NO_AVERAGE,
            2 => RESR::OSR4,
            3 => RESR::OSR16,
            4 => RESR::OSR64,
            5 => RESR::OSR256,
            i => RESR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO_AVERAGE`"]
    #[inline]
    pub fn is_no_average(&self) -> bool {
        *self == RESR::NO_AVERAGE
    }
    #[doc = "Checks if the value of the field is `OSR4`"]
    #[inline]
    pub fn is_osr4(&self) -> bool {
        *self == RESR::OSR4
    }
    #[doc = "Checks if the value of the field is `OSR16`"]
    #[inline]
    pub fn is_osr16(&self) -> bool {
        *self == RESR::OSR16
    }
    #[doc = "Checks if the value of the field is `OSR64`"]
    #[inline]
    pub fn is_osr64(&self) -> bool {
        *self == RESR::OSR64
    }
    #[doc = "Checks if the value of the field is `OSR256`"]
    #[inline]
    pub fn is_osr256(&self) -> bool {
        *self == RESR::OSR256
    }
}
#[doc = r" Value of the field"]
pub struct TAGR {
    bits: bool,
}
impl TAGR {
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
pub struct STMR {
    bits: bool,
}
impl STMR {
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
#[doc = "Possible values of the field `SIGNMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIGNMODER {
    #[doc = "Single-Ended channels: Unsigned conversions.Differential channels: Signed conversions."]
    SE_UNSG_DF_SIGN,
    #[doc = "Single-Ended channels: Signed conversions.Differential channels: Unsigned conversions."]
    SE_SIGN_DF_UNSG,
    #[doc = "All channels: Unsigned conversions."]
    ALL_UNSIGNED,
    #[doc = "All channels: Signed conversions."]
    ALL_SIGNED,
}
impl SIGNMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SIGNMODER::SE_UNSG_DF_SIGN => 0,
            SIGNMODER::SE_SIGN_DF_UNSG => 1,
            SIGNMODER::ALL_UNSIGNED => 2,
            SIGNMODER::ALL_SIGNED => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SIGNMODER {
        match value {
            0 => SIGNMODER::SE_UNSG_DF_SIGN,
            1 => SIGNMODER::SE_SIGN_DF_UNSG,
            2 => SIGNMODER::ALL_UNSIGNED,
            3 => SIGNMODER::ALL_SIGNED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE_UNSG_DF_SIGN`"]
    #[inline]
    pub fn is_se_unsg_df_sign(&self) -> bool {
        *self == SIGNMODER::SE_UNSG_DF_SIGN
    }
    #[doc = "Checks if the value of the field is `SE_SIGN_DF_UNSG`"]
    #[inline]
    pub fn is_se_sign_df_unsg(&self) -> bool {
        *self == SIGNMODER::SE_SIGN_DF_UNSG
    }
    #[doc = "Checks if the value of the field is `ALL_UNSIGNED`"]
    #[inline]
    pub fn is_all_unsigned(&self) -> bool {
        *self == SIGNMODER::ALL_UNSIGNED
    }
    #[doc = "Checks if the value of the field is `ALL_SIGNED`"]
    #[inline]
    pub fn is_all_signed(&self) -> bool {
        *self == SIGNMODER::ALL_SIGNED
    }
}
#[doc = "Values that can be written to the field `CMPMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPMODEW {
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    LOW,
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    HIGH,
    #[doc = "Generates an event when the converted data is in the comparison window."]
    IN,
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    OUT,
}
impl CMPMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMPMODEW::LOW => 0,
            CMPMODEW::HIGH => 1,
            CMPMODEW::IN => 2,
            CMPMODEW::OUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMPMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(CMPMODEW::LOW)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(CMPMODEW::HIGH)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline]
    pub fn in_(self) -> &'a mut W {
        self.variant(CMPMODEW::IN)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline]
    pub fn out(self) -> &'a mut W {
        self.variant(CMPMODEW::OUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMPALLW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPALLW<'a> {
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
pub struct _CMPFILTERW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPFILTERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESW {
    #[doc = "12-bit resolution, AFE sample rate is maximum (no averaging)."]
    NO_AVERAGE,
    #[doc = "13-bit resolution, AFE sample rate divided by 4 (averaging)."]
    OSR4,
    #[doc = "14-bit resolution, AFE sample rate divided by 16 (averaging)."]
    OSR16,
    #[doc = "15-bit resolution, AFE sample rate divided by 64 (averaging)."]
    OSR64,
    #[doc = "16-bit resolution, AFE sample rate divided by 256 (averaging)."]
    OSR256,
}
impl RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESW::NO_AVERAGE => 0,
            RESW::OSR4 => 2,
            RESW::OSR16 => 3,
            RESW::OSR64 => 4,
            RESW::OSR256 => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESW<'a> {
    w: &'a mut W,
}
impl<'a> _RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "12-bit resolution, AFE sample rate is maximum (no averaging)."]
    #[inline]
    pub fn no_average(self) -> &'a mut W {
        self.variant(RESW::NO_AVERAGE)
    }
    #[doc = "13-bit resolution, AFE sample rate divided by 4 (averaging)."]
    #[inline]
    pub fn osr4(self) -> &'a mut W {
        self.variant(RESW::OSR4)
    }
    #[doc = "14-bit resolution, AFE sample rate divided by 16 (averaging)."]
    #[inline]
    pub fn osr16(self) -> &'a mut W {
        self.variant(RESW::OSR16)
    }
    #[doc = "15-bit resolution, AFE sample rate divided by 64 (averaging)."]
    #[inline]
    pub fn osr64(self) -> &'a mut W {
        self.variant(RESW::OSR64)
    }
    #[doc = "16-bit resolution, AFE sample rate divided by 256 (averaging)."]
    #[inline]
    pub fn osr256(self) -> &'a mut W {
        self.variant(RESW::OSR256)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TAGW<'a> {
    w: &'a mut W,
}
impl<'a> _TAGW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _STMW<'a> {
    w: &'a mut W,
}
impl<'a> _STMW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SIGNMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIGNMODEW {
    #[doc = "Single-Ended channels: Unsigned conversions.Differential channels: Signed conversions."]
    SE_UNSG_DF_SIGN,
    #[doc = "Single-Ended channels: Signed conversions.Differential channels: Unsigned conversions."]
    SE_SIGN_DF_UNSG,
    #[doc = "All channels: Unsigned conversions."]
    ALL_UNSIGNED,
    #[doc = "All channels: Signed conversions."]
    ALL_SIGNED,
}
impl SIGNMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SIGNMODEW::SE_UNSG_DF_SIGN => 0,
            SIGNMODEW::SE_SIGN_DF_UNSG => 1,
            SIGNMODEW::ALL_UNSIGNED => 2,
            SIGNMODEW::ALL_SIGNED => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIGNMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SIGNMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIGNMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single-Ended channels: Unsigned conversions.Differential channels: Signed conversions."]
    #[inline]
    pub fn se_unsg_df_sign(self) -> &'a mut W {
        self.variant(SIGNMODEW::SE_UNSG_DF_SIGN)
    }
    #[doc = "Single-Ended channels: Signed conversions.Differential channels: Unsigned conversions."]
    #[inline]
    pub fn se_sign_df_unsg(self) -> &'a mut W {
        self.variant(SIGNMODEW::SE_SIGN_DF_UNSG)
    }
    #[doc = "All channels: Unsigned conversions."]
    #[inline]
    pub fn all_unsigned(self) -> &'a mut W {
        self.variant(SIGNMODEW::ALL_UNSIGNED)
    }
    #[doc = "All channels: Signed conversions."]
    #[inline]
    pub fn all_signed(self) -> &'a mut W {
        self.variant(SIGNMODEW::ALL_SIGNED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline]
    pub fn cmpmode(&self) -> CMPMODER {
        CMPMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:7 - Comparison Selected Channel"]
    #[inline]
    pub fn cmpsel(&self) -> CMPSELR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMPSELR { bits }
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline]
    pub fn cmpall(&self) -> CMPALLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CMPALLR { bits }
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline]
    pub fn cmpfilter(&self) -> CMPFILTERR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMPFILTERR { bits }
    }
    #[doc = "Bits 16:18 - Resolution"]
    #[inline]
    pub fn res(&self) -> RESR {
        RESR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - TAG of the AFEC_LDCR"]
    #[inline]
    pub fn tag(&self) -> TAGR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAGR { bits }
    }
    #[doc = "Bit 25 - Single Trigger Mode"]
    #[inline]
    pub fn stm(&self) -> STMR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STMR { bits }
    }
    #[doc = "Bits 28:29 - Sign Mode"]
    #[inline]
    pub fn signmode(&self) -> SIGNMODER {
        SIGNMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline]
    pub fn cmpmode(&mut self) -> _CMPMODEW {
        _CMPMODEW { w: self }
    }
    #[doc = "Bits 3:7 - Comparison Selected Channel"]
    #[inline]
    pub fn cmpsel(&mut self) -> _CMPSELW {
        _CMPSELW { w: self }
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline]
    pub fn cmpall(&mut self) -> _CMPALLW {
        _CMPALLW { w: self }
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline]
    pub fn cmpfilter(&mut self) -> _CMPFILTERW {
        _CMPFILTERW { w: self }
    }
    #[doc = "Bits 16:18 - Resolution"]
    #[inline]
    pub fn res(&mut self) -> _RESW {
        _RESW { w: self }
    }
    #[doc = "Bit 24 - TAG of the AFEC_LDCR"]
    #[inline]
    pub fn tag(&mut self) -> _TAGW {
        _TAGW { w: self }
    }
    #[doc = "Bit 25 - Single Trigger Mode"]
    #[inline]
    pub fn stm(&mut self) -> _STMW {
        _STMW { w: self }
    }
    #[doc = "Bits 28:29 - Sign Mode"]
    #[inline]
    pub fn signmode(&mut self) -> _SIGNMODEW {
        _SIGNMODEW { w: self }
    }
}
