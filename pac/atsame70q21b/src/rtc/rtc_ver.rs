#[doc = "Reader of register RTC_VER"]
pub type R = crate::R<u32, super::RTC_VER>;
#[doc = "Reader of field `NVTIM`"]
pub type NVTIM_R = crate::R<bool, bool>;
#[doc = "Reader of field `NVCAL`"]
pub type NVCAL_R = crate::R<bool, bool>;
#[doc = "Reader of field `NVTIMALR`"]
pub type NVTIMALR_R = crate::R<bool, bool>;
#[doc = "Reader of field `NVCALALR`"]
pub type NVCALALR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Non-valid Time"]
    #[inline(always)]
    pub fn nvtim(&self) -> NVTIM_R {
        NVTIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Non-valid Calendar"]
    #[inline(always)]
    pub fn nvcal(&self) -> NVCAL_R {
        NVCAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Non-valid Time Alarm"]
    #[inline(always)]
    pub fn nvtimalr(&self) -> NVTIMALR_R {
        NVTIMALR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Non-valid Calendar Alarm"]
    #[inline(always)]
    pub fn nvcalalr(&self) -> NVCALALR_R {
        NVCALALR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
