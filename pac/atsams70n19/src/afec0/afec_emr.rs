#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFEC_EMR {
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
impl crate::ToBits<u8> for CMPMODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CMPMODER::LOW => 0,
            CMPMODER::HIGH => 1,
            CMPMODER::IN => 2,
            CMPMODER::OUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CMPMODE_R = crate::FR<u8, CMPMODER>;
impl CMPMODE_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CMPMODER::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CMPMODER::HIGH
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == CMPMODER::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == CMPMODER::OUT
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMPMODEW::LOW => 0,
            CMPMODEW::HIGH => 1,
            CMPMODEW::IN => 2,
            CMPMODEW::OUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CMPMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPMODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CMPMODEW::LOW)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CMPMODEW::HIGH)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(CMPMODEW::IN)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(CMPMODEW::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CMPSEL_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CMPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPSELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CMPALL_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CMPALLW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPALLW<'a> {
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
pub type CMPFILTER_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CMPFILTERW<'a> {
    w: &'a mut W,
}
impl<'a> _CMPFILTERW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
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
}
impl crate::ToBits<u8> for RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            RESR::NO_AVERAGE => 0,
            RESR::OSR4 => 2,
            RESR::OSR16 => 3,
            RESR::OSR64 => 4,
            RESR::OSR256 => 5,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RES_R = crate::FR<u8, RESR>;
impl RES_R {
    #[doc = "Checks if the value of the field is `NO_AVERAGE`"]
    #[inline(always)]
    pub fn is_no_average(&self) -> bool {
        *self == RESR::NO_AVERAGE
    }
    #[doc = "Checks if the value of the field is `OSR4`"]
    #[inline(always)]
    pub fn is_osr4(&self) -> bool {
        *self == RESR::OSR4
    }
    #[doc = "Checks if the value of the field is `OSR16`"]
    #[inline(always)]
    pub fn is_osr16(&self) -> bool {
        *self == RESR::OSR16
    }
    #[doc = "Checks if the value of the field is `OSR64`"]
    #[inline(always)]
    pub fn is_osr64(&self) -> bool {
        *self == RESR::OSR64
    }
    #[doc = "Checks if the value of the field is `OSR256`"]
    #[inline(always)]
    pub fn is_osr256(&self) -> bool {
        *self == RESR::OSR256
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
    #[inline(always)]
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
#[doc = r"Proxy"]
pub struct _RESW<'a> {
    w: &'a mut W,
}
impl<'a> _RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "12-bit resolution, AFE sample rate is maximum (no averaging)."]
    #[inline(always)]
    pub fn no_average(self) -> &'a mut W {
        self.variant(RESW::NO_AVERAGE)
    }
    #[doc = "13-bit resolution, AFE sample rate divided by 4 (averaging)."]
    #[inline(always)]
    pub fn osr4(self) -> &'a mut W {
        self.variant(RESW::OSR4)
    }
    #[doc = "14-bit resolution, AFE sample rate divided by 16 (averaging)."]
    #[inline(always)]
    pub fn osr16(self) -> &'a mut W {
        self.variant(RESW::OSR16)
    }
    #[doc = "15-bit resolution, AFE sample rate divided by 64 (averaging)."]
    #[inline(always)]
    pub fn osr64(self) -> &'a mut W {
        self.variant(RESW::OSR64)
    }
    #[doc = "16-bit resolution, AFE sample rate divided by 256 (averaging)."]
    #[inline(always)]
    pub fn osr256(self) -> &'a mut W {
        self.variant(RESW::OSR256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type TAG_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TAGW<'a> {
    w: &'a mut W,
}
impl<'a> _TAGW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type STM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _STMW<'a> {
    w: &'a mut W,
}
impl<'a> _STMW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
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
impl crate::ToBits<u8> for SIGNMODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SIGNMODER::SE_UNSG_DF_SIGN => 0,
            SIGNMODER::SE_SIGN_DF_UNSG => 1,
            SIGNMODER::ALL_UNSIGNED => 2,
            SIGNMODER::ALL_SIGNED => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SIGNMODE_R = crate::FR<u8, SIGNMODER>;
impl SIGNMODE_R {
    #[doc = "Checks if the value of the field is `SE_UNSG_DF_SIGN`"]
    #[inline(always)]
    pub fn is_se_unsg_df_sign(&self) -> bool {
        *self == SIGNMODER::SE_UNSG_DF_SIGN
    }
    #[doc = "Checks if the value of the field is `SE_SIGN_DF_UNSG`"]
    #[inline(always)]
    pub fn is_se_sign_df_unsg(&self) -> bool {
        *self == SIGNMODER::SE_SIGN_DF_UNSG
    }
    #[doc = "Checks if the value of the field is `ALL_UNSIGNED`"]
    #[inline(always)]
    pub fn is_all_unsigned(&self) -> bool {
        *self == SIGNMODER::ALL_UNSIGNED
    }
    #[doc = "Checks if the value of the field is `ALL_SIGNED`"]
    #[inline(always)]
    pub fn is_all_signed(&self) -> bool {
        *self == SIGNMODER::ALL_SIGNED
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SIGNMODEW::SE_UNSG_DF_SIGN => 0,
            SIGNMODEW::SE_SIGN_DF_UNSG => 1,
            SIGNMODEW::ALL_UNSIGNED => 2,
            SIGNMODEW::ALL_SIGNED => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SIGNMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SIGNMODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIGNMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Single-Ended channels: Unsigned conversions.Differential channels: Signed conversions."]
    #[inline(always)]
    pub fn se_unsg_df_sign(self) -> &'a mut W {
        self.variant(SIGNMODEW::SE_UNSG_DF_SIGN)
    }
    #[doc = "Single-Ended channels: Signed conversions.Differential channels: Unsigned conversions."]
    #[inline(always)]
    pub fn se_sign_df_unsg(self) -> &'a mut W {
        self.variant(SIGNMODEW::SE_SIGN_DF_UNSG)
    }
    #[doc = "All channels: Unsigned conversions."]
    #[inline(always)]
    pub fn all_unsigned(self) -> &'a mut W {
        self.variant(SIGNMODEW::ALL_UNSIGNED)
    }
    #[doc = "All channels: Signed conversions."]
    #[inline(always)]
    pub fn all_signed(self) -> &'a mut W {
        self.variant(SIGNMODEW::ALL_SIGNED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CMPMODE_R {
        CMPMODE_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 3:7 - Comparison Selected Channel"]
    #[inline(always)]
    pub fn cmpsel(&self) -> CMPSEL_R {
        CMPSEL_R::new(((self.bits() >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    pub fn cmpall(&self) -> CMPALL_R {
        CMPALL_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    pub fn cmpfilter(&self) -> CMPFILTER_R {
        CMPFILTER_R::new(((self.bits() >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:18 - Resolution"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits() >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 24 - TAG of the AFEC_LDCR"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Single Trigger Mode"]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Sign Mode"]
    #[inline(always)]
    pub fn signmode(&self) -> SIGNMODE_R {
        SIGNMODE_R::new(((self.bits() >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&mut self) -> _CMPMODEW {
        _CMPMODEW { w: self }
    }
    #[doc = "Bits 3:7 - Comparison Selected Channel"]
    #[inline(always)]
    pub fn cmpsel(&mut self) -> _CMPSELW {
        _CMPSELW { w: self }
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    pub fn cmpall(&mut self) -> _CMPALLW {
        _CMPALLW { w: self }
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    pub fn cmpfilter(&mut self) -> _CMPFILTERW {
        _CMPFILTERW { w: self }
    }
    #[doc = "Bits 16:18 - Resolution"]
    #[inline(always)]
    pub fn res(&mut self) -> _RESW {
        _RESW { w: self }
    }
    #[doc = "Bit 24 - TAG of the AFEC_LDCR"]
    #[inline(always)]
    pub fn tag(&mut self) -> _TAGW {
        _TAGW { w: self }
    }
    #[doc = "Bit 25 - Single Trigger Mode"]
    #[inline(always)]
    pub fn stm(&mut self) -> _STMW {
        _STMW { w: self }
    }
    #[doc = "Bits 28:29 - Sign Mode"]
    #[inline(always)]
    pub fn signmode(&mut self) -> _SIGNMODEW {
        _SIGNMODEW { w: self }
    }
}
