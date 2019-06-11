#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::US_MR_SPI_MODE {
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
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl USART_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
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
            USART_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USART_MODER {
        match value {
            0 => USART_MODER::NORMAL,
            1 => USART_MODER::RS485,
            2 => USART_MODER::HW_HANDSHAKING,
            3 => USART_MODER::MODEM,
            4 => USART_MODER::IS07816_T_0,
            6 => USART_MODER::IS07816_T_1,
            8 => USART_MODER::IRDA,
            9 => USART_MODER::LON,
            10 => USART_MODER::LIN_MASTER,
            11 => USART_MODER::LIN_SLAVE,
            14 => USART_MODER::SPI_MASTER,
            15 => USART_MODER::SPI_SLAVE,
            i => USART_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == USART_MODER::NORMAL
    }
    #[doc = "Checks if the value of the field is `RS485`"]
    #[inline]
    pub fn is_rs485(&self) -> bool {
        *self == USART_MODER::RS485
    }
    #[doc = "Checks if the value of the field is `HW_HANDSHAKING`"]
    #[inline]
    pub fn is_hw_handshaking(&self) -> bool {
        *self == USART_MODER::HW_HANDSHAKING
    }
    #[doc = "Checks if the value of the field is `MODEM`"]
    #[inline]
    pub fn is_modem(&self) -> bool {
        *self == USART_MODER::MODEM
    }
    #[doc = "Checks if the value of the field is `IS07816_T_0`"]
    #[inline]
    pub fn is_is07816_t_0(&self) -> bool {
        *self == USART_MODER::IS07816_T_0
    }
    #[doc = "Checks if the value of the field is `IS07816_T_1`"]
    #[inline]
    pub fn is_is07816_t_1(&self) -> bool {
        *self == USART_MODER::IS07816_T_1
    }
    #[doc = "Checks if the value of the field is `IRDA`"]
    #[inline]
    pub fn is_irda(&self) -> bool {
        *self == USART_MODER::IRDA
    }
    #[doc = "Checks if the value of the field is `LON`"]
    #[inline]
    pub fn is_lon(&self) -> bool {
        *self == USART_MODER::LON
    }
    #[doc = "Checks if the value of the field is `LIN_MASTER`"]
    #[inline]
    pub fn is_lin_master(&self) -> bool {
        *self == USART_MODER::LIN_MASTER
    }
    #[doc = "Checks if the value of the field is `LIN_SLAVE`"]
    #[inline]
    pub fn is_lin_slave(&self) -> bool {
        *self == USART_MODER::LIN_SLAVE
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline]
    pub fn is_spi_master(&self) -> bool {
        *self == USART_MODER::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline]
    pub fn is_spi_slave(&self) -> bool {
        *self == USART_MODER::SPI_SLAVE
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
impl USCLKSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            USCLKSR::MCK => 0,
            USCLKSR::DIV => 1,
            USCLKSR::PCK => 2,
            USCLKSR::SCK => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> USCLKSR {
        match value {
            0 => USCLKSR::MCK,
            1 => USCLKSR::DIV,
            2 => USCLKSR::PCK,
            3 => USCLKSR::SCK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline]
    pub fn is_mck(&self) -> bool {
        *self == USCLKSR::MCK
    }
    #[doc = "Checks if the value of the field is `DIV`"]
    #[inline]
    pub fn is_div(&self) -> bool {
        *self == USCLKSR::DIV
    }
    #[doc = "Checks if the value of the field is `PCK`"]
    #[inline]
    pub fn is_pck(&self) -> bool {
        *self == USCLKSR::PCK
    }
    #[doc = "Checks if the value of the field is `SCK`"]
    #[inline]
    pub fn is_sck(&self) -> bool {
        *self == USCLKSR::SCK
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
impl CHRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHRLR::_5_BIT => 0,
            CHRLR::_6_BIT => 1,
            CHRLR::_7_BIT => 2,
            CHRLR::_8_BIT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHRLR {
        match value {
            0 => CHRLR::_5_BIT,
            1 => CHRLR::_6_BIT,
            2 => CHRLR::_7_BIT,
            3 => CHRLR::_8_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_5_BIT`"]
    #[inline]
    pub fn is_5_bit(&self) -> bool {
        *self == CHRLR::_5_BIT
    }
    #[doc = "Checks if the value of the field is `_6_BIT`"]
    #[inline]
    pub fn is_6_bit(&self) -> bool {
        *self == CHRLR::_6_BIT
    }
    #[doc = "Checks if the value of the field is `_7_BIT`"]
    #[inline]
    pub fn is_7_bit(&self) -> bool {
        *self == CHRLR::_7_BIT
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline]
    pub fn is_8_bit(&self) -> bool {
        *self == CHRLR::_8_BIT
    }
}
#[doc = r" Value of the field"]
pub struct CLKOR {
    bits: bool,
}
impl CLKOR {
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
pub struct CPHAR {
    bits: bool,
}
impl CPHAR {
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
pub struct WRDBTR {
    bits: bool,
}
impl WRDBTR {
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
    #[inline]
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
#[doc = r" Proxy"]
pub struct _USART_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _USART_MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal mode"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(USART_MODEW::NORMAL)
    }
    #[doc = "RS485"]
    #[inline]
    pub fn rs485(self) -> &'a mut W {
        self.variant(USART_MODEW::RS485)
    }
    #[doc = "Hardware handshaking"]
    #[inline]
    pub fn hw_handshaking(self) -> &'a mut W {
        self.variant(USART_MODEW::HW_HANDSHAKING)
    }
    #[doc = "Modem"]
    #[inline]
    pub fn modem(self) -> &'a mut W {
        self.variant(USART_MODEW::MODEM)
    }
    #[doc = "IS07816 Protocol: T = 0"]
    #[inline]
    pub fn is07816_t_0(self) -> &'a mut W {
        self.variant(USART_MODEW::IS07816_T_0)
    }
    #[doc = "IS07816 Protocol: T = 1"]
    #[inline]
    pub fn is07816_t_1(self) -> &'a mut W {
        self.variant(USART_MODEW::IS07816_T_1)
    }
    #[doc = "IrDA"]
    #[inline]
    pub fn irda(self) -> &'a mut W {
        self.variant(USART_MODEW::IRDA)
    }
    #[doc = "LON"]
    #[inline]
    pub fn lon(self) -> &'a mut W {
        self.variant(USART_MODEW::LON)
    }
    #[doc = "LIN Master mode"]
    #[inline]
    pub fn lin_master(self) -> &'a mut W {
        self.variant(USART_MODEW::LIN_MASTER)
    }
    #[doc = "LIN Slave mode"]
    #[inline]
    pub fn lin_slave(self) -> &'a mut W {
        self.variant(USART_MODEW::LIN_SLAVE)
    }
    #[doc = "SPI Master mode (CLKO must be written to 1 and USCLKS = 0, 1 or 2)"]
    #[inline]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(USART_MODEW::SPI_MASTER)
    }
    #[doc = "SPI Slave mode"]
    #[inline]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(USART_MODEW::SPI_SLAVE)
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            USCLKSW::MCK => 0,
            USCLKSW::DIV => 1,
            USCLKSW::PCK => 2,
            USCLKSW::SCK => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USCLKSW<'a> {
    w: &'a mut W,
}
impl<'a> _USCLKSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USCLKSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Peripheral clock is selected"]
    #[inline]
    pub fn mck(self) -> &'a mut W {
        self.variant(USCLKSW::MCK)
    }
    #[doc = "Peripheral clock divided (DIV = 8) is selected"]
    #[inline]
    pub fn div(self) -> &'a mut W {
        self.variant(USCLKSW::DIV)
    }
    #[doc = "PMC programmable clock (PCK) is selected. If the SCK pin is driven (CLKO = 1), the CD field must be greater than 1."]
    #[inline]
    pub fn pck(self) -> &'a mut W {
        self.variant(USCLKSW::PCK)
    }
    #[doc = "Serial clock (SCK) is selected"]
    #[inline]
    pub fn sck(self) -> &'a mut W {
        self.variant(USCLKSW::SCK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHRLW::_5_BIT => 0,
            CHRLW::_6_BIT => 1,
            CHRLW::_7_BIT => 2,
            CHRLW::_8_BIT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHRLW<'a> {
    w: &'a mut W,
}
impl<'a> _CHRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHRLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Character length is 5 bits"]
    #[inline]
    pub fn _5_bit(self) -> &'a mut W {
        self.variant(CHRLW::_5_BIT)
    }
    #[doc = "Character length is 6 bits"]
    #[inline]
    pub fn _6_bit(self) -> &'a mut W {
        self.variant(CHRLW::_6_BIT)
    }
    #[doc = "Character length is 7 bits"]
    #[inline]
    pub fn _7_bit(self) -> &'a mut W {
        self.variant(CHRLW::_7_BIT)
    }
    #[doc = "Character length is 8 bits"]
    #[inline]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(CHRLW::_8_BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLKOW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKOW<'a> {
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
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WRDBTW<'a> {
    w: &'a mut W,
}
impl<'a> _WRDBTW<'a> {
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
        const OFFSET: u8 = 20;
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
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline]
    pub fn usart_mode(&self) -> USART_MODER {
        USART_MODER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline]
    pub fn usclks(&self) -> USCLKSR {
        USCLKSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Character Length"]
    #[inline]
    pub fn chrl(&self) -> CHRLR {
        CHRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline]
    pub fn clko(&self) -> CLKOR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CLKOR { bits }
    }
    #[doc = "Bit 8 - SPI Clock Phase"]
    #[inline]
    pub fn cpha(&self) -> CPHAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CPHAR { bits }
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CPOLR { bits }
    }
    #[doc = "Bit 20 - Wait Read Data Before Transfer"]
    #[inline]
    pub fn wrdbt(&self) -> WRDBTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WRDBTR { bits }
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
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline]
    pub fn usart_mode(&mut self) -> _USART_MODEW {
        _USART_MODEW { w: self }
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline]
    pub fn usclks(&mut self) -> _USCLKSW {
        _USCLKSW { w: self }
    }
    #[doc = "Bits 6:7 - Character Length"]
    #[inline]
    pub fn chrl(&mut self) -> _CHRLW {
        _CHRLW { w: self }
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline]
    pub fn clko(&mut self) -> _CLKOW {
        _CLKOW { w: self }
    }
    #[doc = "Bit 8 - SPI Clock Phase"]
    #[inline]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 20 - Wait Read Data Before Transfer"]
    #[inline]
    pub fn wrdbt(&mut self) -> _WRDBTW {
        _WRDBTW { w: self }
    }
}
