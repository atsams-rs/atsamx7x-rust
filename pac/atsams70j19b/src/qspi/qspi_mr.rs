#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::QSPI_MR {
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
#[doc = "Possible values of the field `SMM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMMR {
    #[doc = "The QSPI is in SPI mode."]
    SPI,
    #[doc = "The QSPI is in Serial Memory mode."]
    MEMORY,
}
impl SMMR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SMMR::SPI => false,
            SMMR::MEMORY => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMMR {
        match value {
            false => SMMR::SPI,
            true => SMMR::MEMORY,
        }
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline]
    pub fn is_spi(&self) -> bool {
        *self == SMMR::SPI
    }
    #[doc = "Checks if the value of the field is `MEMORY`"]
    #[inline]
    pub fn is_memory(&self) -> bool {
        *self == SMMR::MEMORY
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
impl LLBR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LLBR::DISABLED => false,
            LLBR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LLBR {
        match value {
            false => LLBR::DISABLED,
            true => LLBR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LLBR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LLBR::ENABLED
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
impl WDRBTR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            WDRBTR::DISABLED => false,
            WDRBTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDRBTR {
        match value {
            false => WDRBTR::DISABLED,
            true => WDRBTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WDRBTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WDRBTR::ENABLED
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
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CSMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSMODER::NOT_RELOADED => 0,
            CSMODER::LASTXFER => 1,
            CSMODER::SYSTEMATICALLY => 2,
            CSMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSMODER {
        match value {
            0 => CSMODER::NOT_RELOADED,
            1 => CSMODER::LASTXFER,
            2 => CSMODER::SYSTEMATICALLY,
            i => CSMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOT_RELOADED`"]
    #[inline]
    pub fn is_not_reloaded(&self) -> bool {
        *self == CSMODER::NOT_RELOADED
    }
    #[doc = "Checks if the value of the field is `LASTXFER`"]
    #[inline]
    pub fn is_lastxfer(&self) -> bool {
        *self == CSMODER::LASTXFER
    }
    #[doc = "Checks if the value of the field is `SYSTEMATICALLY`"]
    #[inline]
    pub fn is_systematically(&self) -> bool {
        *self == CSMODER::SYSTEMATICALLY
    }
}
#[doc = "Possible values of the field `NBBITS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBBITSR {
    #[doc = "8 bits for transfer"]
    _8_BIT,
    #[doc = "16 bits for transfer"]
    _16_BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NBBITSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NBBITSR::_8_BIT => 0,
            NBBITSR::_16_BIT => 8,
            NBBITSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NBBITSR {
        match value {
            0 => NBBITSR::_8_BIT,
            8 => NBBITSR::_16_BIT,
            i => NBBITSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline]
    pub fn is_8_bit(&self) -> bool {
        *self == NBBITSR::_8_BIT
    }
    #[doc = "Checks if the value of the field is `_16_BIT`"]
    #[inline]
    pub fn is_16_bit(&self) -> bool {
        *self == NBBITSR::_16_BIT
    }
}
#[doc = r" Value of the field"]
pub struct DLYBCTR {
    bits: u8,
}
impl DLYBCTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DLYCSR {
    bits: u8,
}
impl DLYCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMMW::SPI => false,
            SMMW::MEMORY => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMMW<'a> {
    w: &'a mut W,
}
impl<'a> _SMMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The QSPI is in SPI mode."]
    #[inline]
    pub fn spi(self) -> &'a mut W {
        self.variant(SMMW::SPI)
    }
    #[doc = "The QSPI is in Serial Memory mode."]
    #[inline]
    pub fn memory(self) -> &'a mut W {
        self.variant(SMMW::MEMORY)
    }
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LLBW::DISABLED => false,
            LLBW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LLBW<'a> {
    w: &'a mut W,
}
impl<'a> _LLBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LLBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Local loopback path disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LLBW::DISABLED)
    }
    #[doc = "Local loopback path enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LLBW::ENABLED)
    }
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDRBTW::DISABLED => false,
            WDRBTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDRBTW<'a> {
    w: &'a mut W,
}
impl<'a> _WDRBTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDRBTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect. In SPI mode, a transfer can be initiated whatever the state of the QSPI_RDR is."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDRBTW::DISABLED)
    }
    #[doc = "In SPI mode, a transfer can start only if the QSPI_RDR is empty, i.e., does not contain any unread data. This mode prevents overrun error in reception."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WDRBTW::ENABLED)
    }
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSMODEW::NOT_RELOADED => 0,
            CSMODEW::LASTXFER => 1,
            CSMODEW::SYSTEMATICALLY => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CSMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CSMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The chip select is deasserted if QSPI_TDR.TD has not been reloaded before the end of the current transfer."]
    #[inline]
    pub fn not_reloaded(self) -> &'a mut W {
        self.variant(CSMODEW::NOT_RELOADED)
    }
    #[doc = "The chip select is deasserted when the bit LASTXFER is written at 1 and the character written in QSPI_TDR.TD has been transferred."]
    #[inline]
    pub fn lastxfer(self) -> &'a mut W {
        self.variant(CSMODEW::LASTXFER)
    }
    #[doc = "The chip select is deasserted systematically after each transfer."]
    #[inline]
    pub fn systematically(self) -> &'a mut W {
        self.variant(CSMODEW::SYSTEMATICALLY)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NBBITSW::_8_BIT => 0,
            NBBITSW::_16_BIT => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NBBITSW<'a> {
    w: &'a mut W,
}
impl<'a> _NBBITSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NBBITSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8 bits for transfer"]
    #[inline]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(NBBITSW::_8_BIT)
    }
    #[doc = "16 bits for transfer"]
    #[inline]
    pub fn _16_bit(self) -> &'a mut W {
        self.variant(NBBITSW::_16_BIT)
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
#[doc = r" Proxy"]
pub struct _DLYBCTW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYBCTW<'a> {
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
#[doc = r" Proxy"]
pub struct _DLYCSW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYCSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline]
    pub fn smm(&self) -> SMMR {
        SMMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline]
    pub fn llb(&self) -> LLBR {
        LLBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline]
    pub fn wdrbt(&self) -> WDRBTR {
        WDRBTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline]
    pub fn csmode(&self) -> CSMODER {
        CSMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Number Of Bits Per Transfer"]
    #[inline]
    pub fn nbbits(&self) -> NBBITSR {
        NBBITSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline]
    pub fn dlybct(&self) -> DLYBCTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYBCTR { bits }
    }
    #[doc = "Bits 24:31 - Minimum Inactive QCS Delay"]
    #[inline]
    pub fn dlycs(&self) -> DLYCSR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYCSR { bits }
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
    #[doc = "Bit 0 - Serial Memory Mode"]
    #[inline]
    pub fn smm(&mut self) -> _SMMW {
        _SMMW { w: self }
    }
    #[doc = "Bit 1 - Local Loopback Enable"]
    #[inline]
    pub fn llb(&mut self) -> _LLBW {
        _LLBW { w: self }
    }
    #[doc = "Bit 2 - Wait Data Read Before Transfer"]
    #[inline]
    pub fn wdrbt(&mut self) -> _WDRBTW {
        _WDRBTW { w: self }
    }
    #[doc = "Bits 4:5 - Chip Select Mode"]
    #[inline]
    pub fn csmode(&mut self) -> _CSMODEW {
        _CSMODEW { w: self }
    }
    #[doc = "Bits 8:11 - Number Of Bits Per Transfer"]
    #[inline]
    pub fn nbbits(&mut self) -> _NBBITSW {
        _NBBITSW { w: self }
    }
    #[doc = "Bits 16:23 - Delay Between Consecutive Transfers"]
    #[inline]
    pub fn dlybct(&mut self) -> _DLYBCTW {
        _DLYBCTW { w: self }
    }
    #[doc = "Bits 24:31 - Minimum Inactive QCS Delay"]
    #[inline]
    pub fn dlycs(&mut self) -> _DLYCSW {
        _DLYCSW { w: self }
    }
}
