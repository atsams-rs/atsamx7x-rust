#[doc = "Reader of register MCAN_CREL"]
pub type R = crate::R<u32, super::MCAN_CREL>;
#[doc = "Reader of field `DAY`"]
pub type DAY_R = crate::R<u8, u8>;
#[doc = "Reader of field `MON`"]
pub type MON_R = crate::R<u8, u8>;
#[doc = "Reader of field `YEAR`"]
pub type YEAR_R = crate::R<u8, u8>;
#[doc = "Reader of field `SUBSTEP`"]
pub type SUBSTEP_R = crate::R<u8, u8>;
#[doc = "Reader of field `STEP`"]
pub type STEP_R = crate::R<u8, u8>;
#[doc = "Reader of field `REL`"]
pub type REL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Timestamp Day"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Timestamp Month"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Timestamp Year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Sub-step of Core Release"]
    #[inline(always)]
    pub fn substep(&self) -> SUBSTEP_R {
        SUBSTEP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Step of Core Release"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Core Release"]
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
