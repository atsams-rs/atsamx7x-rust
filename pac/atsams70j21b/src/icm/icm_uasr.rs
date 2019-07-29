#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ICM_UASR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
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
}
impl crate::ToBits<u8> for URATR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            URATR::UNSPEC_STRUCT_MEMBER => 0,
            URATR::ICM_CFG_MODIFIED => 1,
            URATR::ICM_DSCR_MODIFIED => 2,
            URATR::ICM_HASH_MODIFIED => 3,
            URATR::READ_ACCESS => 4,
        }
    }
}
#[doc = r"Reader of the field"]
pub type URAT_R = crate::FR<u8, URATR>;
impl URAT_R {
    #[doc = "Checks if the value of the field is `UNSPEC_STRUCT_MEMBER`"]
    #[inline(always)]
    pub fn is_unspec_struct_member(&self) -> bool {
        *self == URATR::UNSPEC_STRUCT_MEMBER
    }
    #[doc = "Checks if the value of the field is `ICM_CFG_MODIFIED`"]
    #[inline(always)]
    pub fn is_icm_cfg_modified(&self) -> bool {
        *self == URATR::ICM_CFG_MODIFIED
    }
    #[doc = "Checks if the value of the field is `ICM_DSCR_MODIFIED`"]
    #[inline(always)]
    pub fn is_icm_dscr_modified(&self) -> bool {
        *self == URATR::ICM_DSCR_MODIFIED
    }
    #[doc = "Checks if the value of the field is `ICM_HASH_MODIFIED`"]
    #[inline(always)]
    pub fn is_icm_hash_modified(&self) -> bool {
        *self == URATR::ICM_HASH_MODIFIED
    }
    #[doc = "Checks if the value of the field is `READ_ACCESS`"]
    #[inline(always)]
    pub fn is_read_access(&self) -> bool {
        *self == URATR::READ_ACCESS
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Undefined Register Access Trace"]
    #[inline(always)]
    pub fn urat(&self) -> URAT_R {
        URAT_R::new((self.bits() & 0x07) as u8)
    }
}
