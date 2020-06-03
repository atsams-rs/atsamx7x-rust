#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Purpose Backup Register 0"]
    pub sys_gpbr: [SYS_GPBR; 8],
}
#[doc = "General Purpose Backup Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_gpbr](sys_gpbr) module"]
pub type SYS_GPBR = crate::Reg<u32, _SYS_GPBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_GPBR;
#[doc = "`read()` method returns [sys_gpbr::R](sys_gpbr::R) reader structure"]
impl crate::Readable for SYS_GPBR {}
#[doc = "`write(|w| ..)` method takes [sys_gpbr::W](sys_gpbr::W) writer structure"]
impl crate::Writable for SYS_GPBR {}
#[doc = "General Purpose Backup Register 0"]
pub mod sys_gpbr;
