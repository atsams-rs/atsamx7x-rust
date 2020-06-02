#[doc = "Reader of register RSTC_SR"]
pub type R = crate::R<u32, super::RSTC_SR>;
#[doc = "Reader of field `URSTS`"]
pub type URSTS_R = crate::R<bool, bool>;
#[doc = "Reset Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RSTTYP_A {
    #[doc = "0: First power-up reset"]
    GENERAL_RST = 0,
    #[doc = "1: Return from Backup Mode"]
    BACKUP_RST = 1,
    #[doc = "2: Watchdog fault occurred"]
    WDT_RST = 2,
    #[doc = "3: Processor reset required by the software"]
    SOFT_RST = 3,
    #[doc = "4: NRST pin detected low"]
    USER_RST = 4,
}
impl From<RSTTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: RSTTYP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RSTTYP`"]
pub type RSTTYP_R = crate::R<u8, RSTTYP_A>;
impl RSTTYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RSTTYP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RSTTYP_A::GENERAL_RST),
            1 => Val(RSTTYP_A::BACKUP_RST),
            2 => Val(RSTTYP_A::WDT_RST),
            3 => Val(RSTTYP_A::SOFT_RST),
            4 => Val(RSTTYP_A::USER_RST),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GENERAL_RST`"]
    #[inline(always)]
    pub fn is_general_rst(&self) -> bool {
        *self == RSTTYP_A::GENERAL_RST
    }
    #[doc = "Checks if the value of the field is `BACKUP_RST`"]
    #[inline(always)]
    pub fn is_backup_rst(&self) -> bool {
        *self == RSTTYP_A::BACKUP_RST
    }
    #[doc = "Checks if the value of the field is `WDT_RST`"]
    #[inline(always)]
    pub fn is_wdt_rst(&self) -> bool {
        *self == RSTTYP_A::WDT_RST
    }
    #[doc = "Checks if the value of the field is `SOFT_RST`"]
    #[inline(always)]
    pub fn is_soft_rst(&self) -> bool {
        *self == RSTTYP_A::SOFT_RST
    }
    #[doc = "Checks if the value of the field is `USER_RST`"]
    #[inline(always)]
    pub fn is_user_rst(&self) -> bool {
        *self == RSTTYP_A::USER_RST
    }
}
#[doc = "Reader of field `NRSTL`"]
pub type NRSTL_R = crate::R<bool, bool>;
#[doc = "Reader of field `SRCMP`"]
pub type SRCMP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - User Reset Status"]
    #[inline(always)]
    pub fn ursts(&self) -> URSTS_R {
        URSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Reset Type"]
    #[inline(always)]
    pub fn rsttyp(&self) -> RSTTYP_R {
        RSTTYP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 16 - NRST Pin Level"]
    #[inline(always)]
    pub fn nrstl(&self) -> NRSTL_R {
        NRSTL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Software Reset Command in Progress"]
    #[inline(always)]
    pub fn srcmp(&self) -> SRCMP_R {
        SRCMP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
