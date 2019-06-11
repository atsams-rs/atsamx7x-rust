#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AES_ISR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct DATRDYR {
    bits: bool,
}
impl DATRDYR {
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
pub struct URADR {
    bits: bool,
}
impl URADR {
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
#[doc = "Possible values of the field `URAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum URATR {
    #[doc = "Input Data Register written during the data processing when SMOD = 0x2 mode."]
    IDR_WR_PROCESSING,
    #[doc = "Output Data Register read during the data processing."]
    ODR_RD_PROCESSING,
    #[doc = "Mode Register written during the data processing."]
    MR_WR_PROCESSING,
    #[doc = "Output Data Register read during the sub-keys generation."]
    ODR_RD_SUBKGEN,
    #[doc = "Mode Register written during the sub-keys generation."]
    MR_WR_SUBKGEN,
    #[doc = "Write-only register read access."]
    WOR_RD_ACCESS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl URATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            URATR::IDR_WR_PROCESSING => 0,
            URATR::ODR_RD_PROCESSING => 1,
            URATR::MR_WR_PROCESSING => 2,
            URATR::ODR_RD_SUBKGEN => 3,
            URATR::MR_WR_SUBKGEN => 4,
            URATR::WOR_RD_ACCESS => 5,
            URATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> URATR {
        match value {
            0 => URATR::IDR_WR_PROCESSING,
            1 => URATR::ODR_RD_PROCESSING,
            2 => URATR::MR_WR_PROCESSING,
            3 => URATR::ODR_RD_SUBKGEN,
            4 => URATR::MR_WR_SUBKGEN,
            5 => URATR::WOR_RD_ACCESS,
            i => URATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDR_WR_PROCESSING`"]
    #[inline]
    pub fn is_idr_wr_processing(&self) -> bool {
        *self == URATR::IDR_WR_PROCESSING
    }
    #[doc = "Checks if the value of the field is `ODR_RD_PROCESSING`"]
    #[inline]
    pub fn is_odr_rd_processing(&self) -> bool {
        *self == URATR::ODR_RD_PROCESSING
    }
    #[doc = "Checks if the value of the field is `MR_WR_PROCESSING`"]
    #[inline]
    pub fn is_mr_wr_processing(&self) -> bool {
        *self == URATR::MR_WR_PROCESSING
    }
    #[doc = "Checks if the value of the field is `ODR_RD_SUBKGEN`"]
    #[inline]
    pub fn is_odr_rd_subkgen(&self) -> bool {
        *self == URATR::ODR_RD_SUBKGEN
    }
    #[doc = "Checks if the value of the field is `MR_WR_SUBKGEN`"]
    #[inline]
    pub fn is_mr_wr_subkgen(&self) -> bool {
        *self == URATR::MR_WR_SUBKGEN
    }
    #[doc = "Checks if the value of the field is `WOR_RD_ACCESS`"]
    #[inline]
    pub fn is_wor_rd_access(&self) -> bool {
        *self == URATR::WOR_RD_ACCESS
    }
}
#[doc = r" Value of the field"]
pub struct TAGRDYR {
    bits: bool,
}
impl TAGRDYR {
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
    #[doc = "Bit 0 - Data Ready (cleared by setting bit START or bit SWRST in AES_CR or by reading AES_ODATARx)"]
    #[inline]
    pub fn datrdy(&self) -> DATRDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATRDYR { bits }
    }
    #[doc = "Bit 8 - Unspecified Register Access Detection Status (cleared by writing SWRST in AES_CR)"]
    #[inline]
    pub fn urad(&self) -> URADR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        URADR { bits }
    }
    #[doc = "Bits 12:15 - Unspecified Register Access (cleared by writing SWRST in AES_CR)"]
    #[inline]
    pub fn urat(&self) -> URATR {
        URATR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - GCM Tag Ready"]
    #[inline]
    pub fn tagrdy(&self) -> TAGRDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TAGRDYR { bits }
    }
}
