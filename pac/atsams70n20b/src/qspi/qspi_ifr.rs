#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::QSPI_IFR {
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
#[doc = "Possible values of the field `WIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIDTHR {
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    SINGLE_BIT_SPI,
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    DUAL_OUTPUT,
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    QUAD_OUTPUT,
    #[doc = "Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    DUAL_IO,
    #[doc = "Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    QUAD_IO,
    #[doc = "Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    DUAL_CMD,
    #[doc = "Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    QUAD_CMD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WIDTHR::SINGLE_BIT_SPI => 0,
            WIDTHR::DUAL_OUTPUT => 1,
            WIDTHR::QUAD_OUTPUT => 2,
            WIDTHR::DUAL_IO => 3,
            WIDTHR::QUAD_IO => 4,
            WIDTHR::DUAL_CMD => 5,
            WIDTHR::QUAD_CMD => 6,
            WIDTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WIDTHR {
        match value {
            0 => WIDTHR::SINGLE_BIT_SPI,
            1 => WIDTHR::DUAL_OUTPUT,
            2 => WIDTHR::QUAD_OUTPUT,
            3 => WIDTHR::DUAL_IO,
            4 => WIDTHR::QUAD_IO,
            5 => WIDTHR::DUAL_CMD,
            6 => WIDTHR::QUAD_CMD,
            i => WIDTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_BIT_SPI`"]
    #[inline]
    pub fn is_single_bit_spi(&self) -> bool {
        *self == WIDTHR::SINGLE_BIT_SPI
    }
    #[doc = "Checks if the value of the field is `DUAL_OUTPUT`"]
    #[inline]
    pub fn is_dual_output(&self) -> bool {
        *self == WIDTHR::DUAL_OUTPUT
    }
    #[doc = "Checks if the value of the field is `QUAD_OUTPUT`"]
    #[inline]
    pub fn is_quad_output(&self) -> bool {
        *self == WIDTHR::QUAD_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DUAL_IO`"]
    #[inline]
    pub fn is_dual_io(&self) -> bool {
        *self == WIDTHR::DUAL_IO
    }
    #[doc = "Checks if the value of the field is `QUAD_IO`"]
    #[inline]
    pub fn is_quad_io(&self) -> bool {
        *self == WIDTHR::QUAD_IO
    }
    #[doc = "Checks if the value of the field is `DUAL_CMD`"]
    #[inline]
    pub fn is_dual_cmd(&self) -> bool {
        *self == WIDTHR::DUAL_CMD
    }
    #[doc = "Checks if the value of the field is `QUAD_CMD`"]
    #[inline]
    pub fn is_quad_cmd(&self) -> bool {
        *self == WIDTHR::QUAD_CMD
    }
}
#[doc = r" Value of the field"]
pub struct INSTENR {
    bits: bool,
}
impl INSTENR {
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
pub struct ADDRENR {
    bits: bool,
}
impl ADDRENR {
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
pub struct OPTENR {
    bits: bool,
}
impl OPTENR {
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
pub struct DATAENR {
    bits: bool,
}
impl DATAENR {
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
#[doc = "Possible values of the field `OPTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPTLR {
    #[doc = "The option code is 1 bit long."]
    OPTION_1BIT,
    #[doc = "The option code is 2 bits long."]
    OPTION_2BIT,
    #[doc = "The option code is 4 bits long."]
    OPTION_4BIT,
    #[doc = "The option code is 8 bits long."]
    OPTION_8BIT,
}
impl OPTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPTLR::OPTION_1BIT => 0,
            OPTLR::OPTION_2BIT => 1,
            OPTLR::OPTION_4BIT => 2,
            OPTLR::OPTION_8BIT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPTLR {
        match value {
            0 => OPTLR::OPTION_1BIT,
            1 => OPTLR::OPTION_2BIT,
            2 => OPTLR::OPTION_4BIT,
            3 => OPTLR::OPTION_8BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OPTION_1BIT`"]
    #[inline]
    pub fn is_option_1bit(&self) -> bool {
        *self == OPTLR::OPTION_1BIT
    }
    #[doc = "Checks if the value of the field is `OPTION_2BIT`"]
    #[inline]
    pub fn is_option_2bit(&self) -> bool {
        *self == OPTLR::OPTION_2BIT
    }
    #[doc = "Checks if the value of the field is `OPTION_4BIT`"]
    #[inline]
    pub fn is_option_4bit(&self) -> bool {
        *self == OPTLR::OPTION_4BIT
    }
    #[doc = "Checks if the value of the field is `OPTION_8BIT`"]
    #[inline]
    pub fn is_option_8bit(&self) -> bool {
        *self == OPTLR::OPTION_8BIT
    }
}
#[doc = "Possible values of the field `ADDRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRLR {
    #[doc = "The address is 24 bits long."]
    _24_BIT,
    #[doc = "The address is 32 bits long."]
    _32_BIT,
}
impl ADDRLR {
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
            ADDRLR::_24_BIT => false,
            ADDRLR::_32_BIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDRLR {
        match value {
            false => ADDRLR::_24_BIT,
            true => ADDRLR::_32_BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_24_BIT`"]
    #[inline]
    pub fn is_24_bit(&self) -> bool {
        *self == ADDRLR::_24_BIT
    }
    #[doc = "Checks if the value of the field is `_32_BIT`"]
    #[inline]
    pub fn is_32_bit(&self) -> bool {
        *self == ADDRLR::_32_BIT
    }
}
#[doc = "Possible values of the field `TFRTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFRTYPR {
    #[doc = "Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial Flash memory is not possible."]
    TRSFR_READ,
    #[doc = "Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial Flash memory is possible."]
    TRSFR_READ_MEMORY,
    #[doc = "Write transfer into the serial memory.Scrambling is not performed."]
    TRSFR_WRITE,
    #[doc = "Write data transfer into the serial memory.If enabled, scrambling is performed."]
    TRSFR_WRITE_MEMORY,
}
impl TFRTYPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TFRTYPR::TRSFR_READ => 0,
            TFRTYPR::TRSFR_READ_MEMORY => 1,
            TFRTYPR::TRSFR_WRITE => 2,
            TFRTYPR::TRSFR_WRITE_MEMORY => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TFRTYPR {
        match value {
            0 => TFRTYPR::TRSFR_READ,
            1 => TFRTYPR::TRSFR_READ_MEMORY,
            2 => TFRTYPR::TRSFR_WRITE,
            3 => TFRTYPR::TRSFR_WRITE_MEMORY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TRSFR_READ`"]
    #[inline]
    pub fn is_trsfr_read(&self) -> bool {
        *self == TFRTYPR::TRSFR_READ
    }
    #[doc = "Checks if the value of the field is `TRSFR_READ_MEMORY`"]
    #[inline]
    pub fn is_trsfr_read_memory(&self) -> bool {
        *self == TFRTYPR::TRSFR_READ_MEMORY
    }
    #[doc = "Checks if the value of the field is `TRSFR_WRITE`"]
    #[inline]
    pub fn is_trsfr_write(&self) -> bool {
        *self == TFRTYPR::TRSFR_WRITE
    }
    #[doc = "Checks if the value of the field is `TRSFR_WRITE_MEMORY`"]
    #[inline]
    pub fn is_trsfr_write_memory(&self) -> bool {
        *self == TFRTYPR::TRSFR_WRITE_MEMORY
    }
}
#[doc = "Possible values of the field `CRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRMR {
    #[doc = "The Continuous Read mode is disabled."]
    DISABLED,
    #[doc = "The Continuous Read mode is enabled."]
    ENABLED,
}
impl CRMR {
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
            CRMR::DISABLED => false,
            CRMR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRMR {
        match value {
            false => CRMR::DISABLED,
            true => CRMR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CRMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CRMR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct NBDUMR {
    bits: u8,
}
impl NBDUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `WIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIDTHW {
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    SINGLE_BIT_SPI,
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    DUAL_OUTPUT,
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    QUAD_OUTPUT,
    #[doc = "Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    DUAL_IO,
    #[doc = "Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    QUAD_IO,
    #[doc = "Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    DUAL_CMD,
    #[doc = "Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    QUAD_CMD,
}
impl WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WIDTHW::SINGLE_BIT_SPI => 0,
            WIDTHW::DUAL_OUTPUT => 1,
            WIDTHW::QUAD_OUTPUT => 2,
            WIDTHW::DUAL_IO => 3,
            WIDTHW::QUAD_IO => 4,
            WIDTHW::DUAL_CMD => 5,
            WIDTHW::QUAD_CMD => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _WIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    #[inline]
    pub fn single_bit_spi(self) -> &'a mut W {
        self.variant(WIDTHW::SINGLE_BIT_SPI)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    #[inline]
    pub fn dual_output(self) -> &'a mut W {
        self.variant(WIDTHW::DUAL_OUTPUT)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    #[inline]
    pub fn quad_output(self) -> &'a mut W {
        self.variant(WIDTHW::QUAD_OUTPUT)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline]
    pub fn dual_io(self) -> &'a mut W {
        self.variant(WIDTHW::DUAL_IO)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline]
    pub fn quad_io(self) -> &'a mut W {
        self.variant(WIDTHW::QUAD_IO)
    }
    #[doc = "Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline]
    pub fn dual_cmd(self) -> &'a mut W {
        self.variant(WIDTHW::DUAL_CMD)
    }
    #[doc = "Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline]
    pub fn quad_cmd(self) -> &'a mut W {
        self.variant(WIDTHW::QUAD_CMD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _INSTENW<'a> {
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADDRENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRENW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OPTENW<'a> {
    w: &'a mut W,
}
impl<'a> _OPTENW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAENW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OPTL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPTLW {
    #[doc = "The option code is 1 bit long."]
    OPTION_1BIT,
    #[doc = "The option code is 2 bits long."]
    OPTION_2BIT,
    #[doc = "The option code is 4 bits long."]
    OPTION_4BIT,
    #[doc = "The option code is 8 bits long."]
    OPTION_8BIT,
}
impl OPTLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPTLW::OPTION_1BIT => 0,
            OPTLW::OPTION_2BIT => 1,
            OPTLW::OPTION_4BIT => 2,
            OPTLW::OPTION_8BIT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPTLW<'a> {
    w: &'a mut W,
}
impl<'a> _OPTLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPTLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The option code is 1 bit long."]
    #[inline]
    pub fn option_1bit(self) -> &'a mut W {
        self.variant(OPTLW::OPTION_1BIT)
    }
    #[doc = "The option code is 2 bits long."]
    #[inline]
    pub fn option_2bit(self) -> &'a mut W {
        self.variant(OPTLW::OPTION_2BIT)
    }
    #[doc = "The option code is 4 bits long."]
    #[inline]
    pub fn option_4bit(self) -> &'a mut W {
        self.variant(OPTLW::OPTION_4BIT)
    }
    #[doc = "The option code is 8 bits long."]
    #[inline]
    pub fn option_8bit(self) -> &'a mut W {
        self.variant(OPTLW::OPTION_8BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADDRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRLW {
    #[doc = "The address is 24 bits long."]
    _24_BIT,
    #[doc = "The address is 32 bits long."]
    _32_BIT,
}
impl ADDRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDRLW::_24_BIT => false,
            ADDRLW::_32_BIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDRLW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The address is 24 bits long."]
    #[inline]
    pub fn _24_bit(self) -> &'a mut W {
        self.variant(ADDRLW::_24_BIT)
    }
    #[doc = "The address is 32 bits long."]
    #[inline]
    pub fn _32_bit(self) -> &'a mut W {
        self.variant(ADDRLW::_32_BIT)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TFRTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFRTYPW {
    #[doc = "Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial Flash memory is not possible."]
    TRSFR_READ,
    #[doc = "Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial Flash memory is possible."]
    TRSFR_READ_MEMORY,
    #[doc = "Write transfer into the serial memory.Scrambling is not performed."]
    TRSFR_WRITE,
    #[doc = "Write data transfer into the serial memory.If enabled, scrambling is performed."]
    TRSFR_WRITE_MEMORY,
}
impl TFRTYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TFRTYPW::TRSFR_READ => 0,
            TFRTYPW::TRSFR_READ_MEMORY => 1,
            TFRTYPW::TRSFR_WRITE => 2,
            TFRTYPW::TRSFR_WRITE_MEMORY => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFRTYPW<'a> {
    w: &'a mut W,
}
impl<'a> _TFRTYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFRTYPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial Flash memory is not possible."]
    #[inline]
    pub fn trsfr_read(self) -> &'a mut W {
        self.variant(TFRTYPW::TRSFR_READ)
    }
    #[doc = "Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial Flash memory is possible."]
    #[inline]
    pub fn trsfr_read_memory(self) -> &'a mut W {
        self.variant(TFRTYPW::TRSFR_READ_MEMORY)
    }
    #[doc = "Write transfer into the serial memory.Scrambling is not performed."]
    #[inline]
    pub fn trsfr_write(self) -> &'a mut W {
        self.variant(TFRTYPW::TRSFR_WRITE)
    }
    #[doc = "Write data transfer into the serial memory.If enabled, scrambling is performed."]
    #[inline]
    pub fn trsfr_write_memory(self) -> &'a mut W {
        self.variant(TFRTYPW::TRSFR_WRITE_MEMORY)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRMW {
    #[doc = "The Continuous Read mode is disabled."]
    DISABLED,
    #[doc = "The Continuous Read mode is enabled."]
    ENABLED,
}
impl CRMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRMW::DISABLED => false,
            CRMW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRMW<'a> {
    w: &'a mut W,
}
impl<'a> _CRMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The Continuous Read mode is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRMW::DISABLED)
    }
    #[doc = "The Continuous Read mode is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRMW::ENABLED)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NBDUMW<'a> {
    w: &'a mut W,
}
impl<'a> _NBDUMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:2 - Width of Instruction Code, Address, Option Code and Data"]
    #[inline]
    pub fn width(&self) -> WIDTHR {
        WIDTHR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Instruction Enable"]
    #[inline]
    pub fn insten(&self) -> INSTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INSTENR { bits }
    }
    #[doc = "Bit 5 - Address Enable"]
    #[inline]
    pub fn addren(&self) -> ADDRENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADDRENR { bits }
    }
    #[doc = "Bit 6 - Option Enable"]
    #[inline]
    pub fn opten(&self) -> OPTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OPTENR { bits }
    }
    #[doc = "Bit 7 - Data Enable"]
    #[inline]
    pub fn dataen(&self) -> DATAENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATAENR { bits }
    }
    #[doc = "Bits 8:9 - Option Code Length"]
    #[inline]
    pub fn optl(&self) -> OPTLR {
        OPTLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Address Length"]
    #[inline]
    pub fn addrl(&self) -> ADDRLR {
        ADDRLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 12:13 - Data Transfer Type"]
    #[inline]
    pub fn tfrtyp(&self) -> TFRTYPR {
        TFRTYPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - Continuous Read Mode"]
    #[inline]
    pub fn crm(&self) -> CRMR {
        CRMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:20 - Number Of Dummy Cycles"]
    #[inline]
    pub fn nbdum(&self) -> NBDUMR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NBDUMR { bits }
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
    #[doc = "Bits 0:2 - Width of Instruction Code, Address, Option Code and Data"]
    #[inline]
    pub fn width(&mut self) -> _WIDTHW {
        _WIDTHW { w: self }
    }
    #[doc = "Bit 4 - Instruction Enable"]
    #[inline]
    pub fn insten(&mut self) -> _INSTENW {
        _INSTENW { w: self }
    }
    #[doc = "Bit 5 - Address Enable"]
    #[inline]
    pub fn addren(&mut self) -> _ADDRENW {
        _ADDRENW { w: self }
    }
    #[doc = "Bit 6 - Option Enable"]
    #[inline]
    pub fn opten(&mut self) -> _OPTENW {
        _OPTENW { w: self }
    }
    #[doc = "Bit 7 - Data Enable"]
    #[inline]
    pub fn dataen(&mut self) -> _DATAENW {
        _DATAENW { w: self }
    }
    #[doc = "Bits 8:9 - Option Code Length"]
    #[inline]
    pub fn optl(&mut self) -> _OPTLW {
        _OPTLW { w: self }
    }
    #[doc = "Bit 10 - Address Length"]
    #[inline]
    pub fn addrl(&mut self) -> _ADDRLW {
        _ADDRLW { w: self }
    }
    #[doc = "Bits 12:13 - Data Transfer Type"]
    #[inline]
    pub fn tfrtyp(&mut self) -> _TFRTYPW {
        _TFRTYPW { w: self }
    }
    #[doc = "Bit 14 - Continuous Read Mode"]
    #[inline]
    pub fn crm(&mut self) -> _CRMW {
        _CRMW { w: self }
    }
    #[doc = "Bits 16:20 - Number Of Dummy Cycles"]
    #[inline]
    pub fn nbdum(&mut self) -> _NBDUMW {
        _NBDUMW { w: self }
    }
}
