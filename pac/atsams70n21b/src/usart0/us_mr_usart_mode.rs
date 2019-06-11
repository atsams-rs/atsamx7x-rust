#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::US_MR_USART_MODE {
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
pub struct SYNCR {
    bits: bool,
}
impl SYNCR {
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
#[doc = "Possible values of the field `PAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARR {
    #[doc = "Even parity"]
    EVEN,
    #[doc = "Odd parity"]
    ODD,
    #[doc = "Parity forced to 0 (Space)"]
    SPACE,
    #[doc = "Parity forced to 1 (Mark)"]
    MARK,
    #[doc = "No parity"]
    NO,
    #[doc = "Multidrop mode"]
    MULTIDROP,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PARR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PARR::EVEN => 0,
            PARR::ODD => 1,
            PARR::SPACE => 2,
            PARR::MARK => 3,
            PARR::NO => 4,
            PARR::MULTIDROP => 6,
            PARR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PARR {
        match value {
            0 => PARR::EVEN,
            1 => PARR::ODD,
            2 => PARR::SPACE,
            3 => PARR::MARK,
            4 => PARR::NO,
            6 => PARR::MULTIDROP,
            i => PARR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline]
    pub fn is_even(&self) -> bool {
        *self == PARR::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline]
    pub fn is_odd(&self) -> bool {
        *self == PARR::ODD
    }
    #[doc = "Checks if the value of the field is `SPACE`"]
    #[inline]
    pub fn is_space(&self) -> bool {
        *self == PARR::SPACE
    }
    #[doc = "Checks if the value of the field is `MARK`"]
    #[inline]
    pub fn is_mark(&self) -> bool {
        *self == PARR::MARK
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline]
    pub fn is_no(&self) -> bool {
        *self == PARR::NO
    }
    #[doc = "Checks if the value of the field is `MULTIDROP`"]
    #[inline]
    pub fn is_multidrop(&self) -> bool {
        *self == PARR::MULTIDROP
    }
}
#[doc = "Possible values of the field `NBSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBSTOPR {
    #[doc = "1 stop bit"]
    _1_BIT,
    #[doc = "1.5 stop bit (SYNC = 0) or reserved (SYNC = 1)"]
    _1_5_BIT,
    #[doc = "2 stop bits"]
    _2_BIT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NBSTOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NBSTOPR::_1_BIT => 0,
            NBSTOPR::_1_5_BIT => 1,
            NBSTOPR::_2_BIT => 2,
            NBSTOPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NBSTOPR {
        match value {
            0 => NBSTOPR::_1_BIT,
            1 => NBSTOPR::_1_5_BIT,
            2 => NBSTOPR::_2_BIT,
            i => NBSTOPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1_BIT`"]
    #[inline]
    pub fn is_1_bit(&self) -> bool {
        *self == NBSTOPR::_1_BIT
    }
    #[doc = "Checks if the value of the field is `_1_5_BIT`"]
    #[inline]
    pub fn is_1_5_bit(&self) -> bool {
        *self == NBSTOPR::_1_5_BIT
    }
    #[doc = "Checks if the value of the field is `_2_BIT`"]
    #[inline]
    pub fn is_2_bit(&self) -> bool {
        *self == NBSTOPR::_2_BIT
    }
}
#[doc = "Possible values of the field `CHMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHMODER {
    #[doc = "Normal mode"]
    NORMAL,
    #[doc = "Automatic Echo. Receiver input is connected to the TXD pin."]
    AUTOMATIC,
    #[doc = "Local Loopback. Transmitter output is connected to the Receiver Input."]
    LOCAL_LOOPBACK,
    #[doc = "Remote Loopback. RXD pin is internally connected to the TXD pin."]
    REMOTE_LOOPBACK,
}
impl CHMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CHMODER::NORMAL => 0,
            CHMODER::AUTOMATIC => 1,
            CHMODER::LOCAL_LOOPBACK => 2,
            CHMODER::REMOTE_LOOPBACK => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CHMODER {
        match value {
            0 => CHMODER::NORMAL,
            1 => CHMODER::AUTOMATIC,
            2 => CHMODER::LOCAL_LOOPBACK,
            3 => CHMODER::REMOTE_LOOPBACK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == CHMODER::NORMAL
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline]
    pub fn is_automatic(&self) -> bool {
        *self == CHMODER::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `LOCAL_LOOPBACK`"]
    #[inline]
    pub fn is_local_loopback(&self) -> bool {
        *self == CHMODER::LOCAL_LOOPBACK
    }
    #[doc = "Checks if the value of the field is `REMOTE_LOOPBACK`"]
    #[inline]
    pub fn is_remote_loopback(&self) -> bool {
        *self == CHMODER::REMOTE_LOOPBACK
    }
}
#[doc = r" Value of the field"]
pub struct MSBFR {
    bits: bool,
}
impl MSBFR {
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
pub struct MODE9R {
    bits: bool,
}
impl MODE9R {
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
pub struct OVERR {
    bits: bool,
}
impl OVERR {
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
pub struct INACKR {
    bits: bool,
}
impl INACKR {
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
pub struct DSNACKR {
    bits: bool,
}
impl DSNACKR {
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
pub struct VAR_SYNCR {
    bits: bool,
}
impl VAR_SYNCR {
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
pub struct INVDATAR {
    bits: bool,
}
impl INVDATAR {
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
pub struct MAX_ITERATIONR {
    bits: u8,
}
impl MAX_ITERATIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FILTERR {
    bits: bool,
}
impl FILTERR {
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
pub struct MANR {
    bits: bool,
}
impl MANR {
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
pub struct MODSYNCR {
    bits: bool,
}
impl MODSYNCR {
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
pub struct ONEBITR {
    bits: bool,
}
impl ONEBITR {
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
pub struct _SYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCW<'a> {
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
#[doc = "Values that can be written to the field `PAR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARW {
    #[doc = "Even parity"]
    EVEN,
    #[doc = "Odd parity"]
    ODD,
    #[doc = "Parity forced to 0 (Space)"]
    SPACE,
    #[doc = "Parity forced to 1 (Mark)"]
    MARK,
    #[doc = "No parity"]
    NO,
    #[doc = "Multidrop mode"]
    MULTIDROP,
}
impl PARW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PARW::EVEN => 0,
            PARW::ODD => 1,
            PARW::SPACE => 2,
            PARW::MARK => 3,
            PARW::NO => 4,
            PARW::MULTIDROP => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PARW<'a> {
    w: &'a mut W,
}
impl<'a> _PARW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PARW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Even parity"]
    #[inline]
    pub fn even(self) -> &'a mut W {
        self.variant(PARW::EVEN)
    }
    #[doc = "Odd parity"]
    #[inline]
    pub fn odd(self) -> &'a mut W {
        self.variant(PARW::ODD)
    }
    #[doc = "Parity forced to 0 (Space)"]
    #[inline]
    pub fn space(self) -> &'a mut W {
        self.variant(PARW::SPACE)
    }
    #[doc = "Parity forced to 1 (Mark)"]
    #[inline]
    pub fn mark(self) -> &'a mut W {
        self.variant(PARW::MARK)
    }
    #[doc = "No parity"]
    #[inline]
    pub fn no(self) -> &'a mut W {
        self.variant(PARW::NO)
    }
    #[doc = "Multidrop mode"]
    #[inline]
    pub fn multidrop(self) -> &'a mut W {
        self.variant(PARW::MULTIDROP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NBSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBSTOPW {
    #[doc = "1 stop bit"]
    _1_BIT,
    #[doc = "1.5 stop bit (SYNC = 0) or reserved (SYNC = 1)"]
    _1_5_BIT,
    #[doc = "2 stop bits"]
    _2_BIT,
}
impl NBSTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            NBSTOPW::_1_BIT => 0,
            NBSTOPW::_1_5_BIT => 1,
            NBSTOPW::_2_BIT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NBSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _NBSTOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NBSTOPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 stop bit"]
    #[inline]
    pub fn _1_bit(self) -> &'a mut W {
        self.variant(NBSTOPW::_1_BIT)
    }
    #[doc = "1.5 stop bit (SYNC = 0) or reserved (SYNC = 1)"]
    #[inline]
    pub fn _1_5_bit(self) -> &'a mut W {
        self.variant(NBSTOPW::_1_5_BIT)
    }
    #[doc = "2 stop bits"]
    #[inline]
    pub fn _2_bit(self) -> &'a mut W {
        self.variant(NBSTOPW::_2_BIT)
    }
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
#[doc = "Values that can be written to the field `CHMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHMODEW {
    #[doc = "Normal mode"]
    NORMAL,
    #[doc = "Automatic Echo. Receiver input is connected to the TXD pin."]
    AUTOMATIC,
    #[doc = "Local Loopback. Transmitter output is connected to the Receiver Input."]
    LOCAL_LOOPBACK,
    #[doc = "Remote Loopback. RXD pin is internally connected to the TXD pin."]
    REMOTE_LOOPBACK,
}
impl CHMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CHMODEW::NORMAL => 0,
            CHMODEW::AUTOMATIC => 1,
            CHMODEW::LOCAL_LOOPBACK => 2,
            CHMODEW::REMOTE_LOOPBACK => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CHMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal mode"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(CHMODEW::NORMAL)
    }
    #[doc = "Automatic Echo. Receiver input is connected to the TXD pin."]
    #[inline]
    pub fn automatic(self) -> &'a mut W {
        self.variant(CHMODEW::AUTOMATIC)
    }
    #[doc = "Local Loopback. Transmitter output is connected to the Receiver Input."]
    #[inline]
    pub fn local_loopback(self) -> &'a mut W {
        self.variant(CHMODEW::LOCAL_LOOPBACK)
    }
    #[doc = "Remote Loopback. RXD pin is internally connected to the TXD pin."]
    #[inline]
    pub fn remote_loopback(self) -> &'a mut W {
        self.variant(CHMODEW::REMOTE_LOOPBACK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MSBFW<'a> {
    w: &'a mut W,
}
impl<'a> _MSBFW<'a> {
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
pub struct _MODE9W<'a> {
    w: &'a mut W,
}
impl<'a> _MODE9W<'a> {
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
pub struct _OVERW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERW<'a> {
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
#[doc = r" Proxy"]
pub struct _INACKW<'a> {
    w: &'a mut W,
}
impl<'a> _INACKW<'a> {
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
#[doc = r" Proxy"]
pub struct _DSNACKW<'a> {
    w: &'a mut W,
}
impl<'a> _DSNACKW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VAR_SYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _VAR_SYNCW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INVDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _INVDATAW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MAX_ITERATIONW<'a> {
    w: &'a mut W,
}
impl<'a> _MAX_ITERATIONW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FILTERW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTERW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MANW<'a> {
    w: &'a mut W,
}
impl<'a> _MANW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MODSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _MODSYNCW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ONEBITW<'a> {
    w: &'a mut W,
}
impl<'a> _ONEBITW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 8 - Synchronous Mode Select"]
    #[inline]
    pub fn sync(&self) -> SYNCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYNCR { bits }
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline]
    pub fn par(&self) -> PARR {
        PARR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Number of Stop Bits"]
    #[inline]
    pub fn nbstop(&self) -> NBSTOPR {
        NBSTOPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline]
    pub fn chmode(&self) -> CHMODER {
        CHMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Bit Order"]
    #[inline]
    pub fn msbf(&self) -> MSBFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MSBFR { bits }
    }
    #[doc = "Bit 17 - 9-bit Character Length"]
    #[inline]
    pub fn mode9(&self) -> MODE9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MODE9R { bits }
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
    #[doc = "Bit 19 - Oversampling Mode"]
    #[inline]
    pub fn over(&self) -> OVERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OVERR { bits }
    }
    #[doc = "Bit 20 - Inhibit Non Acknowledge"]
    #[inline]
    pub fn inack(&self) -> INACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INACKR { bits }
    }
    #[doc = "Bit 21 - Disable Successive NACK"]
    #[inline]
    pub fn dsnack(&self) -> DSNACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DSNACKR { bits }
    }
    #[doc = "Bit 22 - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
    #[inline]
    pub fn var_sync(&self) -> VAR_SYNCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VAR_SYNCR { bits }
    }
    #[doc = "Bit 23 - Inverted Data"]
    #[inline]
    pub fn invdata(&self) -> INVDATAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INVDATAR { bits }
    }
    #[doc = "Bits 24:26 - Maximum Number of Automatic Iteration"]
    #[inline]
    pub fn max_iteration(&self) -> MAX_ITERATIONR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAX_ITERATIONR { bits }
    }
    #[doc = "Bit 28 - Receive Line Filter"]
    #[inline]
    pub fn filter(&self) -> FILTERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FILTERR { bits }
    }
    #[doc = "Bit 29 - Manchester Encoder/Decoder Enable"]
    #[inline]
    pub fn man(&self) -> MANR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MANR { bits }
    }
    #[doc = "Bit 30 - Manchester Synchronization Mode"]
    #[inline]
    pub fn modsync(&self) -> MODSYNCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MODSYNCR { bits }
    }
    #[doc = "Bit 31 - Start Frame Delimiter Selector"]
    #[inline]
    pub fn onebit(&self) -> ONEBITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ONEBITR { bits }
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
    #[doc = "Bit 8 - Synchronous Mode Select"]
    #[inline]
    pub fn sync(&mut self) -> _SYNCW {
        _SYNCW { w: self }
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline]
    pub fn par(&mut self) -> _PARW {
        _PARW { w: self }
    }
    #[doc = "Bits 12:13 - Number of Stop Bits"]
    #[inline]
    pub fn nbstop(&mut self) -> _NBSTOPW {
        _NBSTOPW { w: self }
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline]
    pub fn chmode(&mut self) -> _CHMODEW {
        _CHMODEW { w: self }
    }
    #[doc = "Bit 16 - Bit Order"]
    #[inline]
    pub fn msbf(&mut self) -> _MSBFW {
        _MSBFW { w: self }
    }
    #[doc = "Bit 17 - 9-bit Character Length"]
    #[inline]
    pub fn mode9(&mut self) -> _MODE9W {
        _MODE9W { w: self }
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline]
    pub fn clko(&mut self) -> _CLKOW {
        _CLKOW { w: self }
    }
    #[doc = "Bit 19 - Oversampling Mode"]
    #[inline]
    pub fn over(&mut self) -> _OVERW {
        _OVERW { w: self }
    }
    #[doc = "Bit 20 - Inhibit Non Acknowledge"]
    #[inline]
    pub fn inack(&mut self) -> _INACKW {
        _INACKW { w: self }
    }
    #[doc = "Bit 21 - Disable Successive NACK"]
    #[inline]
    pub fn dsnack(&mut self) -> _DSNACKW {
        _DSNACKW { w: self }
    }
    #[doc = "Bit 22 - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
    #[inline]
    pub fn var_sync(&mut self) -> _VAR_SYNCW {
        _VAR_SYNCW { w: self }
    }
    #[doc = "Bit 23 - Inverted Data"]
    #[inline]
    pub fn invdata(&mut self) -> _INVDATAW {
        _INVDATAW { w: self }
    }
    #[doc = "Bits 24:26 - Maximum Number of Automatic Iteration"]
    #[inline]
    pub fn max_iteration(&mut self) -> _MAX_ITERATIONW {
        _MAX_ITERATIONW { w: self }
    }
    #[doc = "Bit 28 - Receive Line Filter"]
    #[inline]
    pub fn filter(&mut self) -> _FILTERW {
        _FILTERW { w: self }
    }
    #[doc = "Bit 29 - Manchester Encoder/Decoder Enable"]
    #[inline]
    pub fn man(&mut self) -> _MANW {
        _MANW { w: self }
    }
    #[doc = "Bit 30 - Manchester Synchronization Mode"]
    #[inline]
    pub fn modsync(&mut self) -> _MODSYNCW {
        _MODSYNCW { w: self }
    }
    #[doc = "Bit 31 - Start Frame Delimiter Selector"]
    #[inline]
    pub fn onebit(&mut self) -> _ONEBITW {
        _ONEBITW { w: self }
    }
}
