#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::QSPI_IFR {
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
}
impl crate::ToBits<u8> for WIDTHR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            WIDTHR::SINGLE_BIT_SPI => 0,
            WIDTHR::DUAL_OUTPUT => 1,
            WIDTHR::QUAD_OUTPUT => 2,
            WIDTHR::DUAL_IO => 3,
            WIDTHR::QUAD_IO => 4,
            WIDTHR::DUAL_CMD => 5,
            WIDTHR::QUAD_CMD => 6,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WIDTH_R = crate::FR<u8, WIDTHR>;
impl WIDTH_R {
    #[doc = "Checks if the value of the field is `SINGLE_BIT_SPI`"]
    #[inline(always)]
    pub fn is_single_bit_spi(&self) -> bool {
        *self == WIDTHR::SINGLE_BIT_SPI
    }
    #[doc = "Checks if the value of the field is `DUAL_OUTPUT`"]
    #[inline(always)]
    pub fn is_dual_output(&self) -> bool {
        *self == WIDTHR::DUAL_OUTPUT
    }
    #[doc = "Checks if the value of the field is `QUAD_OUTPUT`"]
    #[inline(always)]
    pub fn is_quad_output(&self) -> bool {
        *self == WIDTHR::QUAD_OUTPUT
    }
    #[doc = "Checks if the value of the field is `DUAL_IO`"]
    #[inline(always)]
    pub fn is_dual_io(&self) -> bool {
        *self == WIDTHR::DUAL_IO
    }
    #[doc = "Checks if the value of the field is `QUAD_IO`"]
    #[inline(always)]
    pub fn is_quad_io(&self) -> bool {
        *self == WIDTHR::QUAD_IO
    }
    #[doc = "Checks if the value of the field is `DUAL_CMD`"]
    #[inline(always)]
    pub fn is_dual_cmd(&self) -> bool {
        *self == WIDTHR::DUAL_CMD
    }
    #[doc = "Checks if the value of the field is `QUAD_CMD`"]
    #[inline(always)]
    pub fn is_quad_cmd(&self) -> bool {
        *self == WIDTHR::QUAD_CMD
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
    #[inline(always)]
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
#[doc = r"Proxy"]
pub struct _WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _WIDTHW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Single-bit SPI"]
    #[inline(always)]
    pub fn single_bit_spi(self) -> &'a mut W {
        self.variant(WIDTHW::SINGLE_BIT_SPI)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_output(self) -> &'a mut W {
        self.variant(WIDTHW::DUAL_OUTPUT)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Single-bit SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_output(self) -> &'a mut W {
        self.variant(WIDTHW::QUAD_OUTPUT)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_io(self) -> &'a mut W {
        self.variant(WIDTHW::DUAL_IO)
    }
    #[doc = "Instruction: Single-bit SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_io(self) -> &'a mut W {
        self.variant(WIDTHW::QUAD_IO)
    }
    #[doc = "Instruction: Dual SPI / Address-Option: Dual SPI / Data: Dual SPI"]
    #[inline(always)]
    pub fn dual_cmd(self) -> &'a mut W {
        self.variant(WIDTHW::DUAL_CMD)
    }
    #[doc = "Instruction: Quad SPI / Address-Option: Quad SPI / Data: Quad SPI"]
    #[inline(always)]
    pub fn quad_cmd(self) -> &'a mut W {
        self.variant(WIDTHW::QUAD_CMD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type INSTEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _INSTENW<'a> {
    w: &'a mut W,
}
impl<'a> _INSTENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ADDREN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ADDRENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type OPTEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _OPTENW<'a> {
    w: &'a mut W,
}
impl<'a> _OPTENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DATAEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DATAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
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
impl crate::ToBits<u8> for OPTLR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            OPTLR::OPTION_1BIT => 0,
            OPTLR::OPTION_2BIT => 1,
            OPTLR::OPTION_4BIT => 2,
            OPTLR::OPTION_8BIT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OPTL_R = crate::FR<u8, OPTLR>;
impl OPTL_R {
    #[doc = "Checks if the value of the field is `OPTION_1BIT`"]
    #[inline(always)]
    pub fn is_option_1bit(&self) -> bool {
        *self == OPTLR::OPTION_1BIT
    }
    #[doc = "Checks if the value of the field is `OPTION_2BIT`"]
    #[inline(always)]
    pub fn is_option_2bit(&self) -> bool {
        *self == OPTLR::OPTION_2BIT
    }
    #[doc = "Checks if the value of the field is `OPTION_4BIT`"]
    #[inline(always)]
    pub fn is_option_4bit(&self) -> bool {
        *self == OPTLR::OPTION_4BIT
    }
    #[doc = "Checks if the value of the field is `OPTION_8BIT`"]
    #[inline(always)]
    pub fn is_option_8bit(&self) -> bool {
        *self == OPTLR::OPTION_8BIT
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPTLW::OPTION_1BIT => 0,
            OPTLW::OPTION_2BIT => 1,
            OPTLW::OPTION_4BIT => 2,
            OPTLW::OPTION_8BIT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _OPTLW<'a> {
    w: &'a mut W,
}
impl<'a> _OPTLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPTLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The option code is 1 bit long."]
    #[inline(always)]
    pub fn option_1bit(self) -> &'a mut W {
        self.variant(OPTLW::OPTION_1BIT)
    }
    #[doc = "The option code is 2 bits long."]
    #[inline(always)]
    pub fn option_2bit(self) -> &'a mut W {
        self.variant(OPTLW::OPTION_2BIT)
    }
    #[doc = "The option code is 4 bits long."]
    #[inline(always)]
    pub fn option_4bit(self) -> &'a mut W {
        self.variant(OPTLW::OPTION_4BIT)
    }
    #[doc = "The option code is 8 bits long."]
    #[inline(always)]
    pub fn option_8bit(self) -> &'a mut W {
        self.variant(OPTLW::OPTION_8BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
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
impl crate::ToBits<bool> for ADDRLR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            ADDRLR::_24_BIT => false,
            ADDRLR::_32_BIT => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ADDRL_R = crate::FR<bool, ADDRLR>;
impl ADDRL_R {
    #[doc = "Checks if the value of the field is `_24_BIT`"]
    #[inline(always)]
    pub fn is_24_bit(&self) -> bool {
        *self == ADDRLR::_24_BIT
    }
    #[doc = "Checks if the value of the field is `_32_BIT`"]
    #[inline(always)]
    pub fn is_32_bit(&self) -> bool {
        *self == ADDRLR::_32_BIT
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
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDRLW::_24_BIT => false,
            ADDRLW::_32_BIT => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADDRLW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The address is 24 bits long."]
    #[inline(always)]
    pub fn _24_bit(self) -> &'a mut W {
        self.variant(ADDRLW::_24_BIT)
    }
    #[doc = "The address is 32 bits long."]
    #[inline(always)]
    pub fn _32_bit(self) -> &'a mut W {
        self.variant(ADDRLW::_32_BIT)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
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
impl crate::ToBits<u8> for TFRTYPR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TFRTYPR::TRSFR_READ => 0,
            TFRTYPR::TRSFR_READ_MEMORY => 1,
            TFRTYPR::TRSFR_WRITE => 2,
            TFRTYPR::TRSFR_WRITE_MEMORY => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TFRTYP_R = crate::FR<u8, TFRTYPR>;
impl TFRTYP_R {
    #[doc = "Checks if the value of the field is `TRSFR_READ`"]
    #[inline(always)]
    pub fn is_trsfr_read(&self) -> bool {
        *self == TFRTYPR::TRSFR_READ
    }
    #[doc = "Checks if the value of the field is `TRSFR_READ_MEMORY`"]
    #[inline(always)]
    pub fn is_trsfr_read_memory(&self) -> bool {
        *self == TFRTYPR::TRSFR_READ_MEMORY
    }
    #[doc = "Checks if the value of the field is `TRSFR_WRITE`"]
    #[inline(always)]
    pub fn is_trsfr_write(&self) -> bool {
        *self == TFRTYPR::TRSFR_WRITE
    }
    #[doc = "Checks if the value of the field is `TRSFR_WRITE_MEMORY`"]
    #[inline(always)]
    pub fn is_trsfr_write_memory(&self) -> bool {
        *self == TFRTYPR::TRSFR_WRITE_MEMORY
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            TFRTYPW::TRSFR_READ => 0,
            TFRTYPW::TRSFR_READ_MEMORY => 1,
            TFRTYPW::TRSFR_WRITE => 2,
            TFRTYPW::TRSFR_WRITE_MEMORY => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TFRTYPW<'a> {
    w: &'a mut W,
}
impl<'a> _TFRTYPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFRTYPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Read transfer from the serial memory.Scrambling is not performed.Read at random location (fetch) in the serial Flash memory is not possible."]
    #[inline(always)]
    pub fn trsfr_read(self) -> &'a mut W {
        self.variant(TFRTYPW::TRSFR_READ)
    }
    #[doc = "Read data transfer from the serial memory.If enabled, scrambling is performed.Read at random location (fetch) in the serial Flash memory is possible."]
    #[inline(always)]
    pub fn trsfr_read_memory(self) -> &'a mut W {
        self.variant(TFRTYPW::TRSFR_READ_MEMORY)
    }
    #[doc = "Write transfer into the serial memory.Scrambling is not performed."]
    #[inline(always)]
    pub fn trsfr_write(self) -> &'a mut W {
        self.variant(TFRTYPW::TRSFR_WRITE)
    }
    #[doc = "Write data transfer into the serial memory.If enabled, scrambling is performed."]
    #[inline(always)]
    pub fn trsfr_write_memory(self) -> &'a mut W {
        self.variant(TFRTYPW::TRSFR_WRITE_MEMORY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
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
impl crate::ToBits<bool> for CRMR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CRMR::DISABLED => false,
            CRMR::ENABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CRM_R = crate::FR<bool, CRMR>;
impl CRM_R {
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRMR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRMR::ENABLED
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
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CRMW::DISABLED => false,
            CRMW::ENABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CRMW<'a> {
    w: &'a mut W,
}
impl<'a> _CRMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The Continuous Read mode is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRMW::DISABLED)
    }
    #[doc = "The Continuous Read mode is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRMW::ENABLED)
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
pub type NBDUM_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _NBDUMW<'a> {
    w: &'a mut W,
}
impl<'a> _NBDUMW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Width of Instruction Code, Address, Option Code and Data"]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new((self.bits() & 0x07) as u8)
    }
    #[doc = "Bit 4 - Instruction Enable"]
    #[inline(always)]
    pub fn insten(&self) -> INSTEN_R {
        INSTEN_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Address Enable"]
    #[inline(always)]
    pub fn addren(&self) -> ADDREN_R {
        ADDREN_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Option Enable"]
    #[inline(always)]
    pub fn opten(&self) -> OPTEN_R {
        OPTEN_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Data Enable"]
    #[inline(always)]
    pub fn dataen(&self) -> DATAEN_R {
        DATAEN_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Option Code Length"]
    #[inline(always)]
    pub fn optl(&self) -> OPTL_R {
        OPTL_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Address Length"]
    #[inline(always)]
    pub fn addrl(&self) -> ADDRL_R {
        ADDRL_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Data Transfer Type"]
    #[inline(always)]
    pub fn tfrtyp(&self) -> TFRTYP_R {
        TFRTYP_R::new(((self.bits() >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Continuous Read Mode"]
    #[inline(always)]
    pub fn crm(&self) -> CRM_R {
        CRM_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Number Of Dummy Cycles"]
    #[inline(always)]
    pub fn nbdum(&self) -> NBDUM_R {
        NBDUM_R::new(((self.bits() >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Width of Instruction Code, Address, Option Code and Data"]
    #[inline(always)]
    pub fn width(&mut self) -> _WIDTHW {
        _WIDTHW { w: self }
    }
    #[doc = "Bit 4 - Instruction Enable"]
    #[inline(always)]
    pub fn insten(&mut self) -> _INSTENW {
        _INSTENW { w: self }
    }
    #[doc = "Bit 5 - Address Enable"]
    #[inline(always)]
    pub fn addren(&mut self) -> _ADDRENW {
        _ADDRENW { w: self }
    }
    #[doc = "Bit 6 - Option Enable"]
    #[inline(always)]
    pub fn opten(&mut self) -> _OPTENW {
        _OPTENW { w: self }
    }
    #[doc = "Bit 7 - Data Enable"]
    #[inline(always)]
    pub fn dataen(&mut self) -> _DATAENW {
        _DATAENW { w: self }
    }
    #[doc = "Bits 8:9 - Option Code Length"]
    #[inline(always)]
    pub fn optl(&mut self) -> _OPTLW {
        _OPTLW { w: self }
    }
    #[doc = "Bit 10 - Address Length"]
    #[inline(always)]
    pub fn addrl(&mut self) -> _ADDRLW {
        _ADDRLW { w: self }
    }
    #[doc = "Bits 12:13 - Data Transfer Type"]
    #[inline(always)]
    pub fn tfrtyp(&mut self) -> _TFRTYPW {
        _TFRTYPW { w: self }
    }
    #[doc = "Bit 14 - Continuous Read Mode"]
    #[inline(always)]
    pub fn crm(&mut self) -> _CRMW {
        _CRMW { w: self }
    }
    #[doc = "Bits 16:20 - Number Of Dummy Cycles"]
    #[inline(always)]
    pub fn nbdum(&mut self) -> _NBDUMW {
        _NBDUMW { w: self }
    }
}
