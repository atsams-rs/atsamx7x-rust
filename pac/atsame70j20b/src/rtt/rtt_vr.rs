#[doc = "Reader of register RTT_VR"]
pub type R = crate::R<u32, super::RTT_VR>;
#[doc = "Reader of field `CRTV`"]
pub type CRTV_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current Real-time Value"]
    #[inline(always)]
    pub fn crtv(&self) -> CRTV_R {
        CRTV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
