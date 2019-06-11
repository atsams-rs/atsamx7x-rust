#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::USBHS_DEVEPTISR_CTRL_MODE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TXINIR {
    bits: bool,
}
impl TXINIR {
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
pub struct RXOUTIR {
    bits: bool,
}
impl RXOUTIR {
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
pub struct RXSTPIR {
    bits: bool,
}
impl RXSTPIR {
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
pub struct NAKOUTIR {
    bits: bool,
}
impl NAKOUTIR {
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
pub struct NAKINIR {
    bits: bool,
}
impl NAKINIR {
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
pub struct OVERFIR {
    bits: bool,
}
impl OVERFIR {
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
pub struct STALLEDIR {
    bits: bool,
}
impl STALLEDIR {
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
pub struct SHORTPACKETR {
    bits: bool,
}
impl SHORTPACKETR {
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
#[doc = "Possible values of the field `DTSEQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTSEQR {
    #[doc = "Data0 toggle sequence"]
    DATA0,
    #[doc = "Data1 toggle sequence"]
    DATA1,
    #[doc = "Reserved for high-bandwidth isochronous endpoint"]
    DATA2,
    #[doc = "Reserved for high-bandwidth isochronous endpoint"]
    MDATA,
}
impl DTSEQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTSEQR::DATA0 => 0,
            DTSEQR::DATA1 => 1,
            DTSEQR::DATA2 => 2,
            DTSEQR::MDATA => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTSEQR {
        match value {
            0 => DTSEQR::DATA0,
            1 => DTSEQR::DATA1,
            2 => DTSEQR::DATA2,
            3 => DTSEQR::MDATA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline]
    pub fn is_data0(&self) -> bool {
        *self == DTSEQR::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline]
    pub fn is_data1(&self) -> bool {
        *self == DTSEQR::DATA1
    }
    #[doc = "Checks if the value of the field is `DATA2`"]
    #[inline]
    pub fn is_data2(&self) -> bool {
        *self == DTSEQR::DATA2
    }
    #[doc = "Checks if the value of the field is `MDATA`"]
    #[inline]
    pub fn is_mdata(&self) -> bool {
        *self == DTSEQR::MDATA
    }
}
#[doc = "Possible values of the field `NBUSYBK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NBUSYBKR {
    #[doc = "0 busy bank (all banks free)"]
    _0_BUSY,
    #[doc = "1 busy bank"]
    _1_BUSY,
    #[doc = "2 busy banks"]
    _2_BUSY,
    #[doc = "3 busy banks"]
    _3_BUSY,
}
impl NBUSYBKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NBUSYBKR::_0_BUSY => 0,
            NBUSYBKR::_1_BUSY => 1,
            NBUSYBKR::_2_BUSY => 2,
            NBUSYBKR::_3_BUSY => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NBUSYBKR {
        match value {
            0 => NBUSYBKR::_0_BUSY,
            1 => NBUSYBKR::_1_BUSY,
            2 => NBUSYBKR::_2_BUSY,
            3 => NBUSYBKR::_3_BUSY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0_BUSY`"]
    #[inline]
    pub fn is_0_busy(&self) -> bool {
        *self == NBUSYBKR::_0_BUSY
    }
    #[doc = "Checks if the value of the field is `_1_BUSY`"]
    #[inline]
    pub fn is_1_busy(&self) -> bool {
        *self == NBUSYBKR::_1_BUSY
    }
    #[doc = "Checks if the value of the field is `_2_BUSY`"]
    #[inline]
    pub fn is_2_busy(&self) -> bool {
        *self == NBUSYBKR::_2_BUSY
    }
    #[doc = "Checks if the value of the field is `_3_BUSY`"]
    #[inline]
    pub fn is_3_busy(&self) -> bool {
        *self == NBUSYBKR::_3_BUSY
    }
}
#[doc = "Possible values of the field `CURRBK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CURRBKR {
    #[doc = "Current bank is bank0"]
    BANK0,
    #[doc = "Current bank is bank1"]
    BANK1,
    #[doc = "Current bank is bank2"]
    BANK2,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CURRBKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CURRBKR::BANK0 => 0,
            CURRBKR::BANK1 => 1,
            CURRBKR::BANK2 => 2,
            CURRBKR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CURRBKR {
        match value {
            0 => CURRBKR::BANK0,
            1 => CURRBKR::BANK1,
            2 => CURRBKR::BANK2,
            i => CURRBKR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BANK0`"]
    #[inline]
    pub fn is_bank0(&self) -> bool {
        *self == CURRBKR::BANK0
    }
    #[doc = "Checks if the value of the field is `BANK1`"]
    #[inline]
    pub fn is_bank1(&self) -> bool {
        *self == CURRBKR::BANK1
    }
    #[doc = "Checks if the value of the field is `BANK2`"]
    #[inline]
    pub fn is_bank2(&self) -> bool {
        *self == CURRBKR::BANK2
    }
}
#[doc = r" Value of the field"]
pub struct RWALLR {
    bits: bool,
}
impl RWALLR {
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
pub struct CTRLDIRR {
    bits: bool,
}
impl CTRLDIRR {
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
pub struct CFGOKR {
    bits: bool,
}
impl CFGOKR {
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
pub struct BYCTR {
    bits: u16,
}
impl BYCTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmitted IN Data Interrupt"]
    #[inline]
    pub fn txini(&self) -> TXINIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXINIR { bits }
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt"]
    #[inline]
    pub fn rxouti(&self) -> RXOUTIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXOUTIR { bits }
    }
    #[doc = "Bit 2 - Received SETUP Interrupt"]
    #[inline]
    pub fn rxstpi(&self) -> RXSTPIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXSTPIR { bits }
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt"]
    #[inline]
    pub fn nakouti(&self) -> NAKOUTIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NAKOUTIR { bits }
    }
    #[doc = "Bit 4 - NAKed IN Interrupt"]
    #[inline]
    pub fn nakini(&self) -> NAKINIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NAKINIR { bits }
    }
    #[doc = "Bit 5 - Overflow Interrupt"]
    #[inline]
    pub fn overfi(&self) -> OVERFIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OVERFIR { bits }
    }
    #[doc = "Bit 6 - STALLed Interrupt"]
    #[inline]
    pub fn stalledi(&self) -> STALLEDIR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STALLEDIR { bits }
    }
    #[doc = "Bit 7 - Short Packet Interrupt"]
    #[inline]
    pub fn shortpacket(&self) -> SHORTPACKETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SHORTPACKETR { bits }
    }
    #[doc = "Bits 8:9 - Data Toggle Sequence"]
    #[inline]
    pub fn dtseq(&self) -> DTSEQR {
        DTSEQR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Number of Busy Banks"]
    #[inline]
    pub fn nbusybk(&self) -> NBUSYBKR {
        NBUSYBKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Current Bank"]
    #[inline]
    pub fn currbk(&self) -> CURRBKR {
        CURRBKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Read/Write Allowed"]
    #[inline]
    pub fn rwall(&self) -> RWALLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RWALLR { bits }
    }
    #[doc = "Bit 17 - Control Direction"]
    #[inline]
    pub fn ctrldir(&self) -> CTRLDIRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CTRLDIRR { bits }
    }
    #[doc = "Bit 18 - Configuration OK Status"]
    #[inline]
    pub fn cfgok(&self) -> CFGOKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CFGOKR { bits }
    }
    #[doc = "Bits 20:30 - Byte Count"]
    #[inline]
    pub fn byct(&self) -> BYCTR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BYCTR { bits }
    }
}
