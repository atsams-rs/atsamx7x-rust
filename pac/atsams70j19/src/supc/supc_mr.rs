#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SUPC_MR {
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
#[doc = "Possible values of the field `BODRSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTENR {
    #[doc = "The core reset signal vddcore_nreset is not affected when a brownout detection occurs."]
    NOT_ENABLE,
    #[doc = "The core reset signal, vddcore_nreset is asserted when a brownout detection occurs."]
    ENABLE,
}
impl crate::ToBits<bool> for BODRSTENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BODRSTENR::NOT_ENABLE => false,
            BODRSTENR::ENABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BODRSTEN_R = crate::FR<bool, BODRSTENR>;
impl BODRSTEN_R {
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == BODRSTENR::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODRSTENR::ENABLE
    }
}
#[doc = "Values that can be written to the field `BODRSTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTENW {
    #[doc = "The core reset signal vddcore_nreset is not affected when a brownout detection occurs."]
    NOT_ENABLE,
    #[doc = "The core reset signal, vddcore_nreset is asserted when a brownout detection occurs."]
    ENABLE,
}
impl BODRSTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BODRSTENW::NOT_ENABLE => false,
            BODRSTENW::ENABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BODRSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _BODRSTENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODRSTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The core reset signal vddcore_nreset is not affected when a brownout detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(BODRSTENW::NOT_ENABLE)
    }
    #[doc = "The core reset signal, vddcore_nreset is asserted when a brownout detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODRSTENW::ENABLE)
    }
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
#[doc = "Possible values of the field `BODDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODDISR {
    #[doc = "The core brownout detector is enabled."]
    ENABLE,
    #[doc = "The core brownout detector is disabled."]
    DISABLE,
}
impl crate::ToBits<bool> for BODDISR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            BODDISR::ENABLE => false,
            BODDISR::DISABLE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type BODDIS_R = crate::FR<bool, BODDISR>;
impl BODDIS_R {
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODDISR::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODDISR::DISABLE
    }
}
#[doc = "Values that can be written to the field `BODDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODDISW {
    #[doc = "The core brownout detector is enabled."]
    ENABLE,
    #[doc = "The core brownout detector is disabled."]
    DISABLE,
}
impl BODDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            BODDISW::ENABLE => false,
            BODDISW::DISABLE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _BODDISW<'a> {
    w: &'a mut W,
}
impl<'a> _BODDISW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The core brownout detector is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODDISW::ENABLE)
    }
    #[doc = "The core brownout detector is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODDISW::DISABLE)
    }
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
#[doc = "Possible values of the field `ONREG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONREGR {
    #[doc = "Internal voltage regulator is not used (external power supply is used)."]
    ONREG_UNUSED,
    #[doc = "Internal voltage regulator is used."]
    ONREG_USED,
}
impl crate::ToBits<bool> for ONREGR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ONREGR::ONREG_UNUSED => false,
            ONREGR::ONREG_USED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ONREG_R = crate::FR<bool, ONREGR>;
impl ONREG_R {
    #[doc = "Checks if the value of the field is `ONREG_UNUSED`"]
    #[inline(always)]
    pub fn is_onreg_unused(&self) -> bool {
        *self == ONREGR::ONREG_UNUSED
    }
    #[doc = "Checks if the value of the field is `ONREG_USED`"]
    #[inline(always)]
    pub fn is_onreg_used(&self) -> bool {
        *self == ONREGR::ONREG_USED
    }
}
#[doc = "Values that can be written to the field `ONREG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONREGW {
    #[doc = "Internal voltage regulator is not used (external power supply is used)."]
    ONREG_UNUSED,
    #[doc = "Internal voltage regulator is used."]
    ONREG_USED,
}
impl ONREGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ONREGW::ONREG_UNUSED => false,
            ONREGW::ONREG_USED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ONREGW<'a> {
    w: &'a mut W,
}
impl<'a> _ONREGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONREGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Internal voltage regulator is not used (external power supply is used)."]
    #[inline(always)]
    pub fn onreg_unused(self) -> &'a mut W {
        self.variant(ONREGW::ONREG_UNUSED)
    }
    #[doc = "Internal voltage regulator is used."]
    #[inline(always)]
    pub fn onreg_used(self) -> &'a mut W {
        self.variant(ONREGW::ONREG_USED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type BKUPRETON_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _BKUPRETONW<'a> {
    w: &'a mut W,
}
impl<'a> _BKUPRETONW<'a> {
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
#[doc = "Possible values of the field `OSCBYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCBYPASSR {
    #[doc = "No effect. Clock selection depends on the value of XTALSEL (SUPC_CR)."]
    NO_EFFECT,
    #[doc = "The 32 kHz crystal oscillator is bypassed if XTALSEL (SUPC_CR) is set. OSCBYPASS must be set prior to setting XTALSEL."]
    BYPASS,
}
impl crate::ToBits<bool> for OSCBYPASSR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            OSCBYPASSR::NO_EFFECT => false,
            OSCBYPASSR::BYPASS => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OSCBYPASS_R = crate::FR<bool, OSCBYPASSR>;
impl OSCBYPASS_R {
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == OSCBYPASSR::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == OSCBYPASSR::BYPASS
    }
}
#[doc = "Values that can be written to the field `OSCBYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCBYPASSW {
    #[doc = "No effect. Clock selection depends on the value of XTALSEL (SUPC_CR)."]
    NO_EFFECT,
    #[doc = "The 32 kHz crystal oscillator is bypassed if XTALSEL (SUPC_CR) is set. OSCBYPASS must be set prior to setting XTALSEL."]
    BYPASS,
}
impl OSCBYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            OSCBYPASSW::NO_EFFECT => false,
            OSCBYPASSW::BYPASS => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _OSCBYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _OSCBYPASSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCBYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect. Clock selection depends on the value of XTALSEL (SUPC_CR)."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(OSCBYPASSW::NO_EFFECT)
    }
    #[doc = "The 32 kHz crystal oscillator is bypassed if XTALSEL (SUPC_CR) is set. OSCBYPASS must be set prior to setting XTALSEL."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(OSCBYPASSW::BYPASS)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `KEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYR {
    #[doc = "Writing any other value in this field aborts the write operation."]
    PASSWD,
}
impl crate::ToBits<u8> for KEYR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            KEYR::PASSWD => 165,
        }
    }
}
#[doc = r"Reader of the field"]
pub type KEY_R = crate::FR<u8, KEYR>;
impl KEY_R {
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == KEYR::PASSWD
    }
}
#[doc = "Values that can be written to the field `KEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYW {
    #[doc = "Writing any other value in this field aborts the write operation."]
    PASSWD,
}
impl KEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            KEYW::PASSWD => 165,
        }
    }
}
#[doc = r"Proxy"]
pub struct _KEYW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEYW::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 12 - Brownout Detector Reset Enable"]
    #[inline(always)]
    pub fn bodrsten(&self) -> BODRSTEN_R {
        BODRSTEN_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Brownout Detector Disable"]
    #[inline(always)]
    pub fn boddis(&self) -> BODDIS_R {
        BODDIS_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Voltage Regulator Enable"]
    #[inline(always)]
    pub fn onreg(&self) -> ONREG_R {
        ONREG_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SRAM On In Backup Mode"]
    #[inline(always)]
    pub fn bkupreton(&self) -> BKUPRETON_R {
        BKUPRETON_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Oscillator Bypass"]
    #[inline(always)]
    pub fn oscbypass(&self) -> OSCBYPASS_R {
        OSCBYPASS_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits() >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 12 - Brownout Detector Reset Enable"]
    #[inline(always)]
    pub fn bodrsten(&mut self) -> _BODRSTENW {
        _BODRSTENW { w: self }
    }
    #[doc = "Bit 13 - Brownout Detector Disable"]
    #[inline(always)]
    pub fn boddis(&mut self) -> _BODDISW {
        _BODDISW { w: self }
    }
    #[doc = "Bit 14 - Voltage Regulator Enable"]
    #[inline(always)]
    pub fn onreg(&mut self) -> _ONREGW {
        _ONREGW { w: self }
    }
    #[doc = "Bit 17 - SRAM On In Backup Mode"]
    #[inline(always)]
    pub fn bkupreton(&mut self) -> _BKUPRETONW {
        _BKUPRETONW { w: self }
    }
    #[doc = "Bit 20 - Oscillator Bypass"]
    #[inline(always)]
    pub fn oscbypass(&mut self) -> _OSCBYPASSW {
        _OSCBYPASSW { w: self }
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    pub fn key(&mut self) -> _KEYW {
        _KEYW { w: self }
    }
}
