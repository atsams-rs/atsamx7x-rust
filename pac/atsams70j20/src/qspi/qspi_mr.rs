#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::QSPI_MR {
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
#[doc = "Possible values of the field `SMM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMMR {
    #[doc = "The QSPI is in SPI mode."]
    SPI,
    #[doc = "The QSPI is in Serial Memory mode."]
    MEMORY,
}
impl crate::ToBits<bool> for SMMR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SMMR::SPI => false,
            SMMR::MEMORY => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SMM_R = crate::FR<bool, SMMR>;
impl SMM_R {
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == SMMR::SPI
    }
    #[doc = "Checks if the value of the field is `MEMORY`"]
    #[inline(always)]
    pub fn is_memory(&self) -> bool {
        *self == SMMR::MEMORY
    }
}
#[doc = "Values that can be written to the field `SMM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMMW {
    #[doc = "The QSPI is in SPI mode."]
    SPI,
    #[doc = "The QSPI is in Serial Memory mode."]
    MEMORY,
}
impl SMMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SMMW::SPI => false,
            SMMW::MEMORY => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SMMW<'a> {
    w: &'a mut W,
}
impl<'a> _SMMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The QSPI is in SPI mode."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(SMMW::SPI)
    }
    #[doc = "The QSPI is in Serial Memory mode."]
    #[inline(always)]
    pub fn memory(self) -> &'a mut W {
        self.variant(SMMW::MEMORY)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Possible values of the field `LLB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLBR {
    #[doc = "Local loopback path disabled."]
    DISABLED,
    #[doc = "Local loopback path enabled."]
    ENABLED,
}
impl crate::ToBits<bool> for LLBR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LLBR::DISABLED => false,
            LLBR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LLB_R = crate::FR<bool, LLBR>;
impl LLB_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LLBR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LLBR::ENABLED
    }
}
#[doc = "Values that can be written to the field `LLB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLBW {
    #[doc = "Local loopback path disabled."]
    DISABLED,
    #[doc = "Local loopback path enabled."]
    ENABLED,
}
impl LLBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LLBW::DISABLED => false,
            LLBW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LLBW<'a> {
    w: &'a mut W,
}
impl<'a> _LLBW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LLBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Local loopback path disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LLBW::DISABLED)
    }
    #[doc = "Local loopback path enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LLBW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `WDRBT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDRBTR {
    #[doc = "No effect. In SPI mode, a transfer can be initiated whatever the state of the QSPI_RDR is."]
    DISABLED,
    #[doc = "In SPI mode, a transfer can start only if the QSPI_RDR is empty, i.e., does not contain any unread data. This mode prevents overrun error in reception."]
    ENABLED,
}
impl crate::ToBits<bool> for WDRBTR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WDRBTR::DISABLED => false,
            WDRBTR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WDRBT_R = crate::FR<bool, WDRBTR>;
impl WDRBT_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDRBTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WDRBTR::ENABLED
    }
}
#[doc = "Values that can be written to the field `WDRBT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDRBTW {
    #[doc = "No effect. In SPI mode, a transfer can be initiated whatever the state of the QSPI_RDR is."]
    DISABLED,
    #[doc = "In SPI mode, a transfer can start only if the QSPI_RDR is empty, i.e., does not contain any unread data. This mode prevents overrun error in reception."]
    ENABLED,
}
impl WDRBTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WDRBTW::DISABLED => false,
            WDRBTW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WDRBTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDRBTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDRBTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect. In SPI mode, a transfer can be initiated whatever the state of the QSPI_RDR is."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDRBTW::DISABLED)
    }
    #[doc = "In SPI mode, a transfer can start only if the QSPI_RDR is empty, i.e., does not contain any unread data. This mode prevents overrun error in reception."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WDRBTW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `CSMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSMODER {
    #[doc = "The chip select is deasserted if QSPI_TDR.TD has not been reloaded before the end of the current transfer."]
    NOT_RELOADED,
    #[doc = "The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in QSPI_TDR.TD has been transferred."]
    LASTXFER,
    #[doc = "The chip select is deasserted systematically after each transfer."]
    SYSTEMATICALLY,
}
impl crate::ToBits<u8> for CSMODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CSMODER::NOT_RELOADED => 0,
            CSMODER::LASTXFER => 1,
            CSMODER::SYSTEMATICALLY => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CSMODE_R = crate::FR<u8, CSMODER>;
impl CSMODE_R {
    #[doc = "Checks if the value of the field is `NOT_RELOADED`"]
    #[inline(always)]
    pub fn is_not_reloaded(&self) -> bool {
        *self == CSMODER::NOT_RELOADED
    }
    #[doc = "Checks if the value of the field is `LASTXFER`"]
    #[inline(always)]
    pub fn is_lastxfer(&self) -> bool {
        *self == CSMODER::LASTXFER
    }
    #[doc = "Checks if the value of the field is `SYSTEMATICALLY`"]
    #[inline(always)]
    pub fn is_systematically(&self) -> bool {
        *self == CSMODER::SYSTEMATICALLY
    }
}
#[doc = "Values that can be written to the field `CSMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSMODEW {
    #[doc = "The chip select is deasserted if QSPI_TDR.TD has not been reloaded before the end of the current transfer."]
    NOT_RELOADED,
    #[doc = "The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in QSPI_TDR.TD has been transferred."]
    LASTXFER,
    #[doc = "The chip select is deasserted systematically after each transfer."]
    SYSTEMATICALLY,
}
impl CSMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSMODEW::NOT_RELOADED => 0,
            CSMODEW::LASTXFER => 1,
            CSMODEW::SYSTEMATICALLY => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CSMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CSMODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The chip select is deasserted if QSPI_TDR.TD has not been reloaded before the end of the current transfer."]
    #[inline(always)]
    pub fn not_reloaded(self) -> &'a mut W {
        self.variant(CSMODEW::NOT_RELOADED)
    }
    #[doc = "The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in QSPI_TDR.TD has been transferred."]
    #[inline(always)]
    pub fn lastxfer(self) -> &'a mut W {
        self.variant(CSMODEW::LASTXFER)
    }
    #[doc = "The chip select is deasserted systematically after each transfer."]
    #[inline(always)]
    pub fn systematically(self) -> &'a mut W {
        self.variant(CSMODEW::SYSTEMATICALLY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `NBBITS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBBITSR {
    #[doc = "8 bits for transfer"]
    _8_BIT,
    #[doc = "16 bits for transfer"]
    _16_BIT,
}
impl crate::ToBits<u8> for NBBITSR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            NBBITSR::_8_BIT => 0,
            NBBITSR::_16_BIT => 8,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NBBITS_R = crate::FR<u8, NBBITSR>;
impl NBBITS_R {
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == NBBITSR::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == NBBITSR::_16_BIT
    }
}
#[doc = "Values that can be written to the field `NBBITS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBBITSW {
    #[doc = "8 bits for transfer"]
    _8_BIT,
    #[doc = "16 bits for transfer"]
    _16_BIT,
}
impl NBBITSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            NBBITSW::_8_BIT => 0,
            NBBITSW::_16_BIT => 8,
        }
    }
}
#[doc = r"Proxy"]
pub struct _NBBITSW<'a> {
    w: &'a mut W,
}
impl<'a> _NBBITSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NBBITSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8 bits for transfer"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(NBBITSW::_8_BIT)
    }
    #[doc = "16 bits for transfer"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(NBBITSW::_16_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DLYBCT_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DLYBCTW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYBCTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DLYCS_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DLYCSW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYCSW<'a> {
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
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline(always)]
    pub fn smm(&self) -> SMM_R {
        SMM_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&self) -> LLB_R {
        LLB_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&self) -> WDRBT_R {
        WDRBT_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline(always)]
    pub fn csmode(&self) -> CSMODE_R {
        CSMODE_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Number Of Bits Per Transfer"]
    #[inline(always)]
    pub fn nbbits(&self) -> NBBITS_R {
        NBBITS_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&self) -> DLYBCT_R {
        DLYBCT_R::new(((self.bits() >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Minimum Inactive QCS Delay"]
    #[inline(always)]
    pub fn dlycs(&self) -> DLYCS_R {
        DLYCS_R::new(((self.bits() >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline(always)]
    pub fn smm(&mut self) -> _SMMW {
        _SMMW { w: self }
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&mut self) -> _LLBW {
        _LLBW { w: self }
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&mut self) -> _WDRBTW {
        _WDRBTW { w: self }
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline(always)]
    pub fn csmode(&mut self) -> _CSMODEW {
        _CSMODEW { w: self }
    }
    #[doc = "Bits 8:11 - Number Of Bits Per Transfer"]
    #[inline(always)]
    pub fn nbbits(&mut self) -> _NBBITSW {
        _NBBITSW { w: self }
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline(always)]
    pub fn dlybct(&mut self) -> _DLYBCTW {
        _DLYBCTW { w: self }
    }
    #[doc = "Bits 24:31 - Minimum Inactive QCS Delay"]
    #[inline(always)]
    pub fn dlycs(&mut self) -> _DLYCSW {
        _DLYCSW { w: self }
    }
}
