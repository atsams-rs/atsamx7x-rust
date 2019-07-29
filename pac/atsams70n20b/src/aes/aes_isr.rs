#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AES_ISR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type DATRDY_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type URAD_R = crate::FR<bool, bool>;
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
}
impl crate::ToBits<u8> for URATR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            URATR::IDR_WR_PROCESSING => 0,
            URATR::ODR_RD_PROCESSING => 1,
            URATR::MR_WR_PROCESSING => 2,
            URATR::ODR_RD_SUBKGEN => 3,
            URATR::MR_WR_SUBKGEN => 4,
            URATR::WOR_RD_ACCESS => 5,
        }
    }
}
#[doc = r"Reader of the field"]
pub type URAT_R = crate::FR<u8, URATR>;
impl URAT_R {
    #[doc = "Checks if the value of the field is `IDR_WR_PROCESSING`"]
    #[inline(always)]
    pub fn is_idr_wr_processing(&self) -> bool {
        *self == URATR::IDR_WR_PROCESSING
    }
    #[doc = "Checks if the value of the field is `ODR_RD_PROCESSING`"]
    #[inline(always)]
    pub fn is_odr_rd_processing(&self) -> bool {
        *self == URATR::ODR_RD_PROCESSING
    }
    #[doc = "Checks if the value of the field is `MR_WR_PROCESSING`"]
    #[inline(always)]
    pub fn is_mr_wr_processing(&self) -> bool {
        *self == URATR::MR_WR_PROCESSING
    }
    #[doc = "Checks if the value of the field is `ODR_RD_SUBKGEN`"]
    #[inline(always)]
    pub fn is_odr_rd_subkgen(&self) -> bool {
        *self == URATR::ODR_RD_SUBKGEN
    }
    #[doc = "Checks if the value of the field is `MR_WR_SUBKGEN`"]
    #[inline(always)]
    pub fn is_mr_wr_subkgen(&self) -> bool {
        *self == URATR::MR_WR_SUBKGEN
    }
    #[doc = "Checks if the value of the field is `WOR_RD_ACCESS`"]
    #[inline(always)]
    pub fn is_wor_rd_access(&self) -> bool {
        *self == URATR::WOR_RD_ACCESS
    }
}
#[doc = r"Reader of the field"]
pub type TAGRDY_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Data Ready (cleared by setting bit START or bit SWRST in AES_CR or by reading AES_ODATARx)"]
    #[inline(always)]
    pub fn datrdy(&self) -> DATRDY_R {
        DATRDY_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 8 - Unspecified Register Access Detection Status (cleared by writing SWRST in AES_CR)"]
    #[inline(always)]
    pub fn urad(&self) -> URAD_R {
        URAD_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - Unspecified Register Access (cleared by writing SWRST in AES_CR)"]
    #[inline(always)]
    pub fn urat(&self) -> URAT_R {
        URAT_R::new(((self.bits() >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - GCM Tag Ready"]
    #[inline(always)]
    pub fn tagrdy(&self) -> TAGRDY_R {
        TAGRDY_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
}
