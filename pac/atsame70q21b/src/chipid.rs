#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip ID Register"]
    pub chipid_cidr: CHIPID_CIDR,
    #[doc = "0x04 - Chip ID Extension Register"]
    pub chipid_exid: CHIPID_EXID,
}
#[doc = "Chip ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chipid_cidr](chipid_cidr) module"]
pub type CHIPID_CIDR = crate::Reg<u32, _CHIPID_CIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIPID_CIDR;
#[doc = "`read()` method returns [chipid_cidr::R](chipid_cidr::R) reader structure"]
impl crate::Readable for CHIPID_CIDR {}
#[doc = "Chip ID Register"]
pub mod chipid_cidr;
#[doc = "Chip ID Extension Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chipid_exid](chipid_exid) module"]
pub type CHIPID_EXID = crate::Reg<u32, _CHIPID_EXID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHIPID_EXID;
#[doc = "`read()` method returns [chipid_exid::R](chipid_exid::R) reader structure"]
impl crate::Readable for CHIPID_EXID {}
#[doc = "Chip ID Extension Register"]
pub mod chipid_exid;
