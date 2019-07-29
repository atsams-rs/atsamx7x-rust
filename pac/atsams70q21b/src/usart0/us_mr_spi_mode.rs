#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::US_MR_SPI_MODE {
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
#[doc = "Possible values of the field `USART_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART_MODER {
    #[doc = "Normal mode"]
    NORMAL,
    #[doc = "RS485"]
    RS485,
    #[doc = "Hardware handshaking"]
    HW_HANDSHAKING,
    #[doc = "Modem"]
    MODEM,
    #[doc = "IS07816 Protocol: T = 0"]
    IS07816_T_0,
    #[doc = "IS07816 Protocol: T = 1"]
    IS07816_T_1,
    #[doc = "IrDA"]
    IRDA,
    #[doc = "LON"]
    LON,
    #[doc = "LIN Master mode"]
    LIN_MASTER,
    #[doc = "LIN Slave mode"]
    LIN_SLAVE,
    #[doc = "SPI Master mode (CLKO must be written to 1 and USCLKS = 0, 1 or 2)"]
    SPI_MASTER,
    #[doc = "SPI Slave mode"]
    SPI_SLAVE,
}
impl crate::ToBits<u8> for USART_MODER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            USART_MODER::NORMAL => 0,
            USART_MODER::RS485 => 1,
            USART_MODER::HW_HANDSHAKING => 2,
            USART_MODER::MODEM => 3,
            USART_MODER::IS07816_T_0 => 4,
            USART_MODER::IS07816_T_1 => 6,
            USART_MODER::IRDA => 8,
            USART_MODER::LON => 9,
            USART_MODER::LIN_MASTER => 10,
            USART_MODER::LIN_SLAVE => 11,
            USART_MODER::SPI_MASTER => 14,
            USART_MODER::SPI_SLAVE => 15,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USART_MODE_R = crate::FR<u8, USART_MODER>;
impl USART_MODE_R {
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == USART_MODER::NORMAL
    }
    #[doc = "Checks if the value of the field is `RS485`"]
    #[inline(always)]
    pub fn is_rs485(&self) -> bool {
        *self == USART_MODER::RS485
    }
    #[doc = "Checks if the value of the field is `HW_HANDSHAKING`"]
    #[inline(always)]
    pub fn is_hw_handshaking(&self) -> bool {
        *self == USART_MODER::HW_HANDSHAKING
    }
    #[doc = "Checks if the value of the field is `MODEM`"]
    #[inline(always)]
    pub fn is_modem(&self) -> bool {
        *self == USART_MODER::MODEM
    }
    #[doc = "Checks if the value of the field is `IS07816_T_0`"]
    #[inline(always)]
    pub fn is_is07816_t_0(&self) -> bool {
        *self == USART_MODER::IS07816_T_0
    }
    #[doc = "Checks if the value of the field is `IS07816_T_1`"]
    #[inline(always)]
    pub fn is_is07816_t_1(&self) -> bool {
        *self == USART_MODER::IS07816_T_1
    }
    #[doc = "Checks if the value of the field is `IRDA`"]
    #[inline(always)]
    pub fn is_irda(&self) -> bool {
        *self == USART_MODER::IRDA
    }
    #[doc = "Checks if the value of the field is `LON`"]
    #[inline(always)]
    pub fn is_lon(&self) -> bool {
        *self == USART_MODER::LON
    }
    #[doc = "Checks if the value of the field is `LIN_MASTER`"]
    #[inline(always)]
    pub fn is_lin_master(&self) -> bool {
        *self == USART_MODER::LIN_MASTER
    }
    #[doc = "Checks if the value of the field is `LIN_SLAVE`"]
    #[inline(always)]
    pub fn is_lin_slave(&self) -> bool {
        *self == USART_MODER::LIN_SLAVE
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == USART_MODER::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == USART_MODER::SPI_SLAVE
    }
}
#[doc = "Values that can be written to the field `USART_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART_MODEW {
    #[doc = "Normal mode"]
    NORMAL,
    #[doc = "RS485"]
    RS485,
    #[doc = "Hardware handshaking"]
    HW_HANDSHAKING,
    #[doc = "Modem"]
    MODEM,
    #[doc = "IS07816 Protocol: T = 0"]
    IS07816_T_0,
    #[doc = "IS07816 Protocol: T = 1"]
    IS07816_T_1,
    #[doc = "IrDA"]
    IRDA,
    #[doc = "LON"]
    LON,
    #[doc = "LIN Master mode"]
    LIN_MASTER,
    #[doc = "LIN Slave mode"]
    LIN_SLAVE,
    #[doc = "SPI Master mode (CLKO must be written to 1 and USCLKS = 0, 1 or 2)"]
    SPI_MASTER,
    #[doc = "SPI Slave mode"]
    SPI_SLAVE,
}
impl USART_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            USART_MODEW::NORMAL => 0,
            USART_MODEW::RS485 => 1,
            USART_MODEW::HW_HANDSHAKING => 2,
            USART_MODEW::MODEM => 3,
            USART_MODEW::IS07816_T_0 => 4,
            USART_MODEW::IS07816_T_1 => 6,
            USART_MODEW::IRDA => 8,
            USART_MODEW::LON => 9,
            USART_MODEW::LIN_MASTER => 10,
            USART_MODEW::LIN_SLAVE => 11,
            USART_MODEW::SPI_MASTER => 14,
            USART_MODEW::SPI_SLAVE => 15,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USART_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _USART_MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(USART_MODEW::NORMAL)
    }
    #[doc = "RS485"]
    #[inline(always)]
    pub fn rs485(self) -> &'a mut W {
        self.variant(USART_MODEW::RS485)
    }
    #[doc = "Hardware handshaking"]
    #[inline(always)]
    pub fn hw_handshaking(self) -> &'a mut W {
        self.variant(USART_MODEW::HW_HANDSHAKING)
    }
    #[doc = "Modem"]
    #[inline(always)]
    pub fn modem(self) -> &'a mut W {
        self.variant(USART_MODEW::MODEM)
    }
    #[doc = "IS07816 Protocol: T = 0"]
    #[inline(always)]
    pub fn is07816_t_0(self) -> &'a mut W {
        self.variant(USART_MODEW::IS07816_T_0)
    }
    #[doc = "IS07816 Protocol: T = 1"]
    #[inline(always)]
    pub fn is07816_t_1(self) -> &'a mut W {
        self.variant(USART_MODEW::IS07816_T_1)
    }
    #[doc = "IrDA"]
    #[inline(always)]
    pub fn irda(self) -> &'a mut W {
        self.variant(USART_MODEW::IRDA)
    }
    #[doc = "LON"]
    #[inline(always)]
    pub fn lon(self) -> &'a mut W {
        self.variant(USART_MODEW::LON)
    }
    #[doc = "LIN Master mode"]
    #[inline(always)]
    pub fn lin_master(self) -> &'a mut W {
        self.variant(USART_MODEW::LIN_MASTER)
    }
    #[doc = "LIN Slave mode"]
    #[inline(always)]
    pub fn lin_slave(self) -> &'a mut W {
        self.variant(USART_MODEW::LIN_SLAVE)
    }
    #[doc = "SPI Master mode (CLKO must be written to 1 and USCLKS = 0, 1 or 2)"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(USART_MODEW::SPI_MASTER)
    }
    #[doc = "SPI Slave mode"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(USART_MODEW::SPI_SLAVE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Possible values of the field `USCLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USCLKSR {
    #[doc = "Peripheral clock is selected"]
    MCK,
    #[doc = "Peripheral clock divided (DIV = 8) is selected"]
    DIV,
    #[doc = "PMC programmable clock (PCK) is selected. If the SCK pin is driven (CLKO = 1), the CD field must be greater than 1."]
    PCK,
    #[doc = "Serial clock (SCK) is selected"]
    SCK,
}
impl crate::ToBits<u8> for USCLKSR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            USCLKSR::MCK => 0,
            USCLKSR::DIV => 1,
            USCLKSR::PCK => 2,
            USCLKSR::SCK => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USCLKS_R = crate::FR<u8, USCLKSR>;
impl USCLKS_R {
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == USCLKSR::MCK
    }
    #[doc = "Checks if the value of the field is `DIV`"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        *self == USCLKSR::DIV
    }
    #[doc = "Checks if the value of the field is `PCK`"]
    #[inline(always)]
    pub fn is_pck(&self) -> bool {
        *self == USCLKSR::PCK
    }
    #[doc = "Checks if the value of the field is `SCK`"]
    #[inline(always)]
    pub fn is_sck(&self) -> bool {
        *self == USCLKSR::SCK
    }
}
#[doc = "Values that can be written to the field `USCLKS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USCLKSW {
    #[doc = "Peripheral clock is selected"]
    MCK,
    #[doc = "Peripheral clock divided (DIV = 8) is selected"]
    DIV,
    #[doc = "PMC programmable clock (PCK) is selected. If the SCK pin is driven (CLKO = 1), the CD field must be greater than 1."]
    PCK,
    #[doc = "Serial clock (SCK) is selected"]
    SCK,
}
impl USCLKSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            USCLKSW::MCK => 0,
            USCLKSW::DIV => 1,
            USCLKSW::PCK => 2,
            USCLKSW::SCK => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USCLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _USCLKSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USCLKSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Peripheral clock is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(USCLKSW::MCK)
    }
    #[doc = "Peripheral clock divided (DIV = 8) is selected"]
    #[inline(always)]
    pub fn div(self) -> &'a mut W {
        self.variant(USCLKSW::DIV)
    }
    #[doc = "PMC programmable clock (PCK) is selected. If the SCK pin is driven (CLKO = 1), the CD field must be greater than 1."]
    #[inline(always)]
    pub fn pck(self) -> &'a mut W {
        self.variant(USCLKSW::PCK)
    }
    #[doc = "Serial clock (SCK) is selected"]
    #[inline(always)]
    pub fn sck(self) -> &'a mut W {
        self.variant(USCLKSW::SCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `CHRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHRLR {
    #[doc = "Character length is 5 bits"]
    _5_BIT,
    #[doc = "Character length is 6 bits"]
    _6_BIT,
    #[doc = "Character length is 7 bits"]
    _7_BIT,
    #[doc = "Character length is 8 bits"]
    _8_BIT,
}
impl crate::ToBits<u8> for CHRLR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CHRLR::_5_BIT => 0,
            CHRLR::_6_BIT => 1,
            CHRLR::_7_BIT => 2,
            CHRLR::_8_BIT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CHRL_R = crate::FR<u8, CHRLR>;
impl CHRL_R {
    #[doc = "Checks if the value of the field is `_5_BIT`"]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        *self == CHRLR::_5_BIT
    }
    #[doc = "Checks if the value of the field is `_6_BIT`"]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        *self == CHRLR::_6_BIT
    }
    #[doc = "Checks if the value of the field is `_7_BIT`"]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        *self == CHRLR::_7_BIT
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == CHRLR::_8_BIT
    }
}
#[doc = "Values that can be written to the field `CHRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHRLW {
    #[doc = "Character length is 5 bits"]
    _5_BIT,
    #[doc = "Character length is 6 bits"]
    _6_BIT,
    #[doc = "Character length is 7 bits"]
    _7_BIT,
    #[doc = "Character length is 8 bits"]
    _8_BIT,
}
impl CHRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHRLW::_5_BIT => 0,
            CHRLW::_6_BIT => 1,
            CHRLW::_7_BIT => 2,
            CHRLW::_8_BIT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CHRLW<'a> {
    w: &'a mut W,
}
impl<'a> _CHRLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHRLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Character length is 5 bits"]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut W {
        self.variant(CHRLW::_5_BIT)
    }
    #[doc = "Character length is 6 bits"]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut W {
        self.variant(CHRLW::_6_BIT)
    }
    #[doc = "Character length is 7 bits"]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut W {
        self.variant(CHRLW::_7_BIT)
    }
    #[doc = "Character length is 8 bits"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(CHRLW::_8_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CLKO_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CLKOW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOW<'a> {
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
pub type CPHA_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type WRDBT_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WRDBTW<'a> {
    w: &'a mut W,
}
impl<'a> _WRDBTW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    pub fn usart_mode(&self) -> USART_MODE_R {
        USART_MODE_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    pub fn usclks(&self) -> USCLKS_R {
        USCLKS_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Character Length"]
    #[inline(always)]
    pub fn chrl(&self) -> CHRL_R {
        CHRL_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    pub fn clko(&self) -> CLKO_R {
        CLKO_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SPI Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Wait Read Data Before Transfer"]
    #[inline(always)]
    pub fn wrdbt(&self) -> WRDBT_R {
        WRDBT_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    pub fn usart_mode(&mut self) -> _USART_MODEW {
        _USART_MODEW { w: self }
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    pub fn usclks(&mut self) -> _USCLKSW {
        _USCLKSW { w: self }
    }
    #[doc = "Bits 6:7 - Character Length"]
    #[inline(always)]
    pub fn chrl(&mut self) -> _CHRLW {
        _CHRLW { w: self }
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    pub fn clko(&mut self) -> _CLKOW {
        _CLKOW { w: self }
    }
    #[doc = "Bit 8 - SPI Clock Phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 20 - Wait Read Data Before Transfer"]
    #[inline(always)]
    pub fn wrdbt(&mut self) -> _WRDBTW {
        _WRDBTW { w: self }
    }
}
