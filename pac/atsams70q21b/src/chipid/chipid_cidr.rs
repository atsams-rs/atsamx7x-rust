#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CHIPID_CIDR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct VERSIONR {
    bits: u8,
}
impl VERSIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `EPROC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPROCR {
    #[doc = "Cortex-M7"]
    SAMX7,
    #[doc = "ARM946ES"]
    ARM946ES,
    #[doc = "ARM7TDMI"]
    ARM7TDMI,
    #[doc = "Cortex-M3"]
    CM3,
    #[doc = "ARM920T"]
    ARM920T,
    #[doc = "ARM926EJS"]
    ARM926EJS,
    #[doc = "Cortex-A5"]
    CA5,
    #[doc = "Cortex-M4"]
    CM4,
}
impl EPROCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EPROCR::SAMX7 => 0,
            EPROCR::ARM946ES => 1,
            EPROCR::ARM7TDMI => 2,
            EPROCR::CM3 => 3,
            EPROCR::ARM920T => 4,
            EPROCR::ARM926EJS => 5,
            EPROCR::CA5 => 6,
            EPROCR::CM4 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EPROCR {
        match value {
            0 => EPROCR::SAMX7,
            1 => EPROCR::ARM946ES,
            2 => EPROCR::ARM7TDMI,
            3 => EPROCR::CM3,
            4 => EPROCR::ARM920T,
            5 => EPROCR::ARM926EJS,
            6 => EPROCR::CA5,
            7 => EPROCR::CM4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAMX7`"]
    #[inline]
    pub fn is_samx7(&self) -> bool {
        *self == EPROCR::SAMX7
    }
    #[doc = "Checks if the value of the field is `ARM946ES`"]
    #[inline]
    pub fn is_arm946es(&self) -> bool {
        *self == EPROCR::ARM946ES
    }
    #[doc = "Checks if the value of the field is `ARM7TDMI`"]
    #[inline]
    pub fn is_arm7tdmi(&self) -> bool {
        *self == EPROCR::ARM7TDMI
    }
    #[doc = "Checks if the value of the field is `CM3`"]
    #[inline]
    pub fn is_cm3(&self) -> bool {
        *self == EPROCR::CM3
    }
    #[doc = "Checks if the value of the field is `ARM920T`"]
    #[inline]
    pub fn is_arm920t(&self) -> bool {
        *self == EPROCR::ARM920T
    }
    #[doc = "Checks if the value of the field is `ARM926EJS`"]
    #[inline]
    pub fn is_arm926ejs(&self) -> bool {
        *self == EPROCR::ARM926EJS
    }
    #[doc = "Checks if the value of the field is `CA5`"]
    #[inline]
    pub fn is_ca5(&self) -> bool {
        *self == EPROCR::CA5
    }
    #[doc = "Checks if the value of the field is `CM4`"]
    #[inline]
    pub fn is_cm4(&self) -> bool {
        *self == EPROCR::CM4
    }
}
#[doc = "Possible values of the field `NVPSIZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NVPSIZR {
    #[doc = "None"]
    NONE,
    #[doc = "8 Kbytes"]
    _8K,
    #[doc = "16 Kbytes"]
    _16K,
    #[doc = "32 Kbytes"]
    _32K,
    #[doc = "64 Kbytes"]
    _64K,
    #[doc = "128 Kbytes"]
    _128K,
    #[doc = "160 Kbytes"]
    _160K,
    #[doc = "256 Kbytes"]
    _256K,
    #[doc = "512 Kbytes"]
    _512K,
    #[doc = "1024 Kbytes"]
    _1024K,
    #[doc = "2048 Kbytes"]
    _2048K,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NVPSIZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NVPSIZR::NONE => 0,
            NVPSIZR::_8K => 1,
            NVPSIZR::_16K => 2,
            NVPSIZR::_32K => 3,
            NVPSIZR::_64K => 5,
            NVPSIZR::_128K => 7,
            NVPSIZR::_160K => 8,
            NVPSIZR::_256K => 9,
            NVPSIZR::_512K => 10,
            NVPSIZR::_1024K => 12,
            NVPSIZR::_2048K => 14,
            NVPSIZR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NVPSIZR {
        match value {
            0 => NVPSIZR::NONE,
            1 => NVPSIZR::_8K,
            2 => NVPSIZR::_16K,
            3 => NVPSIZR::_32K,
            5 => NVPSIZR::_64K,
            7 => NVPSIZR::_128K,
            8 => NVPSIZR::_160K,
            9 => NVPSIZR::_256K,
            10 => NVPSIZR::_512K,
            12 => NVPSIZR::_1024K,
            14 => NVPSIZR::_2048K,
            i => NVPSIZR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == NVPSIZR::NONE
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline]
    pub fn is_8k(&self) -> bool {
        *self == NVPSIZR::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline]
    pub fn is_16k(&self) -> bool {
        *self == NVPSIZR::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline]
    pub fn is_32k(&self) -> bool {
        *self == NVPSIZR::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline]
    pub fn is_64k(&self) -> bool {
        *self == NVPSIZR::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline]
    pub fn is_128k(&self) -> bool {
        *self == NVPSIZR::_128K
    }
    #[doc = "Checks if the value of the field is `_160K`"]
    #[inline]
    pub fn is_160k(&self) -> bool {
        *self == NVPSIZR::_160K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline]
    pub fn is_256k(&self) -> bool {
        *self == NVPSIZR::_256K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline]
    pub fn is_512k(&self) -> bool {
        *self == NVPSIZR::_512K
    }
    #[doc = "Checks if the value of the field is `_1024K`"]
    #[inline]
    pub fn is_1024k(&self) -> bool {
        *self == NVPSIZR::_1024K
    }
    #[doc = "Checks if the value of the field is `_2048K`"]
    #[inline]
    pub fn is_2048k(&self) -> bool {
        *self == NVPSIZR::_2048K
    }
}
#[doc = "Possible values of the field `NVPSIZ2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NVPSIZ2R {
    #[doc = "None"]
    NONE,
    #[doc = "8 Kbytes"]
    _8K,
    #[doc = "16 Kbytes"]
    _16K,
    #[doc = "32 Kbytes"]
    _32K,
    #[doc = "64 Kbytes"]
    _64K,
    #[doc = "128 Kbytes"]
    _128K,
    #[doc = "256 Kbytes"]
    _256K,
    #[doc = "512 Kbytes"]
    _512K,
    #[doc = "1024 Kbytes"]
    _1024K,
    #[doc = "2048 Kbytes"]
    _2048K,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NVPSIZ2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NVPSIZ2R::NONE => 0,
            NVPSIZ2R::_8K => 1,
            NVPSIZ2R::_16K => 2,
            NVPSIZ2R::_32K => 3,
            NVPSIZ2R::_64K => 5,
            NVPSIZ2R::_128K => 7,
            NVPSIZ2R::_256K => 9,
            NVPSIZ2R::_512K => 10,
            NVPSIZ2R::_1024K => 12,
            NVPSIZ2R::_2048K => 14,
            NVPSIZ2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NVPSIZ2R {
        match value {
            0 => NVPSIZ2R::NONE,
            1 => NVPSIZ2R::_8K,
            2 => NVPSIZ2R::_16K,
            3 => NVPSIZ2R::_32K,
            5 => NVPSIZ2R::_64K,
            7 => NVPSIZ2R::_128K,
            9 => NVPSIZ2R::_256K,
            10 => NVPSIZ2R::_512K,
            12 => NVPSIZ2R::_1024K,
            14 => NVPSIZ2R::_2048K,
            i => NVPSIZ2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == NVPSIZ2R::NONE
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline]
    pub fn is_8k(&self) -> bool {
        *self == NVPSIZ2R::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline]
    pub fn is_16k(&self) -> bool {
        *self == NVPSIZ2R::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline]
    pub fn is_32k(&self) -> bool {
        *self == NVPSIZ2R::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline]
    pub fn is_64k(&self) -> bool {
        *self == NVPSIZ2R::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline]
    pub fn is_128k(&self) -> bool {
        *self == NVPSIZ2R::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline]
    pub fn is_256k(&self) -> bool {
        *self == NVPSIZ2R::_256K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline]
    pub fn is_512k(&self) -> bool {
        *self == NVPSIZ2R::_512K
    }
    #[doc = "Checks if the value of the field is `_1024K`"]
    #[inline]
    pub fn is_1024k(&self) -> bool {
        *self == NVPSIZ2R::_1024K
    }
    #[doc = "Checks if the value of the field is `_2048K`"]
    #[inline]
    pub fn is_2048k(&self) -> bool {
        *self == NVPSIZ2R::_2048K
    }
}
#[doc = "Possible values of the field `SRAMSIZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMSIZR {
    #[doc = "48 Kbytes"]
    _48K,
    #[doc = "192 Kbytes"]
    _192K,
    #[doc = "384 Kbytes"]
    _384K,
    #[doc = "6 Kbytes"]
    _6K,
    #[doc = "24 Kbytes"]
    _24K,
    #[doc = "4 Kbytes"]
    _4K,
    #[doc = "80 Kbytes"]
    _80K,
    #[doc = "160 Kbytes"]
    _160K,
    #[doc = "8 Kbytes"]
    _8K,
    #[doc = "16 Kbytes"]
    _16K,
    #[doc = "32 Kbytes"]
    _32K,
    #[doc = "64 Kbytes"]
    _64K,
    #[doc = "128 Kbytes"]
    _128K,
    #[doc = "256 Kbytes"]
    _256K,
    #[doc = "96 Kbytes"]
    _96K,
    #[doc = "512 Kbytes"]
    _512K,
}
impl SRAMSIZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRAMSIZR::_48K => 0,
            SRAMSIZR::_192K => 1,
            SRAMSIZR::_384K => 2,
            SRAMSIZR::_6K => 3,
            SRAMSIZR::_24K => 4,
            SRAMSIZR::_4K => 5,
            SRAMSIZR::_80K => 6,
            SRAMSIZR::_160K => 7,
            SRAMSIZR::_8K => 8,
            SRAMSIZR::_16K => 9,
            SRAMSIZR::_32K => 10,
            SRAMSIZR::_64K => 11,
            SRAMSIZR::_128K => 12,
            SRAMSIZR::_256K => 13,
            SRAMSIZR::_96K => 14,
            SRAMSIZR::_512K => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRAMSIZR {
        match value {
            0 => SRAMSIZR::_48K,
            1 => SRAMSIZR::_192K,
            2 => SRAMSIZR::_384K,
            3 => SRAMSIZR::_6K,
            4 => SRAMSIZR::_24K,
            5 => SRAMSIZR::_4K,
            6 => SRAMSIZR::_80K,
            7 => SRAMSIZR::_160K,
            8 => SRAMSIZR::_8K,
            9 => SRAMSIZR::_16K,
            10 => SRAMSIZR::_32K,
            11 => SRAMSIZR::_64K,
            12 => SRAMSIZR::_128K,
            13 => SRAMSIZR::_256K,
            14 => SRAMSIZR::_96K,
            15 => SRAMSIZR::_512K,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_48K`"]
    #[inline]
    pub fn is_48k(&self) -> bool {
        *self == SRAMSIZR::_48K
    }
    #[doc = "Checks if the value of the field is `_192K`"]
    #[inline]
    pub fn is_192k(&self) -> bool {
        *self == SRAMSIZR::_192K
    }
    #[doc = "Checks if the value of the field is `_384K`"]
    #[inline]
    pub fn is_384k(&self) -> bool {
        *self == SRAMSIZR::_384K
    }
    #[doc = "Checks if the value of the field is `_6K`"]
    #[inline]
    pub fn is_6k(&self) -> bool {
        *self == SRAMSIZR::_6K
    }
    #[doc = "Checks if the value of the field is `_24K`"]
    #[inline]
    pub fn is_24k(&self) -> bool {
        *self == SRAMSIZR::_24K
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline]
    pub fn is_4k(&self) -> bool {
        *self == SRAMSIZR::_4K
    }
    #[doc = "Checks if the value of the field is `_80K`"]
    #[inline]
    pub fn is_80k(&self) -> bool {
        *self == SRAMSIZR::_80K
    }
    #[doc = "Checks if the value of the field is `_160K`"]
    #[inline]
    pub fn is_160k(&self) -> bool {
        *self == SRAMSIZR::_160K
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline]
    pub fn is_8k(&self) -> bool {
        *self == SRAMSIZR::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline]
    pub fn is_16k(&self) -> bool {
        *self == SRAMSIZR::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline]
    pub fn is_32k(&self) -> bool {
        *self == SRAMSIZR::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline]
    pub fn is_64k(&self) -> bool {
        *self == SRAMSIZR::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline]
    pub fn is_128k(&self) -> bool {
        *self == SRAMSIZR::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline]
    pub fn is_256k(&self) -> bool {
        *self == SRAMSIZR::_256K
    }
    #[doc = "Checks if the value of the field is `_96K`"]
    #[inline]
    pub fn is_96k(&self) -> bool {
        *self == SRAMSIZR::_96K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline]
    pub fn is_512k(&self) -> bool {
        *self == SRAMSIZR::_512K
    }
}
#[doc = "Possible values of the field `ARCH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARCHR {
    #[doc = "SAM E70"]
    SAME70,
    #[doc = "SAM S70"]
    SAMS70,
    #[doc = "SAM V71"]
    SAMV71,
    #[doc = "SAM V70"]
    SAMV70,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ARCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ARCHR::SAME70 => 16,
            ARCHR::SAMS70 => 17,
            ARCHR::SAMV71 => 18,
            ARCHR::SAMV70 => 19,
            ARCHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ARCHR {
        match value {
            16 => ARCHR::SAME70,
            17 => ARCHR::SAMS70,
            18 => ARCHR::SAMV71,
            19 => ARCHR::SAMV70,
            i => ARCHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SAME70`"]
    #[inline]
    pub fn is_same70(&self) -> bool {
        *self == ARCHR::SAME70
    }
    #[doc = "Checks if the value of the field is `SAMS70`"]
    #[inline]
    pub fn is_sams70(&self) -> bool {
        *self == ARCHR::SAMS70
    }
    #[doc = "Checks if the value of the field is `SAMV71`"]
    #[inline]
    pub fn is_samv71(&self) -> bool {
        *self == ARCHR::SAMV71
    }
    #[doc = "Checks if the value of the field is `SAMV70`"]
    #[inline]
    pub fn is_samv70(&self) -> bool {
        *self == ARCHR::SAMV70
    }
}
#[doc = "Possible values of the field `NVPTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NVPTYPR {
    #[doc = "ROM"]
    ROM,
    #[doc = "ROMless or on-chip Flash"]
    ROMLESS,
    #[doc = "Embedded Flash Memory"]
    FLASH,
    #[doc = "ROM and Embedded Flash Memory- NVPSIZ is ROM size- NVPSIZ2 is Flash size"]
    ROM_FLASH,
    #[doc = "SRAM emulating ROM"]
    SRAM,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl NVPTYPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            NVPTYPR::ROM => 0,
            NVPTYPR::ROMLESS => 1,
            NVPTYPR::FLASH => 2,
            NVPTYPR::ROM_FLASH => 3,
            NVPTYPR::SRAM => 4,
            NVPTYPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> NVPTYPR {
        match value {
            0 => NVPTYPR::ROM,
            1 => NVPTYPR::ROMLESS,
            2 => NVPTYPR::FLASH,
            3 => NVPTYPR::ROM_FLASH,
            4 => NVPTYPR::SRAM,
            i => NVPTYPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ROM`"]
    #[inline]
    pub fn is_rom(&self) -> bool {
        *self == NVPTYPR::ROM
    }
    #[doc = "Checks if the value of the field is `ROMLESS`"]
    #[inline]
    pub fn is_romless(&self) -> bool {
        *self == NVPTYPR::ROMLESS
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline]
    pub fn is_flash(&self) -> bool {
        *self == NVPTYPR::FLASH
    }
    #[doc = "Checks if the value of the field is `ROM_FLASH`"]
    #[inline]
    pub fn is_rom_flash(&self) -> bool {
        *self == NVPTYPR::ROM_FLASH
    }
    #[doc = "Checks if the value of the field is `SRAM`"]
    #[inline]
    pub fn is_sram(&self) -> bool {
        *self == NVPTYPR::SRAM
    }
}
#[doc = r" Value of the field"]
pub struct EXTR {
    bits: bool,
}
impl EXTR {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Version of the Device"]
    #[inline]
    pub fn version(&self) -> VERSIONR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VERSIONR { bits }
    }
    #[doc = "Bits 5:7 - Embedded Processor"]
    #[inline]
    pub fn eproc(&self) -> EPROCR {
        EPROCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Nonvolatile Program Memory Size"]
    #[inline]
    pub fn nvpsiz(&self) -> NVPSIZR {
        NVPSIZR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Second Nonvolatile Program Memory Size"]
    #[inline]
    pub fn nvpsiz2(&self) -> NVPSIZ2R {
        NVPSIZ2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Internal SRAM Size"]
    #[inline]
    pub fn sramsiz(&self) -> SRAMSIZR {
        SRAMSIZR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:27 - Architecture Identifier"]
    #[inline]
    pub fn arch(&self) -> ARCHR {
        ARCHR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:30 - Nonvolatile Program Memory Type"]
    #[inline]
    pub fn nvptyp(&self) -> NVPTYPR {
        NVPTYPR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - Extension Flag"]
    #[inline]
    pub fn ext(&self) -> EXTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EXTR { bits }
    }
}
