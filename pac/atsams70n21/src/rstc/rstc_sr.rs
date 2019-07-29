#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RSTC_SR {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Reader of the field"]
pub type URSTS_R = crate::FR<bool, bool>;
#[doc = "Possible values of the field `RSTTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTTYPR {
    #[doc = "First power-up reset"]
    GENERAL_RST,
    #[doc = "Return from Backup Mode"]
    BACKUP_RST,
    #[doc = "Watchdog fault occurred"]
    WDT_RST,
    #[doc = "Processor reset required by the software"]
    SOFT_RST,
    #[doc = "NRST pin detected low"]
    USER_RST,
}
impl crate::ToBits<u8> for RSTTYPR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            RSTTYPR::GENERAL_RST => 0,
            RSTTYPR::BACKUP_RST => 1,
            RSTTYPR::WDT_RST => 2,
            RSTTYPR::SOFT_RST => 3,
            RSTTYPR::USER_RST => 4,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RSTTYP_R = crate::FR<u8, RSTTYPR>;
impl RSTTYP_R {
    #[doc = "Checks if the value of the field is `GENERAL_RST`"]
    #[inline(always)]
    pub fn is_general_rst(&self) -> bool {
        *self == RSTTYPR::GENERAL_RST
    }
    #[doc = "Checks if the value of the field is `BACKUP_RST`"]
    #[inline(always)]
    pub fn is_backup_rst(&self) -> bool {
        *self == RSTTYPR::BACKUP_RST
    }
    #[doc = "Checks if the value of the field is `WDT_RST`"]
    #[inline(always)]
    pub fn is_wdt_rst(&self) -> bool {
        *self == RSTTYPR::WDT_RST
    }
    #[doc = "Checks if the value of the field is `SOFT_RST`"]
    #[inline(always)]
    pub fn is_soft_rst(&self) -> bool {
        *self == RSTTYPR::SOFT_RST
    }
    #[doc = "Checks if the value of the field is `USER_RST`"]
    #[inline(always)]
    pub fn is_user_rst(&self) -> bool {
        *self == RSTTYPR::USER_RST
    }
}
#[doc = r"Reader of the field"]
pub type NRSTL_R = crate::FR<bool, bool>;
#[doc = r"Reader of the field"]
pub type SRCMP_R = crate::FR<bool, bool>;
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - User Reset Status"]
    #[inline(always)]
    pub fn ursts(&self) -> URSTS_R {
        URSTS_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Reset Type"]
    #[inline(always)]
    pub fn rsttyp(&self) -> RSTTYP_R {
        RSTTYP_R::new(((self.bits() >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 16 - NRST Pin Level"]
    #[inline(always)]
    pub fn nrstl(&self) -> NRSTL_R {
        NRSTL_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Software Reset Command in Progress"]
    #[inline(always)]
    pub fn srcmp(&self) -> SRCMP_R {
        SRCMP_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
}
