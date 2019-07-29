#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CHIPID_CIDR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type VERSION_R = crate::FR<u8, u8>;
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
impl crate::ToBits<u8> for EPROCR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
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
}
#[doc = r"Reader of the field"]
pub type EPROC_R = crate::FR<u8, EPROCR>;
impl EPROC_R {
    #[doc = "Checks if the value of the field is `SAMX7`"]
    #[inline(always)]
    pub fn is_samx7(&self) -> bool {
        *self == EPROCR::SAMX7
    }
    #[doc = "Checks if the value of the field is `ARM946ES`"]
    #[inline(always)]
    pub fn is_arm946es(&self) -> bool {
        *self == EPROCR::ARM946ES
    }
    #[doc = "Checks if the value of the field is `ARM7TDMI`"]
    #[inline(always)]
    pub fn is_arm7tdmi(&self) -> bool {
        *self == EPROCR::ARM7TDMI
    }
    #[doc = "Checks if the value of the field is `CM3`"]
    #[inline(always)]
    pub fn is_cm3(&self) -> bool {
        *self == EPROCR::CM3
    }
    #[doc = "Checks if the value of the field is `ARM920T`"]
    #[inline(always)]
    pub fn is_arm920t(&self) -> bool {
        *self == EPROCR::ARM920T
    }
    #[doc = "Checks if the value of the field is `ARM926EJS`"]
    #[inline(always)]
    pub fn is_arm926ejs(&self) -> bool {
        *self == EPROCR::ARM926EJS
    }
    #[doc = "Checks if the value of the field is `CA5`"]
    #[inline(always)]
    pub fn is_ca5(&self) -> bool {
        *self == EPROCR::CA5
    }
    #[doc = "Checks if the value of the field is `CM4`"]
    #[inline(always)]
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
}
impl crate::ToBits<u8> for NVPSIZR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
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
        }
    }
}
#[doc = r"Reader of the field"]
pub type NVPSIZ_R = crate::FR<u8, NVPSIZR>;
impl NVPSIZ_R {
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == NVPSIZR::NONE
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == NVPSIZR::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == NVPSIZR::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == NVPSIZR::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == NVPSIZR::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == NVPSIZR::_128K
    }
    #[doc = "Checks if the value of the field is `_160K`"]
    #[inline(always)]
    pub fn is_160k(&self) -> bool {
        *self == NVPSIZR::_160K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == NVPSIZR::_256K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == NVPSIZR::_512K
    }
    #[doc = "Checks if the value of the field is `_1024K`"]
    #[inline(always)]
    pub fn is_1024k(&self) -> bool {
        *self == NVPSIZR::_1024K
    }
    #[doc = "Checks if the value of the field is `_2048K`"]
    #[inline(always)]
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
}
impl crate::ToBits<u8> for NVPSIZ2R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
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
        }
    }
}
#[doc = r"Reader of the field"]
pub type NVPSIZ2_R = crate::FR<u8, NVPSIZ2R>;
impl NVPSIZ2_R {
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == NVPSIZ2R::NONE
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == NVPSIZ2R::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == NVPSIZ2R::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == NVPSIZ2R::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == NVPSIZ2R::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == NVPSIZ2R::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == NVPSIZ2R::_256K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == NVPSIZ2R::_512K
    }
    #[doc = "Checks if the value of the field is `_1024K`"]
    #[inline(always)]
    pub fn is_1024k(&self) -> bool {
        *self == NVPSIZ2R::_1024K
    }
    #[doc = "Checks if the value of the field is `_2048K`"]
    #[inline(always)]
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
impl crate::ToBits<u8> for SRAMSIZR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
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
}
#[doc = r"Reader of the field"]
pub type SRAMSIZ_R = crate::FR<u8, SRAMSIZR>;
impl SRAMSIZ_R {
    #[doc = "Checks if the value of the field is `_48K`"]
    #[inline(always)]
    pub fn is_48k(&self) -> bool {
        *self == SRAMSIZR::_48K
    }
    #[doc = "Checks if the value of the field is `_192K`"]
    #[inline(always)]
    pub fn is_192k(&self) -> bool {
        *self == SRAMSIZR::_192K
    }
    #[doc = "Checks if the value of the field is `_384K`"]
    #[inline(always)]
    pub fn is_384k(&self) -> bool {
        *self == SRAMSIZR::_384K
    }
    #[doc = "Checks if the value of the field is `_6K`"]
    #[inline(always)]
    pub fn is_6k(&self) -> bool {
        *self == SRAMSIZR::_6K
    }
    #[doc = "Checks if the value of the field is `_24K`"]
    #[inline(always)]
    pub fn is_24k(&self) -> bool {
        *self == SRAMSIZR::_24K
    }
    #[doc = "Checks if the value of the field is `_4K`"]
    #[inline(always)]
    pub fn is_4k(&self) -> bool {
        *self == SRAMSIZR::_4K
    }
    #[doc = "Checks if the value of the field is `_80K`"]
    #[inline(always)]
    pub fn is_80k(&self) -> bool {
        *self == SRAMSIZR::_80K
    }
    #[doc = "Checks if the value of the field is `_160K`"]
    #[inline(always)]
    pub fn is_160k(&self) -> bool {
        *self == SRAMSIZR::_160K
    }
    #[doc = "Checks if the value of the field is `_8K`"]
    #[inline(always)]
    pub fn is_8k(&self) -> bool {
        *self == SRAMSIZR::_8K
    }
    #[doc = "Checks if the value of the field is `_16K`"]
    #[inline(always)]
    pub fn is_16k(&self) -> bool {
        *self == SRAMSIZR::_16K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == SRAMSIZR::_32K
    }
    #[doc = "Checks if the value of the field is `_64K`"]
    #[inline(always)]
    pub fn is_64k(&self) -> bool {
        *self == SRAMSIZR::_64K
    }
    #[doc = "Checks if the value of the field is `_128K`"]
    #[inline(always)]
    pub fn is_128k(&self) -> bool {
        *self == SRAMSIZR::_128K
    }
    #[doc = "Checks if the value of the field is `_256K`"]
    #[inline(always)]
    pub fn is_256k(&self) -> bool {
        *self == SRAMSIZR::_256K
    }
    #[doc = "Checks if the value of the field is `_96K`"]
    #[inline(always)]
    pub fn is_96k(&self) -> bool {
        *self == SRAMSIZR::_96K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
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
}
impl crate::ToBits<u8> for ARCHR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ARCHR::SAME70 => 16,
            ARCHR::SAMS70 => 17,
            ARCHR::SAMV71 => 18,
            ARCHR::SAMV70 => 19,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ARCH_R = crate::FR<u8, ARCHR>;
impl ARCH_R {
    #[doc = "Checks if the value of the field is `SAME70`"]
    #[inline(always)]
    pub fn is_same70(&self) -> bool {
        *self == ARCHR::SAME70
    }
    #[doc = "Checks if the value of the field is `SAMS70`"]
    #[inline(always)]
    pub fn is_sams70(&self) -> bool {
        *self == ARCHR::SAMS70
    }
    #[doc = "Checks if the value of the field is `SAMV71`"]
    #[inline(always)]
    pub fn is_samv71(&self) -> bool {
        *self == ARCHR::SAMV71
    }
    #[doc = "Checks if the value of the field is `SAMV70`"]
    #[inline(always)]
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
}
impl crate::ToBits<u8> for NVPTYPR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            NVPTYPR::ROM => 0,
            NVPTYPR::ROMLESS => 1,
            NVPTYPR::FLASH => 2,
            NVPTYPR::ROM_FLASH => 3,
            NVPTYPR::SRAM => 4,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NVPTYP_R = crate::FR<u8, NVPTYPR>;
impl NVPTYP_R {
    #[doc = "Checks if the value of the field is `ROM`"]
    #[inline(always)]
    pub fn is_rom(&self) -> bool {
        *self == NVPTYPR::ROM
    }
    #[doc = "Checks if the value of the field is `ROMLESS`"]
    #[inline(always)]
    pub fn is_romless(&self) -> bool {
        *self == NVPTYPR::ROMLESS
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == NVPTYPR::FLASH
    }
    #[doc = "Checks if the value of the field is `ROM_FLASH`"]
    #[inline(always)]
    pub fn is_rom_flash(&self) -> bool {
        *self == NVPTYPR::ROM_FLASH
    }
    #[doc = "Checks if the value of the field is `SRAM`"]
    #[inline(always)]
    pub fn is_sram(&self) -> bool {
        *self == NVPTYPR::SRAM
    }
}
#[doc = r"Reader of the field"]
pub type EXT_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Version of the Device"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits() & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Embedded Processor"]
    #[inline(always)]
    pub fn eproc(&self) -> EPROC_R {
        EPROC_R::new(((self.bits() >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Nonvolatile Program Memory Size"]
    #[inline(always)]
    pub fn nvpsiz(&self) -> NVPSIZ_R {
        NVPSIZ_R::new(((self.bits() >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Second Nonvolatile Program Memory Size"]
    #[inline(always)]
    pub fn nvpsiz2(&self) -> NVPSIZ2_R {
        NVPSIZ2_R::new(((self.bits() >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Internal SRAM Size"]
    #[inline(always)]
    pub fn sramsiz(&self) -> SRAMSIZ_R {
        SRAMSIZ_R::new(((self.bits() >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - Architecture Identifier"]
    #[inline(always)]
    pub fn arch(&self) -> ARCH_R {
        ARCH_R::new(((self.bits() >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:30 - Nonvolatile Program Memory Type"]
    #[inline(always)]
    pub fn nvptyp(&self) -> NVPTYP_R {
        NVPTYP_R::new(((self.bits() >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - Extension Flag"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
