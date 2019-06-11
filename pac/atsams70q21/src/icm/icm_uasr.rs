#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ICM_UASR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `URAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum URATR {
    #[doc = "Unspecified structure member set to one detected when the descriptor is loaded."]
    UNSPEC_STRUCT_MEMBER,
    #[doc = "ICM_CFG modified during active monitoring."]
    ICM_CFG_MODIFIED,
    #[doc = "ICM_DSCR modified during active monitoring."]
    ICM_DSCR_MODIFIED,
    #[doc = "ICM_HASH modified during active monitoring"]
    ICM_HASH_MODIFIED,
    #[doc = "Write-only register read access"]
    READ_ACCESS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl URATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            URATR::UNSPEC_STRUCT_MEMBER => 0,
            URATR::ICM_CFG_MODIFIED => 1,
            URATR::ICM_DSCR_MODIFIED => 2,
            URATR::ICM_HASH_MODIFIED => 3,
            URATR::READ_ACCESS => 4,
            URATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> URATR {
        match value {
            0 => URATR::UNSPEC_STRUCT_MEMBER,
            1 => URATR::ICM_CFG_MODIFIED,
            2 => URATR::ICM_DSCR_MODIFIED,
            3 => URATR::ICM_HASH_MODIFIED,
            4 => URATR::READ_ACCESS,
            i => URATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNSPEC_STRUCT_MEMBER`"]
    #[inline]
    pub fn is_unspec_struct_member(&self) -> bool {
        *self == URATR::UNSPEC_STRUCT_MEMBER
    }
    #[doc = "Checks if the value of the field is `ICM_CFG_MODIFIED`"]
    #[inline]
    pub fn is_icm_cfg_modified(&self) -> bool {
        *self == URATR::ICM_CFG_MODIFIED
    }
    #[doc = "Checks if the value of the field is `ICM_DSCR_MODIFIED`"]
    #[inline]
    pub fn is_icm_dscr_modified(&self) -> bool {
        *self == URATR::ICM_DSCR_MODIFIED
    }
    #[doc = "Checks if the value of the field is `ICM_HASH_MODIFIED`"]
    #[inline]
    pub fn is_icm_hash_modified(&self) -> bool {
        *self == URATR::ICM_HASH_MODIFIED
    }
    #[doc = "Checks if the value of the field is `READ_ACCESS`"]
    #[inline]
    pub fn is_read_access(&self) -> bool {
        *self == URATR::READ_ACCESS
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Undefined Register Access Trace"]
    #[inline]
    pub fn urat(&self) -> URATR {
        URATR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
